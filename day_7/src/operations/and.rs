pub use crate::operations::Apply;

/// dh AND dj -> dk
pub struct AND {
    input1: Option<u16>,
    input2: Option<u16>,
    output: String,
}

impl Apply for AND {
    fn apply(&self) -> u16 {
        return 9;
    }
}
