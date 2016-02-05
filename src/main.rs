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

const ITER_COUNT: i64 = 100;
const TOP_NO: usize = 30;
const LOOP_UPDATE_EVERY: f32 = 71.0;

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

fn get_args_fn() -> String{
	match env::args().nth(1) {
		Some(file) => file,
		None => {
			usage();
			process::exit(1);
		},
	}
}

fn search(trie: &Trie<String, u32>, query: &String) -> u32{
	let child = trie.get_descendant(&query).unwrap();
	let mut count = 0;
	for item in child.iter() {
		count += 1;
	}
	return count;
}

fn main(){
	let file = get_args_fn();
	let mut trie = Trie::<String, u32>::new();

	if model::load(file, &mut trie).is_err() {
		println!("Cannot read input file");
		process::exit(1);
	}
	print_err!("Input file loaded\n");

	let mut stopwatch = Stopwatch::new();
	let total = trie.len() as f32;
	let mut done = 0f32;
	let mut data = Vec::<(&String, i64)>::new();

	for key in trie.keys() {
		done += 1.0;

		if done % LOOP_UPDATE_EVERY == 0.0 {
			print!("Processing {} of {} ({:.2}%)\r", done, total, (done/total)*100.0);
		}

		let mut total_time = 0;
		for i in 0..ITER_COUNT {
			stopwatch.restart();
			search(&trie, key);
			total_time += stopwatch.elapsed_ms();
		}

		data.push((key, total_time));
	}
	println!("\n\nDone! Processing data...");

	data.sort_by_key(|i| i.1);

	println!("Top {} slowest elements:", TOP_NO);
	for item in data.iter().rev().take(TOP_NO) {
		println!("{}\t\t{}ms", item.0, item.1);
	}

	if cfg!(feature="wait_on_exit") {
		sleep(Duration::from_secs(10000));
	}
}
