mod duplicates;
mod hasher;
mod models;
mod scanner;
mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use scanner::scan_directory;
use std::path::Path;

use crate::duplicates::find_duplicates;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
        match cli.command {
        Commands::Scan { path, min_size } => {
            println!("Scanning: {:?} ", path);
            let mut files = scan_directory(Path::new(&path))?;
            files.retain(|file| file.size >= min_size);

            let duplicates = find_duplicates(files)?;


            for group in duplicates {
                println!("Dimensione: {} byte", group.size);
                let hash_hex: String = group
                    .hash
                    .iter()
                    .map(|byte| format!("{:02x}", byte))
                    .collect();

                println!("SHA-256: {}", hash_hex);

                for path in group.files {
                    println!("  {:?}", path);
                }
            }
        }
    }


    

    Ok(())
}
