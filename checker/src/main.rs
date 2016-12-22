extern crate regex;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use regex::Regex;

fn main() {
    let satis = Regex::new(".*\x2E.*is satisfiable.").unwrap(); 
    let notsatis = Regex::new(".*\x2E.*is not satisfiable.").unwrap();
    let f = match File::open("output.txt") {
        Ok(file) => file,
        Err(e) => panic!("Having problem opening output.txt with error: {}", e)
    };
    let file = BufReader::new(&f);
    let mut n = 0;
    for line in file.lines() {
        let line_unwrapped = line.unwrap();
        let line_str = line_unwrapped.as_str();
        if n % 2 == 0 && !notsatis.is_match(line_str) || n % 2 == 1 && !satis.is_match(line_str) {
            println!("Something went wrong with formula number: {}", n);
        }
        n += 1;
    }
}
