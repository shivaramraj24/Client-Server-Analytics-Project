use analytics_lib::query::{Aggregation, Query};
use analytics_lib::{csv::read_input_csv_file, query::Condition, solution};
use analytics_lib::dataset::{ColumnType, Dataset, Row, Value};

#[test]
fn test_albums() {
    let dataset = read_input_csv_file("../albums.csv");

    let query = Query::new(
        Condition::Equal(String::from("band"), Value::String(String::from("Meshuggah"))),
        String::from("album"),
        Aggregation::Average(String::from("rating")),
    );

    let result = solution::compute_query_on_dataset(&dataset, &query);


    let mut expected = Dataset::new(vec![
        (String::from("album"), ColumnType::String),
        (String::from("Average(rating)"), ColumnType::Integer),
    ]);
    expected.add_row(Row::new(vec![Value::String("Contradictions Collapse".to_string()), Value::Integer(1)]));
    expected.add_row(Row::new(vec![Value::String("Destroy Erase Improve".to_string()), Value::Integer(3)]));
    expected.add_row(Row::new(vec![Value::String("Chaosphere".to_string()), Value::Integer(1)]));
    expected.add_row(Row::new(vec![Value::String("Nothing".to_string()), Value::Integer(4)]));
    expected.add_row(Row::new(vec![Value::String("Catch Thirtythree".to_string()), Value::Integer(4)]));
    expected.add_row(Row::new(vec![Value::String("ObZen".to_string()), Value::Integer(4)]));
    expected.add_row(Row::new(vec![Value::String("Koloss".to_string()), Value::Integer(2)]));
    expected.add_row(Row::new(vec![Value::String("The Violent Sleep of Reason".to_string()), Value::Integer(3)]));
    expected.add_row(Row::new(vec![Value::String("Immutable".to_string()), Value::Integer(3)]));
}