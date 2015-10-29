use std::io::stdin;
use std::env::args;
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
static HELP_TEXT: &'static str = "usage: cat [-benstuv] [file ...]";

fn main() {
    let mut files: Vec<Box<Read>> = args().skip(1)
                                          .map(|path| Box::new(File::open(path).unwrap()) as Box<Read>)
                                          .collect();

    if files.is_empty() {
        files.push(Box::new(stdin()) as Box<Read>);
    }

    for mut file in files.into_iter() {
        read_contents(&mut file);
    }
}

fn read_contents<R: Read>(reader: &mut R) {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();
    print!("{}", buffer);
}

