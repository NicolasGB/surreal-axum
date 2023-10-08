use crate::{Error, Result};
use std::{env, sync::OnceLock};

//We want to get the config only once
pub fn config() -> &'static Config {
    //First the instance is empty
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|err| panic!("FATAL WHILE LOADING CONFIG. Cause: {err:?}"))
    })
}

#[allow(non_snake_case)]
pub struct Config {
    pub WEB_FOLDER: String,

    pub DATABASE_PATH: String,
    pub DATABASE_NS: String,
    pub DATABASE_DB: String,
}

impl Config {
    pub fn load_from_env() -> Result<Config> {
        Ok(Config {
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
            DATABASE_PATH: get_env("SERVICE_DATABASE_PATH")?,
            DATABASE_NS: get_env("SERVICE_DATABASE_NAMESPACE")?,
            DATABASE_DB: get_env("SERVICE_DATABASE_DB")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}
