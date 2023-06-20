const NAME: &str = "DefaultTile";

use super::Tile;
#[derive(Debug)]
pub struct DefaultTile {}
impl DefaultTile {
    //public
    pub fn new() -> Self {
        Self {}
    }
}
impl Tile for DefaultTile {
    fn tile(&self) -> char {
        ' '
    }
    fn name(&self) -> &str {
        NAME
    }
}
