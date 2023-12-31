#[cfg(test)]
mod tests {
    use crate::core::data_store::DataStore;
    use crate::core::model::Model;
    use crate::core::opts;
    use crate::core::opts::On;
    use crate::test::models::User;

    #[tokio::test]
    async fn test_create_user() {
        let connection_options = opts::ConnectionOptions {
            connection_url: "127.0.0.1:8000",
            auth: None,
            on: On {
                database: "testing",
                namespace: "development",
            }
            .into(),
        };
        let data_store = DataStore::init(connection_options).await.unwrap();

        let user = User {
            id: None,
            name: "Hello".into(),
            email_id: "radhakrsna@golok.vrindavan".into(),
            age: 19,
        };

        let data_store = data_store.register_model(Model::<User>::new());

        let user_repo = data_store.get_repository::<User>().unwrap();
        println!("Repo Name: {}", user_repo.get_table_name());

        let created_user = user_repo.create(user).await;
        assert!(created_user.is_ok());

        let created_user = created_user.unwrap();
        dbg!(&created_user);
        assert!(created_user.data().id.is_some())
    }

    #[tokio::test]
    async fn test_find_users() {
        let connection_options = opts::ConnectionOptions {
            connection_url: "127.0.0.1:8000", // Change the running port
            auth: None,
            on: On {
                database: "testing",
                namespace: "development",
            }
            .into(),
        };
        let data_store = DataStore::init(connection_options).await.unwrap();

        let data_store = data_store.register_model(Model::<User>::new());

        let user_repo = data_store.get_repository::<User>().unwrap();
        println!("Repo Name: {}", user_repo.get_table_name());
        let res = user_repo.get_all().await;
        assert!(res.is_ok());
        let _users = res.unwrap();
    }
}
