pub mod allocator_pool;
pub mod intermediate_resource;
pub mod process;
pub mod raw_resource;

use std::collections::HashMap;

use super::{
    allocation::{Allocation, AllocationResult},
    allocation_pool::AllocationPool,
    command::{Command, DemandChange},
    demand::{FacilityDemand, InputDemand, ProductDemand},
    resource::intermediate::Intermediate,
};

#[derive(Default, Clone)]
pub struct Allocator {
    input_demands: HashMap<String, f64>,
    product_demands: HashMap<String, f64>,
}

impl Allocator {
    pub fn new() -> Self {
        Self::default()
    }

    fn change_input_demand(&mut self, demand: InputDemand) {
        self.input_demands
            .entry(demand.name)
            .and_modify(|v| *v += demand.units)
            .or_insert(demand.units);
    }

    fn change_product_demand(&mut self, demand: ProductDemand) {
        self.product_demands
            .entry(demand.name)
            .and_modify(|v| *v += demand.units)
            .or_insert(demand.units);
    }
}

impl Allocation for Allocator {
    fn execute(&mut self, command: &Command) {
        match &command.demand_change {
            DemandChange::Input(name) => self.change_input_demand(InputDemand {
                name: name.to_owned(),
                units: command.units,
            }),
            DemandChange::Product(name) => self.change_product_demand(ProductDemand {
                name: name.to_owned(),
                units: command.units,
            }),
        }
    }

    fn rollback(&mut self, command: &Command) {
        match &command.demand_change {
            DemandChange::Input(name) => self.change_input_demand(InputDemand {
                name: name.to_owned(),
                units: -command.units,
            }),
            DemandChange::Product(name) => self.change_product_demand(ProductDemand {
                name: name.to_owned(),
                units: -command.units,
            }),
        }
    }

    fn allocate(&self, pool: &impl AllocationPool) -> AllocationResult {
        let mut input_demands = self.input_demands.clone();
        let mut overall_products = self.product_demands.clone();
        let mut product_demands = self.product_demands.clone();
        let mut facility_demands = HashMap::new();

        while !product_demands.is_empty() {
            let temp = product_demands;
            product_demands = HashMap::new();
            for (name, units) in temp {
                // Unwrap as it shouldn't happen, and if it does its unretrievable
                let resource: &dyn Intermediate = pool.get_name(&name).product().unwrap();
                // Unwrap as it shouldn't happen, and if it does its unretrievable
                let facility = pool.get_name(resource.producer()).facility().unwrap();
                // Unwrap as it shouldn't happen, and if it does its unretrievable
                let count = facility.amount_for(ProductDemand { name, units }).unwrap();
                facility_demands
                    .entry(facility.name())
                    .and_modify(|v| *v += count)
                    .or_insert(count);

                for product_demand in facility.outputs() {
                    if product_demand.name == resource.name() {
                        continue;
                    }
                    let facility_demand = count * product_demand.units;
                    product_demands
                        .entry(product_demand.name)
                        .and_modify(|v| *v -= facility_demand)
                        .or_insert(-facility_demand);
                }

                for input_demand in facility.raw_inputs() {
                    let facility_demand = count * input_demand.units;
                    input_demands
                        .entry(input_demand.name)
                        .and_modify(|v| *v += facility_demand)
                        .or_insert(facility_demand);
                }

                for product_demand in facility.intermediate_inputs() {
                    let facility_demand = count * product_demand.units;
                    product_demands
                        .entry(product_demand.name)
                        .and_modify(|v| *v += facility_demand)
                        .or_insert(facility_demand);
                }

                overall_products.extend(
                    product_demands
                        .iter()
                        .map(|(name, units)| (name.to_owned(), *units)),
                );
            }
        }

        let inputs = input_demands
            .into_iter()
            .map(|(name, units)| InputDemand {
                // Unwrap as it shouldn't happen, and if it does its unretrievable
                name,
                units,
            })
            .collect();

        let products = overall_products
            .into_iter()
            .map(|(name, units)| ProductDemand { name, units })
            .collect();

        let facilities = facility_demands
            .into_iter()
            .map(|(name, units)| FacilityDemand {
                name: name.to_owned(),
                units,
            })
            .collect();

        AllocationResult {
            inputs,
            products,
            facilities,
        }
    }
}

#[cfg(test)]
mod tests {
    use self::{
        allocator_pool::AllocatorPool, intermediate_resource::IntermediateResource,
        process::Process, raw_resource::RawResource,
    };

    use super::*;

    #[test]
    fn test() {
        let mut pool = AllocatorPool::new();

        pool.add_input(Box::new(RawResource::new("monetary expenses".to_string())));
        pool.add_input(Box::new(RawResource::new("employees".to_string())));
        pool.add_facility(Box::new(Process::new(
            "human resources".to_string(),
            vec![
                InputDemand {
                    name: "employees".to_string(),
                    units: 8.0,
                },
                InputDemand {
                    name: "monetary expenses".to_string(),
                    units: 200.0,
                },
            ],
            vec![],
            vec![ProductDemand {
                name: "worker divisions".to_string(),
                units: 1.0,
            }],
        )));
        pool.add_product(Box::new(IntermediateResource::new(
            "worker divisions".to_string(),
            "human resources".to_string(),
        )));

        let mut allocator = Allocator::new();

        allocator.change_product_demand(ProductDemand {
            name: "worker divisions".to_string(),
            units: 4.0,
        });

        allocator.change_input_demand(InputDemand {
            name: "monetary expenses".to_string(),
            units: -100.0,
        });

        let result = allocator.allocate(&pool);

        println!(
            "Employees: {},\nCost: {},\nHR: {},\nWorker Divisions: {}",
            result
                .inputs
                .iter()
                .find(|d| d.name == "employees")
                .unwrap()
                .units,
            result
                .inputs
                .iter()
                .find(|d| d.name == "monetary expenses")
                .unwrap()
                .units,
            result
                .facilities
                .iter()
                .find(|d| d.name == "human resources")
                .unwrap()
                .units,
            result
                .products
                .iter()
                .find(|d| d.name == "worker divisions")
                .unwrap()
                .units,
        );
    }
}
