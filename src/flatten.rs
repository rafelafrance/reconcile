use crate::flat;
use serde::Deserialize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::{Path, PathBuf};

// Known fields to extract or use
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

pub fn flatten(
    classifications_csv: &PathBuf,
    workflow_id: &Option<String>,
) -> Result<flat::Flat, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(classifications_csv)
        .expect("Could not read the classifications CSV file");

    let workflow_id = get_workflow_id(workflow_id, classifications_csv)?;
    let workflow_name = get_workflow_name(classifications_csv)?;

    let mut flat = flat::Flat::new(&workflow_id, &workflow_name);

    for deserialized_row in reader.deserialize() {
        let raw_row: HashMap<String, String> = deserialized_row?;

        let mut flat_row = flat::FlatRow::new();

        flat_row.insert(
            SUBJECT_ID.to_string(),
            flat::FlatField::Same {
                value: raw_row[SUBJECT_IDS].clone(),
            },
        );

        let annotations: Value = serde_json::from_str(&raw_row[ANNOTATIONS])?;

        match annotations {
            Value::Array(tasks) => {
                for task in tasks {
                    flatten_tasks(&task, "", &mut flat_row);
                }
            }
            _ => panic!("No annotations in this CSV row: {:?}", annotations),
        }

        let targets = [CLASSIFICATION_ID, USER_NAME, GOLD_STD, EXPERT, WORKFLOW_VER];
        for target in targets {
            if raw_row.contains_key(target) {
                flat_row.insert(
                    target.to_string(),
                    flat::FlatField::NoOp {
                        value: raw_row[target].clone(),
                    },
                );
            }
        }

        let metadata: HashMap<String, Value> = serde_json::from_str(&raw_row[METADATA])?;

        for target in [STARTED_AT, FINISHED_AT] {
            if metadata.contains_key(target) {
                flat_row.insert(
                    target.to_string(),
                    flat::FlatField::NoOp {
                        value: metadata[target].to_string().trim_matches('"').to_string(),
                    },
                );
            }
        }

        let subject_data: HashMap<String, Value> = serde_json::from_str(&raw_row[SUBJECT_DATA])
            .expect("Could not parse the subject_data field");

        for values in subject_data.values() {
            if let Value::Object(obj) = values {
                for (header, value) in obj {
                    if header != "retired" {
                        flat_row.insert(
                            header.to_string(),
                            flat::FlatField::Same {
                                value: value.to_string().trim_matches('"').to_string(),
                            },
                        );
                    }
                }
            }
        }
        flat.add_row(&flat_row);
    }

    Ok(flat)
}

fn flatten_tasks(task: &Value, task_id: &str, flat_row: &mut flat::FlatRow) {
    let task_id = get_task_id(task, &task_id);

    if let Value::Object(obj) = task {
        if obj.contains_key("value") && obj["value"].is_array() && obj["value"][0].is_string() {
            let mut field: ListField =
                serde_json::from_value(task.clone()).expect("Invalid list field");
            field.values.sort();
            let joined = field
                .values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            flat_row.insert(
                get_key(&field.task_label, task, &task_id),
                flat::FlatField::List {
                    values: field.values,
                    value: joined,
                },
            );
        } else if obj.contains_key("value") && obj["value"].is_array() {
            let mut task_id = get_task_id(task, &task_id);
            match &task["value"] {
                Value::Array(subtasks) => {
                    for subtask in subtasks {
                        task_id = get_task_id(subtask, &task_id);
                        flatten_tasks(subtask, &task_id, flat_row);
                    }
                }
                _ => panic!("Expected a list: {:?}", task),
            }
        } else if obj.contains_key("select_label") {
            let field: SelectField =
                serde_json::from_value(task.clone()).expect("Invalid select field");

            let value: String = match field.value {
                Some(v) => v,
                None => String::from(""),
            };
            flat_row.insert(
                get_key(&field.select_label, task, &task_id),
                flat::FlatField::Select { value },
            );
        } else if obj.contains_key("task_label") {
            let field: TextField =
                serde_json::from_value(task.clone()).expect("Invalid text field");

            let value: String = match field.value {
                Some(v) => v,
                None => String::from(""),
            };
            flat_row.insert(
                get_key(&field.task_label, task, &task_id),
                flat::FlatField::Text { value },
            );
        } else if obj.contains_key("tool_label") && obj.contains_key("width") {
            let field: BoxField = serde_json::from_value(task.clone()).expect("Invalid box field");

            flat_row.insert(
                get_key(&field.tool_label, task, &task_id),
                flat::FlatField::Box_ {
                    left: field.x.round() as i32,
                    top: field.y.round() as i32,
                    right: (field.x + field.width).round() as i32,
                    bottom: (field.y + field.height).round() as i32,
                },
            );
        } else if obj.contains_key("tool_label") && obj.contains_key("x1") {
            let field: LengthField =
                serde_json::from_value(task.clone()).expect("Invalid length field");

            flat_row.insert(
                get_key(&field.tool_label, task, &task_id),
                flat::FlatField::Length {
                    x1: field.x1.round() as i32,
                    y1: field.y1.round() as i32,
                    x2: field.x2.round() as i32,
                    y2: field.y2.round() as i32,
                },
            );
        } else if obj.contains_key("tool_label") && obj.contains_key("x") {
            let field: PointField =
                serde_json::from_value(task.clone()).expect("Invalid point field");

            flat_row.insert(
                get_key(&field.tool_label, task, &task_id),
                flat::FlatField::Point {
                    x: field.x.round() as i32,
                    y: field.y.round() as i32,
                },
            );
        }
    } else {
        panic!("Unkown field type in: {:?}", task)
    };
}

fn get_key(label: &str, task: &Value, task_id: &str) -> String {
    format!("{}: {}", &get_task_id(task, task_id), label)
}

fn get_task_id(task: &Value, task_id: &str) -> String {
    match task {
        Value::Object(obj) if obj.contains_key("task") => {
            obj["task"].to_string().trim_matches('"').to_string()
        }
        _ => task_id.to_string(),
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
