pub use crate::operations::Apply;

/// ap LSHIFT 1 -> bj
pub struct LSHIFT {
    input: Option<u16>,
    shift: u8,
    output: String,
}

impl Apply for LSHIFT {
    fn apply(&self) -> u16 {
        return 9;
    }
}
