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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::user::User;
    #[test]
    fn test_repository() {
        let user_model = Model::<User>::new();
        let user_repo = Repository::new(user_model);
        let table_name = user_repo.get_table_name();
        println!("Table Name: {}", table_name);
        assert_eq!(table_name, "user");
    }
}
