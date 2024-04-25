use crate::allocator::{
    facility::Facility,
    id::Id,
    resource::{intermediate::Intermediate, Resource},
};

#[derive(Clone)]
pub struct IntermediateResource<'a> {
    id: Id,
    name: String,
    producer: &'a dyn Facility,
}
impl<'a> Resource for IntermediateResource<'a> {
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
impl<'a> Intermediate for IntermediateResource<'a> {
    fn producer(&self) -> &dyn crate::allocator::facility::Facility {
        self.producer
    }
}
