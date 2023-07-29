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
    pub fn new(width: usize) -> Self {
        let mut tiles: Vec<Box<dyn Tile>> = Vec::with_capacity(width);
        for _ in 0..width {
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
pub mod public {
    use self::default::default;
    use crate::world::line::Line;
    pub mod default {
        use crate::world::line::Line;
        use rand::{thread_rng, Rng};
        pub fn default() -> (Line, usize) {
            let width = thread_rng().gen_range(11..114);
            let line = Line::new(width);
            (line, width)
        }
    }
    #[test]
    pub fn deref() {
        let (line, width) = default();
        assert_eq!(line.len(), width);//line.len() is line.deref().len()
    }
    #[test]
    pub fn eq() {
        let (line, width) = default();
        assert_eq!(Line::new(width), line);
    }
    #[test]
    pub fn new() {
        let (line, width) = default();
        assert_eq!(width, line.len());
    }
}
