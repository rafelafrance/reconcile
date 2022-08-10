use serde::Deserialize;
use serde_json::Value;

type CsvRow = std::collections::HashMap<String, String>;

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

const USER_NAME: &str = "user_name";
const SUBJECT_ID: &str = "subject_id";
const SUBJECT_IDS: &str = "subject_ids";
const CLASSIFICATION_ID: &str = "classification_id";

// ---------------------------------------------------------------------------------
pub fn parse(path: &std::path::Path) -> anyhow::Result<(), Box<dyn std::error::Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    for deserialized_row in reader.deserialize() {
        let raw_row: CsvRow = deserialized_row?;
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

        let annotations: Value = serde_json::from_str(&raw_row["annotations"])?;
        match annotations {
            Value::Array(tasks) => {
                for task in tasks {
                    flatten_tasks(&task, String::from(""), &mut row);
                }
            }
            _ => panic!("No annotations in this CSV row: {:?}", annotations),
        }

        println!();
        for cell in &row {
            println!("{}: {:?}", cell.header, cell.cell);
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------------
fn flatten_tasks(task: &Value, task_id: String, row: &mut UnreconciledRow) {
    let task_id = get_task_id(task, task_id);

    if let Value::Object(obj) = task {
        if obj.contains_key("value") && obj["value"].is_array() && obj["value"][0].is_string() {
            let mut field: ListField = serde_json::from_value(task.clone()).unwrap();
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
                        flatten_tasks(subtask, task_id.clone(), row);
                    }
                }
                _ => panic!("Expected a list: {:?}", task),
            }
        } else if obj.contains_key("select_label") {
            let field: SelectField = serde_json::from_value(task.clone()).unwrap();
            let value: String = match field.value {
                Some(v) => v,
                None => String::from(""),
            };
            row.push(UnreconciledCell {
                header: get_key(&field.select_label, task, task_id),
                cell: UnreconciledField::Select { value },
            });
        } else if obj.contains_key("task_label") {
            let field: TextField = serde_json::from_value(task.clone()).unwrap();
            let value: String = match field.value {
                Some(v) => v,
                None => String::from(""),
            };
            row.push(UnreconciledCell {
                header: get_key(&field.task_label, task, task_id),
                cell: UnreconciledField::Text { value },
            });
        } else if obj.contains_key("tool_label") && obj.contains_key("width") {
            let field: BoxField = serde_json::from_value(task.clone()).unwrap();
            row.push(UnreconciledCell {
                header: get_key(&field.tool_label, task, task_id),
                cell: UnreconciledField::Box_ {
                    left: field.x,
                    top: field.y,
                    right: field.x + field.width,
                    bottom: field.y + field.height,
                },
            });
        } else if obj.contains_key("tool_label") && obj.contains_key("x1") {
            let field: LengthField = serde_json::from_value(task.clone()).unwrap();
            row.push(UnreconciledCell {
                header: get_key(&field.tool_label, task, task_id),
                cell: UnreconciledField::Length {
                    x1: field.x1,
                    y1: field.y1,
                    x2: field.x2,
                    y2: field.y2,
                },
            });
        } else if obj.contains_key("tool_label") && obj.contains_key("x") {
            let field: PointField = serde_json::from_value(task.clone()).unwrap();
            row.push(UnreconciledCell {
                header: get_key(&field.tool_label, task, task_id),
                cell: UnreconciledField::Point {
                    x: field.x,
                    y: field.y,
                },
            });
        } else if obj.contains_key("tool_label") && obj.contains_key("details") {
            println!("{} add_values_from_workflow {}", task_id, task);
        }
    } else {
        panic!("Unkown field type in: {:?}", task)
    };
}

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
