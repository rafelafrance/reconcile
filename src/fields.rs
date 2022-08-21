use serde::Deserialize;

// Known fields to extract
pub const USER_NAME: &str = "user_name";
pub const SUBJECT_ID: &str = "subject_id";
pub const SUBJECT_IDS: &str = "subject_ids";
pub const CLASS_ID: &str = "classification_id";
pub const STARTED_AT: &str = "started_at";
pub const FINISHED_AT: &str = "finished_at";
pub const GOLD_STD: &str = "gold_standard";
pub const EXPERT: &str = "expert";
pub const WORKFLOW_VER: &str = "workflow_version";
pub const ANNOTATIONS: &str = "annotations";
pub const METADATA: &str = "metadata";


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

#[derive(Debug)]
pub struct UnreconciledCell {
    pub header: String,
    pub cell: UnreconciledField,
}

pub type UnreconciledRow = Vec<UnreconciledCell>;

#[derive(Debug)]
pub struct Unreconciled {
    pub workflow_id: String,
    pub workflow_name: String,
    pub rows: Vec<UnreconciledRow>,
}
