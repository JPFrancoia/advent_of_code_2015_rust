use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};
use std::{cmp, fs};

use counter::Counter;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    part_a(&content);
    part_b(&content);
}

fn split_elements(elements_str: &str) -> Vec<String> {
    let all_chars: Vec<char> = elements_str.chars().collect();
    let mut elements: Vec<String> = vec![];
    let mut prev_char = all_chars[0];

    // split transformation on lower cases letters
    for c in all_chars.iter().skip(1) {
        if c.is_uppercase() && prev_char.is_uppercase() {
            elements.push(String::from(prev_char));
        } else if prev_char.is_uppercase() {
            elements.push(String::from_iter([prev_char, *c]));
        }

        prev_char = *c;
    }

    if all_chars[all_chars.len() - 1].is_uppercase() {
        elements.push(String::from(all_chars[all_chars.len() - 1]));
    }

    return elements;
}

/// Run some sort of random/greedy search to find the minimum number of steps to syntesize the
/// given molecule.
///
/// # Arguments
///
/// * `content` - data from input.txt. The rules from the challenge
///
fn part_b(content: &String) {
    let lines: Vec<&str> = content.lines().filter(|x| x.len() > 0).collect();

    let mut transformations: Vec<(&str, &str)> = vec![];

    for line in lines.iter().rev().skip(1) {
        let data: Vec<&str> = line.split(" => ").collect();
        transformations.push((data[1], data[0]));
    }

    println!("{:?}", transformations);

    let molecule = lines[lines.len() - 1];

    let mut best = std::u32::MAX;

    for _ in 0..1000 {
        let mut target = String::from(molecule);
        let mut steps = 0;

        while target != "e" {
            let tmp = target.clone();
            for (before, after) in transformations.iter() {
                if target.contains(before) == false {
                    continue;
                }

                target = target.replacen(before, after, 1);
                steps += 1;
            }

            // If no transformation happened, shuffle the order of
            // transformations and retry
            if tmp == target {
                target = String::from(molecule);
                steps = 0;
                transformations.shuffle(&mut thread_rng());
            }
        }
        best = cmp::min(best, steps);
    }

    println!("Best: {}", best);
}

fn part_a(content: &String) -> u32 {
    let lines: Vec<&str> = content.lines().filter(|x| x.len() > 0).collect();

    let mut transformations: HashMap<&str, Vec<Vec<String>>> = HashMap::new();

    for line in lines.iter().rev().skip(1) {
        let data: Vec<&str> = line.split(" => ").collect();

        let elements = split_elements(data[1]);

        transformations
            .entry(data[0])
            .and_modify(|prev_trans| prev_trans.push(elements.clone()))
            .or_insert(vec![elements.clone()]);
    }

    println!("{:?}", transformations);

    let molecule = split_elements(lines[lines.len() - 1]);
    println!("{:?}", molecule);

    let mut calibrated_molecules: HashSet<String> = HashSet::new();

    for (idx, element) in molecule.iter().enumerate() {
        if let Some(all_transformations) = transformations.get(&element as &str) {
            for t in all_transformations.iter() {
                let mut new_molecule: Vec<String> = molecule[..idx].to_vec();
                new_molecule.extend(t.clone());
                new_molecule.extend(molecule[idx + 1..].to_vec());
                calibrated_molecules.insert(String::from_iter(new_molecule));
            }
        }
    }
    println!("{}", calibrated_molecules.len());

    return calibrated_molecules.len() as u32;
}
