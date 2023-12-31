use serde::{Deserialize, Serialize};
use surrealize::model::Model;
use surrealize::opts::{ConnectionOptions, Credentials, On};
use surrealize::sql::Thing;
use surrealize::DataStore;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: Option<Thing>,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Post {
    id: Option<Thing>,
    title: String,
    content: String,
    user_id: Thing,
}

#[tokio::main]
async fn main() {
    let connection_options = ConnectionOptions {
        connection_url: "127.0.0.1:8000",
        auth: Credentials {
            password: "root".to_owned(),
            username: "root".to_owned(),
        }
        .into(),
        on: On {
            namespace: "development",
            database: "testing",
        }
        .into(),
    };

    #[allow(unused_variables)]
    let store = DataStore::init(connection_options)
        .await
        .unwrap()
        .register_model(Model::<User>::new())
        .register_model(Model::<Post>::new());

    // Use this store to register models, get registered repositories

    // Get the user repository
    let user_repo = store.get_repository::<User>().unwrap();
    let new_user = User {
        id: None,
        name: "Radha".into(),
    };
    // Create  a new user
    let user_entity = user_repo.create(new_user).await.unwrap();

    // Get the created user
    let new_user = user_entity.data();

    // Check if the created user is having an id generated by the DB or not
    assert!(new_user.id.is_some());

    // Print the new user
    println!("User Created");
    println!("Id: {}", new_user.id.clone().unwrap());
    println!("Name: {}", new_user.name);

    // Make a new post
    let new_post = Post {
        id: None,
        title: "This is a new Post".to_owned(),
        content: "This is the content of the post".to_owned(),
        user_id: new_user.id.clone().unwrap(),
    };

    // Get the post repo
    let post_repo = store.get_repository::<Post>().unwrap();

    // create a post in the database with the user as the previously created user
    let created_post = post_repo.create(new_post).await.unwrap();

    // Check if the created post has a non-None id
    assert!(created_post.data().id.is_some());

    let post_user_id = created_post.data().user_id.clone();
    let user_id = new_user.id.clone().unwrap();

    // Check if the user and the user of the post are the same
    assert_eq!(post_user_id, user_id);

    // Print the created Post
    println!("\nNew Post Created");
    let new_post = created_post.data_clone();
    println!("Id: {}", new_post.id.unwrap());
    println!("Title: {}", new_post.title);
    println!("Content: {}", new_post.content);
    println!("User Id: {}", new_post.user_id);
}
