use crate::fields;
use crate::fields::{Unreconciled, UnreconciledCell, UnreconciledField, UnreconciledRow};
use serde::Deserialize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
struct BoxField {
    tool_label: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Deserialize)]
struct LengthField {
    tool_label: String,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

#[derive(Deserialize)]
struct ListField {
    task_label: String,
    values: Vec<String>,
}

#[derive(Deserialize)]
struct PointField {
    tool_label: String,
    x: f32,
    y: f32,
}

#[derive(Deserialize)]
struct SelectField {
    select_label: String,
    value: Option<String>,
}

#[derive(Deserialize)]
struct TextField {
    task_label: String,
    value: Option<String>,
}

pub fn parse(
    classifications_csv: &PathBuf,
    workflow_id: &Option<String>,
) -> Result<Unreconciled, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(classifications_csv)
        .expect("Could not read the classifications CSV file");

    let workflow_id = get_workflow_id(workflow_id, classifications_csv).unwrap();
    let workflow_name = get_workflow_name(classifications_csv).unwrap();

    let mut unreconciled: Unreconciled = Unreconciled {
        workflow_id,
        workflow_name,
        rows: Vec::new(),
    };

    for deserialized_row in reader.deserialize() {
        let raw_row: HashMap<String, String> =
            deserialized_row.expect("Could not parse a row in the classifications CSV file");

        let mut row = UnreconciledRow::new(UnreconciledCell {
            header: String::from(fields::SUBJECT_ID),
            cell: UnreconciledField::Same {
                value: raw_row[fields::SUBJECT_IDS].clone(),
            },
        });

        let annotations: Value = serde_json::from_str(&raw_row[fields::ANNOTATIONS])
            .expect("Could not parse the annotations field");

        if let Value::Array(tasks) = annotations {
            for task in tasks {
                flatten_tasks(&task, String::from(""), &mut row);
            }
        } else {
            panic!("No annotations in this CSV row: {:?}", annotations)
        }

        let targets = [
            fields::CLASSIFICATION_ID,
            fields::USER_NAME,
            fields::GOLD_STD,
            fields::EXPERT,
            fields::WORKFLOW_VER,
        ];
        for target in targets {
            if raw_row.contains_key(target) {
                row.push_metadata(UnreconciledCell {
                    header: String::from(target),
                    cell: UnreconciledField::NoOp {
                        value: raw_row[target].clone(),
                    },
                });
            }
        }

        let metadata: HashMap<String, Value> = serde_json::from_str(&raw_row[fields::METADATA])
            .expect("Could not parse the metadata field");

        for target in [fields::STARTED_AT, fields::FINISHED_AT] {
            if metadata.contains_key(target) {
                row.push_metadata(UnreconciledCell {
                    header: target.to_string(),
                    cell: UnreconciledField::NoOp {
                        value: metadata[target].to_string().trim_matches('"').to_string(),
                    },
                })
            }
        }

        let subject_data: HashMap<String, Value> = serde_json::from_str(&raw_row[fields::SUBJECT_DATA])
            .expect("Could not parse the subject_data field");

        for values in subject_data.values() {
            match values {
                Value::Object(obj) => {
                    for (header, value) in obj {
                        if header != "retired" {
                            row.push_subject_data(UnreconciledCell {
                                header: header.to_string(),
                                cell: UnreconciledField::Same {
                                    value: value.to_string(),
                                }
                            })
                        }
                    }
                },
                _ => {},
            }
        }

        unreconciled.rows.push(row);
    }

    Ok(unreconciled)
}

