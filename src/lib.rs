mod world;
pub use world::coord::Coord;
pub use world::World;

mod player;
pub use player::Player;

pub mod game;
pub use crate::game::Game;

pub fn is_debug() -> bool {
    let mut args = std::env::args();
    args.next();
    args.next() == Some("debug".to_string())
}
