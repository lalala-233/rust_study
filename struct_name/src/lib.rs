pub use struct_name_derive::Name;
pub trait Name {
    fn name(&self) -> &'static str;
}
