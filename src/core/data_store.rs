use std::{any::Any, collections::HashMap, sync::Arc};

use crate::error::Error;
use serde::{Deserialize, Serialize};

use super::{model::Model, opts, repository::Repository};
use futures_util::future::LocalBoxFuture;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

/// DataStore represents a manager for repositories and interactions with a Surreal database.
///
/// It holds a connection to the Surreal database and maintains a collection of repositories.
///
/// # Example
///
/// ```rust
/// use serde::{Deserialize, Serialize};
/// use surrealize::model::Model;
/// use surrealize::opts::ConnectionOptions;
/// use surrealize::DataStore;
///
/// #[derive(Serialize, Deserialize, Debug, Clone)]
/// struct User {
///     id: String,
///     name: String,
///     age: u8,
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let connection_options = ConnectionOptions {
///         connection_url: "127.0.0.1:8000",
///         auth: None,
///         on: None,
///     };
///     let mut conn = DataStore::init(connection_options).await.unwrap();
///     conn.register_repository(Model::<User>::new()).unwrap();
///     let user_repo = conn.get_repository::<User>().unwrap();
///     println!("TableName: {}", user_repo.get_table_name());
///     println!("Hello, world!");
///
/// }
/// ```
///
pub struct DataStore {
    db: Arc<Surreal<Client>>,
    repos: HashMap<String, Box<dyn Any>>,
}

impl DataStore {
    fn new(conn: Surreal<Client>) -> Self {
        Self {
            db: Arc::new(conn),
            repos: HashMap::new(),
        }
    }
    /// Initializes a database connection and returns a DataStore which will hold the connection
    /// Use it to register repositories, get repositories etc...
    pub fn init(opts: opts::ConnectionOptions) -> LocalBoxFuture<'static, Result<Self, Error>> {
        Box::pin(async move {
            let conn = Surreal::new::<Ws>(opts.connection_url).await?;

            if let Some(auth) = opts.auth {
                let credentials = Root {
                    username: &auth.username,
                    password: &auth.password,
                };

                conn.signin(credentials).await?;
            }

            if let Some(on) = opts.on {
                conn.use_ns(on.namespace).use_db(on.database).await?;
            }
            let data_store = Self::new(conn);

            Ok(data_store)
        })
    }
    /// Registers a repository..
    ///
    /// If it is already registered it will return a Error
    ///
    pub fn register_repository<T>(&mut self, model: Model<T>) -> Result<(), Error>
    where
        T: Serialize + ?Sized + Deserialize<'static> + 'static,
    {
        let repo_name = model.get_table_name().to_string();
        let repo = Repository::new(model);
        if self.repos.contains_key(&repo_name) {
            return Err(Error::ModelAlreadyRegistered(repo_name.clone()));
        }
        self.repos.insert(repo_name, Box::new(repo));
        Ok(())
    }

    pub fn get_repository<T>(&self) -> Result<&Repository<T>, Error>
    where
        T: Serialize + Deserialize<'static> + 'static,
    {
        let repo_name = Model::<T>::gen_tb_name();
        match self.repos.get(&repo_name) {
            Some(repo) => {
                if let Some(repo) = repo.downcast_ref::<Repository<T>>() {
                    Ok(repo)
                } else {
                    Err(Error::InternalError)
                }
            }
            None => Err(Error::RepositoryNotFound(repo_name.to_string())),
        }
    }
}
