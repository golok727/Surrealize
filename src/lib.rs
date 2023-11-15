mod core;
mod utils;
pub mod error;

pub use crate::core::data_store::DataStore;
pub use crate::core::model;
pub use crate::core::opts;
pub use surrealdb::sql;

#[cfg(test)]
mod test;
