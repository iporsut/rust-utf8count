use std::io::prelude::*;
use std::env;
use std::fs::File;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let mut f = File::open(filename).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    println!("{}", s.trim_right().chars().count());
}
