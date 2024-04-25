use crate::allocator::{
    id::Id,
    resource::{raw_input::RawInput, Resource},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MonetaryExpenses {
    id: Id,
}
impl Resource for MonetaryExpenses {
    fn name(&self) -> &str {
        "monetary expenses"
    }

    fn id(&self) -> crate::allocator::id::Id {
        self.id
    }

    fn set_id(&mut self, id: crate::allocator::id::Id) {
        self.id = id;
    }
}
impl RawInput for MonetaryExpenses {}
