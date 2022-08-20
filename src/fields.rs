
use serde::Deserialize;

pub const USER_NAME: &str = "user_name";
pub const SUBJECT_ID: &str = "subject_id";
pub const SUBJECT_IDS: &str = "subject_ids";
pub const CLASSIFICATION_ID: &str = "classification_id";


#[derive(Deserialize, Debug)]
pub enum UnreconciledField {
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

pub struct UnreconciledCell {
    pub header: String,
    pub cell: UnreconciledField,
}

pub type UnreconciledRow = Vec<UnreconciledCell>;

pub struct Unreconciled {
    pub workflow_id: String,
    pub workflow_name: String,
    pub rows: Vec<UnreconciledRow>,
}
