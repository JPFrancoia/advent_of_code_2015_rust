//A mod declaration makes the Rust compiler look for the corresponding .rs files automatically!
mod operations;

use std::fs;
//use std::collections::HashSet;
use std::collections::HashMap;

use operations::straight::{Apply, STRAIGHT};

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut map_values: HashMap<&str, u16> = HashMap::new();

    for line in contents.lines() {
        let data: Vec<&str> = line.split(' ').collect();

        //println!("{:?}", data);

        let operation = match data.len() {
            3 => STRAIGHT::from_line(data).unwrap(),
            //_ => panic!("Unknown format for line: {}", line),
            _ => continue,
        };

        operation.apply();
    }

    //println!("{:?}", seen);
}
