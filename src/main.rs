use ft::{index, transform};
use std::env;
use std::fs::File;
use std::io::Read;
use svg2polylines::parse;

pub mod c128;
pub mod ft;

use c128::Complex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Format of call: fcnv <path>");
    }

    let mut f = File::open(&args[1]).expect("Could not open file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Could not read file");

    let mut plines = parse(&s).expect("Could not parse .svg file");

    if plines.len() == 0 {
        panic!("No lines found in file");
    }

    let line: Vec<Complex> = plines
        .swap_remove(0)
        .into_iter()
        .map(Complex::from)
        .collect();

    let c = transform(line, 10_000);
}
