pub use anyhow::{bail, ensure, Context, Error, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum YaguiError {
    #[error("invalid YAML")]
    InvalidYaml,

    #[error("missing value with key '{0}' of type '{1}' in YAML")]
    MissingYamlValue(String, &'static str),
}
