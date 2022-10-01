pub mod classifications;
pub mod reconciled;
pub mod unreconciled;

use clap::Parser;
use std::error::Error;
use std::path::PathBuf;
use reconciled::reconcile;
use unreconciled::write_unreconciled;

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

    ///Write the unreconciled classifications to this CSV file
    #[clap(short, long, value_parser, value_name = "FILE")]
    unreconciled_csv: Option<PathBuf>,

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

    let mut unrec = classifications::parse(&args.classifications_csv, &args.workflow_id).unwrap();

    if let Option::Some(unreconciled_csv) = args.unreconciled_csv {
        _ = write_unreconciled(&unreconciled_csv, &mut unrec);
    }

    if let Option::Some(_reconciled_csv) = args.reconciled_csv {
        let _rec = reconcile(&unrec); }

    Ok(())
}
