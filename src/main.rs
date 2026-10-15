mod duplicates;
mod hasher;
mod models;
mod scanner;
mod cli;
mod report;

use clap::Parser;
use cli::{Cli, Commands};
use scanner::scan_directory;
use std::path::Path;
use crate::{
    duplicates::find_duplicates,
    report::calculate_stats,
};
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
            let mut files = scan_directory(&path)?;

            let scanned = files.len();

            files.retain(|file| file.size >= min_size);

            let analyzed = files.len();

            let duplicates = find_duplicates(files)?;

            // Qui chiamerai calculate_stats()
            let stats  = calculate_stats(scanned, analyzed, &duplicates);
            let reclaimable_mb = stats.reclaimable_bytes as f64 / 1_048_576.0;
            // e stamperai il report
            println!(
                "Dupply Scan Report
                --------------------------
                Files scanned:          {}
                Files analyzed:         {}
                Duplicate groups:       {}
                Duplicate copies:       {}
                Reclaimable space:  {:.2} MB",
                stats.scanned,
                stats.analyzed,
                stats.duplicate_groups,
                stats.duplicate_copies,
                reclaimable_mb
            )
        }
    }


    

    Ok(())
}
