use super::{
    facility::Facility,
    id::Id,
    resource::{intermediate::Intermediate, raw_input::RawInput},
};

pub trait AllocationPool {
    fn get_id(&self, id: Id) -> IdRequest;
    fn add_input(&mut self, input: Box<dyn RawInput>);
    fn add_product(&mut self, product: Box<dyn Intermediate>);
    fn add_facility(&mut self, facility: Box<dyn Facility>);
}

pub enum IdRequest<'a> {
    None,
    Input(&'a dyn RawInput),
    Product(&'a dyn Intermediate),
    Facility(&'a dyn Facility),
}
impl<'a> IdRequest<'a> {
    pub fn none(self) -> Option<()> {
        match self {
            IdRequest::None => Some(()),
            _ => None,
        }
    }
    pub fn raw_input(self) -> Option<&'a dyn RawInput> {
        match self {
            IdRequest::Input(t0) => Some(t0),
            _ => None,
        }
    }
    pub fn product(self) -> Option<&'a dyn Intermediate> {
        match self {
            IdRequest::Product(t0) => Some(t0),
            _ => None,
        }
    }
    pub fn facility(self) -> Option<&'a dyn Facility> {
        match self {
            IdRequest::Facility(t0) => Some(t0),
            _ => None,
        }
    }
}
