# ThaiEngine

Project from Software Pattern class.

## Running

(Make sure you have [Rust](https://www.rust-lang.org/downloads.html) installed)

```
$ cargo run --release SyllableDB-V1.dat
```

(Syllable database can be obtained from the class Facebook group)

Optional features:

- `assertion`: Show time used to perform certain operations
- `dump_data`: Print data while loading/searching
- `wait_on_exit`: Don't exit, instead wait for long time
- `interactive`: Interactive build: ask for file to read, ask for search queries

To enable features:

```
$ cargo run --features="assertion dump_data" SyllableDB-V1.dat
```
