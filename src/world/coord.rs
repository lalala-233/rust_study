use std::fmt::Display;
use std::ops::{Add, AddAssign, Sub, SubAssign};
pub type NumType = usize;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Coord {
    x: NumType,
    y: NumType,
}
impl Coord {
    pub fn new(x: NumType, y: NumType) -> Self {
        Self { x, y }
    }
    pub fn x(&self) -> NumType {
        self.x
    }
    pub fn x_y(&self) -> (NumType, NumType) {
        (self.x, self.y)
    }
    pub fn y(&self) -> NumType {
        self.y
    }
}
#[cfg(test)]
pub mod public {
    use self::default::default;
    use crate::Coord;
    pub mod default {
        use crate::{world::coord::NumType, Coord};
        use rand::{thread_rng, Rng};
        pub fn default() -> (Coord, NumType, NumType) {
            let (x, y) = (
                thread_rng().gen_range(0..10000),
                thread_rng().gen_range(0..10000),
            );
            let coord = Coord::new(x, y);
            (coord, x, y)
        }
    }
    #[test]
    pub fn add() {
        let (coord1, _x, _y) = default();
        let (coord2, _x, _y) = default();
        let coord3 = coord1 + coord2;
        let mut coord1 = coord1;
        coord1 += coord2;
        assert_eq!(coord1, coord3);
    }
    #[test]
    fn eq() {
        let (coord, x, y) = default();
        let equal_coord = Coord::new(x, y);
        let not_equal_coord = Coord::new(x + 1, y + 1);
        assert_eq!(coord, equal_coord);
        assert_ne!(coord, not_equal_coord);
    }
    #[test]
    pub fn new() {
        let (coord, x, y) = default();
        assert_eq!(coord.x_y(), (x, y));
    }
    #[test]
    pub fn sub() {
        let (coord1, _x, _y) = default();
        let coord2 = coord1 + coord1;
        let coord3 = coord2 - coord1;
        let mut coord2 = coord2;
        coord2 -= coord1;
        assert_eq!(coord2, coord3);
    }
    #[test]
    pub fn x() {
        let (coord, x, _y) = default();
        assert_eq!(coord.x(), x);
    }
    #[test]
    pub fn x_y() {
        let (coord, x, y) = default();
        assert_eq!(coord.x_y(), (x, y));
    }
    #[test]
    pub fn y() {
        let (coord, _x, y) = default();
        assert_eq!(coord.y(), y);
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl Sub for Coord {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl SubAssign for Coord {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
