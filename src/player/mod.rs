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
            let (length, width) = (
                thread_rng().gen_range(11..114),
                thread_rng().gen_range(19..191),
            );
            let world = World::new(length, width);
            let coord = world.coord(length / 2, width / 2).unwrap();
            let player = Player::new(coord);
            (world, player, length, width)
        }
    }
    #[test]
    pub fn new() {
        let (world, _player, length, width) = default();
        let coord = world.coord(length / 2, width / 2).unwrap();
        let player = Player::new(coord);
        assert_eq!(coord, player.coord)
    }
}
