use std::collections::HashMap;

#[derive(Debug)]
pub enum Operation {
    COMPUTED(u16, String),
    SIMPLE(String, String),
    NOT(String, String),
    AND_SINGLE(u16, String, String),
    AND_DOUBLE(String, String, String),
    OR(String, String, String),
    LSHIFT(String, u8, String),
    RSHIFT(String, u8, String),
}

impl Operation {
    pub fn from_line(data: Vec<&str>) -> Operation {
        let destination = String::from(data[data.len() - 1]);

        match data.len() {
            3 => match data[0].parse::<u16>() {
                Ok(nbr) => {
                    return Operation::COMPUTED(nbr, destination);
                }
                Err(_) => {
                    return Operation::SIMPLE(String::from(data[0]), destination);
                }
            },
            4 => {
                return Operation::NOT(String::from(data[1]), destination);
            }
            5 => {
                let op_type = data[1];
                let input1 = String::from(data[0]);

                match op_type {
                    "OR" => return Operation::OR(input1, String::from(data[2]), destination),
                    "AND" => {
                        match data[0].parse::<u16>() {
                            Ok(nbr) => {
                                return Operation::AND_SINGLE(
                                    nbr,
                                    String::from(data[2]),
                                    destination,
                                );
                            }
                            Err(_) => {
                                return Operation::AND_DOUBLE(
                                    String::from(data[0]),
                                    String::from(data[2]),
                                    destination,
                                );
                            }
                        }
                    }
                    "LSHIFT" => {
                        return Operation::LSHIFT(
                            input1,
                            data[2].parse::<u8>().unwrap(),
                            destination,
                        )
                    }
                    "RSHIFT" => {
                        return Operation::RSHIFT(
                            input1,
                            data[2].parse::<u8>().unwrap(),
                            destination,
                        )
                    }
                    _ => panic!("Unknown operation type: {}", op_type),
                };
            }
            _ => panic!("Format for this line is unknown: {:?}", data),
        }
    }
}

pub fn process_operation(
    operation: &Operation,
    map_values: &HashMap<String, u16>,
) -> Result<(u16, String), &'static str> {

    match operation {
        Operation::COMPUTED(val, output) => return Ok((*val, output.clone())),
        Operation::SIMPLE(input, output) => {
            let val = get_value_if_computed(input, map_values)?;
            return Ok((val, output.clone()));
        }
        Operation::NOT(input, output) => {
            let val = get_value_if_computed(input, map_values)?;
            return Ok((!val, output.clone()));
        }
        Operation::AND_SINGLE(val1, input2, output) => {
            let val2 = get_value_if_computed(input2, map_values)?;
            return Ok((val1 & val2, output.clone()));
        }
        Operation::AND_DOUBLE(input1, input2, output) => {
            let val1 = get_value_if_computed(input1, map_values)?;
            let val2 = get_value_if_computed(input2, map_values)?;
            return Ok((val1 & val2, output.clone()));
        }
        Operation::OR(input1, input2, output) => {
            let val1 = get_value_if_computed(input1, map_values)?;
            let val2 = get_value_if_computed(input2, map_values)?;
            return Ok((val1 | val2, output.clone()));
        }
        Operation::LSHIFT(input, shift, output) => {
            let val = get_value_if_computed(input, map_values)?;
            return Ok((val << shift, output.clone()));
        }
        Operation::RSHIFT(input, shift, output) => {
            let val = get_value_if_computed(input, map_values)?;
            return Ok((val >> shift, output.clone()));
        }
    }
}

pub fn get_value_if_computed(
    key: &String,
    map_values: &HashMap<String, u16>,
) -> Result<u16, &'static str> {
    //https://stackoverflow.com/questions/65549983/trait-borrowstring-is-not-implemented-for-str
    if let Some(val) = map_values.get(key as &str) {
        return Ok(*val);
    } else {
        return Err("Can't compute this operation yet");
    }
}

// 0 -> c
// NOT bs -> bt
// gh OR gi -> gj
// ap LSHIFT 1 -> bj
// dh AND dj -> dk
// et RSHIFT 5 -> ew
