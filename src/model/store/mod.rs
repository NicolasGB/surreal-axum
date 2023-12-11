// region:    --- Modules

mod error;

pub use error::{Error, Result};
use once_cell::sync::Lazy;

use crate::config;

use surrealdb::{
    engine::local::{Db, File},
    Surreal,
};

pub type Database = Surreal<Db>;

pub(in crate::model) static DB: Lazy<Database> = Lazy::new(Surreal::init);

// endregion: --- Modules

pub async fn database_connection() -> Result<()> {
    DB.connect::<File>(&config().DATABASE_PATH)
        .await
        .map_err(|err| Error::FailedToConnectToDb(err.to_string()))?;

    Ok(())
}
