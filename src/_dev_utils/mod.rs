mod dev_db;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::local::{Db, File},
    Surreal,
};
use tracing::{debug, info};

use crate::config;

static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

#[derive(Serialize, Deserialize, Debug)]
struct User {
    first_name: String,
}

pub async fn init_dev() -> Result<(), Box<dyn std::error::Error>> {
    info!("HELLO");
    //Connect to db
    DB.connect::<File>(&config().DATABASE_PATH)
        .await
        .unwrap_or_else(|err| panic!("Could not connect to database, cause: {err:?}"));

    info!("Connected to database");

    DB.use_ns("test")
        .use_db("test")
        .await
        .unwrap_or_else(|err| {
            panic!("Could not set namespace or use designated db, cause: {err:?}")
        });

    // Initialize db data
    let _ = DB.query("DEFINE TABLE user SCHEMALESS").await?;

    let users: Vec<User> = DB.select("user").await?;

    debug!("{users:?}");

    Ok(())
}
