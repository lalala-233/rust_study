use super::*;
use rand::Rng;

pub struct Player {
    coord: Coord,
}
impl Player {
    //public
    pub fn coord(&self) -> &Coord {
        &self.coord
    }
    pub fn new(size: &Coord) -> Self {
        /*
        *****
        *****
        *****
        O****
        size.x()=5
        so use 0..5
         */
        let x = rand::thread_rng().gen_range(0..size.x());
        let y = rand::thread_rng().gen_range(0..size.y());
        let coord = Coord::new(x, y);
        Self { coord }
    }
    pub fn set_coord(&mut self, coord: Coord) {
        self.coord = coord;
    }
}
#[cfg(test)]
mod public {
    use super::*;
    #[test]
    pub fn coord() {
        let world = World::new(5, 4);
        let player = Player::new(&world.size());
        let coord = player.coord();
        assert!(world.contain(coord));
        assert_eq!(coord.x(), player.coord.x());
        assert_eq!(coord.y(), player.coord.y());
    }
    #[test]
    pub fn new() {
        let world = World::new(100, 50);
        let _player = Player::new(&world.size());
    }
    #[test]
    pub fn set_coord() {
        let world = World::new(5, 4);
        let size = world.size();
        let mut player = Player::new(&size);
        let x = rand::thread_rng().gen_range(0..size.x());
        let y = rand::thread_rng().gen_range(0..size.y());
        let coord = Coord::new(x, y);
        player.set_coord(coord);
        assert_eq!(player.coord.x(), x);
        assert_eq!(player.coord.y(), y);
    }
}
