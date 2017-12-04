extern crate ansi_term;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;


fn main() {
    //  read file
    let mut f = File::open("data/input.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let unique_words = |line: &str| {
        let mut table = HashSet::new();
        for word in line.split_whitespace() {
            if table.contains(word) { return false; } else { table.insert(word); }
        };
        return true;
    };

    let count = contents.lines().filter(|line| unique_words(line)).count();
    println!("{}", count);
}
