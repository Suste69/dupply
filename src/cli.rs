use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "dupply")]
#[command(version, about = "Find duplicate files")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Scan {
        path: PathBuf,

        #[arg(long, default_value_t = 0)]
        min_size: u64,
    },
}
