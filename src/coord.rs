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
    pub fn y(&self) -> usize {
        self.y
    }
}
#[cfg(test)]
mod public {
    use super::*;
    #[test]
    pub fn new() {
        let x = 5;
        let y = 4;
        let coord = Coord::new(x, y);
        assert_eq!(coord.x(), x);
        assert_eq!(coord.y(), y);
    }
    #[test]
    pub fn x() {
        let x = 5;
        let y = 4;
        let coord = Coord { x, y };
        assert_eq!(x, coord.x());
    }
    #[test]
    pub fn y() {
        let x = 5;
        let y = 4;
        let coord = Coord { x, y };
        assert_eq!(y, coord.y());
    }
}
