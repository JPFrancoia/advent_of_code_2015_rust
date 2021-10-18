use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut visited_houses: HashSet<(i32, i32)> = HashSet::new();
    let mut santa_pos: (i32, i32) = (0, 0);
    let mut bot_pos: (i32, i32) = (0, 0);

    visited_houses.insert((0, 0));

    //println!("{}", contents);
    for c in contents.chars().step_by(2) {
        santa_pos = parse_movement(c, santa_pos);
        visited_houses.insert(santa_pos);

        println!("{:?}", santa_pos);
    }

    let mut iterator = contents.chars();
    iterator.next();

    for c in iterator.step_by(2) {
        bot_pos = parse_movement(c, bot_pos);
        visited_houses.insert(bot_pos);
    }

    println!("Unique houses: {}", visited_houses.len());

    // Answers:
    //2081
    //2341
}

fn parse_movement(movement: char, position: (i32, i32)) -> (i32, i32) {
    let (mut x, mut y) = position;

    match movement {
        '^' => y += 1,
        '>' => x += 1,
        'v' => y -= 1,
        '<' => x -= 1,
        _ => {
            println!("Incorrect char: {}", movement)
        }
    };

    (x, y)
}
