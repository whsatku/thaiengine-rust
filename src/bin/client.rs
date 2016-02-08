extern crate nanomsg;

use nanomsg::{Socket, Protocol};
use std::io::{Read, Write};

fn read(socket: &mut Socket) -> String{
	let mut msg = String::new();
	socket.read_to_string(&mut msg).unwrap();
	return msg;
}

fn main(){
	let mut socket = Socket::new(Protocol::Pair).unwrap();
	socket.connect("tcp://127.0.0.1:5560").unwrap();
	
	let mut index = 0;
	for item in vec!("ทดสอบ", "ทดลอง") {
		socket.write(format!("add {} {}", index, item).as_bytes()).unwrap();
		read(&mut socket);
		index += 1;
	}

	socket.write("search ทด".as_bytes()).unwrap();

	let count = read(&mut socket).parse::<i32>().unwrap();

	println!("Found {} results", count);

	for _ in 0..count {
		println!("{}", read(&mut socket));
	}
}