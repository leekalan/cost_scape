use crate::allocator::{
    demand::{InputDemand, ProductDemand},
    facility::Facility,
};

pub struct Process {
    name: String,
    raw_inputs: Vec<InputDemand>,
    intermediate_inputs: Vec<ProductDemand>,
    outputs: Vec<ProductDemand>,
}
impl Process {
    pub fn new(
        name: String,
        raw_inputs: Vec<InputDemand>,
        intermediate_inputs: Vec<ProductDemand>,
        outputs: Vec<ProductDemand>,
    ) -> Self {
        Self {
            name,
            raw_inputs,
            intermediate_inputs,
            outputs,
        }
    }
}
impl Facility for Process {
    fn name(&self) -> &str {
        &self.name
    }

    fn amount_for(&self, product_demand: ProductDemand) -> Option<f64> {
        let output = self
            .outputs
            .iter()
            .find(|d| d.name == product_demand.name)?
            .units;

        Some(product_demand.units / output)
    }

    fn raw_inputs<'a>(&'a self) -> Box<dyn Iterator<Item = InputDemand> + 'a> {
        Box::new(self.raw_inputs.iter().cloned())
    }

    fn intermediate_inputs<'a>(&'a self) -> Box<dyn Iterator<Item = ProductDemand> + 'a> {
        Box::new(self.intermediate_inputs.iter().cloned())
    }

    fn outputs<'a>(&'a self) -> Box<dyn Iterator<Item = ProductDemand> + 'a> {
        Box::new(self.outputs.iter().cloned())
    }
}
