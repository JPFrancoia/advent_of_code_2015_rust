use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let containers: Vec<u8> = contents.lines().map(|x| x.parse::<u8>().unwrap()).collect();
    println!("{:?}", containers);

    // One combination of container will be represented as a u32
    // We'll use the bits to check if a container is used or not
    let mut seen_comb: HashSet<u32> = HashSet::new();

    let combinations = fit_eggnog(150, &containers, 0, &mut seen_comb);

    let min_containers = combinations
        .iter()
        .map(|comb| count_containers(*comb))
        .min()
        .unwrap();

    let count_min_containers: u32 = combinations
        .iter()
        .map(|comb| {
            if count_containers(*comb) == min_containers {
                return 1;
            } else {
                return 0;
            }
        })
        .sum();

    println!("Nbr of combinations: {}", combinations.len());
    println!("Min containers: {}", min_containers);
    println!("Nbr of min containers: {}", count_min_containers);
}

fn fit_eggnog(
    eggnog_to_fit: u8,
    containers: &Vec<u8>,
    cur_comb: u32,
    seen_comb: &mut HashSet<u32>,
) -> HashSet<u32> {
    let mut combinations: HashSet<u32> = HashSet::new();

    if eggnog_to_fit == 0 {
        // println!("{:?}", cur_comb);
        combinations.insert(cur_comb);
        return combinations;
    }

    for (idx, capacity) in containers.iter().enumerate() {
        // Prepare a mask for the container at this index
        let mask = 1 << idx;

        // Check if the container at this index is already used before proceeding
        if (cur_comb & mask) == 0 && *capacity <= eggnog_to_fit {
            // Flip the bit at the index matching the current container
            let temp_cur_comb = cur_comb | mask;

            // Check if this combination was seen before. If it was, skip it since it has already been explored
            if seen_comb.contains(&temp_cur_comb) {
                continue;
            } else {
                seen_comb.insert(temp_cur_comb);
            }

            combinations.extend(fit_eggnog(
                eggnog_to_fit - capacity,
                containers,
                temp_cur_comb,
                seen_comb,
            ));
        }
    }

    return combinations;
}

fn count_containers(combination: u32) -> u8 {
    let mut total: u8 = 0;

    for idx in 0..32 {
        if (combination & (1 << idx)) != 0 {
            total += 1;
        };
    }

    return total;
}
