use super::{
    facility::Facility,
    resource::{intermediate::Intermediate, raw_input::RawInput},
};

pub trait AllocationPool {
    fn get_name(&self, name: &str) -> NameRequest;
    fn add_input(&mut self, input: Box<dyn RawInput>);
    fn add_product(&mut self, product: Box<dyn Intermediate>);
    fn add_facility(&mut self, facility: Box<dyn Facility>);
}

pub enum NameRequest<'a> {
    None,
    Input(&'a dyn RawInput),
    Product(&'a dyn Intermediate),
    Facility(&'a dyn Facility),
}
impl<'a> NameRequest<'a> {
    pub fn none(self) -> Option<()> {
        match self {
            NameRequest::None => Some(()),
            _ => None,
        }
    }
    pub fn raw_input(self) -> Option<&'a dyn RawInput> {
        match self {
            NameRequest::Input(t0) => Some(t0),
            _ => None,
        }
    }
    pub fn product(self) -> Option<&'a dyn Intermediate> {
        match self {
            NameRequest::Product(t0) => Some(t0),
            _ => None,
        }
    }
    pub fn facility(self) -> Option<&'a dyn Facility> {
        match self {
            NameRequest::Facility(t0) => Some(t0),
            _ => None,
        }
    }
}
