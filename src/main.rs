#[macro_use]
extern crate log;
extern crate encoding;
extern crate radix_trie;
extern crate env_logger;

mod model;
mod server;

use std::env;
use std::process;
use std::sync::{Arc, RwLock};
use std::thread;
use radix_trie::Trie;
use log::{LogRecord, LogLevelFilter};
use env_logger::LogBuilder;

const BIND: (&'static str, u16) = ("0.0.0.0", 5311);

fn load(lock: &RwLock<Trie<String, u32>>){
	let file = match env::args().nth(1) {
		Some(file) => file,
		None => {
			return;
		},
	};
	info!(target: "loader", "Loading data from file {}", file);
	let ref mut trie = *lock.write().unwrap();
	if model::load(&file, trie).is_err() {
		error!(target: "loader", "Cannot read file {}", file);
		process::exit(1);
	}
	info!(target: "loader", "File {} loaded. {} entires found", file, trie.len());
}

#[cfg(feature="color")]
fn log_format(record: &LogRecord) -> String{
	const RESET: &'static str = "\x1b[0m";

	let color = match record.level() {
		log::LogLevel::Error => "\x1b[37;41;1m",
		log::LogLevel::Warn => "\x1b[33;1m",
		log::LogLevel::Info => "\x1b[46;1m",
		log::LogLevel::Debug => "\x1b[44;37;1m",
		log::LogLevel::Trace => "\x1b[40;1m",
	};

	format!("{}[{}]{} {}", color, record.target(), RESET, record.args())
}

#[cfg(not(feature="color"))]
fn log_format(record: &LogRecord) -> String{
	format!("[{}, {}] {}", record.level(), record.target(), record.args())
}

fn build_logger(){
    let mut builder = LogBuilder::new();
    builder.format(log_format).filter(None, LogLevelFilter::Info);

    if env::var("RUST_LOG").is_ok() {
       builder.parse(&env::var("RUST_LOG").unwrap());
    }

    builder.init().unwrap();
}

fn main(){
	build_logger();

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

	server::start(&BIND, arc.clone());
}
