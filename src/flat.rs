// use crate::flatten::SUBJECT_ID;
use csv::Writer;
use indexmap::IndexMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Clone, Debug)]
pub enum FlatField {
    Box_ {
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
    },
    Length {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
    },
    List {
        values: Vec<String>,
        value: String,
    },
    NoOp {
        value: String,
    },
    Point {
        x: f32,
        y: f32,
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

#[derive(Debug)]
pub struct FlatRow {
    pub row: IndexMap<String, FlatField>,
}

impl FlatRow {
    pub fn new() -> Self {
        FlatRow {
            row: IndexMap::new(),
        }
    }

    pub fn add_field(&mut self, column: &str, field: &FlatField) {
        unsafe {
            self.row.insert(column.to_owned(), field.clone());
        }
    }
}

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

    pub fn add_row(&mut self, row: FlatRow) {
        for (column, field) in row.row.iter() {
            if !self.columns.contains_key(column) {
                self.columns.insert(column.to_owned(), field.clone());
            }
        }
        self.rows.push(row);
    }

    pub fn write_csv(&self, csv_path: &Path) -> Result<(), Box<dyn Error>> {
        let mut writer =
            Writer::from_path(csv_path).expect("Could not open the unreconciled CSV file");

        _ = self.csv_header(&mut writer);

        for (i, row) in self.rows.iter().enumerate() {
            _ = self.csv_row(row, &mut writer);
        }

        Ok(())
    }

    fn csv_header(&self, writer: &mut Writer<File>) -> Result<(), Box<dyn Error>> {
        let mut output: Vec<String> = Vec::new();

        for (column, field_type) in self.columns.iter() {
            match &field_type {
                FlatField::Box_ {
                    left: _,
                    top: _,
                    right: _,
                    bottom: _,
                } => {
                    output.push(format!("{}: left", column));
                    output.push(format!("{}: top", column));
                    output.push(format!("{}: right", column));
                    output.push(format!("{}: bottom", column));
                }
                FlatField::Length {
                    x1: _,
                    y1: _,
                    x2: _,
                    y2: _,
                } => {
                    output.push(format!("{}: x1", column));
                    output.push(format!("{}: y1", column));
                    output.push(format!("{}: x2", column));
                    output.push(format!("{}: y2", column));
                }
                FlatField::List {
                    values: _,
                    value: _,
                } => {
                    output.push(column.to_string());
                }
                FlatField::NoOp { value: _ } => {
                    output.push(column.to_string());
                }
                FlatField::Point { x: _, y: _ } => {
                    output.push(format!("{}: x", column));
                    output.push(format!("{}: y", column));
                }
                FlatField::Same { value: _ } => {
                    output.push(column.to_string());
                }
                FlatField::Select { value: _ } => {
                    output.push(column.to_string());
                }
                FlatField::Text { value: _ } => {
                    output.push(column.to_string());
                }
            }
        }
        writer.write_record(output)?;
        Ok(())
    }

    fn csv_row(&self, row: &FlatRow, writer: &mut Writer<File>) -> Result<(), Box<dyn Error>> {
        let mut output: Vec<String> = Vec::new();

        for (header, field_type) in self.columns.iter() {
            if !row.row.contains_key(header) {
                output.push("".to_string());
            } else {
                let field: FlatField = row.row[header];
                match field_type {
                    FlatField::Box_ {
                        left,
                        top,
                        right,
                        bottom,
                    } => {
                        output.push(format!("{}", left.round()));
                        output.push(format!("{}", top.round()));
                        output.push(format!("{}", right.round()));
                        output.push(format!("{}", bottom.round()));
                    }
                    FlatField::Length { x1, y1, x2, y2 } => {
                        output.push(format!("{}", x1.round()));
                        output.push(format!("{}", y1.round()));
                        output.push(format!("{}", x2.round()));
                        output.push(format!("{}", y2.round()));
                    }
                    FlatField::List { values: _, value } => {
                        output.push(value.clone());
                    }
                    FlatField::NoOp { value } => {
                        output.push(value.clone());
                    }
                    FlatField::Point { x, y } => {
                        output.push(format!("{}", x.round()));
                        output.push(format!("{}", y.round()));
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
        writer.write_record(output)?;
        Ok(())
    }
}
