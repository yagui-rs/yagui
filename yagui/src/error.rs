pub use anyhow::{bail, ensure, Context, Error, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum YaguiError {
    #[error("invalid YAML")]
    InvalidYaml,
}
