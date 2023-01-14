use indexmap::IndexMap;
use polars::prelude::*;
use serde::Deserialize;
use std::iter;

pub const SUBJECT_ID: &str = "subject_id";

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

pub type FlatRow = IndexMap<String, FlatField>;

#[derive(Debug, Default)]
pub struct Flat {
    pub workflow_id: String,
    pub workflow_name: String,
    columns: IndexMap<String, Vec<FlatField>>,
    types: IndexMap<String, FlatField>,
    row_count: usize,
}

impl Flat {
    pub fn new(workflow_id: &str, workflow_name: &str) -> Self {
        Flat {
            workflow_id: workflow_id.to_string(),
            workflow_name: workflow_name.to_string(),
            columns: IndexMap::new(),
            types: IndexMap::new(),
            row_count: 0,
        }
    }

    pub fn add_row(&mut self, row: FlatRow) {
        for (header, field) in row.iter() {
            if !self.columns.contains_key(header) {
                self.columns.insert(header.to_string(), Vec::new());
                self.columns[header].extend(iter::repeat(FlatField::Null).take(self.row_count));
                self.types.insert(header.to_string(), field.clone());
            }
            self.columns[header].push(field.clone());
        }
        for (header, _) in self.types.iter() {
            if !row.contains_key(header) {
                self.columns.get_mut(header).unwrap().push(FlatField::Null);
            }
        }
        self.row_count += 1;
    }

    pub fn to_df(&self) -> DataFrame {
        let mut columns: Vec<Series> = Vec::new();
        for (header, field_type) in self.types.iter() {
            match field_type {
                FlatField::Box_ {
                    left: _,
                    top: _,
                    right: _,
                    bottom: _,
                } => {
                    let mut lefts: Vec<Option<i32>> = Vec::with_capacity(self.row_count);
                    let mut tops: Vec<Option<i32>> = Vec::with_capacity(self.row_count);
                    let mut rights: Vec<Option<i32>> = Vec::with_capacity(self.row_count);
                    let mut bottoms: Vec<Option<i32>> = Vec::with_capacity(self.row_count);
                    for value in self.columns[header].iter() {
                        match value {
                            FlatField::Box_ {
                                left,
                                top,
                                right,
                                bottom,
                            } => {
                                lefts.push(Some(left.clone() as i32));
                                tops.push(Some(top.clone() as i32));
                                rights.push(Some(right.clone() as i32));
                                bottoms.push(Some(bottom.clone() as i32));
                            }
                            _ => {
                                lefts.push(None);
                                tops.push(None);
                                rights.push(None);
                                bottoms.push(None);
                            }
                        };
                    }
                    columns.push(Series::new(&format!("{}: left", header), lefts));
                    columns.push(Series::new(&format!("{}: top", header), tops));
                    columns.push(Series::new(&format!("{}: right", header), rights));
                    columns.push(Series::new(&format!("{}: bottom", header), bottoms));
                }
                FlatField::Length {
                    x1: _,
                    y1: _,
                    x2: _,
                    y2: _,
                } => {
                    let mut x1s: Vec<Option<i32>> = Vec::with_capacity(self.row_count);
                    let mut y1s: Vec<Option<i32>> = Vec::with_capacity(self.row_count);
                    let mut x2s: Vec<Option<i32>> = Vec::with_capacity(self.row_count);
                    let mut y2s: Vec<Option<i32>> = Vec::with_capacity(self.row_count);
                    for value in self.columns[header].iter() {
                        match value {
                            FlatField::Length { x1, y1, x2, y2 } => {
                                x1s.push(Some(x1.clone() as i32));
                                y1s.push(Some(y1.clone() as i32));
                                x2s.push(Some(x2.clone() as i32));
                                y2s.push(Some(y2.clone() as i32));
                            }
                            _ => {
                                x1s.push(None);
                                y1s.push(None);
                                x2s.push(None);
                                y2s.push(None);
                            }
                        };
                    }
                    columns.push(Series::new(&format!("{}: x1", header), x1s));
                    columns.push(Series::new(&format!("{}: y1", header), y1s));
                    columns.push(Series::new(&format!("{}: x2", header), x2s));
                    columns.push(Series::new(&format!("{}: y2", header), y2s));
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
                    let mut xs: Vec<Option<i32>> = Vec::with_capacity(self.row_count);
                    let mut ys: Vec<Option<i32>> = Vec::with_capacity(self.row_count);
                    for value in self.columns[header].iter() {
                        match value {
                            FlatField::Point { x, y } => {
                                xs.push(Some(x.clone() as i32));
                                ys.push(Some(y.clone() as i32));
                            }
                            _ => {
                                xs.push(None);
                                ys.push(None);
                            }
                        };
                    }
                    columns.push(Series::new(&format!("{}: x", header), xs));
                    columns.push(Series::new(&format!("{}: y", header), ys));
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
        let mut df = DataFrame::new(columns).unwrap();
        df.sort_in_place(&[SUBJECT_ID], vec![false]).unwrap();
        df
    }
}
