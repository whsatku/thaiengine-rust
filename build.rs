extern crate gcc;

fn main() {
    gcc::compile_library("libcreader.a", &["src/creader.c"]);
}
