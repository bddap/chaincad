use std::env;
use std::fs::File;
mod parse;

fn main() {
    // open file from args
    let mut args = env::args();
    let in_file_name = match args.next() {
        Some(filename) => filename,
        None => String::from("/dev/stdin")
    };
    let out_file_name = match args.next() {
        Some(filename) => filename,
        None => String::from("/dev/stdout")
    };
    let in_file = File::open(in_file_name).unwrap();
    let out_file = File::create(out_file_name).unwrap();
    
    // parse as clisp
    // let clip = {
    //   let s = String::new();
    //   in_file.readAll(s);
    //   clispParse(s)
    // }
    
    // print as clisp
    // generate stl
    eprintln!("Hello, world!");
}
