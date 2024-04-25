use crate::allocator::resource::{raw_input::RawInput, Resource};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct RawResource {
    name: String,
}
impl RawResource {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
impl Resource for RawResource {
    fn name(&self) -> &str {
        &self.name
    }
}
impl RawInput for RawResource {}
