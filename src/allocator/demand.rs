use super::{
    facility::Facility,
    resource::{intermediate::Intermediate, raw_input::RawInput, Resource},
};

#[derive(Clone, Copy)]
pub struct Demand<'a> {
    pub resource: &'a dyn Resource,
    pub units: f64,
}

#[derive(Clone, Copy)]
pub struct InputDemand<'a> {
    pub resource: &'a dyn RawInput,
    pub units: f64,
}

#[derive(Clone, Copy)]
pub struct ProductDemand<'a> {
    pub resource: &'a dyn Intermediate,
    pub units: f64,
}

#[derive(Clone, Copy)]
pub struct FacilityDemand<'a> {
    pub facility: &'a dyn Facility,
    pub units: f64,
}
