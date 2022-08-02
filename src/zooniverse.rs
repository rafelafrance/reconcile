// Parse the format given to us by Zooniverse in a classifications export

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

// ---------------------------------------------------------------------------------
pub fn parse(path: &std::path::Path) -> anyhow::Result<(), Box<dyn std::error::Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut result = Ok(());

    for deserialized_row in reader.deserialize() {
        let raw_row: CsvRow = deserialized_row?;
        let mut row: UnreconciledRow = Vec::new();

        row.push(UnreconciledCell {
            header: String::from("subject_id"),
            cell: UnreconciledField::Same {
                value: raw_row["subject_ids"].clone(),
            },
        });

        row.push(UnreconciledCell {
            header: String::from("classification_id"),
            cell: UnreconciledField::NoOp {
                value: raw_row["classification_id"].clone(),
            },
        });

        if raw_row.contains_key("user_name") {
            row.push(UnreconciledCell {
                header: String::from("user_name"),
                cell: UnreconciledField::NoOp {
                    value: raw_row["user_name"].clone(),
                },
            });
        }

        let annotations: Value = serde_json::from_str(&raw_row["annotations"])?;
        result = extract_annotations(&annotations, &mut row);

        println!("");
        for cell in row.iter() {
            println!("{}: {:?}", cell.header, cell.cell);
        }
    }

    result
}

// ---------------------------------------------------------------------------------
fn extract_annotations(
    annotations: &Value,
    row: &mut UnreconciledRow,
) -> anyhow::Result<(), Box<dyn std::error::Error>> {
    match annotations {
        Value::Array(tasks_vector) => {
            for tasks in tasks_vector {
                flatten_tasks(tasks, String::from(""), row);
            }
        }
        _ => panic!("No annotations in this row {:?}", annotations),
    }

    Ok(())
}

fn flatten_tasks(task: &Value, annotation_id: String, row: &mut UnreconciledRow) {
    let annotation_id = get_annotation_id(task, annotation_id);

    if let Value::Object(obj) = task {
        if obj.contains_key("value") && obj["value"].is_array() && obj["value"][0].is_string() {
            annotation_list(task, annotation_id, row);
        } else if obj.contains_key("value") && obj["value"].is_array() {
            nested_annotations(task, annotation_id, row);
        } else if obj.contains_key("select_label") {
            selected_annotation(task, annotation_id, row);
        } else if obj.contains_key("task_label") {
            text_annotation(task, annotation_id, row);
        } else if obj.contains_key("tool_label") && obj.contains_key("width") {
            box_annotation(task, annotation_id, row);
        } else if obj.contains_key("tool_label") && obj.contains_key("x1") {
            length_annotation(task, annotation_id, row);
        } else if obj.contains_key("tool_label") && obj.contains_key("x") {
            point_annotation(task, annotation_id, row);
        } else if obj.contains_key("tool_label") && obj.contains_key("details") {
            workflow_annotation(task, annotation_id, row);
        }
    } else {
        panic!("Unkown field type in: {:?}", task)
    };
}

fn nested_annotations(task: &Value, annotation_id: String, row: &mut UnreconciledRow) {
    let mut annotation_id = get_annotation_id(task, annotation_id);
    match &task["value"] {
        Value::Array(subtasks) => {
            for subtask in subtasks {
                annotation_id = get_annotation_id(subtask, annotation_id);
                flatten_tasks(subtask, annotation_id.clone(), row);
            }
        }
        _ => panic!("Expected a list: {:?}", task),
    }
}

fn annotation_list(task: &Value, annotation_id: String, row: &mut UnreconciledRow) {
    let mut field: ListField = serde_json::from_value(task.clone()).unwrap();
    field.values.sort();
    row.push(UnreconciledCell {
        header: get_key(&field.task_label, task, annotation_id),
        cell: UnreconciledField::List {
            values: field.values,
        },
    });
}

fn selected_annotation(task: &Value, annotation_id: String, row: &mut UnreconciledRow) {
    let field: SelectField = serde_json::from_value(task.clone()).unwrap();
    let value: String = match field.value {
        Some(v) => v.clone(),
        None => String::from(""),
    };
    row.push(UnreconciledCell {
        header: get_key(&field.select_label, task, annotation_id),
        cell: UnreconciledField::Select { value },
    });
}

fn text_annotation(task: &Value, annotation_id: String, row: &mut UnreconciledRow) {
    let field: TextField = serde_json::from_value(task.clone()).unwrap();
    let value: String = match field.value {
        Some(v) => v.clone(),
        None => String::from(""),
    };
    row.push(UnreconciledCell {
        header: get_key(&field.task_label, task, annotation_id),
        cell: UnreconciledField::Text { value },
    });
}

fn box_annotation(task: &Value, annotation_id: String, row: &mut UnreconciledRow) {
    let field: BoxField = serde_json::from_value(task.clone()).unwrap();
    row.push(UnreconciledCell {
        header: get_key(&field.tool_label, task, annotation_id),
        cell: UnreconciledField::Box_ {
            left: field.x,
            top: field.y,
            right: field.x + field.width,
            bottom: field.y + field.height,
        },
    });
}

fn length_annotation(task: &Value, annotation_id: String, row: &mut UnreconciledRow) {
    let field: LengthField = serde_json::from_value(task.clone()).unwrap();
    row.push(UnreconciledCell {
        header: get_key(&field.tool_label, task, annotation_id),
        cell: UnreconciledField::Length {
            x1: field.x1,
            y1: field.y1,
            x2: field.x2,
            y2: field.y2,
        },
    });
}

fn point_annotation(task: &Value, annotation_id: String, row: &mut UnreconciledRow) {
    let field: PointField = serde_json::from_value(task.clone()).unwrap();
    row.push(UnreconciledCell {
        header: get_key(&field.tool_label, task, annotation_id),
        cell: UnreconciledField::Point {
            x: field.x,
            y: field.y,
        },
    });
}

fn workflow_annotation(task: &Value, annotation_id: String, _row: &mut UnreconciledRow) {
    println!("{} add_values_from_workflow {}", annotation_id, task);
}

fn get_key(label: &String, task: &Value, annotation_id: String) -> String {
    format!("~{}~ {}", get_annotation_id(task, annotation_id), label)
}

fn get_annotation_id(task: &Value, annotation_id: String) -> String {
    match task {
        Value::Object(obj) if obj.contains_key("task") => strip_quotes(obj["task"].to_string()),
        _ => annotation_id,
    }
}

fn strip_quotes(quoted: String) -> String {
    let end = quoted.len() - 1;
    quoted[1..end].to_string()
}
