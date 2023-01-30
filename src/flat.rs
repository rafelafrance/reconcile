use crate::flatten;
use csv::Writer;
use indexmap::IndexMap;
use std::error::Error;
use std::path::Path;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum FlatField {
    Box_ {
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    },
    Length {
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    },
    List {
        values: Vec<String>,
        value: String,
    },
    NoOp {
        value: String,
    },
    Point {
        x: i32,
        y: i32,
    },
    Same {
        value: String,
    },
    Select {
        value: String,
    },
    Text {
        value: String,
    },
}

pub type FlatRow = IndexMap<String, FlatField>;

#[derive(Debug)]
pub struct Flat {
    pub workflow_id: String,
    pub workflow_name: String,
    columns: IndexMap<String, FlatField>,
    rows: Vec<FlatRow>,
}

impl Flat {
    pub fn new(workflow_id: &str, workflow_name: &str) -> Self {
        Flat {
            workflow_id: workflow_id.to_string(),
            workflow_name: workflow_name.to_string(),
            columns: IndexMap::new(),
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: &FlatRow) {
        for (column, field) in row.iter() {
            if !self.columns.contains_key(column) {
                self.columns.insert(column.to_owned(), field.clone());
            }
        }
        self.rows.push(row.to_owned());
    }

    pub fn sort(&mut self) {
        self.rows
            .sort_unstable_by_key(|row| row[flatten::SUBJECT_ID].clone());
    }

    pub fn group(&self) -> IndexMap<String, Vec<FlatRow>> {
        let mut grouped: IndexMap<String, Vec<FlatRow>> = IndexMap::new();
        for row in &self.rows {
            let key = match row.get(flatten::SUBJECT_ID).unwrap() {
                FlatField::Same { value } => value,
                _ => panic!("Missing key field."),
            };
            if !grouped.contains_key(key) {
                grouped.insert(key.to_string(), Vec::new());
            }
        }
        grouped
    }

    pub fn write_csv(&self, csv_path: &Path) -> Result<(), Box<dyn Error>> {
        let mut writer =
            Writer::from_path(csv_path).expect("Could not write to the unreconciled CSV file");

        let mut output = self.csv_header();
        writer.write_record(output)?;

        for row in self.rows.iter() {
            output = self.csv_row(row);
            writer.write_record(output)?;
        }

        Ok(())
    }

    fn csv_header(&self) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();

        for (column, field_type) in self.columns.iter() {
            match &field_type {
                FlatField::Box_ { .. } => {
                    output.push(format!("{}: left", column));
                    output.push(format!("{}: top", column));
                    output.push(format!("{}: right", column));
                    output.push(format!("{}: bottom", column));
                }
                FlatField::Length { .. } => {
                    output.push(format!("{}: x1", column));
                    output.push(format!("{}: y1", column));
                    output.push(format!("{}: x2", column));
                    output.push(format!("{}: y2", column));
                }
                FlatField::List { .. } => {
                    output.push(column.to_string());
                }
                FlatField::NoOp { .. } => {
                    output.push(column.to_string());
                }
                FlatField::Point { .. } => {
                    output.push(format!("{}: x", column));
                    output.push(format!("{}: y", column));
                }
                FlatField::Same { .. } => {
                    output.push(column.to_string());
                }
                FlatField::Select { .. } => {
                    output.push(column.to_string());
                }
                FlatField::Text { .. } => {
                    output.push(column.to_string());
                }
            }
        }
        output
    }

    fn csv_row(&self, row: &FlatRow) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();

        for (header, _) in self.columns.iter() {
            if !row.contains_key(header) {
                output.push("".to_string());
            } else {
                let field: &FlatField = row.get(header).unwrap();
                match field {
                    FlatField::Box_ {
                        left,
                        top,
                        right,
                        bottom,
                    } => {
                        output.push(format!("{}", left));
                        output.push(format!("{}", top));
                        output.push(format!("{}", right));
                        output.push(format!("{}", bottom));
                    }
                    FlatField::Length { x1, y1, x2, y2 } => {
                        output.push(format!("{}", x1));
                        output.push(format!("{}", y1));
                        output.push(format!("{}", x2));
                        output.push(format!("{}", y2));
                    }
                    FlatField::List { values: _, value } => {
                        output.push(value.clone());
                    }
                    FlatField::NoOp { value } => {
                        output.push(value.clone());
                    }
                    FlatField::Point { x, y } => {
                        output.push(format!("{}", x));
                        output.push(format!("{}", y));
                    }
                    FlatField::Same { value } => {
                        output.push(value.clone());
                    }
                    FlatField::Select { value } => {
                        output.push(value.clone());
                    }
                    FlatField::Text { value } => {
                        output.push(value.clone());
                    }
                }
            }
        }
        output
    }
}
