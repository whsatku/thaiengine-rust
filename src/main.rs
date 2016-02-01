extern crate encoding;
extern crate radix_trie;
extern crate stopwatch;

mod model;

use std::env;
use std::process;
use std::thread::sleep;
use std::time::Duration;
use radix_trie::Trie;
use stopwatch::{Stopwatch};

const DEBUG: bool = false;

macro_rules! print_err {
	($($arg:tt)*) => (
		{
			if DEBUG {
				use std::io::prelude::*;
				if let Err(e) = write!(&mut ::std::io::stderr(), "{}\n", format_args!($($arg)*)) {
					panic!("Failed to write to stderr.\
						\nOriginal error output: {}\
						\nSecondary error writing to stderr: {}", format!($($arg)*), e);
				}
				::std::io::stderr().flush().unwrap()
			}
		}
	)
}

fn load(filename: String, trie: &mut Trie<String, u32>) -> Result<(), String>{
	let mut fp = try!(model::DataFile::open(filename).map_err(|e| e.to_string()));
	let mut last_id = 0;
	let mut trie_elapsed = 0i64;

	while fp.has_next(){
		let record = fp.record();
		if DEBUG && record.id != last_id+1 {
			return Err("ID not continuous".to_owned());
		}

		// println!("id {} text {}", record.id, record.text);
		let sw = Stopwatch::start_new();
		trie.insert(record.text, record.id);
		trie_elapsed += sw.elapsed_ms();
		last_id = record.id;
	}

	print_err!("Trie insertion took {}ms", trie_elapsed);

	Ok(())
}

fn get_args_fn() -> String{
	match env::args().nth(1) {
		Some(file) => file,
		None => {
			let exe_name = env::args().nth(0).unwrap();
			println!("Usage: {0} filename.dat", exe_name);
			process::exit(1);
		},
	}
}

fn main(){
	let file = get_args_fn();
	let mut trie = Trie::<String, u32>::new();

	let sw = Stopwatch::start_new();
	if load(file, &mut trie).is_err() {
		println!("Cannot read input file");
	}
	print_err!("Read take {}ms", sw.elapsed_ms());


	let sw = Stopwatch::start_new();
	let child = trie.get_descendant(&String::from("สม")).unwrap();
	let mut count = 0;
	for item in child.iter() {
		count += 1;
		// println!("Key {} Value {}", item.0, item.1);
	}
	print_err!("Search over {} items take {}ms", count, sw.elapsed_ms());
	println!("Run finished");
	sleep(Duration::from_secs(10000));
}
