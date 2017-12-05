use std::fs::File;
use std::io::prelude::*;

fn main() {
    //  read file
    let mut f = File::open("data/input.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let mut maze: Vec<i32> = contents.lines().map(|line| line.parse::<i32>().unwrap()).collect();
    let mut pos: i32 = 0;
    let mut steps: i32 = 0;

    while pos >= 0 && pos < maze.len() as i32 {
        let dp = maze[ pos as usize ];
        maze[ pos as usize ] += 1;
        pos += dp;
        steps += 1;
    }

    println!("Maze: {:?}", maze);
    println!("Steps: {}", steps);
}
