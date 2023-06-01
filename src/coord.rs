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
mod public {
    use super::*;
    #[test]
    pub fn new() {
        let x = 5;
        let y = 4;
        let coord = Coord::new(x, y);
        assert_eq!(coord.x_y(), (x, y));
    }
    #[test]
    pub fn x() {
        let x = 5;
        let y = 4;
        let coord = Coord::new(x,y);
        assert_eq!(x, coord.x());
    }
    #[test]
    pub fn x_y() {
        let x = 5;
        let y = 10;
        let coord = Coord::new(x, y);
        let x_y = coord.x_y();
        assert_eq!(x_y, (x, y));
        assert_eq!(x_y.0, coord.x());
        assert_eq!(x_y.1, coord.y());
    }
    #[test]
    pub fn y() {
        let x = 5;
        let y = 4;
        let coord = Coord::new(x,y);
        assert_eq!(y, coord.y());
    }
}
