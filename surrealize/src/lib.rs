mod core;
pub mod error;
mod utils;

pub use crate::core::data_store::DataStore;
pub use crate::core::entry;
pub use crate::core::model;
pub use crate::core::opts;
pub use surrealdb::sql;

#[cfg(test)]
mod test;
