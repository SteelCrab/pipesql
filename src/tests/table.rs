use crate::storage::{
    column::{Column, ColumnType},
    table::Table,
    value::{Row, Value},
};

#[test]
fn create_table_and_insert_row() {
    let mut table = Table::new("users".to_string()).unwrap();
    table
        .schema
        .columns
        .push(Column::new("id".to_string(), ColumnType::Integer));

    let row = Row {
        values: vec![Value::Integer(1)],
    };
    table.rows.insert(0, row.clone());

    assert_eq!(table.rows.len(), 1);
    assert_eq!(table.rows.get(&0), Some(&row));
}
