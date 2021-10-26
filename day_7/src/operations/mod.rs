pub mod straight;

pub trait Apply {
    fn apply(&self) -> u16;
}
