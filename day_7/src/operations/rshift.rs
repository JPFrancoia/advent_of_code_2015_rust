pub use crate::operations::Apply;

/// et RSHIFT 5 -> ew
pub struct RSHIFT {
    input: Option<u16>,
    shift: u8,
    output: String,
}

impl Apply for RSHIFT {
    fn apply(&self) -> u16 {
        return 9;
    }
}
