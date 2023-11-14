use super::model::Model;
use serde::{Deserialize, Serialize};
pub struct Repository<T>
where
    T: Serialize + ?Sized + Deserialize<'static> + 'static,
{
    model: Model<T>,
}

impl<T> Repository<T>
where
    T: Serialize + ?Sized + Deserialize<'static> + 'static,
{
    pub(crate) fn new(model: Model<T>) -> Self {
        Self { model }
    }

    pub fn get_table_name(&self) -> &str {
        &self.model.get_table_name()
    }
    pub fn create(&self, new: T) {}
    pub fn update(&self) {}
    pub fn delete(&self) {}
}
