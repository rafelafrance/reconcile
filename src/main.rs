pub mod flat;
pub mod flatten;
pub mod reconcile;
pub mod reconcile_fields;

use clap::Parser;
// use flat::write_flattened;
// use reconcile::reconcile;
use std::error::Error;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(
    about = "This takes raw Notes from Nature classifications and creates a \
            reconciliation of the classifications for a particular workflow. \
            That is, it reduces N classifications per subject to its \"best\" \
            value."
)]
struct Cli {
    ///Read Zooniverse classifications from this CSV file
    #[clap(value_parser, value_name = "FILE")]
    classifications_csv: PathBuf,

    ///Write the flattened classifications to this CSV file
    #[clap(short, long, value_parser, value_name = "FILE")]
    flattened_csv: Option<PathBuf>,

    ///Write the reconciled classifications to this CSV file
    #[clap(short, long, value_parser, value_name = "FILE")]
    reconciled_csv: Option<PathBuf>,

    ///Write the summary of the reconciliation to this HTML file
    #[clap(short, long, value_parser, value_name = "FILE")]
    summary_html: Option<PathBuf>,

    ///The workflow ID
    #[clap(short, long, value_parser, value_name = "ID")]
    workflow_id: Option<String>,

    ///Read workflow strings from this CSV file
    #[clap(long, value_parser, value_name = "FILE")]
    workflow_csv: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    pluralizer::initialize();

    let args = Cli::parse();

    // let mut flattened = flatten::flatten(&args.classifications_csv, &args.workflow_id).unwrap();
    let _ = flatten::flatten(&args.classifications_csv, &args.workflow_id).unwrap();

    // if let Option::Some(flat_csv) = args.flattened_csv {
    //     _ = write_flattened(&flat_csv, &mut flattened);
    // }
    //
    // if let Option::Some(_reconciled_csv) = args.reconciled_csv {
    //     let _rec = reconcile(&flattened);
    // }

    Ok(())
}
