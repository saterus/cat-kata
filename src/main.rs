use std::env::args;
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
static HELP_TEXT: &'static str = "usage: cat [-benstuv] [file ...]";

fn main() {
    let path = args().skip(1).take(1).last().expect("You must pass a path as the first argument.");
    let mut file = File::open(path).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    print!("{}", buffer);
}


