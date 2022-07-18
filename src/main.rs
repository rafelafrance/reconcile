// use anyhow::Context;
// use csv::Error;
use clap::Parser;
use serde_json;
use std::collections::HashMap;

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

type Row = HashMap<String, String>;

fn main() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let mut reader = csv::Reader::from_path(args.zooniverse)?;

    {
        let headers = reader.headers()?;
        println!("{:?}", headers);
    }

    // for anno in json.loads(raw_row["annotations"]):
    //     flatten_annotation(anno, row, workflow_strings)

    // def flatten_annotation(anno, row, workflow_strings, task_id=""):
    //     """Flatten one annotation recursively."""
    //     task_id = anno.get("task", task_id)

    //     match anno:
    //         case {"value": [str(), *__], **___}:
    //             list_annotation(anno, row, task_id)
    //         case {"value": list(), **__}:
    //             subtask_annotation(anno, row, workflow_strings, task_id)
    //         case {"select_label": _, **__}:
    //             select_label_annotation(anno, row, task_id)
    //         case {"task_label": _, **__}:
    //             task_label_annotation(anno, row, task_id)
    //         case {"tool_label": _, "width": __, **___}:
    //             box_annotation(anno, row, task_id)
    //         case {"tool_label": _, "x1": __, **___}:
    //             length_annotation(anno, row, task_id)
    //         case {"tool_label": _, "x": __, **___}:
    //             point_annotation(anno, row, task_id)
    //         case {"tool_label": _, "details": __, **___}:
    //             workflow_annotation(anno, row, workflow_strings, task_id)
    //         case _:
    //             print(f"Annotation type not found: {anno}")

    for raw in reader.deserialize() {
        let row: Row = raw?;
        let annos = serde_json::from_str(&row["annotations"])?;
        // for anno in annos {
        //     println!("{}", anno);
        // }
        //     Value::Array(_) => println!("Array"),
        //     _ => println!("Other"),
        // }
        println!("annotations = {}", row["annotations"]);
    }

    Ok(())
}
