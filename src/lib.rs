struct Position {
    x: usize,
    y: usize,
}
pub struct Map {
    ground: Vec<Vec<char>>,
}
// private
impl Map {}
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
