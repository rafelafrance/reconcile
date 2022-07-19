// use anyhow::Context;
// use csv::Error;
use clap::Parser;
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Parser)]
#[clap(
    about = "This takes raw Notes from Nature classifications and creates a \
            reconciliation of the classifications for a particular workflow. \
            That is, it reduces N classifications per subject to the \"best\" \
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

    {
        let headers = reader.headers()?;
        println!("{:?}\n", headers);
    }

    for raw in reader.deserialize() {
        let row: Row = raw?;

        // TODO Check for other required fields
        if !row.contains_key("annotations") {
            panic!("CSV file does not contain an \"annotations\" field");
        }

        let annotations: Value = serde_json::from_str(&row["annotations"])?;
        match annotations {
            Value::Array(vec) => {
                for val in vec {
                    println!("{:?}\n", val);
                    flatten_annotation(&val, String::from(""));
                }
            }
            _ => {
                println!("other")
            }
        }
        break;
    }

    Ok(())
}

fn flatten_annotation(annotation: &Value, task_id: String) {
    let task_id = get_task_id(annotation, task_id);

    match annotation {
        Value::Object(obj)
            if obj.contains_key("value")
                && obj["value"].is_array()
                && obj["value"][0].is_string() =>
        {
            list_annotation(annotation, task_id);
        }
        Value::Object(obj) if obj.contains_key("value") && obj["value"].is_array() => {
            subtask_annotation(annotation, task_id);
        }
        Value::Object(obj) if obj.contains_key("tool_label") && obj.contains_key("width") => {
            box_annotation(annotation, task_id);
        }
        Value::Object(obj) if obj.contains_key("tool_label") && obj.contains_key("x1") => {
            length_annotation(annotation, task_id);
        }
        Value::Object(obj) if obj.contains_key("tool_label") && obj.contains_key("x") => {
            point_annotation(annotation, task_id);
        }
        Value::Object(obj) if obj.contains_key("select_label") => {
            select_label_annotation(annotation, task_id)
        }
        Value::Object(obj) if obj.contains_key("task_label") => {
            task_label_annotation(annotation, task_id)
        }
        _ => panic!("Unkown field type in: {:?}", annotation),
    }
}

fn subtask_annotation(annotation: &Value, task_id: String) {
    let mut task_id = get_task_id(annotation, task_id);
    match &annotation["value"] {
        Value::Array(tasks) => {
            for subtask in tasks {
                task_id = get_task_id(subtask, task_id);
                flatten_annotation(&subtask, task_id.clone());
            }
        }
        _ => panic!("Nope"),
    }
}

fn list_annotation(annotation: &Value, task_id: String) {
    println!("{} list_annotation {:?}\n", task_id, annotation);
}

fn select_label_annotation(annotation: &Value, task_id: String) {
    println!("{} select_label_annotation {:?}\n", task_id, annotation);
}

fn task_label_annotation(annotation: &Value, task_id: String) {
    println!("{} task_label_annotation {:?}\n", task_id, annotation);
}

fn box_annotation(annotation: &Value, task_id: String) {
    println!("{} box_annotation {:?}\n", task_id, annotation);
}

fn length_annotation(annotation: &Value, task_id: String) {
    println!("{} length_annotation {:?}\n", task_id, annotation);
}

fn point_annotation(annotation: &Value, task_id: String) {
    println!("{} point_annotation {:?}\n", task_id, annotation);
}

fn get_task_id(annotation: &Value, task_id: String) -> String {
    match annotation {
        Value::Object(obj) if obj.contains_key("task") => {
            let quoted = obj["task"].to_string();
            let end = quoted.len() - 1;
            quoted[1..end].to_string()
        }
        _ => task_id,
    }
}
