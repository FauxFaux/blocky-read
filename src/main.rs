extern crate clap;
extern crate rand;

use std::fs;
use std::io;
use std::time;

use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use clap::App;
use clap::Arg;

use rand::Rng;

struct AFile {
    inner: fs::File,
    len: u64,
}

fn main() {
    let matches = App::new("blocky-read")
        .arg(Arg::with_name("FILE").required(true).multiple(true).help(
            "files to read from",
        ))
        .get_matches();

    let mut files: Vec<AFile> = matches
        .values_of("FILE")
        .unwrap()
        .map(|path| {
            let file = fs::File::open(path).expect("file must be readable");
            let len = file.metadata().expect("metadata").len();
            AFile {
                inner: file,
                len,
            }
        })
        .collect();

    let mut rng = rand::weak_rng();

    let mut buf = vec![0u8; 4096];

    let mut read_bytes = 0u64;

    let start = time::Instant::now();

    while start.elapsed().as_secs() < 20 {
        for _ in 0..50 {
            let a_file: &mut AFile = rng.choose_mut(&mut files).unwrap();
            let start = rng.gen_range(0, a_file.len);
            a_file.inner.seek(SeekFrom::Start(start)).expect("seek");
            match a_file.inner.read_exact(&mut buf) {
                Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => {},
                Ok(()) => { read_bytes += buf.len() as u64; },
                _ => panic!(),
            };
        }
    }

    let end = start.elapsed();

    println!("{} bytes in {}.{} seconds", read_bytes, end.as_secs(), end.subsec_nanos());
}
