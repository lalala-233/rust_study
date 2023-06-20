use super::tile::{default::DefaultTile, Tile};
use std::ops::Deref;
#[derive(Debug)]
pub struct Line {
    tiles: Vec<Box<dyn Tile>>,
}
impl Deref for Line {
    type Target = Vec<Box<dyn Tile>>;
    fn deref(&self) -> &Self::Target {
        &self.tiles
    }
}
impl Line {
    //public
    pub fn new(length: usize) -> Self {
        let mut tiles: Vec<Box<dyn Tile>> = Vec::with_capacity(length);
        for _ in 0..length {
            tiles.push(Box::new(DefaultTile::new()))
        }
        Line { tiles }
    }
}
impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        if self.len() == other.len() {
            let (left, mut right) = (self.iter(), other.iter());       
            for left in left {
                let right = right.next().unwrap();
                if left != right {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}
#[cfg(test)]
mod public {
    use super::Line;
    #[test]
    pub fn new() {
        let length = 114;
        let line_length = Line::new(length);
        assert_eq!(length, line_length.tiles.len())
    }
}
