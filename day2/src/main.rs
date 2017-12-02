extern crate ansi_term;

use std::cmp;
use std::fs::File;
use std::io::prelude::*;

fn main()
{
    //  read file
    let mut f = File::open("data/input.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let minmax = |v: &Vec<i32>| -> (i32, i32) {
        v.iter().fold((i32::max_value(), i32::min_value()), |x, i| (cmp::min(x.0, *i), cmp::max(x.1, *i)))
    };

    let mat: Vec<Vec<i32>> = contents.lines().map(
        |line| line.split_whitespace().map(
        |word| word.parse().unwrap())
    .collect()).collect();

    println!("Spreadsheet: \n{:?}", mat);
    println!();

    let sum:i32 = mat.iter().map(|row| { let mm = minmax( row ); mm.1 - mm.0 }).sum();
    println!("Checksum: {}", sum);
}
