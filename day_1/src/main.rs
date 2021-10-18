use std::fs;
use counter::Counter;

fn main() {
    println!("Hello, world!");

    let mut floor = 0;

    let contents = fs::read_to_string("input1.txt").unwrap();

    for (i, char) in contents.chars().enumerate() {
        if char == '(' {
            floor += 1;
        } else if char == ')' {
            floor -= 1;
        } else {
            println!("Wrong char found: {}", char);
        };

        if floor == -1 {
            println!("Level -1 at position {}", i + 1);
        }
    }

    println!("Final floor: {}", floor);

    let counts = contents.chars().collect::<Counter<_>>();

    println!("{:?}", counts);

    let count_floor = counts.get(&'(').unwrap_or(&0) - counts.get(&')').unwrap_or(&0);

    println!("Final floor: {}", count_floor);
}
