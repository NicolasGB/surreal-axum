mod error;
pub mod mw_auth;
pub mod mw_res_map;
pub mod static_routes;

pub use self::error::ClientError;
pub use self::error::{Error, Result};

pub const AUTH_TOKEN: &str = "auth-token";
