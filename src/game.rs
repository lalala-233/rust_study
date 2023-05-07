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
}
