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
        .register_model(Model::<User>::new());

    let user_repo = store.get_repository::<User>().unwrap();
    let new_user = User {
        id: None,
        name: "Radha".into(),
    };
    let user_entity = user_repo.create(new_user).await.unwrap();

    let new_user = user_entity.data();

    assert!(new_user.id.is_some());

    println!("User Created");

    println!("Id: {}", new_user.id.clone().unwrap());
    println!("Name: {}", new_user.name);

    // Use this store to register models, get registered repositories
}
