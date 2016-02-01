#![feature(test)]

extern crate test;
extern crate thaiengine;
extern crate radix_trie;

use test::Bencher;
use radix_trie::Trie;

fn load_file() -> Trie<String, u32> {
	let mut trie = Trie::new();
	thaiengine::load("SyllableDB-V1.dat".to_string(), &mut trie).unwrap();

	return trie;
}

#[bench]
fn bench_load_file(b: &mut Bencher){
	b.iter(|| load_file());
}

#[bench]
#[allow(unused_variables)]
fn bench_query(b: &mut Bencher){
	let trie = load_file();

	b.iter(|| {
		let child = trie.get_descendant(&String::from("สม")).unwrap();
		let mut count = 0;
		for item in child.iter() {
			count += 1;
		}
		return count;
	});
}
