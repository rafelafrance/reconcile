use serde::Deserialize;

// Known fields to extract
pub const ANNOTATIONS: &str = "annotations";
pub const CLASSIFICATION_ID: &str = "classification_id";
pub const EXPERT: &str = "expert";
pub const FINISHED_AT: &str = "finished_at";
pub const GOLD_STD: &str = "gold_standard";
pub const METADATA: &str = "metadata";
pub const STARTED_AT: &str = "started_at";
pub const SUBJECT_DATA: &str = "subject_data";
pub const SUBJECT_ID: &str = "subject_id";
pub const SUBJECT_IDS: &str = "subject_ids";
pub const USER_NAME: &str = "user_name";
pub const WORKFLOW_VER: &str = "workflow_version";


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

// pub type UnreconciledRow = Vec<UnreconciledCell>;
#[derive(Debug)]
pub struct UnreconciledRow {
    annotations: Vec<UnreconciledCell>,
    metadata: Vec<UnreconciledCell>,
    subject_data: Vec<UnreconciledCell>,
}

impl UnreconciledRow {
    pub fn new(subject_id: UnreconciledCell) -> Self {
        Self {
            annotations: Vec::new(),
            metadata: Vec::new(),
            subject_data: vec![subject_id],
        }
    }

    pub fn push_annotation(&self, cell: UnreconciledCell) {
        self.annotations.push(cell);
    }

    pub fn push_metadata(&self, cell: UnreconciledCell) {
        self.metadata.push(cell);
    }

    pub fn push_subject_data(&self, cell: UnreconciledCell) {
        self.subject_data.push(cell);
    }
}

#[derive(Debug)]
pub struct Unreconciled {
    pub workflow_id: String,
    pub workflow_name: String,
    pub rows: Vec<UnreconciledRow>,
}

pub fn get_subject_id(row: &UnreconciledRow) -> String {
    let subject: &UnreconciledCell = &row.subject_data[0];
    let value: String = match &subject.cell {
        UnreconciledField::Same { value } => value.clone(),
        _ => panic!("Missing a subject ID"),
    };
    value
}
