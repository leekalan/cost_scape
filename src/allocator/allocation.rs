use super::{
    allocation_pool::AllocationPool,
    command::Command,
    demand::{FacilityDemand, InputDemand, ProductDemand},
};

pub trait Allocation {
    fn execute(&mut self, command: &Command);
    fn rollback(&mut self, command: &Command);
    fn allocate<'a>(&self, allocation_pool: &'a impl AllocationPool) -> AllocationResult<'a>;
}

pub struct AllocationResult<'a> {
    pub inputs: Vec<InputDemand<'a>>,
    pub products: Vec<ProductDemand<'a>>,
    pub facilities: Vec<FacilityDemand<'a>>,
}
