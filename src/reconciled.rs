// use crate::flat::Flat;
// use crate::flatten::SUBJECT_ID;
use indexmap::IndexMap;

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
pub struct ReconciledResult {
    // flag: ReconciledFlag,
    // notes: String,
}

#[derive(Debug)]
pub enum ReconciledField {
    Box_ {
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
        result: ReconciledResult,
    },
    Length {
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        length: f32,
        pixel_length: f32,
        units: String,
        result: ReconciledResult,
    },
    RulerLength {
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        length: f32,
        pixel_length: f32,
        factor: f32,
        units: String,
        result: ReconciledResult,
    },
    NoOp {
        value: String,
        result: ReconciledResult,
    },
    Point {
        x: i32,
        y: i32,
        result: ReconciledResult,
    },
    Same {
        value: String,
        result: ReconciledResult,
    },
    Select {
        value: String,
        flag: ReconciledResult,
    },
    Text {
        value: String,
        result: ReconciledResult,
    },
}

pub type ReconciledRow = IndexMap<String, ReconciledField>;

#[derive(Debug)]
pub struct Reconciled {
    pub workflow_id: String,
    pub workflow_name: String,
    // columns: IndexMap<String, Vec<ReconciledField>>,
    // types: IndexMap<String, ReconciledField>,
    // row_count: usize,
}

impl Reconciled {
    pub fn new(workflow_id: &str, workflow_name: &str) -> Self {
        Reconciled {
            workflow_id: workflow_id.to_string(),
            workflow_name: workflow_name.to_string(),
            // columns: IndexMap::new(),
            // types: IndexMap::new(),
            // row_count: 0,
        }
    }

    // pub fn reconcile(&mut self, flat: &Flat) {}
}
