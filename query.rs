use crate::dataset::Value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Condition {
    Equal(String, Value),
    Not(Box<Condition>),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
}

#[derive(Serialize, Deserialize, Debug)]

pub enum Aggregation {
    Count(String),
    Sum(String),
    Average(String),
}
impl Aggregation {
    pub fn get_result_column_name(&self) -> String {
        match self {
            Aggregation::Count(column_name) => format!("Count({column_name})"),
            Aggregation::Sum(column_name) => format!("Sum({column_name})"),
            Aggregation::Average(column_name) => format!("Average({column_name})"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct Query {
    filter: Condition,
    group_by: String,
    aggregate: Aggregation,
}
impl Query {
    pub fn new(filter: Condition, group_by: String, aggregate: Aggregation) -> Query {
        return Query {
            filter,
            group_by,
            aggregate,
        };
    }

    pub fn get_filter(&self) -> &Condition {
        return &self.filter;
    }
    pub fn get_group_by(&self) -> &String {
        return &self.group_by;
    }
    pub fn get_aggregate(&self) -> &Aggregation {
        return &self.aggregate;
    }
}