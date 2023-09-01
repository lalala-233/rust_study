pub mod game;
mod player;
mod world;
pub use game::Game;
pub use player::Player;
pub use world::coord::Coord;
pub use world::World;

pub fn is_debug() -> bool {
    let mut args = std::env::args();
    args.next();
    args.next() == Some("debug".to_string())
}
