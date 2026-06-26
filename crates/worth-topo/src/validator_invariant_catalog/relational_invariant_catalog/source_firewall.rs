use std::fs;
use std::path::{Path, PathBuf};

use crate::validator_invariant_catalog::WorthTopologyLegalityCatalogError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyRelationalInvariantCatalogSourceFirewallReport {
    scanned_file_count: usize,
    forbidden_token_count: usize,
    violations: Vec<String>,
    report_digest: String,
}

impl WorthTopologyRelationalInvariantCatalogSourceFirewallReport {
    pub(in crate::validator_invariant_catalog) fn for_relational_invariant_catalog_lane(
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("validator_invariant_catalog")
            .join("relational_invariant_catalog");
        Self::from_scan_root(&root)
    }

    pub(in crate::validator_invariant_catalog) fn from_scan_root(
        root: &Path,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
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
                if allowed_self_firewall_mention(file, token)
                    || allowed_residue_report_mention(file, token)
                {
                    continue;
                }
                if source_contains_forbidden_token(&source, token) {
                    violations.push(format!("{}:{token}", file.display()));
                }
            }
        }
        let report_digest = format!(
            "worth-topo-relational-invariant-catalog-source-firewall-v1|{}|{}|{}",
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

    #[cfg(test)]
    pub(in crate::validator_invariant_catalog) fn from_source_pairs(
        sources: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        let forbidden_tokens = forbidden_tokens();
        let mut scanned_file_count = 0;
        let mut violations = Vec::new();
        for (path, source) in sources {
            scanned_file_count += 1;
            let path = path.into();
            let source = source.into();
            for token in &forbidden_tokens {
                if source_contains_forbidden_token(&source, token) {
                    violations.push(format!("{path} contains forbidden token `{token}`"));
                }
            }
        }
        let report_digest = format!(
            "worth-topo-relational-invariant-catalog-source-firewall-v1|{}|{}|{}",
            scanned_file_count,
            forbidden_tokens.len(),
            violations.join(",")
        );
        Self {
            scanned_file_count,
            forbidden_token_count: forbidden_tokens.len(),
            violations,
            report_digest,
        }
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
        "configure_milestone_one_runtime_builder",
        "milestone_one_runtime_builder",
        "build_milestone_one_runtime",
        ".custom_invariant(",
        "CustomInvariantRegistration::new",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn source_contains_forbidden_token(source: &str, token: &str) -> bool {
    source.match_indices(token).any(|(start, _)| {
        let end = start + token.len();
        let previous = source[..start].chars().next_back();
        let next = source[end..].chars().next();
        !is_identifier_part(previous) && !is_identifier_part(next)
    })
}

fn is_identifier_part(character: Option<char>) -> bool {
    character.is_some_and(|character| character.is_ascii_alphanumeric() || character == '_')
}

fn allowed_self_firewall_mention(file: &Path, token: &str) -> bool {
    file.file_name().and_then(|name| name.to_str()) == Some("source_firewall.rs")
        && forbidden_tokens()
            .iter()
            .any(|registered| registered == token)
}

fn allowed_residue_report_mention(file: &Path, token: &str) -> bool {
    file.file_name().and_then(|name| name.to_str()) == Some("old_pack_residue.rs")
        && token == "milestone_one_runtime_builder"
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
