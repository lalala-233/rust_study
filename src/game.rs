use super::*;
pub struct Game {
    player: Player,
    world: World,
}
impl Game {
    //public
    pub fn new(length: usize, width: usize) -> Self {
        let world = World::new(length, width);
        let player = Player::new(&world.size());
        Game { player, world }
    }
    pub fn set_player(&mut self, coord: Coord) -> Result<(), &'static str> {
        if self.world.contain(&coord) {
            self.player.set_coord(coord);
            Ok(())
        } else {
            Err("坐标在世界外！")
        }
    }
}

#[cfg(test)]
mod public {
    use super::*;
    #[test]
    pub fn new() {
        let length = 10;
        let width = 5;
        let game = Game::new(length, width);
        let size = game.world.size();
        assert_eq!(size.x(), length);
        assert_eq!(size.y(), width);
        assert!(game.world.contain(game.player.coord()));
    }
    #[test]
    pub fn set() {
        let mut game = Game::new(5, 4);
        let size = game.world.size();
        let x = size.x() - 1;
        let y = size.y() - 1;
        let coord = Coord::new(x, y);
        //Ok
        assert!(game.set_player(coord).is_ok());
        // update the coord
        let coord = game.player.coord();
        assert_eq!(coord.x(), x);
        assert_eq!(coord.y(), y);
        //Err
        assert!(game.set_player(Coord::new(x + 1, y)).is_err());
        assert!(game.set_player(Coord::new(x, y + 1)).is_err());
        // update the coord
        let coord = game.player.coord();
        assert_eq!(coord.x(), x);
        assert_eq!(coord.y(), y);
    }
}
