use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::{cmp, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut vertices: HashSet<&str> = HashSet::new();
    let mut edges: HashMap<(&str, &str), u32> = HashMap::new();

    for line in contents.lines() {
        //println!("{}", line);

        let data: Vec<&str> = line.split(" ").collect();

        let from = data[0];
        let to = data[2];
        let distance = data[4].parse::<u32>().unwrap();

        println!("{}, {}, {}", from, to, distance);

        vertices.insert(from);
        vertices.insert(to);

        edges.insert((from, to), distance);
        edges.insert((to, from), distance);
    }

    let mut short_dst = u32::MAX;
    let mut long_dst = 0;

    // Closure version
    //let short_dst = vertices
        //.iter()
        //.permutations(vertices.len())
        //.map(|perm| compute_length_of_trip(&perm, &edges))
        //.min();

    // This isn't the optimal solution, we're doing O(n - 1)! work
    // I could use dynamic programming here but I was lazy, I'm just learning Rust
    for perm in vertices.iter().permutations(vertices.len()){
        //println!("{:?}", perm);
        let length_trip = compute_length_of_trip(&perm, &edges);

        short_dst = cmp::min(short_dst, length_trip);
        long_dst = cmp::max(long_dst, length_trip);
    }

    println!("Shortest trip: {:?}", short_dst);
    println!("Longest trip: {:?}", long_dst);
}

fn compute_length_of_trip(trip: &Vec<&&str>, edges: &HashMap<(&str, &str), u32>) -> u32 {
    let mut total = 0;
    for idx in 0..trip.len() - 1 {
        // Not super safe to just unwrap, we might miss the edge, but let's assume we properly
        // built all the edges
        let distance = edges.get(&(trip[idx], trip[idx + 1])).unwrap_or(&0);

        total += distance;
    }

    return total;
}
