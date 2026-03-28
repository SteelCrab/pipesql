#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Integer(i64),
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub values: Vec<Value>,
}
