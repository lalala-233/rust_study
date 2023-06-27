#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Coord {
    x: usize,
    y: usize,
}
impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn x(&self) -> usize {
        self.x
    }
    pub fn x_y(&self) -> (usize, usize) {
        (self.x, self.y)
    }
    pub fn y(&self) -> usize {
        self.y
    }
}
#[cfg(test)]
pub mod public {
    use self::default::default;
    use crate::Coord;
    pub mod default {
        use crate::Coord;
        use rand::{thread_rng, Rng};
        pub fn default() -> (Coord, usize, usize) {
            let (x, y) = (
                thread_rng().gen_range(0..10000),
                thread_rng().gen_range(0..10000),
            );
            let coord = Coord::new(x, y);
            (coord, x, y)
        }
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
