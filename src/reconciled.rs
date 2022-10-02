use crate::unreconciled::{
    group_rows, Unreconciled, UnreconciledField, UnreconciledGrouped, UnreconciledNames,
};
use lazy_static::lazy_static;
use pluralizer::pluralize;
use regex::Regex;
use std::collections::HashSet;

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
                    let rec_field = reconcile_boxes(fields);
                    println!("{:?} {}", rec_field, col.1);
                    rec_row.push(rec_field);
                }
                UnreconciledField::Length {
                    x1: _,
                    y1: _,
                    x2: _,
                    y2: _,
                } => {
                    let rec_field = reconcile_lengths(fields, &col.1);
                    println!("{:?} {}", rec_field, col.1);
                    rec_row.push(rec_field);
                }
                UnreconciledField::List { values: _ } => {
                    ////////////////////////////////////////////////////////////////////////// TODO
                    println!("{:?}", fields);
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
                    let rec_field = reconcile_points(fields);
                    println!("{:?} {}", rec_field, col.1);
                    rec_row.push(rec_field);
                }
                UnreconciledField::Same { value: _ } => {
                    let rec_field = reconcile_same(fields);
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

fn reconcile_boxes(fields: Vec<&UnreconciledField>) -> ReconciledCell {
    let mut sums = (0.0, 0.0, 0.0, 0.0); // Temp buffer for calculations
    let mut notes = "There are no box records".to_string();
    let mut flag = ReconciledFlag::Empty;

    // Accumulate the box edges
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

    // If there are boxes
    if !fields.is_empty() {
        let len = fields.len() as f32;
        sums.0 /= len;
        sums.1 /= len;
        sums.2 /= len;
        sums.3 /= len;

        flag = ReconciledFlag::Ok;

        let n = fields.len() as isize;
        notes = format!(
            "There {} {} box {}",
            pluralize("is", n, false),
            n,
            pluralize("record", n, false)
        );
    }

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

fn reconcile_lengths(fields: Vec<&UnreconciledField>, header: &str) -> ReconciledCell {
    let mut pixel_length: f32 = 0.0;
    let mut factor: f32 = 0.0;
    let mut units: String = "".to_string();
    let mut is_scale: bool = false;
    let mut notes = "There are no length records".to_string();
    let mut flag = ReconciledFlag::Empty;

    lazy_static! {
        static ref SCALE_RE: Regex =
            Regex::new(r"(?x) (?P<scale> [0-9.]+ ) \s* (?P<units> (mm|cm|dm|m) ) \b").unwrap();
    }

    // Accumulate the lengths
    fields.iter().for_each(|field| {
        if let UnreconciledField::Length { x1, y1, x2, y2 } = field {
            pixel_length += ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt();
        };
    });

    // We have valid lengths
    if !fields.is_empty() {
        pixel_length /= fields.len() as f32;
        flag = ReconciledFlag::Ok;

        let n = fields.len() as isize;
        notes = format!(
            "There {} {} length {}",
            pluralize("is", n, false),
            n,
            pluralize("record", n, false)
        );
    }

    // Is this a scale bar or a measurement
    if let Some(groups) = SCALE_RE.captures(header) {
        units = groups["units"].to_string();
        factor = groups["scale"].parse::<f32>().unwrap() / pixel_length;
        is_scale = true;
    }

    let field = if is_scale {
        ReconciledField::RulerLength {
            length: 0.0,
            pixel_length,
            factor,
            units,
        }
    } else {
        ReconciledField::Length {
            length: 0.0,
            pixel_length,
            units,
        }
    };

    ReconciledCell { flag, notes, field }
}

fn reconcile_points(fields: Vec<&UnreconciledField>) -> ReconciledCell {
    let mut sums = (0.0, 0.0); // Temp buffer for calculations
    let mut notes = "There are no point records".to_string();
    let mut flag = ReconciledFlag::Empty;

    // Accumulate the point coordinates
    fields.iter().for_each(|field| {
        if let UnreconciledField::Point { x, y } = field {
            sums.0 += x;
            sums.1 += y;
        };
    });

    // If there are points
    if !fields.is_empty() {
        let len = fields.len() as f32;
        sums.0 /= len;
        sums.1 /= len;

        flag = ReconciledFlag::Ok;

        let n = fields.len() as isize;
        notes = format!(
            "There {} {} point {}",
            pluralize("is", n, false),
            n,
            pluralize("record", n, false)
        );
    }

    ReconciledCell {
        flag,
        notes,
        field: ReconciledField::Point {
            x: sums.0.round(),
            y: sums.1.round(),
        },
    }
}

fn reconcile_same(fields: Vec<&UnreconciledField>) -> ReconciledCell {
    let mut notes = "".to_string();
    let mut flag = ReconciledFlag::Ok;
    let mut values: HashSet<&String> = HashSet::new();
    let mut value = "".to_string();

    fields.iter().for_each(|field| {
        if let UnreconciledField::Same { value } = field {
            values.insert(value);
        }
    });

    if values.is_empty() {
        flag = ReconciledFlag::Empty;
        notes = "There are no records".to_string();
    } else if values.len() > 1 {
        flag = ReconciledFlag::Error;
        let joined: String = values.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", ");
        notes = format!("Not all values are the same: {}", joined);
    } else {
        let first: Vec<String> = values.iter().take(1).map(|v| v.to_string()).collect();
        value = first[0].to_string();
    }

    ReconciledCell {
        flag,
        notes,
        field: ReconciledField::Same { value },
    }
}
