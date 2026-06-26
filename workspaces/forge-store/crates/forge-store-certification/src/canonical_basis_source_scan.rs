use crate::{
    StoreCanonicalBasisInventoryDenial, StoreCanonicalBasisInventoryRow,
};
use std::{fs, path::Path};

const INCLUDED_FAMILY_TERMS: [&str; 6] = [
    "Evidence",
    "Receipt",
    "Diagnostic",
    "Performance",
    "Handoff",
    "Recovery",
];

const EXCLUDED_SUPPORT_TERMS: [&str; 12] = [
    "Denial",
    "Posture",
    "Counters",
    "Source",
    "Input",
    "Richness",
    "Profile",
    "Authority",
    "Equivalence",
    "Requirement",
    "Outcome",
    "RawBytesExcluded",
];

pub fn certify_scanned_store_canonical_basis_families_are_registered(
    workspace_root: &Path,
    scope_roots: &[&str],
    registered: &[StoreCanonicalBasisInventoryRow],
) -> Result<(), StoreCanonicalBasisInventoryDenial> {
    for scanned in scan_store_canonical_basis_family_surfaces(workspace_root, scope_roots) {
        if !registered.iter().any(|row| {
            row.family_name() == scanned.family_name()
                && row.source_path() == scanned.source_path()
        }) {
            return Err(StoreCanonicalBasisInventoryDenial::UnclassifiedEvidenceFamily {
                family_name: scanned.family_name(),
                classifying_subsystem: scanned.source_path(),
            });
        }
    }

    Ok(())
}

pub fn scan_store_canonical_basis_family_surfaces(
    workspace_root: &Path,
    scope_roots: &[&str],
) -> Vec<ScannedStoreCanonicalBasisFamily> {
    let mut scanned = Vec::new();
    for scope_root in scope_roots {
        scan_scope_root(workspace_root, scope_root, &mut scanned);
    }
    scanned.sort_by(|left, right| {
        left.source_path()
            .cmp(right.source_path())
            .then_with(|| left.family_name().cmp(right.family_name()))
    });
    scanned
}

fn scan_scope_root(
    workspace_root: &Path,
    scope_root: &str,
    scanned: &mut Vec<ScannedStoreCanonicalBasisFamily>,
) {
    let absolute_scope_root = workspace_root.join(scope_root);
    let Ok(entries) = fs::read_dir(&absolute_scope_root) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let Ok(relative_path) = path.strip_prefix(workspace_root) else {
                continue;
            };
            scan_scope_root(
                workspace_root,
                &relative_path.to_string_lossy().replace('\\', "/"),
                scanned,
            );
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            scan_rust_source_file(workspace_root, &path, scanned);
        }
    }
}

fn scan_rust_source_file(
    workspace_root: &Path,
    rust_source_path: &Path,
    scanned: &mut Vec<ScannedStoreCanonicalBasisFamily>,
) {
    let Ok(source_text) = fs::read_to_string(rust_source_path) else {
        return;
    };
    let Ok(relative_path) = rust_source_path.strip_prefix(workspace_root) else {
        return;
    };
    let source_path = relative_path.to_string_lossy().replace('\\', "/");

    for line in source_text.lines() {
        if let Some(family_name) = public_family_name_from_line(line) {
            scanned.push(ScannedStoreCanonicalBasisFamily::new(
                family_name.to_string(),
                source_path.clone(),
            ));
        }
    }
}

fn public_family_name_from_line(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    let remainder = trimmed
        .strip_prefix("pub struct ")
        .or_else(|| trimmed.strip_prefix("pub enum "))?;
    let family_name = remainder
        .split(|character: char| {
            !(character.is_ascii_alphanumeric() || character == '_')
        })
        .next()?;

    if is_canonical_basis_family_surface(family_name) {
        Some(family_name)
    } else {
        None
    }
}

fn is_canonical_basis_family_surface(family_name: &str) -> bool {
    INCLUDED_FAMILY_TERMS
        .iter()
        .any(|term| family_name.contains(term))
        && !EXCLUDED_SUPPORT_TERMS
            .iter()
            .any(|term| family_name.contains(term))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScannedStoreCanonicalBasisFamily {
    family_name: String,
    source_path: String,
}

impl ScannedStoreCanonicalBasisFamily {
    fn new(family_name: String, source_path: String) -> Self {
        Self {
            family_name,
            source_path,
        }
    }

    pub fn family_name(&self) -> &str {
        &self.family_name
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }
}
