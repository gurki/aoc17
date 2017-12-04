extern crate num;

use std::cmp;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    //  read file
    let mut f = File::open("data/input.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let mat: Vec<Vec<i32>> = contents.lines().map(
        |line| line.split_whitespace().map(
        |word| word.parse().unwrap())
    .collect()).collect();

    let evendiv = |v: &Vec<i32>| -> i32 {
        for v1 in v {
            for v2 in v {
                let gcd = num::integer::gcd(*v1, *v2);
                if v1 != v2 && ( gcd == *v1 || gcd == *v2 ) {
                    let min = cmp::min( *v1, *v2 );
                    let max = cmp::max( *v1, *v2 );
                    return max / min;
                }
            }
        }

        return 0;
    };

    println!("Spreadsheet: \n{:?}", mat);
    println!();

    // let sum:i32 = mat.iter().map(|row| { let mm = minmax( row ); mm.1 - mm.0 }).sum();
    let sum:i32 = mat.iter().map(|row| evendiv(row)).sum();
    println!("Checksum: {}", sum);
}
