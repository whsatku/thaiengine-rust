extern crate encoding;

mod model;

use std::env;

fn main(){
	let file = env::args().nth(1);
	if file.is_none(){
		let exe_name = env::args().nth(0).unwrap();
		println!("Usage: {0} filename.dat", exe_name);
		return;
	}

	let file = file.unwrap();

	let fp = model::DataFile::open(file);
	if !fp.is_ok() {
		println!("Cannot read input file");
		return;
	}

	let mut fp = fp.unwrap();
	let mut last_id = 0;

	while fp.has_next(){
		let record = fp.record();
		if record.id != last_id+1 {
			println!("Error ID not continuous {} vs {}", record.id, last_id);
			return;
		}

		println!("id {} text {}", record.id, record.text);
		last_id = record.id;
	}
}
