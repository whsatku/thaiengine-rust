use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufReader, BufRead, Error};
use std::sync::{Arc, RwLock};
use std::thread;
use radix_trie::Trie;
use model;

const CMD_SEARCH: &'static str = "search";
const CMD_ADD: &'static str = "add";
const RESP_OK: &'static [u8] = b"ok\n";
const RESP_ERR: &'static [u8] = b"err\n";

macro_rules! write_or_err {
	($socket:expr, $text:expr) => {
		match $socket.write($text){
			Ok(_) => {}
			Err(e) => {
				let peer = $socket.peer_addr().unwrap();
				warn!(target: "server", "client {} write error {}", peer, e.to_string());
			}
		}
	}
}

fn process_msg(socket: &mut TcpStream, message: &String, lock: &RwLock<Trie<String, u32>>){
	let tokens: Vec<&str> = message.split(" ").collect();
	let peer = socket.peer_addr().unwrap();

	match tokens[0] {
		CMD_SEARCH => {
			if tokens.len() < 2 {
				write_or_err!(socket, RESP_ERR);
				return;
			}

			let query = &tokens[1..].join(" ");
			debug!(target: "process_msg", "client {} search {}", peer, query);

			let ref trie = lock.read().unwrap();
			let result = model::search(trie, query);

			debug!(target: "process_msg", "client {} search returned {} results", peer, result.len());

			write_or_err!(socket, format!("{}\n", result.len()).as_bytes());

			for item in result {
				write_or_err!(socket, format!("{} {}\n", item.1, item.0).as_bytes());
			}
		},
		CMD_ADD => {
			if tokens.len() < 3 {
				write_or_err!(socket, RESP_ERR);
				return;
			}
			let index = tokens[1].parse::<u32>().unwrap();
			let query = String::from(tokens[2..].join(" "));
			debug!(target: "process_msg", "client {} add {} {}", peer, index, query);

			let mut len = 0;
			{
				let ref mut trie = lock.write().unwrap();
				trie.insert(query, index);
				len = trie.len();
			}

			debug!(target: "process_msg", "trie size is {}", len);
			write_or_err!(socket, RESP_OK);
		}
		_ => {
			write_or_err!(socket, RESP_ERR);
		}
	}
}

fn handle_client(stream: TcpStream, lock: Arc<RwLock<Trie<String, u32>>>){
	let peer = stream.peer_addr().unwrap();
	let mut reader = BufReader::new(stream);
	let ref lock = *lock;

	let mut buffer = String::new();
	loop{
		buffer.clear();

		match reader.read_line(&mut buffer) {
			Ok(size) => {
				if size == 0 {
					info!(target: "server", "client {} disconnected", peer);
					break;
				}
			}
			Err(e) => {
				error!(target: "server", "client {} error: {}", peer, e.to_string());
				break;
			}
		}

		// chomp end of line
		let len = buffer.len();
		let mut suffix = 1;

		if buffer.ends_with("\r\n") {
			suffix = 2;
		}

		buffer.truncate(len - suffix);

		process_msg(reader.get_mut(), &buffer, &lock);
	}
}

pub fn start(bind: &str, lock: Arc<RwLock<Trie<String, u32>>>) -> Option<Error>{
	let listener = match TcpListener::bind(bind){
		Ok(socket) => socket,
		Err(err) => {
			return Some(err);
		}
	};

	for stream in listener.incoming() {
	    match stream {
	        Ok(stream) => {
				info!(target: "server", "client {} connect", stream.peer_addr().unwrap());
				let thread_lock = lock.clone();
	            thread::spawn(move || {
	                handle_client(stream, thread_lock);
	            });
	        }
	        Err(e) => {
				error!(target:"server", "client connect error {}", e.to_string());
			}
	    }
	}

	None
}
