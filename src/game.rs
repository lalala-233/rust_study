use super::*;
pub struct Game {
    player: Player,
    world: World,
}
impl Game {
    //private
    fn move_player(&mut self, coord: Coord) -> Result<(), &'static str> {
        if self.world.contain(&coord) {
            self.player.set_coord(coord);
            Ok(())
        } else {
            Err("坐标在世界外！")
        }
    }
    //public
    pub fn new(length: usize, width: usize) -> Self {
        let world = World::new(length, width);
        let player = Player::new(&world.size());
        Game { player, world }
    }
}

#[cfg(test)]
mod private {
    use super::*;
    #[test]
    fn move_player() {
        let mut game = Game::new(5, 4);
        let size = game.world.size();
        let x = size.x() - 1;
        let y = size.y() - 1;
        let coord = Coord::new(x, y);
        //Ok
        assert!(game.move_player(coord).is_ok());
        // update the coord
        let coord = game.player.coord();
        assert_eq!(coord.x_y(), (x, y));
        //Err
        assert!(game.move_player(Coord::new(x + 1, y)).is_err());
        assert!(game.move_player(Coord::new(x, y + 1)).is_err());
        // update the coord
        let coord = game.player.coord();
        assert_eq!(coord.x_y(), (x, y));
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
        assert_eq!(size.x_y(), (length, width));
        assert!(game.world.contain(game.player.coord()));
    }
}
