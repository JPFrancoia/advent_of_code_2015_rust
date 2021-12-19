static ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn main() {
    // let data = "hepxcrrq";
    let data = "hepxxyzz";

    let mut input: Vec<char> = data.chars().collect();

    // println!("{:?}", increment_password(&input.chars().collect()));
    // println!("{}", increasing_straight(&input.chars().collect()));
    // println!("{}", excluded_letters(&input.chars().collect()));
    // println!("{}", two_pairs(&input.chars().collect()));

    let filters: Vec<fn(&Vec<char>) -> bool> =
        vec![increasing_straight, excluded_letters, two_pairs];

    let result = loop {
        input = increment_password(&input);
        // println!("{:?}", input);

        if filters.iter().all(|f| f(&input)) {
            break input;
        }

    };
    println!("{}", result.iter().collect::<String>());
}

fn increment_password(input: &Vec<char>) -> Vec<char> {

    let mut output: Vec<char> = input.iter().rev().copied().collect();

    for char in output.iter_mut() {
        if *char == ALPHABET[ALPHABET.len() - 1] {
            *char = ALPHABET[0];
        } else {
            let idx_start = ALPHABET.binary_search(char).unwrap();
            *char = ALPHABET[idx_start + 1];
            break;
        }
    }

    output.reverse();

    return output;
}

fn increasing_straight(input: &Vec<char>) -> bool {
    for w in input.windows(3) {
        // We could use a hashmap or do a linear scan, but this is more fun
        let idx_start = ALPHABET.binary_search(&w[0]).unwrap();

        // Skip the check if 1st letter of windo is 'y' or 'z', we can't have an
        // increasing straight in this case
        if idx_start > 23 {
            continue;
        }

        if w[1] == ALPHABET[idx_start + 1] && w[2] == ALPHABET[idx_start + 2] {
            return true;
        }
    }

    return false;
}

fn excluded_letters(input: &Vec<char>) -> bool {
    !input.iter().any(|&c| c == 'i' || c == 'o' || c == 'l')
}

fn two_pairs(input: &Vec<char>) -> bool {
    let mut prev_pair: Option<char> = None;

    for w in input.windows(2) {
        if w[0] == w[1] {
            match prev_pair {
                Some(c) => {
                    if c != w[0] {
                        return true;
                    }
                }
                None => prev_pair = Some(w[0]),
            };
        }
    }

    return false;
}
