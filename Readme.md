# Rust-phash prototype

Prototype code to test using the `img_hash` crate to create pHash image hashes

To build, install the Rust toolchain: https://www.rust-lang.org/tools/install

Clone the `img_hash` repo and check out my `hash-alg-median` branch:
```bash
cd ..
git clone git@github.com:micahsnyder/img_hash.git
git checkout hash-alg-median
```
> _Tip_: A clone of `img_hash` next to the `rust_phash` repo is needed because my Cargo.toml has a relative path to use a local copy of `img_hash`.  This is so I can test changes to `img_hash`. 

Then run:
```bash
cd rust_phash
cargo build --release
```

Then to hash something, run:
```
./target/release/rust-phash /path/to/image.png
```

> _Disclaimer_: hash times will be very slow if you omit the `--release` flag
> and do a debug build instead.
