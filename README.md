## 🚧🚧🚧🚧 This Project is Under Development 🚧🚧🚧🚧
# Surrealize
Surrealize is a versatile client designed to interact seamlessly with SurrealDB, providing an intuitive interface to harness the power of SurrealDB's features. This client empowers developers to effortlessly integrate SurrealDB into their applications and projects.

> Surrealize is inspired from TypeORM design

# Overview
Surrealize simplifies the interaction with SurrealDB, a cutting-edge database designed for modern applications. With Surrealize, you can leverage the advanced capabilities of SurrealDB without the complexity.


# Usage

```rust
use surrealize::sql::Thing;
use surrealize::data_store::DataStore;
use surrealize::model::Model;
use surrealize::opts::{ ConnectionOptions, On};
use serde::{Serialize, Deserialize};

struct User {
    id: Option<Thing>,
    username: String,
    email: String,
    password: String,
}

#[tokio::main]
async fn main() {
    // Set up connection options
    let connection_options = opts::ConnectionOptions {
        connection_url: "127.0.0.1:8000",
        auth: None,
        on: On {
            namespace: "development",
            database: "testing",
        }
        .into(),
    };

    // Initialize data store and register the User model
    let data_store = DataStore::init(connection_options)
        .await
        .expect("Failed to initialize DataStore")
        .register_model(Model::<User>::new());


    let new_user = User {
        id: None, 
        username: "john_doe".to_owned(),
        email: "john.doe@example.com".to_owned(),
        password: "secure_password".to_owned(),
    };

    // Access the repository for the User model
    let user_repo = data_store.get_repository::<User>().unwrap();

    // Create the user in the database
    let created_user_entry = user_repo.create(new_user.clone()).await.expect("Failed to create user");
    let created_user = created_user_entry.data();

    // Output the created user details
    println!("User created successfully:");
    println!("ID: {:?}", created_user.id);
    println!("Username: {}", created_user.username);
    println!("Email: {}", created_user.email);
}


```

