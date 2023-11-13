use std::{any::Any, collections::HashMap, hash::Hash, sync::Arc};

use crate::surrealize::error::Error;
use serde::{Deserialize, Serialize};

use super::{model::Model, opts, repository::Repository};
use futures_util::future::LocalBoxFuture;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use crate::test::user::User;

    use super::*;

    #[tokio::test]
    async fn test_connect_database() {
        let connection_options = opts::ConnectionOptions {
            connection_url: "127.0.0.1:8000",
            auth: None,
            on: None,
        };
        let data_store = DataStore::init(connection_options).await;
        assert!(data_store.is_ok());

        let _data_store = data_store.unwrap();
    }

    #[tokio::test]
    async fn test_failed_connection() {
        let connection_options = opts::ConnectionOptions {
            connection_url: "127.0.0.1:800", // Change the running port
            auth: None,
            on: None,
        };
        let data_store = DataStore::init(connection_options).await;
        assert!(data_store.is_err());
    }

    #[tokio::test]
    async fn test_registering_repo() {
        let connection_options = opts::ConnectionOptions {
            connection_url: "127.0.0.1:8000", // Change the running port
            auth: None,
            on: None,
        };
        let mut data_store = DataStore::init(connection_options).await.unwrap();

        let user = User {
            id: None,
            name: "Radha".into(),
            email_id: "someemail".into(),
            age: 19,
        };
        let user_model = Model::<User>::new();
        let res = data_store.register_repository(user_model.clone());
        assert!(res.is_ok());

        let user_model = Model::<User>::new();
        let res = data_store.register_repository(user_model.clone());
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_get_repo() {
        let connection_options = opts::ConnectionOptions {
            connection_url: "127.0.0.1:8000", // Change the running port
            auth: None,
            on: None,
        };
        let mut data_store = DataStore::init(connection_options).await.unwrap();

        let user = User {
            id: None,
            name: "Radha".into(),
            email_id: "someemail".into(),
            age: 19,
        };

        let user_model = Model::<User>::new();
        let res = data_store.register_repository(user_model.clone());
        assert!(res.is_ok());

        let repo = data_store.get_repository::<User>();
        assert!(repo.is_ok());
        let repo = repo.unwrap();

        repo.create(user); // Todo

        assert_eq!(repo.get_table_name(), "user");
    }
}
