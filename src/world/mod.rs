pub mod coord;
pub mod line;
pub mod tile;
use std::ops::Index;

use self::{line::Line, tile::Tile};
use crate::Coord;
#[derive(Debug)]
pub struct World {
    ground: Vec<Line>,
}
impl World {
    // private
    fn contain(&self, coord: Coord) -> bool {
        let (x_max, y_max) = self.right_top().x_y();
        coord.x() <= x_max && coord.y() <= y_max
    }
    fn right_top(&self) -> Coord {
        let length = self.ground[0].len() - 1;
        let width = self.ground.len() - 1;
        Coord::new(length, width)
    }
}
impl World {
    // public
    pub fn coord(&self, x: usize, y: usize) -> Result<Coord, &'static str> {
        let coord = Coord::new(x, y);
        if self.contain(coord) {
            Ok(coord)
        } else {
            Err("坐标在世界外！")
        }
    }
    pub fn new(length: usize, width: usize) -> Self {
        let ground = (0..width).map(|_| Line::new(length)).collect();
        Self { ground }
    }
}
impl Index<Coord> for World {
    type Output = Box<dyn Tile>;
    fn index(&self, index: Coord) -> &Self::Output {
        let (x, y) = index.x_y();
        &self[y][x]
    }
}
impl Index<usize> for World {
    type Output = Line;
    fn index(&self, index: usize) -> &Self::Output {
        let y_max = self.right_top().y();
        &self.ground[y_max - index]
    }
}

#[cfg(test)]
mod private {
    use crate::{world::public::default::default, Coord};
    #[test]
    pub fn contain() {
        let (world, length, width) = default();
        let (x, y) = (length - 1, width - 1);
        let can_contain = Coord::new(x, y);
        assert!(world.contain(can_contain));
        let can_not_contain = Coord::new(x + 1, y);
        assert!(!world.contain(can_not_contain));
        let can_not_contain = Coord::new(x, y + 1);
        assert!(!world.contain(can_not_contain));
        let can_not_contain = Coord::new(x + 1, y + 1);
        assert!(!world.contain(can_not_contain));
    }
    #[test]
    pub fn right_top() {
        let (world, length, width) = default();
        let size = world.right_top();
        assert_eq!(size.x_y(), (length - 1, width - 1));
    }
}
#[cfg(test)]
pub mod public {
    use crate::world::public::default::default;
    pub mod default {
        use crate::World;
        use rand::{thread_rng, Rng};
        pub fn default() -> (World, usize, usize) {
            let (length, width) = (
                thread_rng().gen_range(11..114),
                thread_rng().gen_range(19..191),
            );
            let world = World::new(length, width);
            (world, length, width)
        }
    }
    #[test]
    pub fn coord() {
        let (world, length, width) = default();
        //索引需要减 1
        let coord = world.coord(length - 1, width - 1);
        assert!(coord.is_ok());
        let coord = world.coord(length, width - 1);
        assert!(coord.is_err());
        let coord = world.coord(length - 1, width);
        assert!(coord.is_err());
        let coord = world.coord(length, width);
        assert!(coord.is_err());
    }
    #[test]
    pub fn index_coord() {
        let (world, length, width) = default();
        let (length, width) = (length - 1, width - 1);
        for y in 0..=width {
            for x in 0..=length {
                let coord = world.coord(x, y).unwrap();
                assert_eq!(*world[y][x], *world[coord]);
            }
        }
    }
    #[test]
    pub fn index_usize() {
        let (world, _length, width) = default();
        for i in 0..width {
            //索引需要减 1
            assert_eq!(world.ground[width - i - 1], world[i]);
        }
    }
    #[test]
    pub fn new() {
        let (world, length, width) = default();
        assert_eq!(length, world.ground[0].len());
        assert_eq!(width, world.ground.len());
    }
}
