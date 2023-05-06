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
}
#[cfg(test)]
mod private {
    use super::*;
}
#[cfg(test)]
mod public {
    use super::*;
    #[test]
    pub fn new() {
        let world = World::new(100, 50);
        let _player = Player::new(&world);
    }
}
