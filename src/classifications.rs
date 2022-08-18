use serde::Deserialize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------------
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

pub struct UnreconciledCell {
    header: String,
    cell: UnreconciledField,
}

pub type UnreconciledRow = Vec<UnreconciledCell>;

pub struct Unreconciled {
    workflow_id: String,
    workflow_name: String,
    rows: Vec<UnreconciledRow>,
}

struct WorkflowStrings {
    labels: HashMap<String, Vec<String>>,
    values: HashMap<String, HashMap<String, String>>,
}

const USER_NAME: &str = "user_name";
const SUBJECT_ID: &str = "subject_id";
const SUBJECT_IDS: &str = "subject_ids";
const CLASSIFICATION_ID: &str = "classification_id";

// ---------------------------------------------------------------------------------
pub fn parse(
    classifications_csv: &PathBuf,
    workflow_csv: &Option<PathBuf>,
    workflow_id: &Option<String>,
) -> anyhow::Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(classifications_csv)
        .expect("Could not read the classifications CSV file");

    let workflow_id = get_workflow_id(workflow_id, classifications_csv).unwrap();
    let workflow_name = get_workflow_name(classifications_csv).unwrap();
    let workflow_stings = get_workflow_strings(workflow_csv, &workflow_id).unwrap();

    let mut unreconciled: Unreconciled = Unreconciled {
        workflow_id,
        workflow_name,
        rows: Vec::new(),
    };

    for deserialized_row in reader.deserialize() {
        let raw_row: HashMap<String, String> =
            deserialized_row.expect("Could not parse a row in the classifications CSV file");

        let mut row: UnreconciledRow = vec![
            UnreconciledCell {
                header: String::from(SUBJECT_ID),
                cell: UnreconciledField::Same {
                    value: raw_row[SUBJECT_IDS].clone(),
                },
            },
            UnreconciledCell {
                header: String::from(CLASSIFICATION_ID),
                cell: UnreconciledField::NoOp {
                    value: raw_row[CLASSIFICATION_ID].clone(),
                },
            },
        ];

        if raw_row.contains_key(USER_NAME) {
            row.push(UnreconciledCell {
                header: String::from(USER_NAME),
                cell: UnreconciledField::NoOp {
                    value: raw_row[USER_NAME].clone(),
                },
            });
        }

        let annotations: Value = serde_json::from_str(&raw_row["annotations"])
            .expect("Could not parse the annotations field");

        match annotations {
            Value::Array(tasks) => {
                for task in tasks {
                    flatten_tasks(&task, String::from(""), &mut row, &workflow_stings);
                }
            }
            _ => panic!("No annotations in this CSV row: {:?}", annotations),
        }

        unreconciled.rows.push(row);
    }
    println!(
        "{} {} {}",
        unreconciled.workflow_id,
        unreconciled.workflow_name,
        unreconciled.rows.len()
    );

    Ok(())
}

// ---------------------------------------------------------------------------------
fn flatten_tasks(task: &Value, task_id: String, row: &mut UnreconciledRow, workflow_strings: &WorkflowStrings) {
    let task_id = get_task_id(task, task_id);

    if let Value::Object(obj) = task {
        if obj.contains_key("value") && obj["value"].is_array() && obj["value"][0].is_string() {
            let mut field: ListField =
                serde_json::from_value(task.clone()).expect("Could not parse a list field");

            field.values.sort();
            row.push(UnreconciledCell {
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
                        flatten_tasks(subtask, task_id.clone(), row, workflow_strings);
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
            row.push(UnreconciledCell {
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
            row.push(UnreconciledCell {
                header: get_key(&field.task_label, task, task_id),
                cell: UnreconciledField::Text { value },
            });
        } else if obj.contains_key("tool_label") && obj.contains_key("width") {
            let field: BoxField =
                serde_json::from_value(task.clone()).expect("Could not parse a box field");

            row.push(UnreconciledCell {
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

            row.push(UnreconciledCell {
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

            row.push(UnreconciledCell {
                header: get_key(&field.tool_label, task, task_id),
                cell: UnreconciledField::Point {
                    x: field.x.round(),
                    y: field.y.round(),
                },
            });
        } else if obj.contains_key("tool_label") && obj.contains_key("details") {
            //workflow_task(task, task_id, workflow_strings);
            println!("{} add_values_from_workflow {}", task_id, task);
        }
    } else {
        panic!("Unkown field type in: {:?}", task)
    };
}

// fn workflow_task(task: &Value, task_id: String, row: &mut UnreconciledRow) {
// }

fn get_key(label: &String, task: &Value, task_id: String) -> String {
    format!("{}~{}", get_task_id(task, task_id), label)
}

fn get_task_id(task: &Value, task_id: String) -> String {
    match task {
        Value::Object(obj) if obj.contains_key("task") => {
            obj["task"].to_string().trim_end_matches('"').to_string()
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


fn get_workflow_strings(
    workflow_csv: &Option<PathBuf>,
    workflow_id: &str,
) -> Result<WorkflowStrings, Box<dyn Error>> {
    let workflow_strings = WorkflowStrings {
        labels: HashMap::new(),
        values: HashMap::new(),
    };
    let workflow_strings = match workflow_csv {
        None => workflow_strings,
        Some(file) => {
            let mut reader = csv::Reader::from_path(file)
                .expect("Could not read the workflow CSV file");
            let mut strings = String::new();
            for wrapped_row in reader.deserialize() {
                let row: HashMap<String, String> = wrapped_row
                    .expect("Could not parse a row in the workflow CSV file");
                if row["workflow_id"] == workflow_id {
                    strings = row["strings"].clone();
                }
            }
            let strings: Value = serde_json::from_str(&strings)
                .expect("Could not parse workflow strings field");
            let instructions: HashMap<String, String> = HashMap::new();
            // string.iter().map(|s|
            workflow_strings
        }
    };
    Ok(workflow_strings)
}
