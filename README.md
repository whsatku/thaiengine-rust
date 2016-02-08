# ThaiEngine

Project from Software Pattern class.

## Running server

(Make sure you have [Rust](https://www.rust-lang.org/downloads.html) installed)

```
$ cargo run --bin thaiengine --release SyllableDB-V1.dat
```

(Syllable database can be obtained from the class Facebook group)

Optional features:

- `dump_data`: Print data while loading/searching
- `color` (default): Use colored log. Disable if your platform have problem rendering it
- Set `RUST_LOG=debug` to view more logs

To enable features:

```
$ cargo run --features="dump_data" --bin thaiengine --release SyllableDB-V1.dat
```

**Note**: Running without `--release` will use another code path with assertions, resulting in slower program.

## Running client

```
$ cargo run --bin client --release
```
