extern crate stopwatch;

use std::io::{Write, BufReader, BufRead};
use std::net::TcpStream;
use stopwatch::{Stopwatch};

pub struct Record{
	pub id: u32,
	pub text: String,
}

const BIND: (&'static str, u16) = ("127.0.0.1", 5311);

fn chomp_eol(msg: &mut String){
	let len = msg.len();
	let mut suffix = 1;

	if msg.ends_with("\r\n") {
		suffix = 2;
	}

	msg.truncate(len - suffix);
}

fn read(reader: &mut BufReader<TcpStream>) -> String{
	let mut msg = String::new();
	reader.read_line(&mut msg).unwrap();

	chomp_eol(&mut msg);
	return msg;
}

fn add(reader: &mut BufReader<TcpStream>, id: u32, text: &String){
	{
		let socket = reader.get_mut();
		socket.write(format!("add {} {}\n", id, text).as_bytes()).unwrap();
	}
	read(reader);
}

fn search(reader: &mut BufReader<TcpStream>, keyword: &str) -> Vec<Record>{
	reader.get_mut().write(format!("search {}\n", keyword).as_bytes()).unwrap();
	let count = read(reader).parse::<usize>().unwrap();
	let mut out = Vec::with_capacity(count);

	for _ in 0..count {
		let text = read(reader);
		let item: Vec<&str> = text.split(" ").collect();
		out.push(Record{
			id: item[0].parse::<u32>().unwrap(),
			text: item[1..].join(" "),
		});
	}

	return out;
}

fn main(){
	println!("Connecting to {}:{}", BIND.0, BIND.1);
	let mut sw = Stopwatch::new();
	let socket = TcpStream::connect(BIND).unwrap();
	println!("Connected in {}ms", sw.elapsed_ms());
	let mut reader = BufReader::new(socket);

	let mut index = 0;
	for item in ["ทดสอบ", "ทดลอง"].iter() {
		print!("Adding #{} {}...", index, item);

		let text = String::from(*item);
		sw.restart();
		add(&mut reader, index, &text);
		println!(" {}ms", sw.elapsed_ms());
		index += 1;
	}

	println!("\n");

	let mut input = BufReader::new(std::io::stdin());
	let mut query = String::new();
	loop{
		print!("Search: ");
		std::io::stdout().flush().unwrap();
		input.read_line(&mut query).unwrap();
		chomp_eol(&mut query);

		println!("Searching for {}...", query);
		std::io::stdout().flush().unwrap();

		sw.restart();
		let result = search(&mut reader, &query);
		let time = sw.elapsed_ms();
		for item in &result {
			println!("#{}: {}", item.id, item.text);
		}
		println!("Found {} results in {}ms\n", result.len(), time);

		query.clear();
	}
}
