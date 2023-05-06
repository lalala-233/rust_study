use crate::*;
use rand::Rng;

pub struct Player<'a> {
    coord: world::Coord,
    world: &'a World,
}
impl<'a> Player<'a> {
    //private
}
impl<'a> Player<'a> {
    pub fn new(world: &'a World) -> Self {
        /*
        *****
        *****
        *****
        O****
        size.x=5
        so use 0..5
         */
        let size = world.size();
        let x = rand::thread_rng().gen_range(0..size.x());
        let y = rand::thread_rng().gen_range(0..size.y());
        let coord = world.coord(x, y).unwrap();
        Self { coord, world }
    }
    //public
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
        let player = Player::new(&world);
    }
}
