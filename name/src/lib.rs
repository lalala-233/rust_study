pub use name_derive::Name;
pub trait Name {
    fn name() -> &'static str;
}
