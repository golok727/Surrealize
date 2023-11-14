use serde::{Deserialize, Serialize};
use surrealize::model::Model;
use surrealize::opts::ConnectionOptions;
use surrealize::DataStore;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: String,
    name: String,
    age: u8,
}

#[tokio::main]
async fn main() {
    let connection_options = ConnectionOptions {
        connection_url: "127.0.0.1:8000",
        auth: None,
        on: None,
    };
    let mut conn = DataStore::init(connection_options).await.unwrap();
    conn.register_repository(Model::<User>::new()).unwrap();
    let user_repo = conn.get_repository::<User>().unwrap();
    println!("TableName: {}", user_repo.get_table_name());
    println!("Hello, world!");
}
