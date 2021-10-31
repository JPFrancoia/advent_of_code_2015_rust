use std::collections::{HashMap, VecDeque};
use std::fs;

use day_7::{Operation, process_operation};

fn main() {

    let contents = fs::read_to_string("input.txt").unwrap();
    //let contents = fs::read_to_string("example.txt").unwrap();

    let mut map_values: HashMap<String, u16> = HashMap::new();
    let mut unprocessed_ops: VecDeque<Operation> = VecDeque::new();

    // Comment to get answer for part 1
    map_values.insert(String::from("b"), 46065);

    //46065 part 1
    //14134 too high

    for line in contents.lines() {
        let data: Vec<&str> = line.split(' ').collect();

        println!("{:?}", data);

        let op = Operation::from_line(data);

        println!("{:?}\n", op);

        match process_operation(&op, &map_values) {
            Ok((val, output)) => {
                map_values.entry(output).or_insert(val);
            },
            Err(_) => unprocessed_ops.push_back(op),
        }
    }

    println!("Starting processing operations");

    while unprocessed_ops.len() > 0 {

        let op = unprocessed_ops.pop_front().unwrap();

        match process_operation(&op, &map_values) {
            Ok((val, output)) => {
                map_values.entry(output).or_insert(val);
            },
            Err(_) => unprocessed_ops.push_back(op),
        }
    }

    println!("Unprocessed ops: {}", unprocessed_ops.len());
    println!("{:?}", map_values);
    println!("{}", map_values.get(&String::from("a")).unwrap());
}
