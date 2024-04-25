use super::Resource;

pub trait Intermediate: Resource {
    fn producer(&self) -> &str;
}
