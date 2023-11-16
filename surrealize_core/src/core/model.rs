use serde::{Deserialize, Serialize};

use crate::utils::PascalToSnake;

#[derive(Clone, Debug)]

pub struct Model<T>
where
    T: Serialize + ?Sized + for<'de> Deserialize<'de> + 'static,
{
    tb: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Model<T>
where
    T: Serialize + ?Sized + for<'de> Deserialize<'de> + 'static,
{
    pub(crate) fn gen_tb_name() -> String {
        let type_name = std::any::type_name::<T>();
        let table_name = type_name
            .rsplit("::")
            .next()
            .unwrap_or(type_name)
            .to_snake_case();
        table_name
    }
    pub fn new() -> Self {
        let table_name = Self::gen_tb_name();

        Self {
            tb: table_name,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn get_table_name(&self) -> &str {
        &self.tb
    }
}
