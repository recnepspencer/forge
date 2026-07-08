mod cargo_graph;
mod config;
mod dependency_rules;
mod diagnostics;
mod manifest_types;
mod naming;
mod seed_contracts;
mod subworkspace_rules;

use crate::cargo_graph::discover_road1_packages;
use crate::config::Road1Config;
use crate::dependency_rules::validate_dependency_rules;
use crate::diagnostics::{render_human, render_json, Diagnostic};
use crate::naming::validate_package_names;
use crate::seed_contracts::validate_seed_crate_contracts;
use crate::subworkspace_rules::validate_root_and_subworkspaces;
use std::env;
use std::fs;
use std::path::PathBuf;

enum OutputFormat {
    Human,
    Json,
}

fn main() {
    let (root, config_path, format) = match parse_args() {
        Ok(args) => args,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };

    match run(root, config_path) {
        Ok(()) => println!("boundary-check: Road 1 Cargo topology is valid"),
        Err(diagnostics) => {
            let output = match format {
                OutputFormat::Human => Ok(render_human(&diagnostics)),
                OutputFormat::Json => render_json(&diagnostics),
            };
            eprintln!("{}", output.unwrap_or_else(|error| error));
            std::process::exit(1);
        }
    }
}

fn parse_args() -> Result<(PathBuf, PathBuf, OutputFormat), String> {
    let mut root = PathBuf::from(".");
    let mut config = PathBuf::from("tools/boundary-check/config/road1.toml");
    let mut format = OutputFormat::Human;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => root = PathBuf::from(args.next().ok_or("missing --root value")?),
            "--config" => config = PathBuf::from(args.next().ok_or("missing --config value")?),
            "--format" => {
                let value = args.next().ok_or("missing --format value")?;
                format = match value.as_str() {
                    "human" => OutputFormat::Human,
                    "json" => OutputFormat::Json,
                    other => return Err(format!("unknown --format value: {other}")),
                };
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    Ok((root, config, format))
}

fn run(root: PathBuf, config_path: PathBuf) -> Result<(), Vec<Diagnostic>> {
    let root = fs::canonicalize(root).map_err(|error| {
        vec![Diagnostic {
            code: crate::diagnostics::DiagnosticCode::Bc5002SubworkspaceContractViolation,
            subject: ".".to_owned(),
            message: format!("canonicalize root failed: {error}"),
        }]
    })?;
    let resolved_config_path = resolve_config_path(&root, &config_path);
    let config_text = match fs::read_to_string(&resolved_config_path) {
        Ok(text) => text,
        Err(error) => {
            return Err(vec![Diagnostic {
                code: crate::diagnostics::DiagnosticCode::Bc5002SubworkspaceContractViolation,
                subject: resolved_config_path.display().to_string(),
                message: format!("read config failed: {error}"),
            }])
        }
    };
    let config: Road1Config = toml::from_str(&config_text).map_err(|error| {
        vec![Diagnostic {
            code: crate::diagnostics::DiagnosticCode::Bc5002SubworkspaceContractViolation,
            subject: resolved_config_path.display().to_string(),
            message: format!("parse config failed: {error}"),
        }]
    })?;

    let mut diagnostics = Vec::<Diagnostic>::new();
    diagnostics.extend(validate_machine_authority(
        &root,
        &resolved_config_path,
        &config.machine_authority,
    ));
    diagnostics.extend(
        validate_root_and_subworkspaces(&root, &config).map_err(|error| {
            vec![Diagnostic {
                code: crate::diagnostics::DiagnosticCode::Bc5002SubworkspaceContractViolation,
                subject: "root".to_owned(),
                message: error,
            }]
        })?,
    );

    let packages = discover_road1_packages(&root, &config.subworkspaces).map_err(|error| {
        vec![Diagnostic {
            code: crate::diagnostics::DiagnosticCode::Bc5002SubworkspaceContractViolation,
            subject: "cad/workspaces".to_owned(),
            message: error,
        }]
    })?;
    diagnostics.extend(validate_package_names(&packages, &config.naming));
    diagnostics.extend(validate_dependency_rules(&packages, &config.rule_contracts));
    diagnostics.extend(
        validate_seed_crate_contracts(&root, &config.born_crates, &config.seed_skeletons).map_err(
            |error| {
                vec![Diagnostic {
                    code: crate::diagnostics::DiagnosticCode::Bc5003SeedContractViolation,
                    subject: "seed-contracts".to_owned(),
                    message: error,
                }]
            },
        )?,
    );

    if diagnostics.is_empty() {
        return Ok(());
    }

    Err(diagnostics)
}

fn resolve_config_path(root: &std::path::Path, config_path: &PathBuf) -> PathBuf {
    if config_path.is_absolute() {
        return config_path.clone();
    }
    root.join(config_path)
}

fn validate_machine_authority(
    root: &std::path::Path,
    resolved_config_path: &std::path::Path,
    machine_authority: &crate::config::MachineAuthorityConfig,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let canonical_path = root.join(&machine_authority.canonical_config);
    if canonical_path != resolved_config_path {
        diagnostics.push(Diagnostic {
            code: crate::diagnostics::DiagnosticCode::Bc5002SubworkspaceContractViolation,
            subject: resolved_config_path.display().to_string(),
            message: format!(
                "loaded config path does not match machine_authority.canonical_config {}",
                machine_authority.canonical_config
            ),
        });
    }
    for doc_path in &machine_authority.mirrored_docs {
        if !root.join(doc_path).is_file() {
            diagnostics.push(Diagnostic {
                code: crate::diagnostics::DiagnosticCode::Bc5002SubworkspaceContractViolation,
                subject: doc_path.clone(),
                message: "machine_authority mirrored doc is missing".to_owned(),
            });
        }
    }
    diagnostics
}
