use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::{cmp, fs};

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("input.txt").unwrap();
    // let contents = fs::read_to_string("example.txt").unwrap();

    let mut happiness: HashMap<(&str, &str), i32> = HashMap::new();
    let mut people: HashSet<&str> = HashSet::new();

    println!("{}", contents);

    for line in contents.lines() {
        let data: Vec<&str> = line.split(" ").collect();

        let from = data[0];
        let to = data[data.len() - 1].strip_suffix(".").unwrap();
        let value: i32 = data[3].parse().unwrap();
        let sign = if data[2] == "gain" { 1 } else { -1 };

        happiness.insert((from, to), sign * value);
        people.insert(from);
    }

    add_myself(&mut happiness, &mut people);

    let mut best_happines: i32 = 0;

    for perm in people.iter().permutations(people.len()) {
        let happiness_count = compute_happiness(&perm, &happiness);
        best_happines = cmp::max(best_happines, happiness_count);
    }

    println!("Max happiness: {}", best_happines);
}

fn add_myself<'a>(happiness: &mut HashMap<(&'a str, &'a str), i32>, people: &mut HashSet<&'a str>) {
    for person in people.iter() {
        happiness.insert(("Me", person), 0);
        happiness.insert((person, "Me"), 0);
    }

    people.insert("Me");
}

fn compute_happiness(perm: &Vec<&&str>, happiness: &HashMap<(&str, &str), i32>) -> i32 {
    // The table is around, so we need to account for both ends of the vector
    let mut total: i32 = happiness
        .get(&(perm[perm.len() - 1], perm[0]))
        .unwrap_or(&0)
        .clone();
    total += happiness
        .get(&(perm[0], perm[perm.len() - 1]))
        .unwrap_or(&0)
        .clone();

    total += perm
        .windows(2)
        .map(|w| happiness.get(&(w[0], w[1])).unwrap_or(&0))
        .sum::<i32>();
    total += perm
        .windows(2)
        .map(|w| happiness.get(&(w[1], w[0])).unwrap_or(&0))
        .sum::<i32>();

    return total;
}
