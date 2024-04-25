use super::demand::{InputDemand, ProductDemand};

pub trait Facility {
    fn name(&self) -> &str;
    fn amount_for(&self, product_demand: ProductDemand) -> Option<f64>;

    fn raw_inputs<'a>(&'a self) -> Box<dyn Iterator<Item = InputDemand> + 'a>;
    fn intermediate_inputs<'a>(&'a self) -> Box<dyn Iterator<Item = ProductDemand> + 'a>;
    fn outputs<'a>(&'a self) -> Box<dyn Iterator<Item = ProductDemand> + 'a>;
}
