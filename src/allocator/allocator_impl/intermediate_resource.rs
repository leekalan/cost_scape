use crate::allocator::resource::{intermediate::Intermediate, Resource};

#[derive(Clone)]
pub struct IntermediateResource {
    name: String,
    producer: String,
}
impl IntermediateResource {
    pub fn new(name: String, producer: String) -> Self {
        Self { name, producer }
    }
}
impl Resource for IntermediateResource {
    fn name(&self) -> &str {
        &self.name
    }
}
impl Intermediate for IntermediateResource {
    fn producer(&self) -> &str {
        &self.producer
    }
}
