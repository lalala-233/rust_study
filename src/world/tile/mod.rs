use std::fmt::Debug;

use crate::Player;

mod bomb;
pub mod default;

pub trait Tile: Debug {
    fn name(&self) -> &str;
    fn tile(&self) -> char;
    fn can_stepped(&self) -> bool;
    fn when_stepped(&self, player: &mut Player) {}
}
impl PartialEq for dyn Tile {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}
