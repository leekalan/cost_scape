#[derive(Clone)]
pub struct InputDemand {
    pub name: String,
    pub units: f64,
}

#[derive(Clone)]
pub struct ProductDemand {
    pub name: String,
    pub units: f64,
}

#[derive(Clone)]
pub struct FacilityDemand {
    pub name: String,
    pub units: f64,
}
