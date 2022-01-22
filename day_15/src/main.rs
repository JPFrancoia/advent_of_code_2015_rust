use counter::Counter;
use std::collections::{HashMap, VecDeque};
use std::{fs, cmp};

fn main() {

    let contents = fs::read_to_string("input.txt").unwrap();
    // let contents = fs::read_to_string("example.txt").unwrap();
    let spoons = 100;
    let max_cals = 500;

    let ingredients: Vec<Ingredient> = contents
        .lines()
        .map(|line| Ingredient::from_line(line))
        .collect();

    // 21367368
    // let best_score = find_best_cookie_bfs_memoized(ingredients, spoons);

    // For part B
    // 1766400
    let best_score = find_best_cookie_brute_force(ingredients, spoons, max_cals);

    println!("Best score: {:?}", best_score);
}


fn find_best_cookie_brute_force(ingredients: Vec<Ingredient>, spoons: u32, max_cals: u32) -> u32 {

    let mut best_score = 0;

    for idx_first_ing in 0..spoons{
        for idx_second_ing in 0..spoons-idx_first_ing {
            for idx_third_ing in 0..spoons-idx_first_ing-idx_second_ing {

                let idx_fourth_ing = spoons - idx_first_ing - idx_second_ing - idx_third_ing;

                let mut ing_map: HashMap<&Ingredient, u32> = HashMap::new();

                ing_map.insert(&ingredients[0], idx_first_ing);
                ing_map.insert(&ingredients[1], idx_second_ing);
                ing_map.insert(&ingredients[2], idx_third_ing);
                ing_map.insert(&ingredients[3], idx_fourth_ing);

                let (score, cals) = compute_score_and_cals(ing_map);

                if cals > max_cals {
                    continue
                }

                best_score = cmp::max(best_score, score);

            }

        }
    }

    return best_score
}

fn find_best_cookie_bfs_memoized(ingredients: Vec<Ingredient>, spoons: u32) -> u32 {
    // Stores {score: len(ingredients)}
    let mut cache: HashMap<u32, u32> = HashMap::new();

    let mut best_node: Node = Node::new();

    let mut dq: VecDeque<Node> = VecDeque::new();

    // This is hackish, but we need to start with at least one of each ingredient
    // for my input, each ingredient has only one positive field
    dq.push_back(Node {
        ingredients: ingredients.iter().collect(),
    });

    while !dq.is_empty() {
        let node = dq.pop_front().unwrap();
        println!("Nbr ingredients: {:?}", node.ingredients.len());

        if node.ingredients.len() as u32 == spoons {
            if node.score() > best_node.score() {
                best_node = node;
            }
            continue;
        }

        if let Some(nbr_ingredients) = cache.get(&node.score()) {
            // We already saw this score with a smaller combination of
            // ingredients, give up on this solution
            // TODO: maybe we could use a BTree map and check if there is
            // a better solution in the cache already (higher score,
            // less ingredients)
            if node.ingredients.len() as u32 >= *nbr_ingredients {
                // println!("{:?}", cache);
                continue;
            }
        } else if !node.ingredients.is_empty() {
            cache.insert(node.score(), node.ingredients.len() as u32);
        }

        for ing in ingredients.iter() {
            let mut next_ing: Vec<&Ingredient> = node.ingredients.clone();
            next_ing.push(ing);
            dq.push_back(Node {
                ingredients: next_ing,
            });
        }
    }

    let count = best_node
        .ingredients
        .iter()
        .map(|ing| String::from(&ing.name))
        .collect::<Counter<_>>();
    println!("Best cookie: {:?}", count);
    return best_node.score();
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: u32,
}

impl Ingredient {
    /// Create un ingredient from a line like this:
    /// Candy: capacity 0, durability -1, flavor 0, texture 5, calories 8
    fn from_line(line: &str) -> Ingredient {
        let parse_nbr_str = |input: &str| input.strip_suffix(",").unwrap().parse::<i32>().unwrap();

        let data: Vec<&str> = line.split(" ").collect();

        let name = String::from(data[0].strip_suffix(":").unwrap());
        let capacity: i32 = parse_nbr_str(data[2]);
        let durability: i32 = parse_nbr_str(data[4]);
        let flavor: i32 = parse_nbr_str(data[6]);
        let texture: i32 = parse_nbr_str(data[8]);
        let calories: u32 = data[10].parse().unwrap();

        Ingredient {
            name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

#[derive(Debug)]
struct Node<'a> {
    ingredients: Vec<&'a Ingredient>,
}

impl<'a> Node<'a> {
    fn score(&self) -> u32 {
        let mut tot_capacity: i32 = 0;
        let mut tot_durability: i32 = 0;
        let mut tot_flavor: i32 = 0;
        let mut tot_texture: i32 = 0;

        for ing in self.ingredients.iter() {
            tot_capacity += ing.capacity;
            tot_durability += ing.durability;
            tot_flavor += ing.flavor;
            tot_texture += ing.texture;
        }

        if tot_capacity <= 0 || tot_durability <= 0 || tot_flavor <= 0 || tot_texture <= 0 {
            return 0;
        } else {
            return (tot_capacity * tot_durability * tot_flavor * tot_texture) as u32;
        }
    }

    fn new() -> Node<'a> {
        Node {
            ingredients: Vec::new(),
        }
    }
}

fn compute_score_and_cals(ingredients_amounts: HashMap<&Ingredient, u32>) -> (u32, u32){

    let mut tot_capacity: i32 = 0;
    let mut tot_durability: i32 = 0;
    let mut tot_flavor: i32 = 0;
    let mut tot_texture: i32 = 0;
    let mut tot_cals: u32 = 0;

    for (ing, amount) in ingredients_amounts {
        tot_capacity += ing.capacity * amount as i32;
        tot_durability += ing.durability * amount as i32;
        tot_flavor += ing.flavor as i32 * amount as i32;
        tot_texture += ing.texture as i32 * amount as i32;
        tot_cals += ing.calories * amount;
    }

    if tot_capacity <= 0 || tot_durability <= 0 || tot_flavor <= 0 || tot_texture <= 0 {
        return (0, 0);
    } else {
        return ((tot_capacity * tot_durability * tot_flavor * tot_texture) as u32, tot_cals);
    }
}
