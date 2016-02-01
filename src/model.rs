#![allow(unused_must_use)]
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::SeekFrom;
use std::io::Seek;
use std::io::Read;
use std::io::BufRead;
use std::mem;
use std::thread;
use std::sync::mpsc::channel;
use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_874;
use radix_trie::Trie;

const HEADER_SIZE: u64 = 256;

pub struct DataFile{
	reader: BufReader<File>,
}

pub struct Record{
	pub id: u32,
	pub text: String,
}

impl DataFile{
	pub fn open(path: String) -> Result<DataFile, io::Error>{
		let mut fp = try!(File::open(path));

		try!(fp.seek(SeekFrom::Start(HEADER_SIZE)));
		let reader = BufReader::new(fp);

		Ok(DataFile{
			reader: reader,
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
		self.reader.seek(SeekFrom::Current(11));

		let mut buffer = [0u8; 1];
		self.reader.read(&mut buffer);
		return unsafe{mem::transmute_copy(&buffer)};
	}

	fn read_text(&mut self) -> String{
		let has_tail = self.read_tail_space();
		self.reader.seek(SeekFrom::Current(8));

		if has_tail {
			let mut buffer = [0u8; 1023];
			self.reader.read(&mut buffer);
			return WINDOWS_874.decode(&buffer, DecoderTrap::Ignore).unwrap();
		}else{
			let mut buffer = Vec::new();
			self.reader.read_until(0u8, &mut buffer);
			return WINDOWS_874.decode(&buffer, DecoderTrap::Ignore).unwrap();
		}
	}
}

pub fn load(filename: String, trie: &mut Trie<String, u32>) -> Result<(), String>{
	let (tx, rx) = channel::<Result<Record, String>>();
	thread::spawn(move || {
		let fp = DataFile::open(filename);

		if fp.is_err() {
			tx.send(Err(fp.err().unwrap().to_string())).unwrap();
			return;
		}

		let mut fp = fp.unwrap();
		let mut last_id = 0;

		while fp.has_next() {
			let record = fp.record();
			// if cfg!(feature="assertion") && record.id != last_id+1 {
			// 	tx.send(Err("ID not continuous".to_owned()));
			// 	return;
			// }

			if cfg!(feature="dump_data") {
				println!("id {} text {}", record.id, record.text);
			}

			tx.send(Ok(fp.record())).unwrap();
			last_id = record.id;
		}
	});

	loop{
		let record = rx.recv();
		if record.is_err() {
			// channel disconnection
			break;
		}

		let record = try!(record.unwrap());

		trie.insert(record.text, record.id);
	}

	Ok(())
}
