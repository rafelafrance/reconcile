use crate::flat::Flat;
use crate::flatten::SUBJECT_ID;
use indexmap::IndexMap;
use polars::prelude::*;

#[derive(Debug)]
pub enum ReconciledFlag {
    Error,
    Ok,
    Empty,
    AllBlank,
    Unanimous,
    Majority,
    OnlyOne,
    NoMatch,
    Fuzzy,
}

#[derive(Debug)]
pub enum ReconciledField {
    Box_ {
        flag: ReconciledFlag,
        notes: String,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    },
    Length {
        flag: ReconciledFlag,
        notes: String,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        length: f32,
        pixel_length: f32,
    },
    RulerLength {
        flag: ReconciledFlag,
        notes: String,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        length: f32,
        pixel_length: f32,
        factor: f32,
        units: String,
    },
    NoOp {
        flag: ReconciledFlag,
        notes: String,
        value: String,
    },
    Point {
        flag: ReconciledFlag,
        notes: String,
        x: i32,
        y: i32,
    },
    Same {
        flag: ReconciledFlag,
        notes: String,
        value: String,
    },
    Select {
        flag: ReconciledFlag,
        notes: String,
        value: String,
    },
    Text {
        flag: ReconciledFlag,
        notes: String,
        value: String,
    },
}

pub type ReconciledRow = IndexMap<String, ReconciledField>;

#[derive(Debug)]
pub struct Reconciled {
    pub workflow_id: String,
    pub workflow_name: String,
    columns: IndexMap<String, Vec<ReconciledField>>,
    types: IndexMap<String, ReconciledField>,
    row_count: usize,
}

impl Reconciled {
    pub fn new(workflow_id: &str, workflow_name: &str) -> Self {
        Reconciled {
            workflow_id: workflow_id.to_string(),
            workflow_name: workflow_name.to_string(),
            columns: IndexMap::new(),
            types: IndexMap::new(),
            row_count: 0,
        }
    }

    pub fn reconcile(&mut self, flat: &Flat) {}
}
