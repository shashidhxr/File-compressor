extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
// use std::env::args;
use std::fs::File;
// use std::io::copy;
// use std::io::BufReader;
// use std::io::BufWriter;
use std::io::{BufReader, BufWriter, copy};
use std::time::Instant;
use std::env;
// use std::io::Error;

fn main(){
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: `source`, `target`");
        return;
    }

    let src_name = &args[1];
    let target_name = &args[1];

    let src = File::open(src_name).expect("Failed to open source file");
    let src_meta = src.metadata().expect("Failed to read source file");
    let src_size = src_meta.len();

    let mut input = BufReader::new(src);

    let target = File::create(target_name).expect("Failed to create target file");

    let mut encoder = GzEncoder::new(BufWriter::new(target), Compression::default());

    let start = Instant::now();
    copy(&mut input, &mut encoder).expect("Failed to compress data");
    encoder.finish().expect("Failed to finish compression");

    let target_size = File::open(target_name)
        .expect("Failed to open target file")
        .metadata()
        .expect("Failed to read target file metadata")
        .len();

    println!("Source length: {}", src_size);
    println!("Target length: {}", target_size);
    println!("Elapsed time: {:?}", start.elapsed());
    
}