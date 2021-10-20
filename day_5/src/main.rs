use std::fs;
//use std::collections::HashSet;
use phf::phf_set;

const VOWELS: phf::Set<char> = phf_set! {'a', 'e', 'i', 'o', 'u'};

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut count_nice = 0;

    for line in contents.lines() {
        if is_line_valid(line) {
            count_nice += 1
        }
    }

    println!("Number of nice strings: {}", count_nice);
}

fn is_line_valid(line: &str) -> bool {
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
