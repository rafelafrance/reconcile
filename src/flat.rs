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

pub type FlatRow = Vec<(String, FlatField)>;

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
        // TODO sort self.order!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        // TODO Handle ragged rows!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        for (name, field) in row.iter() {
            if !self.types.contains_key(name) {
                self.add_column(name, field);
            }
            let entry = self.columns.entry(name.to_string()).or_insert(Vec::new());
            entry.push(field.clone());
        }
    }

    fn add_column(&mut self, name: &str, field: &FlatField) {
        self.order.push(name.to_string());
        self.types.insert(name.to_string(), field.clone());
        let len = self.len();
        let entry = self.columns.entry(name.to_string()).or_insert(Vec::new());
        entry.extend(iter::repeat(FlatField::Null).take(len));
    }

    fn len(&self) -> usize {
        match self.order.first() {
            Some(name) => self.columns[name].len(),
            _ => 0,
        }
    }

    pub fn to_df(&self) -> DataFrame {
        let mut columns: Vec<Series> = Vec::new();
        columns.push(Series::new("a", &[1i32, 2, 3]));
        columns.push(Series::new("b", &[10i32, 20, 30]));
        for header in self.order.iter() {
            match self.types[header] {
                FlatField::Box_ {
                    left: _,
                    top: _,
                    right: _,
                    bottom: _,
                } => {
                    columns.push(Series::new(
                        &format!("{}: left", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::Box_ {
                                    left,
                                    top: _,
                                    right: _,
                                    bottom: _,
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
                                    left: _,
                                    top,
                                    right: _,
                                    bottom: _,
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
                                    left: _,
                                    top: _,
                                    right,
                                    bottom: _,
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
                                    left: _,
                                    top: _,
                                    right: _,
                                    bottom,
                                } => Some(bottom.clone() as i32),
                                _ => None,
                            })
                            .collect::<Vec<Option<i32>>>(),
                    ));
                }
                FlatField::Length {
                    x1: _,
                    y1: _,
                    x2: _,
                    y2: _,
                } => {
                    columns.push(Series::new(
                        &format!("{}: x1", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::Length {
                                    x1,
                                    y1: _,
                                    x2: _,
                                    y2: _,
                                } => Some(x1.clone() as i32),
                                _ => None,
                            })
                            .collect::<Vec<Option<i32>>>(),
                    ));
                    columns.push(Series::new(
                        &format!("{}: y1", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::Length {
                                    x1: _,
                                    y1,
                                    x2: _,
                                    y2: _,
                                } => Some(y1.clone() as i32),
                                _ => None,
                            })
                            .collect::<Vec<Option<i32>>>(),
                    ));
                    columns.push(Series::new(
                        &format!("{}: x2", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::Length {
                                    x1: _,
                                    y1: _,
                                    x2,
                                    y2: _,
                                } => Some(x2.clone() as i32),
                                _ => None,
                            })
                            .collect::<Vec<Option<i32>>>(),
                    ));
                    columns.push(Series::new(
                        &format!("{}: y2", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::Length {
                                    x1: _,
                                    y1: _,
                                    x2: _,
                                    y2,
                                } => Some(y2.clone() as i32),
                                _ => None,
                            })
                            .collect::<Vec<Option<i32>>>(),
                    ));
                }
                FlatField::List {
                    values: _,
                    value: _,
                } => {
                    columns.push(Series::new(
                        &format!("{}", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::List { values: _, value } => Some(value.clone()),
                                _ => None,
                            })
                            .collect::<Vec<Option<String>>>(),
                    ));
                }
                FlatField::NoOp { value: _ } => {
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
                FlatField::Point { x: _, y: _ } => {
                    columns.push(Series::new(
                        &format!("{}: x", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::Point { x, y: _ } => Some(x.clone() as i32),
                                _ => None,
                            })
                            .collect::<Vec<Option<i32>>>(),
                    ));
                    columns.push(Series::new(
                        &format!("{}: y", header),
                        self.columns[header]
                            .iter()
                            .map(|x| match x {
                                FlatField::Point { x: _, y } => Some(y.clone() as i32),
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
}
