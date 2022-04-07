use std::{cmp, fs};

const SIZE: usize = 100;
// const SIZE: usize = 6;

const NBR_STEPS: u32 = 100;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    // let contents = fs::read_to_string("test_input.txt").unwrap();

    println!("{}", contents);

    let mut grid = [[false; SIZE]; SIZE];

    for (idx_row, line) in contents.lines().enumerate() {
        for (idx_col, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    grid[idx_row][idx_col] = false;
                }
                '#' => {
                    grid[idx_row][idx_col] = true;
                }
                _ => (),
            }
        }
    }

    // Part B, set all corners to ON
    grid[0][0] = true;
    grid[0][SIZE - 1] = true;
    grid[SIZE - 1][0] = true;
    grid[SIZE - 1][SIZE - 1] = true;

    for _ in 0..NBR_STEPS {
        grid = animate_grid(&grid);
    }

    let mut total = 0;

    for idx_row in 0..grid.len() {
        for idx_col in 0..grid[0].len() {
            if grid[idx_row][idx_col] == true {
                total += 1;
            }
        }
    }

    println!("Total: {}", total);
}

fn animate_grid(grid: &[[bool; SIZE]; SIZE]) -> [[bool; SIZE]; SIZE] {
    // Initialize next grid; we don't want to mutate the input grid
    let mut next_grid = [[false; SIZE]; SIZE];

    for idx_row in 0..grid.len() {
        for idx_col in 0..grid[0].len() {
            let mut lights_on = 0;

            // Check the 8 adjacent lights where possible. Account for lights on sides/corners
            for i in cmp::max(0, idx_row as i32 - 1)..=cmp::min(SIZE as i32 - 1, idx_row as i32 + 1)
            {
                for j in
                    cmp::max(0, idx_col as i32 - 1)..=cmp::min(SIZE as i32 - 1, idx_col as i32 + 1)
                {
                    if i == idx_row as i32 && j == idx_col as i32 {
                        continue;
                    }
                    if grid[i as usize][j as usize] == true {
                        lights_on += 1;
                    }
                }
            }

            // Turn on the lights according to state of adjacent lights, and state of current light
            match (grid[idx_row][idx_col], lights_on) {
                (true, 2) => next_grid[idx_row][idx_col] = true,
                (true, 3) => next_grid[idx_row][idx_col] = true,
                (false, 3) => next_grid[idx_row][idx_col] = true,
                _ => (),
            }

            // Part B, set all corners to ON, always
            next_grid[0][0] = true;
            next_grid[0][SIZE - 1] = true;
            next_grid[SIZE - 1][0] = true;
            next_grid[SIZE - 1][SIZE - 1] = true;
        }
    }

    return next_grid;
}
