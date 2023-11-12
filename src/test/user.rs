use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    id: Option<Thing>,
    name: String,
    email_id: String,
    age: u8,
}
