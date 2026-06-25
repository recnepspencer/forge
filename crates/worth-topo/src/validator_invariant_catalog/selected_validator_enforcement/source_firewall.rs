use std::fs;
use std::path::{Path, PathBuf};

use crate::validator_invariant_catalog::WorthTopologyLegalityCatalogError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthTopologySelectedValidatorEnforcementSourceFirewallReport {
    scanned_file_count: usize,
    forbidden_token_count: usize,
    violations: Vec<String>,
    report_digest: String,
}

impl WorthTopologySelectedValidatorEnforcementSourceFirewallReport {
    pub fn for_selected_validator_enforcement_lane(
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("validator_invariant_catalog")
            .join("selected_validator_enforcement");
        let files = rust_files_recursive(&root)?;
        let forbidden_tokens = forbidden_tokens();
        let mut violations = Vec::new();
        for file in &files {
            let source = fs::read_to_string(file).map_err(|error| {
                WorthTopologyLegalityCatalogError::SourceFirewall(format!(
                    "failed to read `{}`: {error}",
                    file.display()
                ))
            })?;
            for token in &forbidden_tokens {
                if allowed_self_firewall_mention(file, token) {
                    continue;
                }
                if source.contains(token) {
                    violations.push(format!("{}:{token}", file.display()));
                }
            }
        }
        let report_digest = format!(
            "worth-topo-selected-validator-enforcement-source-firewall-v1|{}|{}|{}",
            files.len(),
            forbidden_tokens.len(),
            violations.join("|")
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

    pub fn violations(&self) -> &[String] {
        &self.violations
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

fn forbidden_tokens() -> Vec<String> {
    [
        "materialized_validation_report(",
        "derived_validation_report(",
        "validation::loop_wiring::validate(",
        "TopologyValidator::",
        "workspace.materialize(",
        "workspace.read(",
        "retained_artifact().materialized",
        "external_row(",
        "validator_expectation",
        "ValidatorExpectation",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn allowed_self_firewall_mention(file: &Path, token: &str) -> bool {
    file.file_name().and_then(|name| name.to_str()) == Some("source_firewall.rs")
        && forbidden_tokens()
            .iter()
            .any(|registered| registered == token)
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
