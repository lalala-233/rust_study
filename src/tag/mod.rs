use std::any::Any;
use std::fmt::Debug;
pub use struct_name::Name;
pub mod owner;
pub mod tile;
pub mod walkable;

pub trait Tag: Debug + Name {}
impl PartialEq for dyn Tag {
    fn eq(&self, other: &Self) -> bool {
        self.type_id() == other.type_id()
    }
}
impl Eq for dyn Tag {}
