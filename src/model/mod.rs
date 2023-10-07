mod error;

pub use self::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct ModelManager {}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        //TODO: 07/10/2023 - implemented later
        Ok(ModelManager {})
    }
}
