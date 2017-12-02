extern crate ansi_term;

use std::fs::File;
use std::io::prelude::*;
use ansi_term::Colour::Red;

fn main()
{
    //  read file
    let mut f = File::open("data/input.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    //  truncate
    let len = contents.len();
    contents.truncate(len - 1);

    println!("Captcha: \n{}", contents);
    println!();

    //  iterate
    println!("Parser:");

    let c1 = contents.chars();
    let c2 = contents.chars().cycle().skip((len-1)/2);
    let c12 = c1.zip(c2);

    let redeq = |x: &(char, char)| {
        if x.0 == x.1 {
            print!("{}", Red.paint(x.0.to_string()));
        } else {
            print!("{}", x.0);
        }
    };

    let sum = c12
        .inspect(|x| redeq(x))
        .fold(0, |sum, x| if x.0 == x.1 { sum + x.0.to_digit(10).unwrap() } else { sum });

    println!("\n");

    //  result
    println!("Sum: {}", sum);
}
