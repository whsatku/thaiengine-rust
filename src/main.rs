extern crate encoding;
extern crate radix_trie;

mod model;

use std::env;
use std::process;
use std::thread::sleep;
use std::time::Duration;
use radix_trie::Trie;

macro_rules! print_err {
	($($arg:tt)*) => (
		{
			if cfg!(feature="assertion") {
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

	let load = model::load(file, &mut trie);
	if load.is_err() {
		println!("Cannot read input file: {}", load.err().unwrap().to_string());
		return;
	}
	print_err!("Input file loaded");


	let child = trie.get_descendant(&String::from("สม")).unwrap();
	let mut count = 0;
	for item in child.iter() {
		count += 1;
		if cfg!(feature="dump_data") {
			println!("Key {} Value {}", item.0, item.1);
		}
	}
	print_err!("Search found {} items", count);

	if cfg!(feature="wait_on_exit") {
		println!("Run finished");
		sleep(Duration::from_secs(10000));
	}
}
