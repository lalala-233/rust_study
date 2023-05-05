//use std::{env, process};

fn main() {
    let map = game::Map::new(10, 10);
    let coord = map.coord(2, 3).unwrap();
}
