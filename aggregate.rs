use std::collections::HashMap;
use std::f32::consts::E;

use analytics_lib::query::Aggregation;
use analytics_lib::{csv::read_input_csv_file, query::Condition, solution};
use analytics_lib::dataset::{ColumnType, Dataset, Row, Value};

#[test]
fn test_aggregate_average_grade_per_section() {
    // Construct the data
    let mut data = HashMap::new();
    
    let mut a1 = Dataset::new(vec![
        (String::from("name"), ColumnType::String),
        (String::from("section"), ColumnType::String),
        (String::from("grade"), ColumnType::Integer),
    ]);
    a1.add_row(Row::new(vec![Value::String("Alice".to_string()), Value::String("A1".to_string()), Value::Integer(80)]));
    a1.add_row(Row::new(vec![Value::String("Bob".to_string()), Value::String("A1".to_string()), Value::Integer(90)]));
    a1.add_row(Row::new(vec![Value::String("Sophie".to_string()), Value::String("A1".to_string()), Value::Integer(100)]));

    let mut b1 = Dataset::new(vec![
        (String::from("name"), ColumnType::String),
        (String::from("section"), ColumnType::String),
        (String::from("grade"), ColumnType::Integer),
    ]);
    b1.add_row(Row::new(vec![Value::String("Carol".to_string()), Value::String("B1".to_string()), Value::Integer(85)]));
    b1.add_row(Row::new(vec![Value::String("Corinn".to_string()), Value::String("B1".to_string()), Value::Integer(90)]));

    data.insert(Value::String(String::from("A1")), a1);
    data.insert(Value::String(String::from("B1")), b1);

    // Check that the average grade per section is correct.
    let aggregation = Aggregation::Average(String::from("grade"));
    let result = solution::aggregate_dataset(data, &aggregation);

    let mut expected = HashMap::new();
    expected.insert(Value::String(String::from("A1")), Value::Integer(90));
    expected.insert(Value::String(String::from("B1")), Value::Integer(87));

    assert_eq!(result, expected);
}
