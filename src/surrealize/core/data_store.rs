use std::sync::Arc;

use super::opts;
use futures_util::future::LocalBoxFuture;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

#[derive(Debug)]
pub struct DataStore {
    db: Arc<Surreal<Client>>,
}

impl DataStore {
    pub fn init(
        opts: opts::ConnectionOptions,
    ) -> LocalBoxFuture<'static, Result<Self, crate::surrealize::error::Error>> {
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
            let data_store = Self { db: Arc::new(conn) };

            Ok(data_store)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn connect_database() {
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
    async fn failed_connection() {
        let connection_options = opts::ConnectionOptions {
            connection_url: "127.0.0.1:800", // Change the running port
            auth: None,
            on: None,
        };
        let data_store = DataStore::init(connection_options).await;
        assert!(data_store.is_err());
    }
}
