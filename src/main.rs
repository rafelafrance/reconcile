// use anyhow::Context;
// use csv::Error;
use clap::Parser;
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Parser)]
#[clap(
    about = "This takes raw Notes from Nature classifications and creates a \
            reconciliation of the classifications for a particular workflow. \
            That is, it reduces N classifications per subject to its \"best\" \
            values."
)]
struct Cli {
    ///Read Zooniverse classifications from this CSV file
    #[clap(value_parser, value_name = "FILE")]
    zooniverse: std::path::PathBuf,

    ///Write the unreconciled workflow classifications to this CSV file
    #[clap(short, long, value_parser, value_name = "FILE")]
    unreconciled: Option<std::path::PathBuf>,

    ///Write the reconciled classifications to this CSV file
    #[clap(short, long, value_parser, value_name = "FILE")]
    reconciled: Option<std::path::PathBuf>,

    ///Write the summary of the reconciliation to this HTML file
    #[clap(short, long, value_parser, value_name = "FILE")]
    summary: Option<std::path::PathBuf>,
}

type Row = BTreeMap<String, String>;

fn main() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let mut reader = csv::Reader::from_path(args.zooniverse)?;

    for raw_row in reader.deserialize() {
        let row: Row = raw_row?;

        let annotations: Value = serde_json::from_str(&row["annotations"])?;
        match annotations {
            Value::Array(tasks_vector) => {
                for tasks in tasks_vector {
                    flatten_tasks(&tasks, String::from(""));
                }
            }
            _ => {
                panic!("No annotations in this row {:?}", row);
            }
        }
    }

    Ok(())
}

fn flatten_tasks(task: &Value, task_id: String) {
    let task_id = get_task_id(task, task_id);

    if let Value::Object(obj) = task {
        if obj.contains_key("value") && obj["value"].is_array() && obj["value"][0].is_string() {
            add_list_of_values(task, task_id);
        } else if obj.contains_key("value") && obj["value"].is_array() {
            nested_tasks(task, task_id);
        } else if obj.contains_key("tool_label") && obj.contains_key("width") {
            add_box_values(task, task_id);
        } else if obj.contains_key("tool_label") && obj.contains_key("x1") {
            add_length_values(task, task_id);
        } else if obj.contains_key("tool_label") && obj.contains_key("x") {
            add_point_values(task, task_id);
        } else if obj.contains_key("tool_label") && obj.contains_key("details") {
            add_values_from_workflow(task, task_id);
        } else if obj.contains_key("select_label") {
            add_selected_value(task, task_id);
        } else if obj.contains_key("task_label") {
            add_text_value(task, task_id);
        }
    } else {
        panic!("Unkown field type in: {:?}", task);
    };
}

fn nested_tasks(task: &Value, task_id: String) {
    let mut task_id = get_task_id(task, task_id);
    match &task["value"] {
        Value::Array(subtasks) => {
            for subtask in subtasks {
                task_id = get_task_id(subtask, task_id);
                flatten_tasks(&subtask, task_id.clone());
            }
        }
        _ => panic!("Expected a list: {:?}", task),
    }
}

fn add_list_of_values(task: &Value, task_id: String) {
    println!("{} add_list_of_values {:?}\n", task_id, task);
}

fn add_selected_value(task: &Value, task_id: String) {
    println!("{} add_selected_value {:?}\n", task_id, task);
}

fn add_text_value(task: &Value, task_id: String) {
    println!("{} add_text_value {:?}\n", task_id, task);
}

fn add_box_values(task: &Value, task_id: String) {
    println!("{} add_box_values {:?}\n", task_id, task);
}

fn add_length_values(task: &Value, task_id: String) {
    println!("{} add_length_values {:?}\n", task_id, task);
}

fn add_point_values(task: &Value, task_id: String) {
    println!("{} add_point_values {:?}\n", task_id, task);
}

fn add_values_from_workflow(task: &Value, task_id: String) {
    println!("{} add_values_from_workflow {:?}\n", task_id, task);
}

fn get_task_id(task: &Value, task_id: String) -> String {
    match task {
        Value::Object(obj) if obj.contains_key("task") => strip_quotes(obj["task"].to_string()),
        _ => task_id,
    }
}

fn strip_quotes(quoted: String) -> String {
    let end = quoted.len() - 1;
    quoted[1..end].to_string()
}
