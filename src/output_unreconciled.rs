use crate::fields;
use crate::fields::{Unreconciled, UnreconciledField, UnreconciledRow};
use csv::Writer;
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub fn write_unreconciled(
    unreconciled_csv: &Path,
    unreconciled: &mut Unreconciled,
) -> Result<(), Box<dyn Error>> {
    unreconciled.rows.sort_by_key(fields::get_subject_id);

    let mut writer = Writer::from_path(unreconciled_csv)
        .expect("Could not open the unreconciled CSV file");

    for (i, row) in unreconciled.rows.iter().enumerate() {
        if i == 0 {
            _ = print_header(row, &mut writer);
        }
        _ = print_row(row, &mut writer);
    }
    Ok(())
}

fn print_row(row: &UnreconciledRow, writer: &mut Writer<File>) -> Result<(), Box<dyn Error>> {
    let mut output: Vec<String> = Vec::new();
    for cell in row {
        match &cell.cell {
            UnreconciledField::Box_ {
                left,
                top,
                right,
                bottom,
            } => {
                output.push(format!("{}", left.round()));
                output.push(format!("{}", top.round()));
                output.push(format!("{}", right.round()));
                output.push(format!("{}", bottom.round()));
            }
            UnreconciledField::Length { x1, y1, x2, y2 } => {
                output.push(format!("{}", x1.round()));
                output.push(format!("{}", y1.round()));
                output.push(format!("{}", x2.round()));
                output.push(format!("{}", y2.round()));
            }
            UnreconciledField::List { values } => {
                output.push(values.join(" "));
            }
            UnreconciledField::NoOp { value } => {
                output.push(value.clone());
            }
            UnreconciledField::Point { x, y } => {
                output.push(format!("{}", x.round()));
                output.push(format!("{}", y.round()));
            }
            UnreconciledField::Same { value } => {
                output.push(value.clone());
            }
            UnreconciledField::Select { value } => {
                output.push(value.clone());
            }
            UnreconciledField::Text { value } => {
                output.push(value.clone());
            }
        }
    }
    writer.write_record(output)?;
    Ok(())
}

fn print_header(row: &UnreconciledRow, writer: &mut Writer<File>) -> Result<(), Box<dyn Error>> {
    let mut header: Vec<String> = Vec::new();
    for cell in row {
        match &cell.cell {
            UnreconciledField::Box_ {
                left: _,
                top: _,
                right: _,
                bottom: _,
            } => {
                header.push(format!("{}_left", cell.header));
                header.push(format!("{}_top", cell.header));
                header.push(format!("{}_right", cell.header));
                header.push(format!("{}_bottom", cell.header));
            }
            UnreconciledField::Length {
                x1: _,
                y1: _,
                x2: _,
                y2: _,
            } => {
                header.push(format!("{}_x1", cell.header));
                header.push(format!("{}_y1", cell.header));
                header.push(format!("{}_x2", cell.header));
                header.push(format!("{}_y2", cell.header));
            }
            UnreconciledField::List { values: _ } => {
                header.push(cell.header.clone());
            }
            UnreconciledField::NoOp { value: _ } => {
                header.push(cell.header.clone());
            }
            UnreconciledField::Point { x: _, y: _ } => {
                header.push(format!("{}_x", cell.header));
                header.push(format!("{}_y", cell.header));
            }
            UnreconciledField::Same { value: _ } => {
                header.push(cell.header.clone());
            }
            UnreconciledField::Select { value: _ } => {
                header.push(cell.header.clone());
            }
            UnreconciledField::Text { value: _ } => {
                header.push(cell.header.clone());
            }
        }
    }
    writer.write_record(header)?;
    Ok(())
}
