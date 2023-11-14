use std::sync::Arc;

use crate::error::Error;

use super::model::Model;
use futures_util::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql, Surreal};
pub struct Repository<T>
where
    T: Serialize + ?Sized + for<'de> Deserialize<'de> + 'static,
{
    model: Model<T>,
    db: Arc<Surreal<Client>>,
}

impl<T> Repository<T>
where
    T: Serialize + ?Sized + for<'de> Deserialize<'de> + 'static,
{
    pub(crate) fn new(model: Model<T>, db: Arc<Surreal<Client>>) -> Self {
        Self { model, db }
    }

    pub fn get_table_name(&self) -> &str {
        &self.model.get_table_name()
    }
    pub fn create(&self, new: T) -> LocalBoxFuture<Result<T, Error>> {
        Box::pin(async move {
            let table_name = self.model.get_table_name().to_string();
            let query = "CREATE type::table($tb) CONTENT $data RETURN *;";
            let content = sql::to_value(new)?;

            let db = Arc::clone(&self.db);
            let mut res = db
                .query(query)
                .bind(("tb", table_name))
                .bind(("data", content))
                .await?;

            let created_entity: Option<T> = res.take(0)?;

            match created_entity {
                Some(user) => Ok(user),
                None => Err(Error::ResponseTakeError),
            }
        })
    }
    pub fn get_all(&self) -> LocalBoxFuture<Result<Vec<T>, Error>> {
        Box::pin(async move {
            let table_name = self.model.get_table_name().to_string();
            let query = "SELECT * FROM type::table($tb);";

            let db = Arc::clone(&self.db);
            let mut res = db.query(query).bind(("tb", table_name)).await?;

            let all_entires: Vec<T> = res.take(0)?;

            Ok(all_entires)
        })
    }
    pub fn update(&self) {}
    pub fn delete(&self) {}
}
