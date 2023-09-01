use crate::{Coord, World};
use std::rc::{Rc, Weak};
pub mod camera;

pub struct Player {
    alive: bool,
    coord: Coord,
    world: Weak<World>,
}
impl Player {
    //public
    pub fn die(&mut self) {
        self.alive = false
    }
    pub fn is_alive(&self) -> bool {
        self.alive
    }
    pub fn new(coord: Coord) -> Self {
        Self {
            coord,
            alive: true,
            world: Weak::new(),
        }
    }
    pub fn set_world(&mut self, world: &Rc<World>) {
        self.world = Rc::downgrade(world)
    }
}
#[cfg(test)]
pub mod public {
    use self::default::default;
    use crate::Player;
    use std::rc::Rc;
    pub mod default {
        use crate::{Player, World};
        use rand::{thread_rng, Rng};
        pub fn default() -> (World, Player, usize, usize) {
            let (width, height) = (
                thread_rng().gen_range(11..114),
                thread_rng().gen_range(19..191),
            );
            let world = World::new(width, height);
            let coord = world.coord(width / 2, height / 2).unwrap();
            let player = Player::new(coord);
            (world, player, width, height)
        }
    }
    #[test]
    pub fn die_and_alive() {
        let (_world, mut player, _width, _height) = default();
        assert!(player.is_alive());
        player.die();
        assert!(!player.is_alive());
    }
    #[test]
    pub fn new() {
        let (world, _player, width, height) = default();
        let coord = world.coord(width / 2, height / 2).unwrap();
        let player = Player::new(coord);
        assert_eq!(coord, player.coord)
    }
    #[test]
    pub fn set_world() {
        let (world, mut player, _width, _height) = default();
        let world = &Rc::new(world);
        player.set_world(world);
        assert_eq!(*world.as_ref(), *player.world.upgrade().unwrap())
    }
}
