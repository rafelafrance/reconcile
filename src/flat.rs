use polars::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::iter;

#[derive(Clone, Debug, Deserialize)]
pub enum FlatField {
    Null,
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

pub type FlatRow<'a> = Vec<(&'a str, FlatField)>;

#[derive(Debug, Default)]
pub struct Flat {
    pub workflow_id: String,
    pub workflow_name: String,
    pub columns: HashMap<String, Vec<FlatField>>,
    order: Vec<String>,
    types: HashMap<String, FlatField>,
}

impl Flat {
    pub fn new(workflow_id: &str, workflow_name: &str) -> Self {
        Flat {
            workflow_id: workflow_id.to_string(),
            workflow_name: workflow_name.to_string(),
            columns: HashMap::new(),
            order: Vec::new(),
            types: HashMap::new(),
        }
    }

    pub fn add_row(&mut self, row: FlatRow) {
        for (name, field) in row.iter() {
            if !self.types.contains_key(name.clone()) {
                self.add_column(name, field);
            }
            self.columns[name.clone()].push(field.clone());
        }
    }

    pub fn to_df(&self) -> DataFrame {
        let columns: Vec<Series> = Vec::new();
        columns.push(Series::new("a", &[1i32, 2, 3]));
        columns.push(Series::new("b", &[10i32, 20, 30]));
        for header in self.order.iter() {
            match self.types[header] {
                FlatField::Box_ {
                    left,
                    top,
                    right,
                    bottom,
                } => {
                    columns.push(Series::new(
                        &format!("{}: left", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::Box_ {
                                    left,
                                    top,
                                    right,
                                    bottom,
                                } => Some(left.clone() as i32),
                                _ => None,
                            })
                            .collect::<Vec<Option<i32>>>(),
                    ));
                    columns.push(Series::new(
                        &format!("{}: top", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::Box_ {
                                    left,
                                    top,
                                    right,
                                    bottom,
                                } => Some(top.clone() as i32),
                                _ => None,
                            })
                            .collect::<Vec<Option<i32>>>(),
                    ));
                    columns.push(Series::new(
                        &format!("{}: right", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::Box_ {
                                    left,
                                    top,
                                    right,
                                    bottom,
                                } => Some(right.clone() as i32),
                                _ => None,
                            })
                            .collect::<Vec<Option<i32>>>(),
                    ));
                    columns.push(Series::new(
                        &format!("{}: bottom", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::Box_ {
                                    left,
                                    top,
                                    right,
                                    bottom,
                                } => Some(bottom.clone() as i32),
                                _ => None,
                            })
                            .collect::<Vec<Option<i32>>>(),
                    ));
                }
                FlatField::Length { x1, y1, x2, y2 } => {
                    columns.push(Series::new(
                        &format!("{}: x1", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::Length { x1, y1, x2, y2 } => Some(x1.clone() as i32),
                                _ => None,
                            })
                            .collect::<Vec<Option<i32>>>(),
                    ));
                    columns.push(Series::new(
                        &format!("{}: y1", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::Length { x1, y1, x2, y2 } => Some(y1.clone() as i32),
                                _ => None,
                            })
                            .collect::<Vec<Option<i32>>>(),
                    ));
                    columns.push(Series::new(
                        &format!("{}: x2", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::Length { x1, y1, x2, y2 } => Some(x2.clone() as i32),
                                _ => None,
                            })
                            .collect::<Vec<Option<i32>>>(),
                    ));
                    columns.push(Series::new(
                        &format!("{}: y2", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::Length { x1, y1, x2, y2 } => Some(y2.clone() as i32),
                                _ => None,
                            })
                            .collect::<Vec<Option<i32>>>(),
                    ));
                }
                FlatField::List { values, value } => {
                    columns.push(Series::new(
                        &format!("{}", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::List { values, value } => Some(value.clone()),
                                _ => None,
                            })
                            .collect::<Vec<Option<String>>>(),
                    ));
                }
                FlatField::NoOp { value } => {
                    columns.push(Series::new(
                        &format!("{}", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::NoOp { value } => Some(value.clone()),
                                _ => None,
                            })
                            .collect::<Vec<Option<String>>>(),
                    ));
                }
                FlatField::Point { x, y } => {
                    columns.push(Series::new(
                        &format!("{}: x", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::Point { x, y } => Some(x.clone() as i32),
                                _ => None,
                            })
                            .collect::<Vec<Option<i32>>>(),
                    ));
                    columns.push(Series::new(
                        &format!("{}: y", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::Point { x, y } => Some(y.clone() as i32),
                                _ => None,
                            })
                            .collect::<Vec<Option<i32>>>(),
                    ));
                }
                FlatField::Same { value: _ } => {
                    columns.push(Series::new(
                        &format!("{}", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::Same { value } => Some(value.clone()),
                                _ => None,
                            })
                            .collect::<Vec<Option<String>>>(),
                    ));
                }
                FlatField::Select { value: _ } => {
                    columns.push(Series::new(
                        &format!("{}", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::Select { value } => Some(value.clone()),
                                _ => None,
                            })
                            .collect::<Vec<Option<String>>>(),
                    ));
                }
                FlatField::Text { value: _ } => {
                    columns.push(Series::new(
                        &format!("{}", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::Text { value } => Some(value.clone()),
                                _ => None,
                            })
                            .collect::<Vec<Option<String>>>(),
                    ));
                }
                FlatField::Null => {}
            }
        }
        let df = DataFrame::new(columns).unwrap();
        df
    }

    fn add_column(&mut self, name: &str, field: &FlatField) {
        let name = name.to_owned();
        self.order.push(name.clone());
        self.types.insert(name, field.clone());
        self.columns[&name] = Vec::new();
        self.columns[&name].extend(iter::repeat(FlatField::Null).take(self.len()));
    }

    fn len(&self) -> usize {
        match self.order.first() {
            Some(name) => self.columns[name].len(),
            _ => 0,
        }
    }
}
