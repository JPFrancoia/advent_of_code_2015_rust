use phf::phf_set;
use std::collections::HashSet;
use std::fs;

const VOWELS: phf::Set<char> = phf_set! {'a', 'e', 'i', 'o', 'u'};

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut count_nice = 0;

    for line in contents.lines() {
        //if is_line_valid1(line) {
            //count_nice += 1
        //}
        if is_line_valid2(line) {
            count_nice += 1
        }
    }

    println!("Number of nice strings: {}", count_nice);
}

/// Check a line for part 1
fn is_line_valid1(line: &str) -> bool {
    let mut count_vowels: u16 = 0;
    let mut double_letter = false;

    let chars: Vec<char> = line.chars().collect();

    for (idx, c) in chars.iter().enumerate() {
        if idx > 0 {
            match (chars[idx - 1], c) {
                ('a', 'b') => return false,
                ('c', 'd') => return false,
                ('p', 'q') => return false,
                ('x', 'y') => return false,
                (prev, curr) if &prev == curr => double_letter = true,
                _ => (),
            };
        }

        if VOWELS.contains(c) {
            count_vowels += 1;
        }
    }

    if count_vowels >= 3 && double_letter {
        return true;
    } else {
        return false;
    }
}

/// Check a line for part 2
fn is_line_valid2(line: &str) -> bool {
    let mut seen: HashSet<(char, char)> = HashSet::new();
    let chars: Vec<char> = line.chars().collect();

    let mut prev_pair = (chars[0], chars[1]);

    let mut double_pair = false;
    let mut triple_sequence = false;

    for (idx, c) in chars.iter().enumerate() {
        if idx == 0 {
            continue;
        }

        let curr_pair = (chars[idx - 1], *c);

        if curr_pair != prev_pair && seen.contains(&curr_pair) {
            double_pair = true;
        }

        seen.insert(curr_pair);

        prev_pair = curr_pair;

        if idx > 1 {
            match (chars[idx - 2], chars[idx - 1], c) {
                (x, _, z) if x == *z => triple_sequence = true,
                _ => (),
            }
        }
    }

    if double_pair && triple_sequence {
        return true;
    } else {
        return false;
    }
}
