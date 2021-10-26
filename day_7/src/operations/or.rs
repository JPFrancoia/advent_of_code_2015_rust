pub use crate::operations::Apply; 

/// gh OR gi -> gj
pub struct OR {
    input1: Option<u16>,
    input2: Option<u16>,
    output: String,
}

impl Apply for OR {
    fn apply(&self) -> u16 {
        return 9;
    }
}
