use std::collections::{HashMap, VecDeque};
use std::fs;

use day_7::{Operation, process_operation};

fn main() {

    let contents = fs::read_to_string("input.txt").unwrap();
    //let contents = fs::read_to_string("example.txt").unwrap();

    //let test: u16 = 42;
    //println!("{:016b}", test);
    //println!("{:016b}", !test);

    let op = Operation::COMPUTED(23, "ab".to_string());

    let mut map_values: HashMap<String, u16> = HashMap::new();
    let mut unprocessed_ops: VecDeque<Operation> = VecDeque::new();

    for line in contents.lines() {
        let data: Vec<&str> = line.split(' ').collect();

        println!("{:?}", data);

        let op = Operation::from_line(data);

        println!("{:?}\n", op);

        match process_operation(&op, &map_values) {
            Ok((val, output)) => {
                map_values.insert(output, val);
            },
            Err(_) => unprocessed_ops.push_back(op),
        }
    }

    println!("Starting processing operations");

    while unprocessed_ops.len() > 0 {

        let op = unprocessed_ops.pop_front().unwrap();

        match process_operation(&op, &map_values) {
            Ok((val, output)) => {
                map_values.insert(output, val);
            },
            Err(_) => unprocessed_ops.push_back(op),
        }

        //println!("{:?}", unprocessed_ops);
        //println!("{:?}", map_values);

    }

    println!("Unprocessed ops: {}", unprocessed_ops.len());
    println!("{:?}", map_values);
    println!("{}", map_values.get(&String::from("a")).unwrap());
}
