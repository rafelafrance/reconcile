use crate::unreconciled::{
    group_rows, Unreconciled, UnreconciledField, UnreconciledGrouped, UnreconciledNames,
};
use pluralizer::pluralize;

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
pub enum ReconciledFlag {
    Ok,
    Empty,
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
                    rec_row.push(reconcile_box(fields));
                }
                UnreconciledField::Length {
                    x1: _,
                    y1: _,
                    x2: _,
                    y2: _,
                } => {
                    reconcile_length(fields);
                }
                UnreconciledField::List { values: _ } => {
                    println!("{:?}", fields);
                }
                UnreconciledField::NoOp { value: _ } => {
                    println!("{:?}", fields);
                }
                UnreconciledField::Point { x: _, y: _ } => {
                    println!("{:?}", fields);
                }
                UnreconciledField::Same { value: _ } => {
                    println!("{:?}", fields);
                }
                UnreconciledField::Select { value: _ } => {
                    println!("{:?}", fields);
                }
                UnreconciledField::Text { value: _ } => {
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

fn reconcile_box(fields: Vec<&UnreconciledField>) -> ReconciledCell {
    let mut sums = (0.0, 0.0, 0.0, 0.0); // Have to use a temp buffer for calculations

    fields.iter().for_each(|field| {
        if let UnreconciledField::Box_ {
            left,
            top,
            right,
            bottom,
        } = field
        {
            sums.0 += left;
            sums.1 += top;
            sums.2 += right;
            sums.3 += bottom;
        };
    });

    let n = fields.len() as isize;
    let mut notes = "There are no box records".to_string();
    let mut flag = ReconciledFlag::Empty;

    if !fields.is_empty() {
        let len = fields.len() as f32;
        sums.0 /= len;
        sums.1 /= len;
        sums.2 /= len;
        sums.3 /= len;

        flag = ReconciledFlag::Ok;

        notes = format!(
            "There {} {} box {}",
            pluralize("is", n, false),
            n,
            pluralize("record", n, false)
        );
    }

    println!("{}", notes);

    ReconciledCell {
        flag,
        notes,
        field: ReconciledField::Box_ {
            left: sums.0.round(),
            top: sums.1.round(),
            right: sums.2.round(),
            bottom: sums.3.round(),
        },
    }
}

fn reconcile_length(fields: Vec<&UnreconciledField>) {
    //}-> ReconciledCell {
    fields
        .iter()
        .for_each(|f| println!("length field {:?} ****************", f));
}
