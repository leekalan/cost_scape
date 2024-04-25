use super::resource::{intermediate::Intermediate, raw_input::RawInput};

#[derive(Clone, Copy)]
pub struct Command<'a> {
    pub demand_change: DemandChange<'a>,
    pub units: f64,
}

#[derive(Clone, Copy)]
pub enum DemandChange<'a> {
    Input(&'a dyn RawInput),
    Product(&'a dyn Intermediate),
}
