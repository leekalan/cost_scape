use crate::allocator::facility::Facility;

use super::Resource;

pub trait Intermediate: Resource {
    fn producer(&self) -> &dyn Facility;
}
