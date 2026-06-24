use std::collections::HashMap;

use analytics_lib::{csv::read_input_csv_file, query::Condition, solution};
use analytics_lib::dataset::{ColumnType, Dataset, Row, Value};

#[test]
fn test_group_by_section() {
    let dataset = read_input_csv_file("../grades.csv");
    let result = solution::group_by_dataset(dataset, &String::from("section"));

    let mut expected = HashMap::new();
    
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

    expected.insert(Value::String(String::from("A1")), a1);
    expected.insert(Value::String(String::from("B1")), b1);

    assert_eq!(result, expected)
}

#[test]
fn test_group_by_grade() {
    let dataset = read_input_csv_file("../grades.csv");
    let result = solution::group_by_dataset(dataset, &String::from("grade"));

    let mut expected = HashMap::new();
    
    let mut d80 = Dataset::new(vec![
        (String::from("name"), ColumnType::String),
        (String::from("section"), ColumnType::String),
        (String::from("grade"), ColumnType::Integer),
    ]);
    let mut d85 = Dataset::new(vec![
        (String::from("name"), ColumnType::String),
        (String::from("section"), ColumnType::String),
        (String::from("grade"), ColumnType::Integer),
    ]);
    let mut d90 = Dataset::new(vec![
        (String::from("name"), ColumnType::String),
        (String::from("section"), ColumnType::String),
        (String::from("grade"), ColumnType::Integer),
    ]);
    let mut d100 = Dataset::new(vec![
        (String::from("name"), ColumnType::String),
        (String::from("section"), ColumnType::String),
        (String::from("grade"), ColumnType::Integer),
    ]);
    d80.add_row(Row::new(vec![Value::String("Alice".to_string()), Value::String("A1".to_string()), Value::Integer(80)]));
    d85.add_row(Row::new(vec![Value::String("Carol".to_string()), Value::String("B1".to_string()), Value::Integer(85)]));
    d90.add_row(Row::new(vec![Value::String("Bob".to_string()), Value::String("A1".to_string()), Value::Integer(90)]));
    d90.add_row(Row::new(vec![Value::String("Corinn".to_string()), Value::String("B1".to_string()), Value::Integer(90)]));
    d100.add_row(Row::new(vec![Value::String("Sophie".to_string()), Value::String("A1".to_string()), Value::Integer(100)]));

    expected.insert(Value::Integer(80), d80);
    expected.insert(Value::Integer(85), d85);
    expected.insert(Value::Integer(90), d90);
    expected.insert(Value::Integer(100), d100);

    assert_eq!(result, expected)
}
