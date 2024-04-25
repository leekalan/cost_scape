use super::{
    demand::{InputDemand, ProductDemand},
    id::Id,
};

pub trait Facility {
    fn name(&self) -> &str;
    fn id(&self) -> Id;
    fn set_id(&mut self, id: Id);
    fn amount_for(&self, units: f64) -> f64;
    fn cost_for(&self, operating_facilities: f64) -> f64;

    fn raw_inputs(&self) -> Box<dyn Iterator<Item = InputDemand>>;
    fn intermediate_inputs(&self) -> Box<dyn Iterator<Item = ProductDemand>>;
    fn outputs(&self) -> Box<dyn Iterator<Item = ProductDemand>>;
}
