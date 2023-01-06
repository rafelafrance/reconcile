use serde::Deserialize;
use std::collections::HashMap;
use std::iter;

#[derive(Clone, Debug, Deserialize)]
pub enum FlatField {
    Null,
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
        value: String,
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

pub type FlatRow<'a> = Vec<(&'a str, FlatField)>;

#[derive(Debug, Default)]
pub struct Flat {
    pub workflow_id: String,
    pub workflow_name: String,
    pub columns: HashMap<String, Vec<FlatField>>,
    order: Vec<String>,
    types: HashMap<String, FlatField>,
}

impl Flat {
    pub fn new(workflow_id: &str, workflow_name: &str) -> Self {
        Flat {
            workflow_id: workflow_id.to_string(),
            workflow_name: workflow_name.to_string(),
            columns: HashMap::new(),
            order: Vec::new(),
            types: HashMap::new(),
        }
    }

    fn len(&self) -> usize {
        match self.order.first() {
            Some(name) => self.columns[name].len(),
            _ => 0,
        }
    }

    fn add_column(&mut self, name: &str, field: &FlatField) {
        self.order.push(name.to_string());
        self.types.insert(name.to_string(), field.clone());
        self.columns[name] = Vec::new();
        self.columns[name].extend(iter::repeat(FlatField::Null).take(self.len()));
    }

    pub fn add_row(&mut self, row: FlatRow) {
        for (name, field) in row.iter() {
            let name = name.clone();
            if !self.types.contains_key(name) {
                self.add_column(name, field);
            }
            self.columns[name].push(field.clone());
        }
    }
}

// pub fn sort_rows(&mut self) {
//     let sub_col = self.names[SUBJECT_ID].0;
//     self.rows.sort_unstable_by_key(|row| match &row[sub_col] {
//         FlatField::Same { value } => value.clone(),
//         _ => "".to_string(),
//     })
// }

// pub fn group_rows(flat: &'_ Flat) -> FlatGrouped {
//     let sub_col = flat.names[SUBJECT_ID].0;
//     let mut grouped: FlatGrouped = BTreeMap::new();
//     flat.rows.iter().for_each(|row| {
//         let key = match &row[sub_col] {
//             FlatField::Same { value } => value,
//             _ => panic!("Flat row is missing a subject ID"),
//         };
//         grouped.entry(key).or_insert(Vec::new()).push(row);
//     });
//     grouped
// }
//
// pub fn write_flattened(flat_csv: &Path, flat: &mut Flat) -> Result<(), Box<dyn Error>> {
//     let mut writer = Writer::from_path(flat_csv).expect("Could not open the flat CSV file");
//
//     for (i, row) in flat.rows.iter().enumerate() {
//         if i == 0 {
//             _ = print_header(&flat.names, &mut writer);
//         }
//         _ = print_row(row, &mut writer);
//     }
//     Ok(())
// }
//
// fn print_header(names: &FlatNames, writer: &mut Writer<File>) -> Result<(), Box<dyn Error>> {
//     let mut header: Vec<String> = Vec::new();
//
//     let mut names: Vec<_> = names.iter().collect();
//     names.sort_by(|a, b| a.1 .0.cmp(&b.1 .0));
//
//     for (name, (_, cell)) in names {
//         match &cell {
//             FlatField::Box_ {
//                 left: _,
//                 top: _,
//                 right: _,
//                 bottom: _,
//             } => {
//                 header.push(format!("{}_left", name));
//                 header.push(format!("{}_top", name));
//                 header.push(format!("{}_right", name));
//                 header.push(format!("{}_bottom", name));
//             }
//             FlatField::Length {
//                 x1: _,
//                 y1: _,
//                 x2: _,
//                 y2: _,
//             } => {
//                 header.push(format!("{}_x1", name));
//                 header.push(format!("{}_y1", name));
//                 header.push(format!("{}_x2", name));
//                 header.push(format!("{}_y2", name));
//             }
//             FlatField::List {
//                 values: _,
//                 value: _,
//             } => {
//                 header.push(name.to_string());
//             }
//             FlatField::NoOp { value: _ } => {
//                 header.push(name.to_string());
//             }
//             FlatField::Point { x: _, y: _ } => {
//                 header.push(format!("{}_x", name));
//                 header.push(format!("{}_y", name));
//             }
//             FlatField::Same { value: _ } => {
//                 header.push(name.to_string());
//             }
//             FlatField::Select { value: _ } => {
//                 header.push(name.to_string());
//             }
//             FlatField::Text { value: _ } => {
//                 header.push(name.to_string());
//             }
//             FlatField::Null => {}
//         }
//     }
//     writer.write_record(header)?;
//     Ok(())
// }
//
// fn print_row(row: &FlatRow, writer: &mut Writer<File>) -> Result<(), Box<dyn Error>> {
//     let mut output: Vec<String> = Vec::new();
//     for cell in row {
//         match &cell {
//             FlatField::Box_ {
//                 left,
//                 top,
//                 right,
//                 bottom,
//             } => {
//                 output.push(format!("{}", left.round()));
//                 output.push(format!("{}", top.round()));
//                 output.push(format!("{}", right.round()));
//                 output.push(format!("{}", bottom.round()));
//             }
//             FlatField::Length { x1, y1, x2, y2 } => {
//                 output.push(format!("{}", x1.round()));
//                 output.push(format!("{}", y1.round()));
//                 output.push(format!("{}", x2.round()));
//                 output.push(format!("{}", y2.round()));
//             }
//             FlatField::List { values, value: _ } => {
//                 output.push(values.join(" "));
//             }
//             FlatField::NoOp { value } => {
//                 output.push(value.clone());
//             }
//             FlatField::Point { x, y } => {
//                 output.push(format!("{}", x.round()));
//                 output.push(format!("{}", y.round()));
//             }
//             FlatField::Same { value } => {
//                 output.push(value.clone());
//             }
//             FlatField::Select { value } => {
//                 output.push(value.clone());
//             }
//             FlatField::Text { value } => {
//                 output.push(value.clone());
//             }
//             FlatField::Null => output.push("".to_string()),
//         }
//     }
//     writer.write_record(output)?;
//     Ok(())
// }
