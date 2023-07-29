use crate::Coord;
pub mod camera;

pub struct Player {
    coord: Coord,
}
impl Player {
    //public
    pub fn new(coord: Coord) -> Self {
        Self { coord }
    }
}
#[cfg(test)]
pub mod public {
    use crate::Player;

    use self::default::default;
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
    pub fn new() {
        let (world, _player, width, height) = default();
        let coord = world.coord(width / 2, height / 2).unwrap();
        let player = Player::new(coord);
        assert_eq!(coord, player.coord)
    }
}
