use crate::coord;
use rand::Rng;

pub struct World {
    ground: Vec<Vec<char>>,
}
impl World {
    // private
    fn get(&self, coord: &coord::Coord) -> char {
        /*
        [*****]
        [*****]
        [*****]
        [O****]
        O=(0,0)
        actual=(0,3)
         */
        // row first, column second
        self.ground[coord.y()][coord.x()]
    }
    fn set(&mut self, coord: &coord::Coord, target: char) {
        /*
        [*****]
        [*****]
        [*****]
        [O****]
        O=(0,0)
        actual=(0,3)
         */
        // row first, column second
        self.ground[coord.y()][coord.x()] = target;
    }
}
impl World {
    // public
    pub fn contain(&self, coord: &coord::Coord) -> bool {
        let size = &self.size();
        let x_max = size.x() - 1;
        let y_max = size.y() - 1;
        coord.x() <= x_max && coord.y() <= y_max
    }
    pub fn coord(&self, x: usize, y: usize) -> Result<coord::Coord, &'static str> {
        /*
        *****
        **P**
        *****
        O****

        p=(2,2)
        actual=(2,(4-1)-2)=(1,1)
         */
        let coord = coord::Coord::new(x, y);
        if self.contain(&coord) {
            let x = x;
            let y = (self.size().y() - 1) - y;
            let coord = coord::Coord::new(x, y);
            Ok(coord)
        } else {
            Err("坐标在世界外！")
        }
    }
    pub fn new(length: usize, width: usize) -> Self {
        /*
        [[*****],
        [*****],
        [*****],
        [O****]]
        width=4
        0..4=[0,1,2,3]
         */
        let mut row = Vec::new();
        for _y in 0..width {
            let mut line = Vec::new();
            for _x in 0..length {
                if rand::thread_rng().gen_bool(0.1) {
                    line.push('A');
                } else {
                    line.push(' ');
                }
            }
            row.push(line);
        }
        Self { ground: row }
    }
    pub fn size(&self) -> coord::Coord {
        /*
        *****
        *****
        *****
        O****
        length=5
        width=4
         */
        let length = self.ground[0].len();
        let width = self.ground.len();
        coord::Coord::new(length, width)
    }
}
#[cfg(test)]
mod private {
    use super::*;
    #[test]
    fn get_and_set() {
        let mut world = World::new(5, 4);
        let coord = world.coord(2, 3).unwrap();

        world.set(&coord, ' ');
        let value = world.get(&coord);
        assert_eq!(value, ' ');

        world.set(&coord, 'x');
        let value = world.get(&coord);
        assert_eq!(value, 'x');
    }
}
#[cfg(test)]
mod public {
    use super::*;
    #[test]
    pub fn coord() {
        let world = World::new(5, 4);
        /*
        ****C
        *****
        *****
        O****
         */
        let coord = world.coord(4, 3).unwrap();
        assert_eq!(coord.x(), 4);
        assert_eq!(coord.y(), 0);
        let coord = world.coord(5, 3);
        assert!(coord.is_err());
        let coord = world.coord(4, 4);
        assert!(coord.is_err());
        let coord = world.coord(5, 4);
        assert!(coord.is_err());
    }
    #[test]
    pub fn contain() {
        let world = World::new(5, 4);
        /*
        ****p
        *****
        *****
        O****
        p=(4,3)
         */
        let can_contain = coord::Coord::new(4, 3);
        assert!(world.contain(&can_contain));
        let can_not_contain = coord::Coord::new(5, 3);
        assert!(!world.contain(&can_not_contain));
        let can_not_contain = coord::Coord::new(3, 4);
        assert!(!world.contain(&can_not_contain));
        let can_not_contain = coord::Coord::new(5, 4);
        assert!(!world.contain(&can_not_contain));
    }
    #[test]
    pub fn new() {
        let world = World::new(3, 2);
        /*
        [[x,x,x],
        [x,x,x]]
         */
        assert_eq!(world.ground.len(), 2);
        assert_eq!(world.ground[0].len(), 3);
    }
    #[test]
    pub fn size() {
        let world = World::new(5, 4);
        let size = world.size();
        assert_eq!(size.x(), 5);
        assert_eq!(size.y(), 4);
    }
}
