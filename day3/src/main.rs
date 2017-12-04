fn main() {
    let mut size = 0;
    let mut pos = (0, 0);
    let mut id = 1;

    enum Direction {
        North,
        East,
        South,
        West,
    }
    let mut dir = Direction::East;

    let target = 368078;

    fn mv(x: &mut (i32, i32), dir: &Direction) {
        match *dir {
            Direction::East => x.0 += 1,
            Direction::North => x.1 -= 1,
            Direction::West => x.0 -= 1,
            Direction::South => x.1 += 1
        };
    };

    while id < target {
        mv(&mut pos, &dir);
        println!("pos ({},{})", pos.0, pos.1);

        match dir {
            Direction::East => if pos.0 > size {
                size += 1;
                println!("size {}", size);
                dir = Direction::North;
                println!("N");
            },
            Direction::North => if pos.1 == -size { dir = Direction::West; println!("W"); },
            Direction::West => if pos.0 == -size { dir = Direction::South; println!("S"); },
            Direction::South => if pos.1 == size { dir = Direction::East; println!("E"); }
        }

        id += 1;
    }

    println!("{}", pos.0.abs() + pos.1.abs());
}
