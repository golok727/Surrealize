mod core;
pub mod error;

pub use crate::core::data_store::DataStore;
pub use crate::core::model;
pub use crate::core::opts;

#[cfg(test)]
mod test;
