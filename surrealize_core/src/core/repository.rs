use std::sync::Arc;

use crate::error::Error;

use super::entry::Entry;
use super::model::Model;
use futures_util::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

#[derive(Debug)]
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

    /// Gets the table name of the Model linked to the repository
    pub fn get_table_name(&self) -> &str {
        &self.model.get_table_name()
    }
    /// # Usage
    /// Create a new instance of the model in the database and returns a `Entity<T>` with the created data from the database
    ///
    /// ```ignore
    /// use serde::{Serialize, Deserialize};
    /// use surrealize::sql::Thing;
    /// use surrealize::DataStore;
    /// use surrealize::entry::Entry;
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct User {
    /// id: Option<Thing>,
    /// name: String,
    /// }
    ///  async fn create_some_user(ds: &DataStore) {
    ///         let user_repo = ds.get_repository::<User>().unwrap();
    ///         let new_user = User {id: None, name: "Radha".to_owned()};
    ///         let created_user: Entry<'_, User>  = user_repo.create(new_user).await.unwrap();
    ///         let user_data: &User = created_user.data(); // get the reference to the created_user
    ///    }  
    ///  
    /// ```
    ///
    ///
    pub fn create(&self, new: T) -> LocalBoxFuture<Result<Entry<T>, Error>> {
        Box::pin(async move {
            let table_name = self.model.get_table_name().to_string();
            let query = "CREATE type::table($tb) CONTENT $data RETURN *;";
            // let content = sql::to_value(new)?;

            let db = &self.db;
            let mut res = db
                .query(query)
                .bind(("tb", table_name))
                .bind(("data", new))
                .await?;

            let created_entity: Option<T> = res.take(0)?;

            match created_entity {
                Some(user) => Ok(Entry::new(user, &self)),
                None => Err(Error::ResponseTakeError),
            }
        })
    }
    /// # Usage
    /// Gets all the entries from a given repository
    /// ```ignore
    /// use serde::{Serialize, Deserialize};
    /// use surrealize::sql::Thing;
    /// use surrealize::DataStore;
    /// use surrealize::entry::Entry;
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct User {
    /// id: Option<Thing>,
    /// name: String,
    /// }
    ///  async fn get_all_users(data_store: &DataStore) {
    ///         let user_repo = data_store.get_repository::<User>().unwrap();
    ///         let res: Vec<Entry<'_, User>> = user_repo.get_all().await.unwrap();
    ///    }  
    ///  
    /// ```
    pub fn get_all(&self) -> LocalBoxFuture<Result<Vec<Entry<T>>, Error>> {
        Box::pin(async move {
            let table_name = self.model.get_table_name().to_string();
            let query = "SELECT * FROM type::table($tb);";

            let db = &self.db;
            let mut res = db.query(query).bind(("tb", table_name)).await?;

            let data: Vec<T> = res.take(0)?;
            let all_entries = data
                .into_iter()
                .map(|entry| Entry::new(entry, &self))
                .collect::<Vec<Entry<T>>>();

            Ok(all_entries)
        })
    }
    pub fn update(&self) {}

    pub fn delete(&self) {}
}

