use std::collections::HashMap;

fn main() {
    let mut size = 0;
    let mut pos = (0, 0);
    let mut field = HashMap::new();

    let mut value: i32 = 1;
    let target = 368078;
    field.insert((0, 0), value);

    enum Direction {
        North,
        East,
        South,
        West,
    }
    let mut dir = Direction::East;

    fn mv(x: &mut (i32, i32), dir: &Direction) {
        match *dir {
            Direction::East => x.0 += 1,
            Direction::North => x.1 -= 1,
            Direction::West => x.0 -= 1,
            Direction::South => x.1 += 1
        };
    };

    fn get_val(f: &mut HashMap<(i32,i32),i32>, p: (i32, i32)) -> i32 {
        return
            *f.entry((p.0+0,p.1+0)).or_insert(0) +
            *f.entry((p.0+1,p.1+0)).or_insert(0) +
            *f.entry((p.0+1,p.1+1)).or_insert(0) +
            *f.entry((p.0+0,p.1+1)).or_insert(0) +
            *f.entry((p.0-1,p.1+1)).or_insert(0) +
            *f.entry((p.0-1,p.1+0)).or_insert(0) +
            *f.entry((p.0-1,p.1-1)).or_insert(0) +
            *f.entry((p.0+0,p.1-1)).or_insert(0) +
            *f.entry((p.0+1,p.1-1)).or_insert(0)
    }

    while value < target {
        mv(&mut pos, &dir);

        value = get_val(&mut field, pos);
        field.insert(pos, value);

        match dir {
            Direction::East => if pos.0 > size {
                size += 1;
                dir = Direction::North;
            },
            Direction::North => if pos.1 == -size { dir = Direction::West; },
            Direction::West => if pos.0 == -size { dir = Direction::South; },
            Direction::South => if pos.1 == size { dir = Direction::East; }
        }
    }
    
    println!("{}", value);
}
