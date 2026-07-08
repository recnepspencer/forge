mod boundary_model;
mod config;
mod freshness;
mod render;

use crate::boundary_model::load_orientations;
use crate::config::{parse_args, CliConfig, Mode};
use crate::freshness::{check_freshness, write_contexts};
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let cli = match parse_args() {
        Ok(cli) => cli,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };

    if let Err(errors) = run(cli) {
        for error in errors {
            eprintln!("{error}");
        }
        std::process::exit(1);
    }
}

fn run(cli: CliConfig) -> Result<(), Vec<String>> {
    let root = fs::canonicalize(&cli.root)
        .map_err(|e| vec![format!("canonicalize root {}: {e}", cli.root.display())])?;
    let config_path = resolve_config_path(&root, &cli.config);
    let orientations = load_orientations(&root, &config_path).map_err(|e| vec![e])?;

    match cli.mode {
        Mode::Generate => {
            write_contexts(&root, &orientations).map_err(|e| vec![e])?;
            println!(
                "agent-context: generated {} crate contexts",
                orientations.len()
            );
            Ok(())
        }
        Mode::Check => check_freshness(&root, &orientations),
    }
}

fn resolve_config_path(root: &Path, config_path: &PathBuf) -> PathBuf {
    if config_path.is_absolute() {
        return config_path.clone();
    }
    root.join(config_path)
}
