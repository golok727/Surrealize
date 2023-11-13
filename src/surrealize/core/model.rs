use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Model<T>
where
    T: Serialize + ?Sized + Deserialize<'static> + 'static,
{
    tb: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Model<T>
where
    T: Serialize + ?Sized + Deserialize<'static> + 'static,
{
    pub fn gen_tb_name() -> String {
        let type_name = std::any::type_name::<T>();
        let table_name = type_name
            .rsplit("::")
            .next()
            .unwrap_or(type_name)
            .to_lowercase();
        table_name
    }
    pub fn new() -> Self {
        let type_name = std::any::type_name::<T>();
        let table_name = type_name
            .rsplit("::")
            .next()
            .unwrap_or(type_name)
            .to_lowercase();

        Self {
            tb: table_name,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn get_table_name(&self) -> &str {
        &self.tb
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::user::User;
    #[test]
    fn test_model() {
        let user_model = Model::<User>::new();
        assert_eq!(user_model.get_table_name(), "user");
    }
}
