use std::{borrow::Borrow, collections::HashMap};

use crate::allocator::{
    allocation_pool::{AllocationPool, IdRequest},
    facility::Facility,
    id::Id,
    resource::{intermediate::Intermediate, raw_input::RawInput},
};

#[derive(Default)]
pub struct AllocatorPool {
    inputs: HashMap<Id, Box<dyn RawInput>>,
    products: HashMap<Id, Box<dyn Intermediate>>,
    facilities: HashMap<Id, Box<dyn Facility>>,
    index: u32,
}
impl AllocatorPool {
    pub fn new() -> Self {
        Self::default()
    }
}
impl AllocationPool for AllocatorPool {
    fn get_id(&self, id: Id) -> IdRequest {
        if let Some(input) = self.inputs.get(&id) {
            IdRequest::Input(input.borrow())
        } else if let Some(product) = self.products.get(&id) {
            IdRequest::Product(product.borrow())
        } else if let Some(facility) = self.facilities.get(&id) {
            IdRequest::Facility(facility.borrow())
        } else {
            IdRequest::None
        }
    }

    fn add_input(&mut self, input: Box<dyn RawInput>) {
        self.inputs.insert(Id { id: self.index }, input);
        self.index += 1;
    }

    fn add_product(&mut self, product: Box<dyn Intermediate>) {
        self.products.insert(Id { id: self.index }, product);
        self.index += 1;
    }

    fn add_facility(&mut self, facility: Box<dyn Facility>) {
        self.facilities.insert(Id { id: self.index }, facility);
        self.index += 1;
    }
}
