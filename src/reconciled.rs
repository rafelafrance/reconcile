use crate::reconcile_fields;
use crate::unreconciled::{
    group_rows, Unreconciled, UnreconciledField, UnreconciledGrouped, UnreconciledNames,
};

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
    pub fn new(unreconciled: &Unreconciled) -> Self {
        Reconciled {
            workflow_id: unreconciled.workflow_id.to_string(),
            workflow_name: unreconciled.workflow_name.to_string(),
            names: Vec::new(),
            rows: Vec::new(),
        }
    }
}

pub fn reconcile(unreconciled: &Unreconciled) -> Reconciled {
    let reconciled = Reconciled::new(unreconciled);
    let grouped: UnreconciledGrouped = group_rows(unreconciled);
    let columns = order_columns(&unreconciled.names);

    grouped.iter().for_each(|(_, rows)| {
        let mut rec_row: ReconciledRow = Vec::new();

        columns.iter().for_each(|col| {
            let mut fields: Vec<&UnreconciledField> = Vec::new();
            rows.iter().for_each(|row| {
                fields.push(&row[col.0]);
            });

            match &col.2 {
                UnreconciledField::Box_ {
                    left: _,
                    top: _,
                    right: _,
                    bottom: _,
                } => {
                    let rec_field = reconcile_fields::reconcile_boxes(fields);
                    println!("{:?} {}", rec_field, col.1);
                    rec_row.push(rec_field);
                }
                UnreconciledField::Length {
                    x1: _,
                    y1: _,
                    x2: _,
                    y2: _,
                } => {
                    let rec_field = reconcile_fields::reconcile_lengths(fields, &col.1);
                    println!("{:?} {}", rec_field, col.1);
                    rec_row.push(rec_field);
                }
                UnreconciledField::List {
                    values: _,
                    value: _,
                } => {
                    ////////////////////////////////////////////////////////////////////////// TODO
                    // let rec_field = reconcile_text(joined);
                    println!("{:?}", fields);
                    // rec_row.push(rec_field);
                }
                UnreconciledField::NoOp { value: _ } => {
                    let rec_field = ReconciledCell {
                        flag: ReconciledFlag::Ok,
                        notes: "".to_string(),
                        field: ReconciledField::NoOp {
                            value: "".to_string(),
                        },
                    };
                    println!("{:?} {}", rec_field, col.1);
                    rec_row.push(rec_field);
                }
                UnreconciledField::Point { x: _, y: _ } => {
                    let rec_field = reconcile_fields::reconcile_points(fields);
                    println!("{:?} {}", rec_field, col.1);
                    rec_row.push(rec_field);
                }
                UnreconciledField::Same { value: _ } => {
                    let rec_field = reconcile_fields::reconcile_same(fields);
                    println!("{:?} {}", rec_field, col.1);
                    rec_row.push(rec_field);
                }
                UnreconciledField::Select { value: _ } => {
                    ////////////////////////////////////////////////////////////////////////// TODO
                    println!("{:?}", fields);
                }
                UnreconciledField::Text { value: _ } => {
                    ////////////////////////////////////////////////////////////////////////// TODO
                    println!("{:?}", fields);
                }
                _ => {}
            }
        });
    });
    reconciled
}

fn order_columns(names: &UnreconciledNames) -> Vec<(usize, String, UnreconciledField)> {
    let mut columns: Vec<(usize, String, UnreconciledField)> = Vec::new();
    names.iter().for_each(|(k, v)| {
        columns.push((v.0, k.clone(), v.1.clone()));
    });
    columns.sort_by_key(|t| t.0);
    columns
}
