use std::fs;
use std::path::{Path, PathBuf};

use crate::validator_invariant_catalog::source_catalog::{
    current_invariant_family_inputs, current_validator_family_inputs,
};
use crate::validator_invariant_catalog::WorthTopologyLegalityCatalogError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyLegalityCatalogSourceFirewallReport {
    scanned_file_count: usize,
    forbidden_token_count: usize,
    violations: Vec<WorthTopologyLegalityCatalogSourceFirewallViolation>,
    report_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyLegalityCatalogSourceFirewallViolation {
    path: String,
    forbidden_token: String,
}

impl WorthTopologyLegalityCatalogSourceFirewallReport {
    pub fn for_query_lowering() -> Result<Self, WorthTopologyLegalityCatalogError> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("validator_invariant_catalog")
            .join("query_lowering");
        Self::from_scan_root(&root)
    }

    pub fn for_selection_authority() -> Result<Self, WorthTopologyLegalityCatalogError> {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
        let roots = [
            manifest
                .join("src")
                .join("validator_invariant_catalog")
                .join("selection_from_touched_closure"),
            manifest.join("src").join("topology_operators"),
        ];
        let forbidden_tokens = selection_authority_forbidden_tokens();
        Self::from_scan_roots_and_tokens(&roots, &forbidden_tokens, true)
    }

    pub(in crate::validator_invariant_catalog) fn from_scan_root(
        root: &Path,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        let forbidden_tokens = current_family_tokens()?;
        Self::from_scan_roots_and_tokens(&[root.to_path_buf()], &forbidden_tokens, false)
    }

    fn from_scan_roots_and_tokens(
        roots: &[PathBuf],
        forbidden_tokens: &[String],
        recursive: bool,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        let mut files = Vec::new();
        for root in roots {
            if recursive {
                files.extend(rust_files_recursive(root)?);
            } else {
                files.extend(rust_files(root)?);
            }
        }
        files.sort();
        files.dedup();
        let mut violations = Vec::new();
        for file in &files {
            let source = fs::read_to_string(file).map_err(|error| {
                WorthTopologyLegalityCatalogError::SourceFirewall(format!(
                    "failed to read `{}`: {error}",
                    file.display()
                ))
            })?;
            for token in forbidden_tokens {
                if source.contains(token) {
                    violations.push(WorthTopologyLegalityCatalogSourceFirewallViolation {
                        path: file.display().to_string(),
                        forbidden_token: token.clone(),
                    });
                }
            }
        }
        let report_digest = format!(
            "worth-topo-legality-source-firewall:{}:{}:{}",
            files.len(),
            forbidden_tokens.len(),
            violations
                .iter()
                .map(|violation| format!("{}:{}", violation.path, violation.forbidden_token))
                .collect::<Vec<_>>()
                .join("|")
        );
        Ok(Self {
            scanned_file_count: files.len(),
            forbidden_token_count: forbidden_tokens.len(),
            violations,
            report_digest,
        })
    }

    pub const fn scanned_file_count(&self) -> usize {
        self.scanned_file_count
    }

    pub const fn forbidden_token_count(&self) -> usize {
        self.forbidden_token_count
    }

    pub fn violations(&self) -> &[WorthTopologyLegalityCatalogSourceFirewallViolation] {
        &self.violations
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

fn selection_authority_forbidden_tokens() -> Vec<String> {
    [
        "derived_validation_report",
        "materialized_validation_report",
        "milestone_one_invariant_registrations",
        "global_validation",
        "validator_expectation",
        "ValidatorExpectation",
        "static invariant pack",
        "selected validator array",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

impl WorthTopologyLegalityCatalogSourceFirewallViolation {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn forbidden_token(&self) -> &str {
        &self.forbidden_token
    }
}

fn current_family_tokens() -> Result<Vec<String>, WorthTopologyLegalityCatalogError> {
    let mut tokens = Vec::new();
    for row in current_validator_family_inputs("source-firewall")? {
        tokens.push(row.input.identity.name().to_string());
    }
    for row in current_invariant_family_inputs("source-firewall")? {
        tokens.push(row.input.identity.name().to_string());
    }
    tokens.sort();
    tokens.dedup();
    Ok(tokens)
}

fn rust_files(root: &Path) -> Result<Vec<PathBuf>, WorthTopologyLegalityCatalogError> {
    let entries = fs::read_dir(root).map_err(|error| {
        WorthTopologyLegalityCatalogError::SourceFirewall(format!(
            "failed to scan `{}`: {error}",
            root.display()
        ))
    })?;
    let mut files = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|error| {
            WorthTopologyLegalityCatalogError::SourceFirewall(format!(
                "failed to inspect `{}`: {error}",
                root.display()
            ))
        })?;
        let path = entry.path();
        if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}

fn rust_files_recursive(root: &Path) -> Result<Vec<PathBuf>, WorthTopologyLegalityCatalogError> {
    let mut files = Vec::new();
    collect_rust_files_recursive(root, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_rust_files_recursive(
    root: &Path,
    files: &mut Vec<PathBuf>,
) -> Result<(), WorthTopologyLegalityCatalogError> {
    let entries = fs::read_dir(root).map_err(|error| {
        WorthTopologyLegalityCatalogError::SourceFirewall(format!(
            "failed to scan `{}`: {error}",
            root.display()
        ))
    })?;
    for entry in entries {
        let entry = entry.map_err(|error| {
            WorthTopologyLegalityCatalogError::SourceFirewall(format!(
                "failed to inspect `{}`: {error}",
                root.display()
            ))
        })?;
        let path = entry.path();
        if path.is_dir() {
            collect_rust_files_recursive(&path, files)?;
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    Ok(())
}
