use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::local::{Db, File},
    Surreal,
};
use tracing::info;

use crate::config;

// static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

#[derive(Serialize, Deserialize, Debug)]
struct User {
    first_name: String,
}

pub async fn init_dev() -> Result<(), surrealdb::Error> {
    //Connect to db
    let db = Surreal::new::<File>(&config().DATABASE_PATH)
        .await
        .unwrap_or_else(|err| panic!("Could not connect to database, cause: {err:?}"));

    info!("Connected to database");

    db.use_ns("test")
        .use_db("test")
        .await
        .unwrap_or_else(|err| {
            panic!("Could not set namespace or use designated db, cause: {err:?}")
        });

    // Initialize db data
    db.query("DEFINE TABLE user SCHEMALESS").await?;

    Ok(())
}
