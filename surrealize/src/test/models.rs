use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Option<Thing>,
    pub name: String,
    pub email_id: String,
    pub age: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub id: Option<Thing>,
    pub title: String,
    pub content: String,
    pub user_id: Thing,
}
