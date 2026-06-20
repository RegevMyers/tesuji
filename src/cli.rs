use clap::Parser;

use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long)]
    pub sgf: Option<PathBuf>,

    #[arg(long)]
    pub id: Option<u64>,
}
