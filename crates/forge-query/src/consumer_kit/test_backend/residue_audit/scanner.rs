use std::path::Path;

use crate::consumer_kit::boundary_audit::{
    ForgeQueryBoundaryAuditError, ForgeQueryBoundaryAuditErrorKind,
};

use super::report::ForgeQueryTestBackendResidueFinding;

const FORBIDDEN_TEST_BACKEND_RESIDUE_PATTERNS: &[(&str, &str)] = &[
    (
        "impl ForgeQueryRuntimeSchemaAdapter",
        "runtime-schema-adapter",
    ),
    (
        "impl ForgeQueryRuntimeSourceAdapter",
        "runtime-source-adapter",
    ),
    (
        "impl ForgeQueryRuntimeWriteAuthorityAdapter",
        "runtime-write-authority-adapter",
    ),
    (
        "impl ForgeQueryRuntimeSignalSinkAdapter",
        "runtime-signal-sink-adapter",
    ),
    (
        "impl ForgeQueryRuntimeSnapshotIdentityAdapter",
        "runtime-snapshot-identity-adapter",
    ),
    (
        "impl ForgeQueryRuntimeSubscriptionActivationAdapter",
        "runtime-subscription-activation-adapter",
    ),
    (
        "impl ForgeQueryRuntimePreviewBasisAdapter",
        "runtime-preview-basis-adapter",
    ),
    (
        "impl ForgeQueryRuntimeInspectorEvidenceAdapter",
        "runtime-inspector-evidence-adapter",
    ),
    ("RuntimeBridge::", "runtime-bridge-hand-assembly"),
    (
        "ForgeQueryMutationReceipt::from_authoritative_parts",
        "fabricated-mutation-receipt",
    ),
    (
        "ForgeQueryMutationReceipt::from_bridge_authoritative_parts",
        "fabricated-bridge-mutation-receipt",
    ),
    (
        "WriteAuthorityExecutionReceipt",
        "fabricated-write-authority-receipt",
    ),
];

pub(super) fn scan_root(
    root: &Path,
    findings: &mut Vec<ForgeQueryTestBackendResidueFinding>,
    scanned_file_count: &mut usize,
) -> Result<(), ForgeQueryBoundaryAuditError> {
    let entries = std::fs::read_dir(root).map_err(|error| {
        ForgeQueryBoundaryAuditError::new(
            ForgeQueryBoundaryAuditErrorKind::SourceInventoryReadFailed,
            format!(
                "failed to read test backend residue root `{}`: {error}",
                root.display()
            ),
        )
    })?;

    for entry in entries {
        let entry = entry.map_err(|error| {
            ForgeQueryBoundaryAuditError::new(
                ForgeQueryBoundaryAuditErrorKind::SourceInventoryReadFailed,
                format!(
                    "failed to read test backend residue entry under `{}`: {error}",
                    root.display()
                ),
            )
        })?;
        scan_entry(&entry.path(), findings, scanned_file_count)?;
    }
    Ok(())
}

pub(super) fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn scan_entry(
    path: &Path,
    findings: &mut Vec<ForgeQueryTestBackendResidueFinding>,
    scanned_file_count: &mut usize,
) -> Result<(), ForgeQueryBoundaryAuditError> {
    if path.is_dir() {
        scan_root(path, findings, scanned_file_count)?;
        return Ok(());
    }
    if path.extension().and_then(|extension| extension.to_str()) != Some("rs") {
        return Ok(());
    }
    *scanned_file_count += 1;
    scan_rust_source_file(path, findings)
}

fn scan_rust_source_file(
    path: &Path,
    findings: &mut Vec<ForgeQueryTestBackendResidueFinding>,
) -> Result<(), ForgeQueryBoundaryAuditError> {
    let source = std::fs::read_to_string(path).map_err(|error| {
        ForgeQueryBoundaryAuditError::new(
            ForgeQueryBoundaryAuditErrorKind::SourceInventoryReadFailed,
            format!(
                "failed to read test backend residue source `{}`: {error}",
                path.display()
            ),
        )
    })?;
    findings.extend(FORBIDDEN_TEST_BACKEND_RESIDUE_PATTERNS.iter().filter_map(
        |(pattern, residue_class)| source_contains_residue(&source, pattern, residue_class, path),
    ));
    Ok(())
}

fn source_contains_residue(
    source: &str,
    pattern: &'static str,
    residue_class: &'static str,
    path: &Path,
) -> Option<ForgeQueryTestBackendResidueFinding> {
    source.contains(pattern).then(|| {
        ForgeQueryTestBackendResidueFinding::discovered(
            normalize_path(path),
            residue_class,
            pattern,
        )
    })
}
