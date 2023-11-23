pub use tag_derive::Name;
pub trait Name {
    fn name(&self) -> &'static str;
}
