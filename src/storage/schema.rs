use crate::storage::column::Column;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SchemaError {
    #[error("schema init error: {0}")]
    SchemaInitError(String),
}

/// # schema
/// * schema is a data structure that stores table information.
/// * schema has a list of columns.
#[derive(Debug, PartialEq, Eq)]
pub struct Schema {
    pub columns: Vec<Column>,
}

impl Schema {
    pub fn new() -> Result<Self, SchemaError> {
        Ok(Self {
            columns: Vec::new(),
        })
    }
}
