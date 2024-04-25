pub mod intermediate;
pub mod raw_input;

pub trait Resource {
    fn name(&self) -> &str;
}
