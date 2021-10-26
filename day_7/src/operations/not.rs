pub use crate::operations::Apply;

/// NOT bs -> bt
pub struct NOT {
    input: Option<u16>,
    output: String,
}

impl Apply for NOT {
    fn apply(&self) -> u16 {
        return 9;
    }
}
