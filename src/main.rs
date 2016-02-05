extern crate encoding;
extern crate radix_trie;
#[cfg(feature="interactive")]
extern crate copperline;

mod model;

use std::env;
use std::process;
use std::thread::sleep;
use std::time::Duration;
use radix_trie::Trie;
#[cfg(feature="interactive")]
use copperline::Copperline;

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

fn usage(){
	let exe_name = env::args().nth(0).unwrap();
	println!("Usage: {0} filename.dat", exe_name);
}

#[cfg(feature="interactive")]
fn get_args_fn() -> String{
	let mut copperline = Copperline::new();
	match env::args().nth(1) {
		Some(file) => file,
		None => {
			match copperline.read_line_utf8("Input file to read: ") {
				Ok(filename) => filename,
				Err(_) => {
					usage();
					process::exit(1);
				}
			}
		},
	}
}

#[cfg(not(feature="interactive"))]
fn get_args_fn() -> String{
	match env::args().nth(1) {
		Some(file) => file,
		None => {
			usage();
			process::exit(1);
		},
	}
}

fn search(trie: &Trie<String, u32>, query: &String){
	let child = trie.get_descendant(&query);

	if child.is_none() {
		print_err!("Search found 0 item\n\n");
		return;
	}

	let child = child.unwrap();
	let mut count = 0;
	for item in child.iter() {
		count += 1;
		if cfg!(feature="dump_data") || cfg!(feature="interactive") {
			println!("#{} {} -> {}", count, item.0, item.1);
		}
	}
	print_err!("Search found {} item\n\n", count);
}

#[cfg(feature="interactive")]
fn interactive(trie: &Trie<String, u32>){
	let mut copperline = Copperline::new();
	loop {
		match copperline.read_line_utf8("Search: ") {
			Ok(query) => {
				copperline.add_history(query.clone());
				search(&trie, &query);
			},
			Err(_) => break
		};
	}
}
#[cfg(not(feature="interactive"))]
#[allow(unused_variables)]
#[inline]
fn interactive(trie: &Trie<String, u32>){}

fn main(){
	let file = get_args_fn();
	let mut trie = Trie::<String, u32>::new();

	if model::load(file, &mut trie).is_err() {
		println!("Cannot read input file");
		process::exit(1);
	}
	print_err!("Input file loaded\n");

	if cfg!(feature="interactive") {
		interactive(&trie);
	}else{
		search(&trie, &String::from("สม"));
		if cfg!(feature="wait_on_exit") {
			println!("Run finished");
			sleep(Duration::from_secs(10000));
		}
	}
}
