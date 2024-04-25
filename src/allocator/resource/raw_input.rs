use crate::allocator::id::Id;

use super::Resource;

pub trait RawInput: Resource {
    fn cost_for(&self, units: Id) -> Id;
}
