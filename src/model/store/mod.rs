// region:    --- Modules

mod error;

pub use error::{Error, Result};

use crate::config;

use surrealdb::{
    engine::local::{Db, File},
    Surreal,
};

pub type Database = Surreal<Db>;

// endregion: --- Modules

pub async fn database_connection() -> Result<Database> {
    Surreal::new::<File>(&config().DATABASE_PATH)
        .await
        .map_err(|err| Error::FailedToConnectToDb(err.to_string()))
}
