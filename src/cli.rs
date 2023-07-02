use clap::crate_authors;
use clap::{Parser, ValueHint};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author=crate_authors!(),
    about="Grab Reddit saved posts.",
)]
pub struct Cli {
    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    #[arg(short, long, value_hint = ValueHint::FilePath)]
    pub output: Option<PathBuf>,

    #[arg(short, long)]
    pub subreddit: Option<String>,
}
