use derive_more::Display;
use serde::Serialize;
use surrealdb::Error as SurrealDBError;

#[derive(Serialize, Display, Debug)]
pub enum Error {
    #[display(fmt = "Database Error: {}", _0)]
    DatabaseError(SurrealDBError),

    #[display(fmt = "Database Error: {}", _0)]
    SurrealErrorDB(surrealdb::error::Db),

    #[display(fmt = "Model '{}' is already registered..", _0)]
    ModelAlreadyRegistered(String),

    #[display(fmt = "Repository '{}' not found", _0)]
    RepositoryNotFound(String),

    #[display(fmt = "Internal Error")]
    InternalError,

    #[display(fmt = "ResponseTakeError: Entity May be created but was unable to take")]
    ResponseTakeError,
}

impl From<SurrealDBError> for Error {
    fn from(error: SurrealDBError) -> Self {
        Error::DatabaseError(error)
    }
}

impl From<surrealdb::error::Db> for Error {
    fn from(error: surrealdb::error::Db) -> Self {
        Error::SurrealErrorDB(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_error() {
        let surreal_error = SurrealDBError::Api(surrealdb::error::Api::AlreadyConnected);
        let custom_error = Error::DatabaseError(surreal_error);
        let string_version = custom_error.to_string();
        assert_eq!(string_version, "Database Error: Already connected")
    }
}
