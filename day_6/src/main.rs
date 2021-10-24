use std::{fs, cmp};

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut grid: [[i32; 1000]; 1000] = [[0; 1000]; 1000];

    for line in contents.lines() {
        let t = Transformation::from_line(line).unwrap();
        //println!("{:?}\n", t);

        for row_idx in t.top_left_row..=t.bot_right_row {
            for col_idx in t.top_left_col..=t.bot_right_col {
                //println!("{:?}", grid[0][0]);
                let cell: &mut i32 = &mut grid[row_idx as usize][col_idx as usize];
                match t.transform_type {
                    TransformationType::On => *cell += 1,
                    TransformationType::Off => *cell = cmp::max(0, *cell - 1),
                    TransformationType::Toggle => *cell += 2,
                }
            }
        }
    }

    let mut count = 0;

    for row in grid.iter() {
        for col in row.iter() {
            count += col;
        }
    }
    println!("Total brightness: {}", count);
}

#[derive(Debug)]
enum TransformationType {
    Toggle,
    Off,
    On,
}

#[derive(Debug)]
struct Transformation {
    transform_type: TransformationType,
    top_left_row: u32,
    top_left_col: u32,
    bot_right_row: u32,
    bot_right_col: u32,
}

impl Transformation {
    fn from_line(line: &str) -> Result<Transformation, String> {
        let data: Vec<&str> = line.split(' ').collect();

        let transform_type = parse_transformation_type(&data)?;

        let (top_left_col, top_left_row) = parse_coordinates(data[data.len() - 3]);
        let (bot_right_col, bot_right_row) = parse_coordinates(data[data.len() - 1]);

        //println!("{:?}", data);
        //println!("{:?}", (top_left_row, top_left_col));
        //println!("{:?}", (bot_right_row, bot_right_col));

        Ok(Transformation {
            transform_type,
            top_left_row,
            top_left_col,
            bot_right_row,
            bot_right_col,
        })
    }
}

//FIXME: should be implemented only on Transformation?
fn parse_transformation_type(data: &Vec<&str>) -> Result<TransformationType, String> {
    if data.len() == 4 {
        return Ok(TransformationType::Toggle);
    } else if data.len() == 5 {
        match data[1] {
            "on" => return Ok(TransformationType::On),
            "off" => return Ok(TransformationType::Off),
            _ => return Err("Unknown value for transformation type".to_string()),
        };
    } else {
        return Err("Unexpected length for split line".to_string());
    }
}

//FIXME: should be implemented only on Transformation?
fn parse_coordinates(coo: &str) -> (u32, u32) {
    let data: Vec<u32> = coo
        .split(',')
        .map(|nbr| nbr.parse::<u32>().unwrap())
        .collect();

    (data[0], data[1])
}
