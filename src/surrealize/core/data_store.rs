use std::sync::Arc;

use futures_util::future::LocalBoxFuture;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub struct On {
    pub namespace: &'static str,
    pub database: &'static str,
}
pub struct ConnectionOptions {
    pub connection_url: &'static str,
    pub auth: Option<Root<'static>>,
    pub on: Option<On>,
}
pub struct DataStore {
    db: Arc<Surreal<Client>>,
}

impl DataStore {
    pub fn init(
        opts: ConnectionOptions,
    ) -> LocalBoxFuture<'static, Result<Self, surrealdb::Error>> {
        Box::pin(async move {
            let conn = Surreal::new::<Ws>(opts.connection_url).await?;

            if let Some(auth) = opts.auth {
                conn.signin(auth).await?;
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
        let connection_options = ConnectionOptions {
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
        let connection_options = ConnectionOptions {
            connection_url: "127.0.0.1:800", // Change the running port
            auth: None,
            on: None,
        };
        let data_store = DataStore::init(connection_options).await;
        assert!(data_store.is_err());
    }
}
