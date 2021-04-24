#![forbid(unsafe_code)]
// This crate has a library architecture for easier testing, with binaries that make use of the library in `src/bin`

mod load_env;
mod db;
mod schemas;
mod graphql;
mod oid;

pub use crate::graphql::{get_schema, AppSchema};
pub use crate::load_env::load_env;
