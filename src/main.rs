use std::env;
use std::fs;
use std::io::Read;
mod clip;
mod eval;

fn main() {
    // open file from args
    let mut args = env::args();
    args.next();
    let in_file_name = match args.next() {
        Some(filename) => filename,
        None => String::from("/dev/stdin"),
    };
    let mut in_file = fs::File::open(in_file_name).expect("Could not open file.");

    let mut s = String::new();
    in_file.read_to_string(&mut s).expect("Could not read file.");
    let b = &s.chars().collect::<Vec<char>>();

    // parse as clisp
    let clip = clip::Clip::from_slice(b).expect("Invalid clisp.");

    // print as clisp
    println!("clip: {}", clip);
    
    // generate stl
}
