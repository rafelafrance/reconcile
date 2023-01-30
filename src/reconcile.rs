// use crate::flat::FlatField;
// use crate::reconcile::{ReconciledCell, ReconciledField, ReconciledFlag};
// use lazy_static::lazy_static;
// use pluralizer::pluralize;
// use regex::Regex;
// use std::collections::BTreeSet;
//
// pub fn reconcile_boxes(fields: Vec<&FlatField>) -> ReconciledCell {
//     let mut sums = (0.0, 0.0, 0.0, 0.0); // Temp buffer for calculations
//     let mut notes = "There are no box records".to_string();
//     let mut flag = ReconciledFlag::Empty;
//     let mut count = 0;
//
//     // Accumulate the box edges
//     fields.iter().for_each(|field| {
//         if let FlatField::Box_ {
//             left,
//             top,
//             right,
//             bottom,
//         } = field
//         {
//             count += 1;
//             sums.0 += left;
//             sums.1 += top;
//             sums.2 += right;
//             sums.3 += bottom;
//         };
//     });
//
//     // If there are boxes
//     if count > 0 {
//         let len = fields.len() as f32;
//         sums.0 /= len;
//         sums.1 /= len;
//         sums.2 /= len;
//         sums.3 /= len;
//
//         flag = ReconciledFlag::Ok;
//
//         notes = format!(
//             "There {} {} box {}",
//             pluralize("is", count, false),
//             count,
//             pluralize("record", count, false)
//         );
//     }
//
//     ReconciledCell {
//         flag,
//         notes,
//         field: ReconciledField::Box_ {
//             left: sums.0.round(),
//             top: sums.1.round(),
//             right: sums.2.round(),
//             bottom: sums.3.round(),
//         },
//     }
// }
//
// pub fn reconcile_lengths(fields: Vec<&FlatField>, header: &str) -> ReconciledCell {
//     let mut pixel_length: f32 = 0.0;
//     let mut factor: f32 = 0.0;
//     let mut units: String = "".to_string();
//     let mut is_scale: bool = false;
//     let mut notes = "There are no length records".to_string();
//     let mut flag = ReconciledFlag::Empty;
//     let mut count = 0;
//
//     lazy_static! {
//         static ref SCALE_RE: Regex =
//             Regex::new(r"(?x) (?P<scale> [0-9.]+ ) \s* (?P<units> (mm|cm|dm|m) ) \b").unwrap();
//     }
//
//     // Accumulate the lengths
//     fields.iter().for_each(|field| {
//         if let FlatField::Length { x1, y1, x2, y2 } = field {
//             count += 1;
//             pixel_length += ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt();
//         };
//     });
//
//     // We have valid lengths
//     if count > 0 {
//         pixel_length /= fields.len() as f32;
//         flag = ReconciledFlag::Ok;
//
//         notes = format!(
//             "There {} {} length {}",
//             pluralize("is", count, false),
//             count,
//             pluralize("record", count, false)
//         );
//     }
//
//     // Is this a scale bar or a measurement
//     if let Some(groups) = SCALE_RE.captures(header) {
//         units = groups["units"].to_string();
//         factor = groups["scale"].parse::<f32>().unwrap() / pixel_length;
//         is_scale = true;
//     }
//
//     let field = if is_scale {
//         ReconciledField::RulerLength {
//             length: 0.0,
//             pixel_length,
//             factor,
//             units,
//         }
//     } else {
//         ReconciledField::Length {
//             length: 0.0,
//             pixel_length,
//             units,
//         }
//     };
//
//     ReconciledCell { flag, notes, field }
// }
//
// pub fn reconcile_points(fields: Vec<&FlatField>) -> ReconciledCell {
//     let mut sums = (0.0, 0.0); // Temp buffer for calculations
//     let mut notes = "There are no point records".to_string();
//     let mut flag = ReconciledFlag::Empty;
//     let mut count = 0;
//
//     // Accumulate the point coordinates
//     fields.iter().for_each(|field| {
//         if let FlatField::Point { x, y } = field {
//             count += 1;
//             sums.0 += x;
//             sums.1 += y;
//         };
//     });
//
//     // If there are points
//     if count > 0 {
//         let len = fields.len() as f32;
//         sums.0 /= len;
//         sums.1 /= len;
//
//         flag = ReconciledFlag::Ok;
//
//         notes = format!(
//             "There {} {} point {}",
//             pluralize("is", count, false),
//             count,
//             pluralize("record", count, false)
//         );
//     }
//
//     ReconciledCell {
//         flag,
//         notes,
//         field: ReconciledField::Point {
//             x: sums.0.round(),
//             y: sums.1.round(),
//         },
//     }
// }
//
// pub fn reconcile_same(fields: Vec<&FlatField>) -> ReconciledCell {
//     let mut notes = "".to_string();
//     let mut flag = ReconciledFlag::Ok;
//     let mut values: BTreeSet<&String> = BTreeSet::new();
//     let mut value = "".to_string();
//
//     fields.iter().for_each(|field| {
//         if let FlatField::Same { value } = field {
//             values.insert(value);
//         }
//     });
//
//     if values.len() == 0 {
//         flag = ReconciledFlag::Empty;
//         notes = "There are no records".to_string();
//     } else if values.len() > 1 {
//         flag = ReconciledFlag::Error;
//         let joined: String = values
//             .iter()
//             .map(|v| v.to_string())
//             .collect::<Vec<String>>()
//             .join(", ");
//         notes = format!("Not all values are the same: {}", joined);
//     } else {
//         let first: Vec<String> = values.iter().take(1).map(|v| v.to_string()).collect();
//         value = first[0].to_string();
//     }
//
//     ReconciledCell {
//         flag,
//         notes,
//         field: ReconciledField::Same { value },
//     }
// }
//
// pub fn reconcile_select(fields: Vec<&FlatField>) -> ReconciledCell {
//     let mut notes = "".to_string();
//     let mut flag = ReconciledFlag::Ok;
//     let mut values: BTreeSet<&String> = BTreeSet::new();
//
//     // Nobody chose a value
//     // Top values are tied
//     // We have a winner
//     // Only one person chose a value
//     // Everyone chose a different value
//
//     ReconciledCell {
//         flag,
//         notes,
//         field: ReconciledField::Select {
//             value: "".to_string(),
//         },
//     }
// }
