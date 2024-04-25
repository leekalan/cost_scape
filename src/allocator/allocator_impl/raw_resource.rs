use crate::allocator::{
    id::Id,
    resource::{raw_input::RawInput, Resource},
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct RawResource {
    id: Id,
    name: String,
}
impl Resource for RawResource {
    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> crate::allocator::id::Id {
        self.id
    }

    fn set_id(&mut self, id: crate::allocator::id::Id) {
        self.id = id;
    }
}
impl RawInput for RawResource {}
