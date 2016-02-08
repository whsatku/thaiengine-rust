extern crate encoding;
extern crate radix_trie;
extern crate nanomsg;

mod model;
mod server;

use std::env;
use std::process;
use std::sync::{Arc, RwLock};
use std::thread;
use radix_trie::Trie;

macro_rules! print_err {
	($($arg:tt)*) => (
		{
			if cfg!(feature="assertion") {
				use std::io::prelude::*;
				if let Err(e) = write!(&mut ::std::io::stderr(), "{}", format_args!($($arg)*)) {
					panic!("Failed to write to stderr.\
						\nOriginal error output: {}\
						\nSecondary error writing to stderr: {}", format!($($arg)*), e);
				}
				::std::io::stderr().flush().unwrap()
			}
		}
	)
}

fn load(lock: &RwLock<Trie<String, u32>>){
	let file = match env::args().nth(1) {
		Some(file) => file,
		None => {
			return;
		},
	};
	let ref mut trie = *lock.write().unwrap();
	if model::load(file, trie).is_err() {
		println!("Cannot read input file");
		process::exit(1);
	}
	print_err!("Input file loaded\n");
}

fn main(){
	let trie = Trie::<String, u32>::new();
	let lock = RwLock::new(trie);
	let arc = Arc::new(lock);

	{
		let load_lock = arc.clone();
		thread::spawn(move || {
			let ref lock = *load_lock;
			load(&lock);
		});
	}

	server::start("tcp://127.0.0.1:5560", arc.clone());
}
