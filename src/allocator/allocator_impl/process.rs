use crate::allocator::{
    demand::{InputDemand, ProductDemand},
    facility::Facility,
    id::Id,
};

pub struct Process<'a> {
    id: Id,
    name: String,
    raw_inputs: Vec<InputDemand<'a>>,
    intermediate_inputs: Vec<ProductDemand<'a>>,
    outputs: Vec<ProductDemand<'a>>,
}
impl Facility for Process<'_> {
    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> Id {
        self.id
    }

    fn set_id(&mut self, id: Id) {
        self.id = id;
    }

    fn amount_for(&self, product_demand: ProductDemand) -> Option<f64> {
        let output = self
            .outputs
            .iter()
            .find(|d| d.resource.id() == product_demand.resource.id())?
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
