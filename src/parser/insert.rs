use sqlparser::ast::{Expr, Query, SetExpr, Statement, Value, Values};

#[derive(PartialEq, Debug)]
pub struct InsertParser {
    pub table_name: String,
    pub columns: Vec<String>,
    pub values: Vec<Vec<String>>,
}

impl InsertParser {
    pub fn new(statement: &Statement) -> Result<InsertParser, String> {
        if let Statement::Insert {
            table_name,
            columns,
            source,
            ..
        } = statement {
            Ok(InsertParser {
                table_name: table_name.to_string(),
                columns: columns.iter().map(|c| c.to_string()).collect(),
                values: get_values_from_source(&source)
            })
        } else {
            Err(String::from("Cannot parse insert query"))
        }
    }
}

fn get_values_from_source(query: &Box<Query>) -> Vec<Vec<String>> {
    let mut all_vals: Vec<Vec<String>> = vec![];
    let Query {
        body,
        ..
    } = &**query;

    if let SetExpr::Values(values) = body {
        let Values(expressions) = values; 
        for i in expressions {
            let mut value_set: Vec<String> = vec![];
            for e in i {
                match e {
                    Expr::Value(v) => match v {
                        Value::Number(n, _) => {
                            value_set.push(n.to_string());
                        }
                        Value::Boolean(b) => match *b {
                            true => value_set.push("true".to_string()),
                            false => value_set.push("false".to_string()),
                        },
                        Value::SingleQuotedString(sqs) => {
                            value_set.push(sqs.to_string());
                        }
                        Value::Null => {
                            value_set.push("Null".to_string());
                        }
                        _ => {}
                    },
                    Expr::Identifier(i) => {
                        value_set.push(i.to_string());
                    }
                    _ => {}
                }
            }
            all_vals.push(value_set);
        }
    }

    return all_vals;
}