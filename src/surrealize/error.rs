use derive_more::Display;
use serde::Serialize;
use surrealdb::Error as SurrealDBError;

#[derive(Serialize, Display, Debug)]
pub enum Error {
    #[display(fmt = "Database Error: {}", _0)]
    DatabaseError(SurrealDBError),
}

impl From<SurrealDBError> for Error {
    fn from(error: SurrealDBError) -> Self {
        Error::DatabaseError(error)
    }
}
