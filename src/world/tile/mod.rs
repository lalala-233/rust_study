use std::fmt::Debug;

pub mod default;
mod bomb;

pub trait Tile: Debug {
    fn name(&self) -> &str;
    fn tile(&self) -> char;
}
impl PartialEq for dyn Tile {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}
