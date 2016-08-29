use std::io::{self, Read};
use std::path::Path;
use std::fs::File;
extern crate regex;
use regex::Regex;

fn main() {
    let path = Path::new("./.agignore");
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let files = buffer.split("\n").filter(|file| file.to_string() != "");

    if path.is_file() {
        let mut file = File::open(&path).unwrap();
        let mut agignore = String::new();
        file.read_to_string(&mut agignore).unwrap();
        for file in files {
            let mut ignore_patterns = agignore.split("\n").filter(|pat| pat.to_string() != "");
            if !ignore_patterns.any(|pat| {
                Regex::new(pat).unwrap().is_match(file)
            }) {
                println!("{}", file);
            }
        }
    } else {
        for file in files {
            println!("{}", file);
        }
    }
}
