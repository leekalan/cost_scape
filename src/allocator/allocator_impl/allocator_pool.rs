use std::{borrow::Borrow, collections::HashMap};

use crate::allocator::{
    allocation_pool::{AllocationPool, NameRequest},
    facility::Facility,
    resource::{intermediate::Intermediate, raw_input::RawInput},
};

#[derive(Default)]
pub struct AllocatorPool {
    inputs: HashMap<String, Box<dyn RawInput>>,
    products: HashMap<String, Box<dyn Intermediate>>,
    facilities: HashMap<String, Box<dyn Facility>>,
}
impl AllocatorPool {
    pub fn new() -> Self {
        Self::default()
    }
}
impl AllocationPool for AllocatorPool {
    fn get_name(&self, name: &str) -> NameRequest {
        if let Some(input) = self.inputs.get(name) {
            NameRequest::Input(input.borrow())
        } else if let Some(product) = self.products.get(name) {
            NameRequest::Product(product.borrow())
        } else if let Some(facility) = self.facilities.get(name) {
            NameRequest::Facility(facility.borrow())
        } else {
            NameRequest::None
        }
    }

    fn add_input(&mut self, input: Box<dyn RawInput>) {
        self.inputs.insert(input.name().to_owned(), input);
    }

    fn add_product(&mut self, product: Box<dyn Intermediate>) {
        self.products.insert(product.name().to_owned(), product);
    }

    fn add_facility(&mut self, facility: Box<dyn Facility>) {
        self.facilities.insert(facility.name().to_owned(), facility);
    }
}
