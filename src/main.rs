use std::fmt::Write;

use clap::{AppSettings, Clap};
use anyhow;

extern crate image;
extern crate img_hash;

/// rust-phash prototype
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Micah Snyder <micasnyd@cisco.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Input image file.
    input: String,
}

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();

    let my_image = image::open(&opts.input).unwrap();

    // configure the hasher
    let hasher = img_hash::HasherConfig::new()
        .hash_size(8, 8)
        .preproc_dct()
        .hash_alg(img_hash::HashAlg::Mean)
        .to_hasher();

    // calculate the hash, which gives us an ImageHash structure
    let hash = hasher.hash_image(&my_image);

    // convert that to an array of bytes
    let hash_bytes = hash.as_bytes();

    // and convert that to a hex string
    let mut hash_bytes_hex = String::with_capacity(2 * hash_bytes.len());
    for byte in hash_bytes {
        write!(hash_bytes_hex, "{:02X}", byte)?;
    }

    println!("Hash: {}", hash_bytes_hex);

    Ok(())

    // `file` goes out of scope, and the "hello.txt" file gets closed
}
