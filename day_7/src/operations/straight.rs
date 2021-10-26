pub use crate::operations::Apply;
//use std::num::ParseIntError;

/// 0 -> c
#[derive(Debug)]
pub struct STRAIGHT {
    pub input: String,
    pub output: String,
}

impl STRAIGHT {
    pub fn from_line(data: Vec<&str>) -> Result<STRAIGHT, &str> {

        if data.len() != 3 {
            return Err("Wrong number of elements to parse");
        }

        Ok(STRAIGHT {
            input: String::from(data[0]),
            output: String::from(data[data.len() - 1]),
        })
    }
}

impl Apply for STRAIGHT {

    //TODO: apply should return Result<u16, &str>
    fn apply(&self) -> u16 {
        println!("{:?}", self);

        return 9;
    }
}
