#[cfg(test)]
mod tests {

    use crate::mvcc::{Row, Value};
    use serde_json::{self};
    #[test]
    fn test_row_create() {
        let row = Row {
            values: vec![
                Value::Integer(1),
                Value::String("test".to_string()),
                Value::Boolean(true),
            ],
        };
        assert_eq!(row.values.len(), 3);

        //serialization && deserialization test
        let json = serde_json::to_string(&row).unwrap();
        let deserialized_row: Row = serde_json::from_str(&json).unwrap();
        assert_eq!(json, serde_json::to_string(&deserialized_row).unwrap());
        assert_eq!(row.values, deserialized_row.values);
    }
}
