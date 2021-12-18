fn main() {
    let nbr_iter = 50;
    let mut input = String::from("1321131112");

    for _ in 0..nbr_iter {
        input = look_and_say(&input);
    }

    println!("Final length: {}", input.len());
}

fn look_and_say(input: &str) -> String {

    let mut output = String::new();

    // NOTE: we're not handling the case where input is empty
    let mut prev_char = input.chars().nth(0).unwrap();
    let mut count = 1;

    for char in input.chars().skip(1) {
        if char == prev_char {
            count += 1;
        } else {
            output.push_str(&count.to_string());
            output.push(prev_char);
            prev_char = char;
            count = 1;
        }
    }

    output.push_str(&count.to_string());
    output.push(prev_char);

    return output;
}
