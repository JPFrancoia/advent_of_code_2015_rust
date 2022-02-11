use std::collections::HashMap;
use std::fs;


fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("input.txt").unwrap();

    let target_sue = Sue{
        id: 0,
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    for line in contents.lines() {
        let sue = Sue::from_line(line);

        if sue == target_sue {
            println!("{:?}", sue);
            break;
        }
    }
}

// https://stackoverflow.com/questions/19650265/is-there-a-faster-shorter-way-to-initialize-variables-in-a-rust-struct
#[derive(Debug)]
struct Sue {
    id: u32,
    children: Option<u8>,
    cats: Option<u8>,
    samoyeds: Option<u8>,
    pomeranians: Option<u8>,
    akitas: Option<u8>,
    vizslas: Option<u8>,
    goldfish: Option<u8>,
    trees: Option<u8>,
    cars: Option<u8>,
    perfumes: Option<u8>,
}

impl Sue {
    fn from_line(line: &str) -> Self {
        let data: Vec<&str> = line.split(" ").collect();

        let items: HashMap<String, u8> = data[2..data.len()]
            .chunks(2)
            .map(|item| {
                (
                    String::from(item[0].trim_end_matches(":")),
                    String::from(item[1].trim_end_matches(",")).parse().unwrap(),
                )
            })
            .collect();

        Sue {
            id: data[1].trim_end_matches(":").parse().unwrap(),
            children: items.get("children").cloned(),
            cats: items.get("cats").cloned(),
            samoyeds: items.get("samoyeds").cloned(),
            pomeranians: items.get("pomeranians").cloned(),
            akitas: items.get("akitas").cloned(),
            vizslas: items.get("vizslas").cloned(),
            goldfish: items.get("goldfish").cloned(),
            trees: items.get("trees").cloned(),
            cars: items.get("cars").cloned(),
            perfumes: items.get("perfumes").cloned(),
        }
    }

}


impl PartialEq for Sue {
    fn eq(&self, other: &Self) -> bool {

        let go = match (self.children, other.children) {
            (Some(val1), Some(val2)) => val1 == val2,
            _ => true,
        };
        if go == false {
            return false;
        }

        let go = match (self.cats, other.cats) {
            (Some(val1), Some(val2)) => val1 > val2,
            _ => true,
        };
        if go == false {
            return false;
        }

        let go = match (self.samoyeds, other.samoyeds) {
            (Some(val1), Some(val2)) => val1 == val2,
            _ => true,
        };
        if go == false {
            return false;
        }

        let go = match (self.pomeranians, other.pomeranians) {
            (Some(val1), Some(val2)) => val1 < val2,
            _ => true,
        };
        if go == false {
            return false;
        }

        let go = match (self.akitas, other.akitas) {
            (Some(val1), Some(val2)) => val1 == val2,
            _ => true,
        };
        if go == false {
            return false;
        }

        let go = match (self.vizslas, other.vizslas) {
            (Some(val1), Some(val2)) => val1 == val2,
            _ => true,
        };
        if go == false {
            return false;
        }

        let go = match (self.goldfish, other.goldfish) {
            (Some(val1), Some(val2)) => val1 < val2,
            _ => true,
        };
        if go == false {
            return false;
        }

        let go = match (self.trees, other.trees) {
            (Some(val1), Some(val2)) => val1 > val2,
            _ => true,
        };
        if go == false {
            return false;
        }

        let go = match (self.cars, other.cars) {
            (Some(val1), Some(val2)) => val1 == val2,
            _ => true,
        };
        if go == false {
            return false;
        }

        let go = match (self.perfumes, other.perfumes) {
            (Some(val1), Some(val2)) => val1 == val2,
            _ => true,
        };
        if go == false {
            return false;
        }

        return true;
    }
}
