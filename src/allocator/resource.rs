use super::id::Id;

pub mod intermediate;
pub mod raw_input;

pub trait Resource {
    fn name(&self) -> &str;
    fn id(&self) -> Id;
    fn set_id(&mut self, id: Id);
}
