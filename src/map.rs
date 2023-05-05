pub struct Map {
    ground: Vec<Vec<char>>,
}
impl Map {
    // private
    fn contain(&self, coord: &Coord) -> bool {
        let size = &self.size();
        let x_max = size.x - 1;
        let y_max = size.y - 1;
        coord.x <= x_max && coord.y <= y_max
    }
    fn get(&self, coord: &Coord) -> char {
        /*
        [*****]
        [*****]
        [*****]
        [O****]
        O=(0,0)
        actual=(0,3)
         */
        // row first, column second
        self.ground[coord.y][coord.x]
    }
    fn set(&mut self, coord: &Coord, target: char) {
        /*
        [*****]
        [*****]
        [*****]
        [O****]
        O=(0,0)
        actual=(0,3)
         */
        // row first, column second
        self.ground[coord.y][coord.x] = target;
    }
    fn size(&self) -> Coord {
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
        Coord {
            x: length,
            y: width,
        }
    }
}
impl Map {
    // public
    pub fn coord(&self, x: usize, y: usize) -> Result<Coord, &'static str> {
        /*
        *****
        *****
        *P***
        O****

        p=(1,1)
        actual=(1,(4-1)-1)=(1,2)
         */
        let x = x;
        let y = self.size().y - y - 1;
        let coord = Coord { x, y };
        if self.contain(&coord) {
            Ok(Coord { x, y })
        } else {
            Err("")
        }
    }
    pub fn new(length: usize, width: usize) -> Self {
        let ground = vec![vec![' '; length]; width];
        Map { ground }
    }
}
#[cfg(test)]
mod private {
    use super::*;
    #[test]
    fn contain() {
        let map = Map::new(5, 4);
        /*
        ****p
        *****
        *****
        O****
        p=(4,3)
         */
        let can_contain = Coord { x: 4, y: 3 };
        assert!(map.contain(&can_contain));
        let can_not_contain = Coord { x: 5, y: 3 };
        assert!(!map.contain(&can_not_contain));
        let can_not_contain = Coord { x: 3, y: 4 };
        assert!(!map.contain(&can_not_contain));
        let can_not_contain = Coord { x: 5, y: 4 };
        assert!(!map.contain(&can_not_contain));
    }
    #[test]
    fn get_and_set() {
        let mut map = Map::new(5, 4);
        let coord = map.coord(2, 3).unwrap();
        let value = map.get(&coord);
        assert_eq!(value, ' ');

        map.set(&coord, 'x');
        let value = map.get(&coord);
        assert_eq!(value, 'x');
    }
    #[test]
    fn size() {
        let map = Map::new(5, 4);
        let size = map.size();
        assert_eq!(size.x, 5);
        assert_eq!(size.y, 4);
    }
}
#[cfg(test)]
mod public {
    use super::*;
    #[test]
    pub fn coord() {
        let map = Map::new(5, 4);
        /*
        **A**
        C****
        *****
        O***B
         */
        let point_a = map.coord(2, 3).unwrap();
        let point_b = map.coord(4, 0).unwrap();
        let point_c = map.coord(0, 2).unwrap();

        assert_eq!(point_a.x, 2);
        assert_eq!(point_b.x, 4);
        assert_eq!(point_c.x, 0);

        assert_eq!(point_a.y, 0);
        assert_eq!(point_b.y, 3);
        assert_eq!(point_c.y, 1);
    }
    #[test]
    pub fn new() {
        let map = Map::new(3, 2);
        let expect = vec![vec![' ', ' ', ' ']; 2];
        /*
        [[x,x,x],
        [x,x,x]]
         */
        assert_eq!(map.ground, expect);
    }
}

pub struct Coord {
    x: usize,
    y: usize,
}
