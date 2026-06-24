use std::fmt::{Debug, Display};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum ColumnType {
    String,
    Integer,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Hash, Eq, Debug, PartialOrd, Ord)]
pub enum Value {
    String(String),
    Integer(i32),
}
impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::String(value) => value.to_string(),
            Value::Integer(value) => value.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Row {
    values: Vec<Value>,
}
impl Row {
    pub fn new(values: Vec<Value>) -> Row {
        return Row { values };
    }
    pub fn get_values(&self) -> &Vec<Value> {
        return &self.values;
    }
    pub fn get_value(&self, index: usize) -> &Value {
        return &self.values[index];
    }
    pub fn move_values(self) -> Vec<Value> {
        return self.values;
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Dataset {
    columns: Vec<(String, ColumnType)>,
    rows: Vec<Row>,
}
impl Dataset {
    pub fn new(columns: Vec<(String, ColumnType)>) -> Dataset {
        return Dataset {
            columns,
            rows: Vec::new(),
        };
    }
    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub fn columns(&self) -> &Vec<(String, ColumnType)> {
        return &self.columns;
    }
    pub fn column_type(&self, column_name: &String) -> &ColumnType {
       let i = self.column_index(column_name);
        return &self.columns[i].1;
    }
    pub fn column_index(&self, column_name: &String) -> usize {
        for i in 0..self.columns.len() {
            let (cname, _ctype) = &self.columns[i];
            if cname == column_name {
                return i;
            }
        }
        panic!("Column {} not found", column_name);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Row> {
        return self.rows.iter();
    }

    pub fn into_iter(self) -> std::vec::IntoIter<Row> {
        return self.rows.into_iter();
    }

    pub fn len(&self) -> usize {
        return self.rows.len();
    }
}

impl Debug for Dataset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Dataset:")?;
        write!(f, "|")?;
        for (colname, coltype) in &self.columns {
            let description = format!("{colname}: {coltype:?}");
            write!(f, " {description: <28}|")?;
        }
        writeln!(f, "")?;

        write!(f, "|")?;
        for _ in &self.columns {
            write!(f, "=============================|")?;
        }
        writeln!(f, "")?;

        for row in &self.rows {
            write!(f, "|")?;
            for value in row.get_values() {
                write!(f, " {: <28}|", value.to_string())?;
            }
            writeln!(f, "")?;
        }
        return Ok(());
    }
}
impl Display for Dataset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return Debug::fmt(self, f);
    }
}

impl PartialEq for Dataset {
    fn eq(&self, other: &Self) -> bool {
        if self.columns != other.columns {
            return false;
        }

        let mut rows = self.rows.clone();
        rows.sort();
        let mut rows2 = other.rows.clone();
        rows2.sort();

        return rows == rows2;
    }
}