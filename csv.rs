use std::iter::zip;
use csv::StringRecord;
use csv_sniffer::{Sniffer, Type};
use crate::dataset::{ColumnType, Dataset, Row, Value};

pub fn read_input_csv_file(filename: &str) -> Dataset {
    // Sniff CSV column types.
    let mut sniffer = Sniffer::new();
    let metadata = sniffer.sniff_path(filename).unwrap();
    let types = metadata.types;

    // Build header.
    let mut reader = ::csv::Reader::from_path(filename).unwrap();
    let header = reader.headers().unwrap();
    let columns = zip(header.into_iter(), types.iter())
        .map(|(cname, ctype)| {
            match ctype {
                Type::Text => return (cname.to_string(), ColumnType::String),
                Type::Unsigned | Type::Signed => return (cname.to_string(), ColumnType::Integer),
                _ => panic!("invalid value type in file"),
            }
        })
        .collect();

    // Build dataset one row at a time.
    let mut dataset = Dataset::new(columns);
    let mut record = StringRecord::new();
    while reader.read_record(&mut record).unwrap() {
        let mut values = Vec::new();
        for (value, ctype) in zip(record.into_iter(), types.iter()) {
            match ctype {
                Type::Text => {
                    values.push(Value::String(value.to_string()));
                },
                Type::Signed | Type::Unsigned => {
                    values.push(Value::Integer(value.parse().unwrap()));
                },
                _ => panic!("invalid value type in file"),
            }
        }
        dataset.add_row(Row::new(values));
    }

    return dataset;
}