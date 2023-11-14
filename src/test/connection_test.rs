#[cfg(test)]
mod tests {
    use crate::surrealize::core::data_store::DataStore;
    use crate::surrealize::core::opts;

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
}
