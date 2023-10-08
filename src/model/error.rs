use serde::Serialize;

use crate::model::store;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    //Specific surreal errors
    InitDB(surrealdb::Error),

    //SurrealDB Error
    GeneralSurrealError(surrealdb::Error),

    //Module errors to wrap
    Store(store::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// region:    --- Froms

impl From<store::Error> for Error {
    fn from(value: store::Error) -> Self {
        Error::Store(value)
    }
}

impl From<surrealdb::Error> for Error {
    fn from(value: surrealdb::Error) -> Self {
        Error::GeneralSurrealError(value)
    }
}

// endregion: --- Froms
