use crate::storage::schema::{Schema, SchemaError};
use crate::storage::value::Row;
use std::collections::HashMap;
use thiserror::Error;

/// table structure
#[derive(Debug, PartialEq, Eq)]
pub struct Table {
    pub name: TableName,
    pub schema: Schema,
    pub rows: HashMap<u64, Row>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TableName(String);

impl TableName {
    pub fn new(name: String) -> Result<Self, TableError> {
        if name.is_empty() {
            return Err(TableError::Init("Table name cannot be empty".to_string()));
        }
        if name.len() > 64 {
            return Err(TableError::Init(
                "Table name exceeds maximum length".to_string(),
            ));
        }
        if name.contains(" ") {
            return Err(TableError::Init("Table name cannot be space".to_string()));
        }
        Ok(Self(name))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Error)]
pub enum TableError {
    #[error("table init error: {0}")]
    Init(String),

    #[error("schema error: {0}")]
    Schema(#[from] SchemaError),
}

impl Table {
    pub fn new(name: String) -> Result<Self, TableError> {
        Ok(Self {
            name: TableName::new(name)?,
            schema: Schema::new()?,
            rows: HashMap::new(),
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_table_name() -> Result<(), TableError> {
        let table_name = TableName::new(String::from("pista"))?;
        assert_eq!(table_name.as_str(), "pista".to_string());
        Ok(())
    }

    #[test]
    fn err_table_name_empty() {
        let err_table_name = TableName::new(String::new());
        //error check
        assert!(err_table_name.is_err());
        match err_table_name {
            Err(TableError::Init(msg)) => assert_eq!(msg, "Table name cannot be empty"),
            _ => panic!("Expected TableError::Init"),
        }
    }
    #[test]
    fn err_table_name_maximum() {
        let table_name: String = "pista".repeat(20);

        let err_table_name = TableName::new(table_name);

        // error check
        assert!(err_table_name.is_err());
        match err_table_name {
            Err(TableError::Init(msg)) => assert_eq!(msg, "Table name exceeds maximum length"),
            _ => panic!("Expected TableError::Init"),
        }
    }
}
