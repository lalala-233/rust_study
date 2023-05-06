use crate::coord;
use crate::World;
use rand::Rng;

pub struct Player<'a> {
    coord: coord::Coord,
    world: &'a World,
}
impl<'a> Player<'a> {
    //private
}
impl<'a> Player<'a> {
    //public
    pub fn coord(&self) -> &coord::Coord {
        &self.coord
    }
    pub fn new(world: &'a World) -> Self {
        /*
        *****
        *****
        *****
        O****
        size.x()=5
        so use 0..5
         */
        let size = world.size();
        let x = rand::thread_rng().gen_range(0..size.x());
        let y = rand::thread_rng().gen_range(0..size.y());
        let coord = coord::Coord::new(x, y);
        Self { coord, world }
    }
    pub fn set(&mut self, x: usize, y: usize) -> Result<(), &'static str> {
        let coord = coord::Coord::new(x, y);
        if self.world.contain(&coord) {
            self.coord = coord;
            Ok(())
        } else {
            Err("坐标在世界外！")
        }
    }
}
#[cfg(test)]
mod private {
    use super::*;
}
#[cfg(test)]
mod public {
    use super::*;
    #[test]
    pub fn coord_and_set() {
        let world = World::new(5, 4);
        let mut player = Player::new(&world);
        let x = world.size().x() - 1;
        let y = world.size().y() - 1;

        assert!(player.set(x, y).is_ok());
        assert_eq!(player.coord.x(), x);
        assert_eq!(player.coord.y(), y);
        // Err
        assert!(player.set(x + 1, y).is_err());
        assert!(player.set(x, y + 1).is_err());
        assert!(player.set(x + 1, y + 1).is_err());
        assert_eq!(player.coord.x(), x);
        assert_eq!(player.coord.y(), y);
    }
    #[test]
    pub fn new() {
        let world = World::new(100, 50);
        let _player = Player::new(&world);
    }
}