fn flatten_tasks(task: &Value, task_id: String, row: &mut UnreconciledRow) {
    let task_id = get_task_id(task, task_id);

    if let Value::Object(obj) = task {
        if obj.contains_key("value") && obj["value"].is_array() && obj["value"][0].is_string() {
            let mut field: ListField =
                serde_json::from_value(task.clone()).expect("Could not parse a list field");

            field.values.sort();
            row.push_annotation(UnreconciledCell {
                header: get_key(&field.task_label, task, task_id),
                cell: UnreconciledField::List {
                    values: field.values,
                },
            });
        } else if obj.contains_key("value") && obj["value"].is_array() {
            let mut task_id = get_task_id(task, task_id);
            match &task["value"] {
                Value::Array(subtasks) => {
                    for subtask in subtasks {
                        task_id = get_task_id(subtask, task_id);
                        flatten_tasks(subtask, task_id.clone(), row);
                    }
                }
                _ => panic!("Expected a list: {:?}", task),
            }
        } else if obj.contains_key("select_label") {
            let field: SelectField =
                serde_json::from_value(task.clone()).expect("Could not parse a select field");

            let value: String = match field.value {
                Some(v) => v,
                None => String::from(""),
            };
            row.push_annotation(UnreconciledCell {
                header: get_key(&field.select_label, task, task_id),
                cell: UnreconciledField::Select { value },
            });
        } else if obj.contains_key("task_label") {
            let field: TextField =
                serde_json::from_value(task.clone()).expect("Could not parse a text field");

            let value: String = match field.value {
                Some(v) => v,
                None => String::from(""),
            };
            row.push_annotation(UnreconciledCell {
                header: get_key(&field.task_label, task, task_id),
                cell: UnreconciledField::Text { value },
            });
        } else if obj.contains_key("tool_label") && obj.contains_key("width") {
            let field: BoxField =
                serde_json::from_value(task.clone()).expect("Could not parse a box field");

            row.push_annotation(UnreconciledCell {
                header: get_key(&field.tool_label, task, task_id),
                cell: UnreconciledField::Box_ {
                    left: field.x.round(),
                    top: field.y.round(),
                    right: (field.x + field.width).round(),
                    bottom: (field.y + field.height).round(),
                },
            });
        } else if obj.contains_key("tool_label") && obj.contains_key("x1") {
            let field: LengthField =
                serde_json::from_value(task.clone()).expect("Could not parse a length field");

            row.push_annotation(UnreconciledCell {
                header: get_key(&field.tool_label, task, task_id),
                cell: UnreconciledField::Length {
                    x1: field.x1.round(),
                    y1: field.y1.round(),
                    x2: field.x2.round(),
                    y2: field.y2.round(),
                },
            });
        } else if obj.contains_key("tool_label") && obj.contains_key("x") {
            let field: PointField =
                serde_json::from_value(task.clone()).expect("Could not parse a point field");

            row.push_annotation(UnreconciledCell {
                header: get_key(&field.tool_label, task, task_id),
                cell: UnreconciledField::Point {
                    x: field.x.round(),
                    y: field.y.round(),
                },
            });
        }
    } else {
        panic!("Unkown field type in: {:?}", task)
    };
}

fn get_key(label: &String, task: &Value, task_id: String) -> String {
    format!("{}: {}", get_task_id(task, task_id), label)
}

fn get_task_id(task: &Value, task_id: String) -> String {
    match task {
        Value::Object(obj) if obj.contains_key("task") => {
            obj["task"].to_string().trim_matches('"').to_string()
        }
        _ => task_id,
    }
}

fn get_workflow_id(
    workflow_id: &Option<String>,
    classifications_csv: &Path,
) -> Result<String, Box<dyn Error>> {
    let id = match workflow_id {
        Some(id) => Some(id.clone()),
        None => {
            let mut reader = csv::Reader::from_path(classifications_csv)
                .expect("Could not read the classifications CSV file");
            let mut ids: HashSet<String> = HashSet::new();
            for deserialized_row in reader.deserialize() {
                let raw_row: HashMap<String, String> = deserialized_row
                    .expect("Could not parse a row in the classifications CSV file");
                ids.insert(raw_row["workflow_id"].clone());
            }
            if ids.len() == 1 {
                ids.iter().next().cloned()
            } else {
                panic!("More than 1 workflow in this file, you must provide a workflow ID.")
            }
        }
    };
    Ok(id.unwrap())
}

fn get_workflow_name(classifications_csv: &Path) -> Result<String, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(classifications_csv).unwrap();
    let row: HashMap<String, String> = reader.deserialize().next().unwrap().unwrap();
    Ok(row["workflow_name"].clone())
}
