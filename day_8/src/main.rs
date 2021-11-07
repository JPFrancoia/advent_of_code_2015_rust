use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    //let contents = fs::read_to_string("example.txt").unwrap();

    let re = Regex::new(r"\\x[0-9a-fA-F]{2}").unwrap();

    let mut code_chars = 0;
    let mut memory_chars = 0;

    //For part 2
    let mut encoded_chars = 0;

    for line in contents.lines() {
        println!("{}", line);

        let tmp_line = String::from(line);
        let mut stripped = String::from(tmp_line.strip_prefix('"').unwrap());
        stripped = String::from(stripped.strip_suffix('"').unwrap());

        let count_slash = stripped.matches("\\\\").count();
        let count_quote = stripped.matches("\\\"").count();
        let count_hex = re.find_iter(line).count();

        //count_quote -> +2 char per count_quote
        //count_slash -> +2 chars per count_slash
        //count_hex -> +1 char per count_hex
        //+4 for end and start quote

        // For debugging
        //let tmp_memory_chars = line.len() - count_slash - count_quote - 3 * count_hex - 2;
        let tmp_encoded_chars = line.len() + 2 * count_quote + 2 * count_slash + count_hex + 4;

        println!("{}\n", tmp_encoded_chars);

        code_chars += line.len();
        memory_chars += line.len() - count_slash - count_quote - 3 * count_hex - 2;
        encoded_chars += line.len() + 2 * count_quote + 2 * count_slash + count_hex + 4;


        //println!("{} {} {} {}", stripped, count_slash, count_quote, count_hex);
        //println!("{} {}\n", line.len(), tmp_memory_chars);
    }

    println!("Code chars: {}; Memory chars: {}", code_chars, memory_chars);
    println!("Answer: {}", code_chars - memory_chars);

    println!("Code chars: {}; Encoded chars: {}", code_chars, encoded_chars);
    println!("Answer: {}", encoded_chars - code_chars);
}
