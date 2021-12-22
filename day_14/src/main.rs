use std::collections::HashMap;
use std::{cmp, fs};

fn main() {
    println!("Hello, world!");
    let seconds = 2503;
    // let seconds = 1000;

    let contents = fs::read_to_string("input.txt").unwrap();
    // let contents = fs::read_to_string("example.txt").unwrap();

    let deers: Vec<Reindeer> = contents
        .lines()
        .map(|line| Reindeer::from_line(line))
        .collect();

    let winning_deer_a = find_winning_deer_part_a(&deers, seconds);

    println!("{:?}", winning_deer_a);
    println!("{:?}", fly_deer(winning_deer_a, seconds));

    let winning_deer_b = find_winning_deer_part_b(&deers, seconds);

    println!("{:?}", winning_deer_b);
}

/// Find the winning deer by giving one point to the winning deer after each second
fn find_winning_deer_part_b(deers: &Vec<Reindeer>, seconds: u32) -> &Reindeer {
    let mut scores: HashMap<&str, u32> = HashMap::new();

    // Start at one bc we count points at the end of a round
    for s in 1..seconds {
        // Find the position of each deer for this round
        let positions: HashMap<&str, u32> = deers
            .iter()
            .map(|deer| (deer.name.as_str(), fly_deer(deer, s)))
            .collect();

        // Find the maximum distance traveled by any deer for this round
        // This is necessary to find the deers that could be ex aequo
        let winning_distance = positions.values().max().unwrap();

        // Give one point to leading deers
        for (deer_name, position) in positions.iter() {
            if position == winning_distance {
                let points = scores.entry(deer_name).or_insert(0);
                *points += 1;
            }
        }
    }

    let max_points = scores.values().max().unwrap();

    println!("Max points: {}", max_points);

    // Find the name of the winning deer
    let winning_name = String::from(
        scores
            .into_iter()
            .max_by_key(|key_value| key_value.1)
            .unwrap()
            .0,
    );

    // Find the Reindeer struct matching the winning name
    deers.iter().find(|deer| deer.name == winning_name).unwrap()
}

/// Find the deer that traveled the most
fn find_winning_deer_part_a(deers: &Vec<Reindeer>, seconds: u32) -> &Reindeer {
    deers
        .iter()
        .max_by_key(|deer| fly_deer(deer, seconds))
        .unwrap()
}

/// Make a deer travel and return its position
/// The deer will travel in bursts, according to its speed and rest.
fn fly_deer(deer: &Reindeer, seconds: u32) -> u32 {
    let mut distance: u32 = 0;
    let mut seconds = seconds;

    while seconds > 0 {
        let fly_for = cmp::min(seconds, deer.fly_for);
        distance += fly_for * deer.speed;
        seconds -= fly_for;

        seconds -= cmp::min(seconds, deer.rest_for);
    }

    return distance;
}

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: u32,
    fly_for: u32,
    rest_for: u32,
}

impl Reindeer {
    /// Create a struct from a line like:
    /// Rudolph can fly 22 km/s for 8 seconds, but then must rest for 165 seconds.
    fn from_line(line: &str) -> Reindeer {
        let data: Vec<&str> = line.split(" ").collect();

        let name = String::from(data[0]);
        let speed: u32 = data[3].parse().unwrap();
        let fly_for: u32 = data[6].parse().unwrap();
        let rest_for: u32 = data[13].parse().unwrap();

        Reindeer {
            name,
            speed,
            fly_for,
            rest_for,
        }
    }
}
