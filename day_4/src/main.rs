use std::{fs, u32};
use md5;


fn main() {
    let key = fs::read_to_string("input.txt").unwrap();
    let key = key.trim();

    println!("Key: {}", key);

    let mut counter = 0;

    for n in 1..u32::MAX {
        //println!("{}", n);
        let input = format!("{}{}", key, n);
        let digest = format!("{:x}", md5::compute(input.as_bytes()));

        ////For debug
        //if n == 609043 {
            //println!("{}", digest);
            //println!("{}", digest.starts_with("00000"));
            //break
        //}

        if digest.starts_with("00000") {
            println!("{}", digest);
            counter = n;
            break
        }
    }

    println!("Suffix: {}", counter);
}
