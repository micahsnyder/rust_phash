# Rust-phash prototype

Prototype code to test using the `img_hash` crate to create pHash image hashes

To build, install the Rust toolchain: https://www.rust-lang.org/tools/install

Then run:
```bash
cargo build --release
```

Then to hash something, run:
```
./target/release/rust-phash /path/to/image.png
```

> _Disclaimer_: hash times will be very slow if you omit the `--release` flag
> and do a debug build instead.
