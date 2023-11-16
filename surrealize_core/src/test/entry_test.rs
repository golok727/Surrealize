#[cfg(test)]
mod tests {
    use surrealdb::sql::thing;

    use crate::core::data_store::DataStore;
    use crate::core::model::Model;
    use crate::core::opts;
    use crate::core::opts::On;
    use crate::test::models::Post;

    #[tokio::test]
    async fn test_entry() {
        let connection_options = opts::ConnectionOptions {
            connection_url: "127.0.0.1:8000",
            auth: None,
            on: On {
                database: "testing",
                namespace: "development",
            }
            .into(),
        };
        let data_store = DataStore::init(connection_options)
            .await
            .unwrap()
            .register_model(Model::<Post>::new());

        let user_id = thing("user:cz3b95a0nharymou3n49").unwrap();

        let post = Post {
            id: None,
            title: "Krsna Is Love".to_owned(),
            content: "Shyam, Shyam , Shyam".to_owned(),
            user_id: user_id.clone(),
        };

        let post_repo = data_store.get_repository::<Post>().unwrap();
        let created_post_entry = post_repo.create(post.clone()).await.unwrap();
        let created_post = created_post_entry.data();

        assert!(created_post.id.is_some(), "Created Post should have an id");
        assert_eq!(
            created_post.content, post.content,
            "Post Contents should have been the same"
        );
        assert_eq!(
            created_post.title, post.title,
            "Post title should have been the same"
        );
        assert_eq!(
            created_post.user_id, post.user_id,
            "User id should have been the same"
        );
    }
}
