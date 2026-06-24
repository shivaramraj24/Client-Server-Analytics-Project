use analytics_lib::{csv::read_input_csv_file, query::Condition, solution};
use analytics_lib::dataset::{ColumnType, Dataset, Row, Value};

#[test]
fn test_filter_equals() {
    let dataset = read_input_csv_file("../grades.csv");
    let condition = Condition::Equal(
        String::from("section"),
        Value::String(String::from("A1"))
    );

    let result = solution::filter_dataset(&dataset, &condition);

    let mut expected = Dataset::new(vec![
        (String::from("name"), ColumnType::String),
        (String::from("section"), ColumnType::String),
        (String::from("grade"), ColumnType::Integer),
    ]);
    expected.add_row(Row::new(vec![Value::String("Alice".to_string()), Value::String("A1".to_string()), Value::Integer(80)]));
    expected.add_row(Row::new(vec![Value::String("Bob".to_string()), Value::String("A1".to_string()), Value::Integer(90)]));
    expected.add_row(Row::new(vec![Value::String("Sophie".to_string()), Value::String("A1".to_string()), Value::Integer(100)]));

    assert_eq!(result, expected)
}


#[test]
fn test_filter_not_equals() {
    let dataset = read_input_csv_file("../grades.csv");
    let condition = Condition::Not(Box::new(Condition::Equal(
        String::from("section"),
        Value::String(String::from("A1"))
    )));

    let result = solution::filter_dataset(&dataset, &condition);

    let mut expected = Dataset::new(vec![
        (String::from("name"), ColumnType::String),
        (String::from("section"), ColumnType::String),
        (String::from("grade"), ColumnType::Integer),
    ]);
    expected.add_row(Row::new(vec![Value::String("Carol".to_string()), Value::String("B1".to_string()), Value::Integer(85)]));
    expected.add_row(Row::new(vec![Value::String("Corinn".to_string()), Value::String("B1".to_string()), Value::Integer(90)]));
    assert_eq!(result, expected)
}



#[test]
fn test_filter_and() {
    let dataset = read_input_csv_file("../grades.csv");
    let condition = Condition::And(
        Box::new(Condition::Equal(
            String::from("section"),
            Value::String(String::from("A1"))
        )),
        Box::new(Condition::Equal(
            String::from("grade"),
            Value::Integer(90)
        ))
    );

    let result = solution::filter_dataset(&dataset, &condition);

    let mut expected = Dataset::new(vec![
        (String::from("name"), ColumnType::String),
        (String::from("section"), ColumnType::String),
        (String::from("grade"), ColumnType::Integer),
    ]);
    expected.add_row(Row::new(vec![Value::String("Bob".to_string()), Value::String("A1".to_string()), Value::Integer(90)]));
    assert_eq!(result, expected)
}


#[test]
fn test_filter_or() {
    let dataset = read_input_csv_file("../grades.csv");
    let condition = Condition::Or(
        Box::new(Condition::Equal(
            String::from("section"),
            Value::String(String::from("A1"))
        )),
        Box::new(Condition::Equal(
            String::from("grade"),
            Value::Integer(90)
        ))
    );

    let result = solution::filter_dataset(&dataset, &condition);

    let mut expected = Dataset::new(vec![
        (String::from("name"), ColumnType::String),
        (String::from("section"), ColumnType::String),
        (String::from("grade"), ColumnType::Integer),
    ]);
    expected.add_row(Row::new(vec![Value::String("Alice".to_string()), Value::String("A1".to_string()), Value::Integer(80)]));
    expected.add_row(Row::new(vec![Value::String("Bob".to_string()), Value::String("A1".to_string()), Value::Integer(90)]));
    expected.add_row(Row::new(vec![Value::String("Sophie".to_string()), Value::String("A1".to_string()), Value::Integer(100)]));
    expected.add_row(Row::new(vec![Value::String("Corinn".to_string()), Value::String("B1".to_string()), Value::Integer(90)]));
    assert_eq!(result, expected)
}