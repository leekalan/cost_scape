pub mod allocator_pool;

use std::collections::HashMap;

use super::{
    allocation::{Allocation, AllocationResult},
    allocation_pool::AllocationPool,
    command::{Command, DemandChange},
    demand::{FacilityDemand, InputDemand, ProductDemand},
    id::Id,
};

#[derive(Default, Clone)]
pub struct Allocator {
    input_demands: HashMap<Id, f64>,
    product_demands: HashMap<Id, f64>,
}

impl Allocator {
    pub fn new() -> Self {
        Self::default()
    }

    fn change_input_demand(&mut self, demand: InputDemand) {
        self.input_demands
            .entry(demand.resource.id())
            .and_modify(|v| *v += demand.units)
            .or_insert(demand.units);
    }

    fn change_product_demand(&mut self, demand: ProductDemand) {
        self.product_demands
            .entry(demand.resource.id())
            .and_modify(|v| *v += demand.units)
            .or_insert(demand.units);
    }
}

impl Allocation for Allocator {
    fn execute(&mut self, command: &Command) {
        match command.demand_change {
            DemandChange::Input(t0) => self.change_input_demand(InputDemand {
                resource: t0,
                units: command.units,
            }),
            DemandChange::Product(t0) => self.change_product_demand(ProductDemand {
                resource: t0,
                units: command.units,
            }),
        }
    }

    fn rollback(&mut self, command: &Command) {
        match command.demand_change {
            DemandChange::Input(t0) => self.change_input_demand(InputDemand {
                resource: t0,
                units: -command.units,
            }),
            DemandChange::Product(t0) => self.change_product_demand(ProductDemand {
                resource: t0,
                units: -command.units,
            }),
        }
    }

    fn allocate<'a>(&self, pool: &'a impl AllocationPool) -> AllocationResult<'a> {
        let mut input_demands = self.input_demands.clone();
        let mut overall_products = self.product_demands.clone();
        let mut product_demands = self.product_demands.clone();
        let mut facility_demands: HashMap<Id, f64> = HashMap::new();

        while !product_demands.is_empty() {
            let temp = product_demands;
            product_demands = HashMap::new();
            for (id, units) in temp {
                // Unwrap as it shouldn't happen, and if it does its unretrievable
                let resource = pool.get_id(id).product().unwrap();
                let facility = resource.producer();
                let count = facility.amount_for(units);
                facility_demands
                    .entry(facility.id())
                    .and_modify(|v| *v += count)
                    .or_insert(count);

                for input_demand in facility.raw_inputs() {
                    let facility_demand = count * input_demand.units;
                    input_demands
                        .entry(input_demand.resource.id())
                        .and_modify(|v| *v += facility_demand)
                        .or_insert(facility_demand);
                }

                for product_demand in facility.intermediate_inputs() {
                    let facility_demand = count * product_demand.units;
                    product_demands
                        .entry(product_demand.resource.id())
                        .and_modify(|v| *v += facility_demand)
                        .or_insert(facility_demand);
                }
                overall_products.extend(product_demands.iter());
            }
        }

        let inputs = input_demands
            .into_iter()
            .map(|(id, units)| InputDemand {
                // Unwrap as it shouldn't happen, and if it does its unretrievable
                resource: pool.get_id(id).raw_input().unwrap(),
                units,
            })
            .collect();

        let products = overall_products
            .into_iter()
            .map(|(id, units)| ProductDemand {
                // Unwrap as it shouldn't happen, and if it does its unretrievable
                resource: pool.get_id(id).product().unwrap(),
                units,
            })
            .collect();

        let facilities = facility_demands
            .into_iter()
            .map(|(id, units)| FacilityDemand {
                // Unwrap as it shouldn't happen, and if it does its unretrievable
                facility: pool.get_id(id).facility().unwrap(),
                units,
            })
            .collect();

        AllocationResult { inputs, products, facilities }
    }
}
