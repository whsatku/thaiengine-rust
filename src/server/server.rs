extern crate nanomsg;

use std::io::{Read, Write};
use std::sync::{Arc, RwLock};
use radix_trie::Trie;
use nanomsg::{Socket, Protocol, Error};
use model;

const CMD_SEARCH: &'static str = "search";
const CMD_ADD: &'static str = "add";
const RESP_OK: &'static [u8] = b"ok";
const RESP_ERR: &'static [u8] = b"err";

pub fn start(bind: &str, lock: Arc<RwLock<Trie<String, u32>>>) -> Option<Error>{
	let mut socket = match Socket::new(Protocol::Pair) {
		Ok(socket) => socket,
		Err(err) => {
			return Some(err);
		}
	};

	match socket.bind(bind){
		Ok(_) => {},
		Err(err) => {
			return Some(err);
		}
	}

	let mut msg = String::new();
	loop{
		msg.clear();
		match socket.read_to_string(&mut msg) {
			Ok(_) => {},
			Err(_) => {
				continue;
			}
		}
		
		let tokens: Vec<&str> = msg.split(" ").collect();

		if tokens.len() == 0 {
			continue;
		}

		if tokens[0] == CMD_SEARCH {
			let query = &tokens[1..].join(" ");
			println!("search {}", query);

			let ref trie = lock.read().unwrap();
			let result = model::search(trie, query);

			println!("search returned {} results", result.len());

			write!(socket, "{}", result.len()).unwrap();

			for item in result {
				// write! send fragmented packets
				match socket.write(format!("{} {}", item.1, item.0).as_bytes()){
					Ok(_) => {}
					Err(e) => {
						println!("write error: {}", e.to_string());
					}
				}
			}
		}else if tokens[0] == CMD_ADD {
			if tokens.len() < 3 {
				match socket.write(RESP_ERR){
					Ok(_) => {}
					Err(e) => {
						println!("nb_write error: {}", e.to_string());
					}
				}
				continue;
			}
			let index = tokens[1].parse::<u32>().unwrap();
			let query = String::from(tokens[2..].join(" "));
			println!("add {} {}", index, query);

			let ref mut trie = lock.write().unwrap();
			trie.insert(query, index);

			println!("db size {}", trie.len());

			match socket.write(RESP_OK){
				Ok(_) => {}
				Err(e) => {
					println!("nb_write error: {}", e.to_string());
				}
			}
		}
	}
}