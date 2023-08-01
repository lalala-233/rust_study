use super::Tile;
trait BombDLC: Tile {
    fn is_bomb(&self) -> bool {
        self.name() == BOMBNAME
    }
}
const BOMBNAME: &str = "Bomb";
#[derive(Debug)]
struct Bomb {}
impl BombDLC for Bomb {}
impl Tile for Bomb {
    fn can_stepped(&self) -> bool {
        true
    }
    fn when_stepped(&self, player: &mut crate::Player) {
        player.die()
    }
    fn name(&self) -> &str {
        BOMBNAME
    }
    fn tile(&self) -> char {
        '*'
    }
}
