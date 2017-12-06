use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    //  read file
    let mut f = File::open("data/input.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let mut blocks: Vec<i32> = contents.split_whitespace().map(
        |x| x.parse::<i32>().unwrap()
    ).collect();

    let mut configs = HashSet::new();
    let mut count = 0;
    let mut id;
    let mut first_cycle = true;
    let mut first_block = Vec::new();

    loop
    {
        configs.insert(blocks.clone());
        // println!("{:?}", blocks);

        let maxit = blocks.iter().enumerate().fold( (-1, -1),
            |acc, enu| if *enu.1 > acc.1 { (enu.0 as i32, *enu.1) } else { acc }
        );

        let mut dist = maxit.1;
        id = maxit.0;

        blocks[id as usize] = 0;
        id = (id + 1) % blocks.len() as i32;

        while dist > 0 {
            blocks[id as usize] += 1;
            id = (id + 1) % blocks.len() as i32;
            dist -= 1;
        }

        count += 1;

        if first_cycle
        {
            if configs.contains(&blocks)
            {
                println!("first cycle done");
                println!("count: {}", count);
                println!("blocks: {:?}", blocks);

                first_block = blocks.clone();
                count = 0;
                first_cycle = false;
            }
        }
        else
        {
            if first_block == blocks {
                break;
            }
        }
    }

    println!("{}", count);
}
