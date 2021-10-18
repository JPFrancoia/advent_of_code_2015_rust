use std::fs;

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("input.txt").unwrap();

    let mut total_square_feet = 0;
    let mut total_length_ribbon = 0;

    for line in contents.lines() {
        let dimensions = Dimensions::from_line(line).unwrap();

        total_square_feet += dimensions.surface_area() + dimensions.smallest_side_area();
        total_length_ribbon += dimensions.smallest_perimeter() + dimensions.volume();
    }

    //let d = Dimensions{l: 1, w: 1, h: 10};
    let d = Dimensions { l: 2, w: 3, h: 4 };
    println!("{}", d.surface_area() + d.smallest_side_area());
    println!("{}", d.smallest_perimeter() + d.volume());

    println!("Total square feet: {}", total_square_feet);
    println!("Total length ribbon: {}", total_length_ribbon);
}

struct Dimensions {
    l: u32,
    w: u32,
    h: u32,
}

impl Dimensions {
    fn from_line(line: &str) -> Result<Dimensions, &str> {
        //https://stackoverflow.com/questions/34090639/how-do-i-convert-a-vector-of-strings-to-a-vector-of-integers-in-a-functional-way
        let mut data: Vec<u32> = line
            .split('x')
            .map(|nbr| nbr.parse::<u32>().unwrap())
            .collect();

        //println!("{:?}", data);

        // By sorting the array, we make sure that l*w will return the area of the smallest side
        data.sort();

        //Note the if let instead of plain let. Slice patterns are refutable,
        // we execute the if only if we match the correct variant
        if let [l, w, h] = data[0..3] {
            return Ok(Dimensions { l, w, h });
        } else {
            return Err("Incorrect number of vars to unpack");
        }
    }

    fn surface_area(&self) -> u32 {
        2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l
    }

    fn smallest_side_area(&self) -> u32 {
        // l and w are the smallest dimensions
        self.l * self.w
    }

    fn smallest_perimeter(&self) -> u32 {
        2 * self.l + 2 * self.w
    }

    fn volume(&self) -> u32 {
        self.l * self.w * self.h
    }
}
