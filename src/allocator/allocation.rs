use super::{
    allocation_pool::AllocationPool,
    command::Command,
    demand::{FacilityDemand, InputDemand, ProductDemand},
};

pub trait Allocation {
    fn execute(&mut self, command: &Command);
    fn rollback(&mut self, command: &Command);
    fn allocate(&self, allocation_pool: &impl AllocationPool) -> AllocationResult;
}

pub struct AllocationResult {
    pub inputs: Vec<InputDemand>,
    pub products: Vec<ProductDemand>,
    pub facilities: Vec<FacilityDemand>,
}
