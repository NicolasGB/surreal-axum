mod error;
mod store;
pub mod task;

use crate::{
    config,
    model::store::{database_connection, Database},
};

pub use self::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct ModelManager {
    db: Database,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db = database_connection().await?;

        //Setup the db namespace this might be better to do in the database connection, but for now
        //we're not using singletons, maybe should, still need to learn
        db.use_ns(&config().DATABASE_NS)
            .use_db(&config().DATABASE_DB)
            .await
            .map_err(Error::InitDB)?;

        Ok(ModelManager { db })
    }

    /// Returns the connection to the surrealdb only in the model layer
    pub(in crate::model) fn db(&self) -> &Database {
        &self.db
    }
}
