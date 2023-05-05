struct Coord {
    x: usize,
    y: usize,
}
pub struct Map {
    ground: Vec<Vec<char>>,
}
// private
impl Map {
    fn contain(&self, coord: &Coord) -> bool {
        let size = &self.size();
        let x_max = size.x - 1;
        let y_max = size.y - 1;
        coord.x <= x_max && coord.y <= y_max
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
// public
impl Map {
    pub fn new(length: usize, width: usize) -> Self {
        let ground = vec![vec![' '; length]; width];
        Map { ground }
    }
}
#[cfg(test)]
mod test {
    use super::*;
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
        fn size() {
            let map = Map::new(5, 4);
            let size = map.size();
            assert_eq!(size.x, 5);
            assert_eq!(size.y, 4);
        }
    }
    mod public {
        use super::*;
        #[test]
        fn new() {
            let map = Map::new(3, 2);
            let expect = vec![vec![' ', ' ', ' ']; 2];
            /*
            [[x,x,x],
            [x,x,x]]
             */
            assert_eq!(map.ground, expect);
        }
    }
}
