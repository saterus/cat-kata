static HELP_TEXT: &'static str = "usage: cat [-benstuv] [file ...]";

fn main() {
    print_usage();
}

fn print_usage() {
    println!("{}", HELP_TEXT);
}
