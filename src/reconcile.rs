use crate::flat::{group_rows, Flat, FlatField, FlatGrouped, FlatNames};
use crate::reconcile_fields;

#[derive(Debug)]
pub enum ReconciledField {
    Box_ {
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
    },
    Length {
        length: f32,
        pixel_length: f32,
        units: String,
    },
    RulerLength {
        length: f32,
        pixel_length: f32,
        factor: f32,
        units: String,
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
pub struct ReconciledCell {
    pub flag: ReconciledFlag,
    pub field: ReconciledField,
    pub notes: String,
}

pub type ReconciledRow = Vec<ReconciledCell>;
pub type ReconciledNames = Vec<String>;

#[derive(Debug, Default)]
pub struct Reconciled {
    pub workflow_id: String,
    pub workflow_name: String,
    pub names: ReconciledNames,
    pub rows: Vec<ReconciledRow>,
}

impl Reconciled {
    pub fn new(flat: &Flat) -> Self {
        Reconciled {
            workflow_id: flat.workflow_id.to_string(),
            workflow_name: flat.workflow_name.to_string(),
            names: Vec::new(),
            rows: Vec::new(),
        }
    }
}

pub fn reconcile(flat: &Flat) -> Reconciled {
    let reconciled = Reconciled::new(flat);
    let grouped: FlatGrouped = group_rows(flat);
    let columns = order_columns(&flat.names);

    grouped.iter().for_each(|(_, rows)| {
        let mut rec_row: ReconciledRow = Vec::new();

        columns.iter().for_each(|col| {
            let mut fields: Vec<&FlatField> = Vec::new();
            rows.iter().for_each(|row| {
                fields.push(&row[col.0]);
            });

            match &col.2 {
                FlatField::Box_ {
                    left: _,
                    top: _,
                    right: _,
                    bottom: _,
                } => {
                    let rec_field = reconcile_fields::reconcile_boxes(fields);
                    rec_row.push(rec_field);
                }
                FlatField::Length {
                    x1: _,
                    y1: _,
                    x2: _,
                    y2: _,
                } => {
                    let rec_field = reconcile_fields::reconcile_lengths(fields, &col.1);
                    rec_row.push(rec_field);
                }
                FlatField::List {
                    values: _,
                    value: _,
                } => {
                    ////////////////////////////////////////////////////////////////////////// TODO
                    // let rec_field = reconcile_text(joined);
                    println!("{:?}", fields);
                    // rec_row.push(rec_field);
                }
                FlatField::NoOp { value: _ } => {
                    let rec_field = ReconciledCell {
                        flag: ReconciledFlag::Ok,
                        notes: "".to_string(),
                        field: ReconciledField::NoOp {
                            value: "".to_string(),
                        },
                    };
                    rec_row.push(rec_field);
                }
                FlatField::Point { x: _, y: _ } => {
                    let rec_field = reconcile_fields::reconcile_points(fields);
                    rec_row.push(rec_field);
                }
                FlatField::Same { value: _ } => {
                    let rec_field = reconcile_fields::reconcile_same(fields);
                    rec_row.push(rec_field);
                }
                FlatField::Select { value: _ } => {
                    ////////////////////////////////////////////////////////////////////////// TODO
                    println!("{:?}", fields);
                }
                FlatField::Text { value: _ } => {
                    ////////////////////////////////////////////////////////////////////////// TODO
                    println!("{:?}", fields);
                }
                _ => {}
            }
        });
    });
    reconciled
}

fn order_columns(names: &FlatNames) -> Vec<(usize, String, FlatField)> {
    let mut columns: Vec<(usize, String, FlatField)> = Vec::new();
    names.iter().for_each(|(k, v)| {
        columns.push((v.0, k.clone(), v.1.clone()));
    });
    columns.sort_by_key(|t| t.0);
    columns
}
