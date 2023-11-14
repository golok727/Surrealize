#[cfg(test)]
mod tests {
    use crate::surrealize::core::data_store::DataStore;
    use crate::surrealize::core::model::Model;
    use crate::surrealize::core::opts;
    use crate::surrealize::core::repository::Repository;
    use crate::test::user::User;

    #[test]
    fn test_model() {
        let user_model = Model::<User>::new();
        assert_eq!(user_model.get_table_name(), "user");
    }
    #[test]
    fn test_repository() {
        let user_model = Model::<User>::new();
        let user_repo = Repository::new(user_model);
        let table_name = user_repo.get_table_name();
        println!("Table Name: {}", table_name);
        assert_eq!(table_name, "user");
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
