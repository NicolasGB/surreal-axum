mod error;
mod store;
pub mod task;

pub use self::error::{Error, Result};
use crate::{
    config,
    model::store::{Database, DB},
};

#[derive(Debug, Clone)]
pub struct ModelManager {
    db: Database,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        //Setup the db namespace this might be better to do in the database connection, but for now
        //we're not using singletons, maybe should, still need to learn
        DB.use_ns(&config().DATABASE_NS)
            .use_db(&config().DATABASE_DB)
            .await
            .map_err(Error::InitDB)?;

        Ok(ModelManager { db: DB.clone() })
    }

    /// Returns the connection to the surrealdb only in the model layer
    pub(in crate::model) fn db(&self) -> &Database {
        &self.db
    }
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{_dev_utils::init_test, ctx::Ctx};
    use anyhow::Result;

    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // Setup & fixtures
        let mm = init_test().await;
        let ctx = Ctx::root_ctx();
        Ok(())
    }
}

// endregion: --- Tests
