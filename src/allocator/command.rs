#[derive(Clone)]
pub struct Command {
    pub demand_change: DemandChange,
    pub units: f64,
}

#[derive(Clone)]
pub enum DemandChange {
    Input(String),
    Product(String),
}
