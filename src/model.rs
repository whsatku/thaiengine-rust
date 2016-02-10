#![allow(dead_code)]
#![allow(unused_must_use)]
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::SeekFrom;
use std::io::Seek;
use std::io::Read;
use std::io::BufRead;
use std::mem;
use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_874;
use radix_trie::Trie;

const HEADER_SIZE: u64 = 256;

pub struct DataFile{
	reader: BufReader<File>,
	is_64bit: Option<bool>
}

pub struct Record{
	pub id: u32,
	pub text: String,
}

impl DataFile{
	pub fn open(path: &String) -> Result<DataFile, io::Error>{
		let mut fp = try!(File::open(path));

		try!(fp.seek(SeekFrom::Start(HEADER_SIZE)));
		let reader = BufReader::new(fp);

		Ok(DataFile{
			reader: reader,
			is_64bit: None,
		})
	}

	pub fn has_next(&mut self) -> bool{
		let mut buffer = [0u8; 1];
		let result = self.reader.read(&mut buffer);

		let rt = result.is_ok() && result.ok().unwrap() > 0;
		self.reader.seek(SeekFrom::Current(-1));
		return rt;
	}

	pub fn record(&mut self) -> Record{
		let id = self.read_id();
		let text = self.read_text();

		Record{
			id: id,
			text: text
		}
	}

	fn read_id(&mut self) -> u32{
		let mut buffer = [0u8; 4];
		self.reader.read(&mut buffer);

		return unsafe{mem::transmute_copy(&buffer)};
	}

	fn read_tail_space(&mut self) -> bool{
		// metadata is a 4 byte struct
		// with 2 last bytes are padding
		// we only interested in the second byte

		self.reader.seek(SeekFrom::Current(1));
		let mut buffer = [0u8; 1];
		self.reader.read(&mut buffer);
		self.reader.seek(SeekFrom::Current(4+2));
		return (buffer[0] & 1<<3) >> 3 == 1;
	}

	fn read_text(&mut self) -> String{
		let has_tail = self.read_tail_space();

		match self.is_64bit {
			None => {
				// detect for 64 bit time_t
				// read 8 bit time_t + 4 bit padding
				let mut buffer = [0u8; 8+4];
				self.reader.read(&mut buffer);

				if buffer.ends_with(&[0, 0, 0, 0]) {
					self.is_64bit = Some(true);
				}else{
					self.reader.seek(SeekFrom::Current(-4));
					self.is_64bit = Some(false);
				}
			}
			Some(true) => {
				self.reader.seek(SeekFrom::Current(8+4));
			}
			Some(false) => {
				self.reader.seek(SeekFrom::Current(4));
			}
		}

		if has_tail {
			let mut buffer = [0u8; 1023];
			self.reader.read(&mut buffer);

			let mut out = WINDOWS_874.decode(&buffer, DecoderTrap::Ignore).unwrap();

			// remove trailing \0
			if cfg!(debug_assertion) {
				assert_eq!(out.pop(), Some('\0'));
			}else{
				let size = buffer.len();
				out.truncate(size - 1);
			}

			return out;
		}else{
			let mut buffer = Vec::new();
			self.reader.read_until(0u8, &mut buffer);

			// remove trailing \0
			if cfg!(debug_assertion) {
				assert_eq!(buffer.pop(), Some(0));
			}else{
				let size = buffer.len();
				buffer.resize(size - 1, 0);
			}

			return WINDOWS_874.decode(&buffer, DecoderTrap::Ignore).unwrap();
		}
	}
}

pub fn load(filename: &String, trie: &mut Trie<String, u32>) -> Result<(), String>{
	let mut fp = try!(DataFile::open(filename).map_err(|e| e.to_string()));
	let mut last_id = 0;

	while fp.has_next(){
		let record = fp.record();
		if cfg!(debug_assertion) && record.id != last_id+1 {
			return Err("ID not continuous".to_owned());
		}

		if cfg!(feature="dump_data") {
			println!("id {} text {}", record.id, record.text);
		}
		trie.insert(record.text, record.id);
		last_id = record.id;
	}

	Ok(())
}

pub fn search<'a>(trie: &'a Trie<String, u32>, query: &String) -> Vec<(&'a String, &'a u32)>{
	let child = trie.get_descendant(&query);

	if child.is_none() {
		return Vec::new();
	}

	let child = child.unwrap();
	let mut out = Vec::new();
	for item in child.iter() {
		out.push(item);
		if cfg!(feature="dump_data") {
			println!("{} -> {}", item.0, item.1);
		}
	}

	return out;
}
