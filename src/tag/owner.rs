use super::Tag;
use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TagOwner {
    tags: BTreeMap<TypeId, Box<dyn Tag>>,
}

impl TagOwner {
    //public
    pub fn search_tag<T: Tag + 'static>(&self, tag: &T) -> Option<&T> {
        let option_box = self.tags.get(&tag.type_id());
        option_box.map(|boxed_tag| {
            let dyn_any = boxed_tag as &dyn Any;
            dyn_any.downcast_ref::<T>().unwrap()
        })
    }
}
#[cfg(test)]
pub mod public {
    pub mod default {
        use crate::tag::{owner::TagOwner, walkable::Walkable};
        pub fn default() {
            let owner = TagOwner::default();
            let walkable = Walkable {};
        }
    }
}
