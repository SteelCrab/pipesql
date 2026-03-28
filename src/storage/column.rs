//! # column
//! * column is a data structure that stores data in a table.
//! * column has a name and a data type.
#[derive(Debug, PartialEq, Eq)]
pub enum ColumnType {
    Integer,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Column {
    name: String,
    data_type: ColumnType,
}

impl Column {
    /// * 인자를 받아 구조체로 구현
    pub fn new(name: String, data_type: ColumnType) -> Self {
        Self { name, data_type }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn data_type(&self) -> &ColumnType {
        &self.data_type
    }
}
