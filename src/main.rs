use clap::Parser;

mod zooniverse;

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

    ///Write the unreconciled classifications to this CSV file
    #[clap(short, long, value_parser, value_name = "FILE")]
    unreconciled: Option<std::path::PathBuf>,

    ///Write the reconciled classifications to this CSV file
    #[clap(short, long, value_parser, value_name = "FILE")]
    reconciled: Option<std::path::PathBuf>,

    ///Write the summary of the reconciliation to this HTML file
    #[clap(short, long, value_parser, value_name = "FILE")]
    summary: Option<std::path::PathBuf>,

    ///The workflow ID
    #[clap(short, long, value_parser, value_name = "INT")]
    workflow_id: Option<i32>,

    ///Read workflow strings from this CSV file
    #[clap(short, long, value_parser, value_name = "FILE")]
    workflow_csv: Option<std::path::PathBuf>,
}

fn main() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    zooniverse::parse(&args.zooniverse, &args.workflow_csv)
}
