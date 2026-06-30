use super::catalog::{CoveredEvidenceLookupSurface, EvidenceLookupCatalogDiscoveryExpectation};
use super::discovery::{
    current_evidence_lookup_discovered_surface_report, EvidenceLookupDiscoveredSurface,
    EvidenceLookupDiscoveredSurfaceReport,
};
use super::error::{EvidenceLookupInventoryError, EvidenceLookupInventoryErrorKind};
use super::row::EvidenceLookupInventoryRowScope;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupCoverageGuardReport {
    discovered_surface_count: usize,
    covered_surface_count: usize,
    classified_surface_count: usize,
}

impl EvidenceLookupCoverageGuardReport {
    pub(crate) const fn clean(
        discovered_surface_count: usize,
        covered_surface_count: usize,
        classified_surface_count: usize,
    ) -> Self {
        Self {
            discovered_surface_count,
            covered_surface_count,
            classified_surface_count,
        }
    }

    pub const fn discovered_surface_count(&self) -> usize {
        self.discovered_surface_count
    }

    pub const fn covered_surface_count(&self) -> usize {
        self.covered_surface_count
    }

    pub const fn classified_surface_count(&self) -> usize {
        self.classified_surface_count
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupCatalogRowDiscoveryStatus {
    DiscoveredLookupShaped,
    MissingSource,
    SourceExistsButNoLookupShape,
    ManualCatalogOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupCatalogValidationRow {
    source_path: String,
    row_scope: EvidenceLookupInventoryRowScope,
    discovery_expectation: EvidenceLookupCatalogDiscoveryExpectation,
    status: EvidenceLookupCatalogRowDiscoveryStatus,
}

impl EvidenceLookupCatalogValidationRow {
    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub const fn row_scope(&self) -> EvidenceLookupInventoryRowScope {
        self.row_scope
    }

    pub const fn status(&self) -> EvidenceLookupCatalogRowDiscoveryStatus {
        self.status
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupCatalogValidationReport {
    rows: Vec<EvidenceLookupCatalogValidationRow>,
}

impl EvidenceLookupCatalogValidationReport {
    #[cfg(test)]
    pub(crate) fn empty() -> Self {
        Self { rows: Vec::new() }
    }

    pub fn rows(&self) -> &[EvidenceLookupCatalogValidationRow] {
        &self.rows
    }

    pub fn unexpected_missing_source_rows(&self) -> Vec<&EvidenceLookupCatalogValidationRow> {
        self.rows
            .iter()
            .filter(|row| row.status == EvidenceLookupCatalogRowDiscoveryStatus::MissingSource)
            .collect()
    }

    pub fn unexpected_non_discovered_rows(&self) -> Vec<&EvidenceLookupCatalogValidationRow> {
        self.rows
            .iter()
            .filter(|row| {
                row.discovery_expectation
                    == EvidenceLookupCatalogDiscoveryExpectation::DiscoveryRequired
                    && row.status != EvidenceLookupCatalogRowDiscoveryStatus::DiscoveredLookupShaped
            })
            .collect()
    }
}

pub(crate) fn validate_current_evidence_lookup_surfaces(
    covered_surfaces: &[CoveredEvidenceLookupSurface],
) -> Result<EvidenceLookupCoverageGuardReport, EvidenceLookupInventoryError> {
    let report = current_evidence_lookup_discovered_surface_report();
    validate_discovered_surface_report(report, covered_surfaces)
}

#[cfg(test)]
pub(crate) fn validate_discovered_evidence_lookup_surfaces(
    discovered: &[EvidenceLookupDiscoveredSurface],
    covered: &[CoveredEvidenceLookupSurface],
) -> Result<EvidenceLookupCoverageGuardReport, EvidenceLookupInventoryError> {
    validate_discovered_surface_slice(discovered, covered)
}

pub(crate) fn validate_catalog_rows_against_current_sources(
    covered: &[CoveredEvidenceLookupSurface],
) -> Result<EvidenceLookupCatalogValidationReport, EvidenceLookupInventoryError> {
    let rows: Vec<_> = covered
        .iter()
        .map(|surface| {
            let discovery_expectation = surface.discovery_expectation();
            let status = catalog_discovery_status(surface.source_path(), discovery_expectation);
            EvidenceLookupCatalogValidationRow {
                source_path: surface.source_path().to_string(),
                row_scope: surface.row_scope(),
                discovery_expectation,
                status,
            }
        })
        .collect();
    let report = EvidenceLookupCatalogValidationReport { rows };
    if let Some(row) = report.unexpected_non_discovered_rows().into_iter().next() {
        return Err(EvidenceLookupInventoryError::with_message(
            EvidenceLookupInventoryErrorKind::ExpectedCoveredSurfaceMissingLookupShape,
            row.source_path().to_string(),
        ));
    }
    Ok(report)
}

fn validate_discovered_surface_report(
    report: EvidenceLookupDiscoveredSurfaceReport,
    covered: &[CoveredEvidenceLookupSurface],
) -> Result<EvidenceLookupCoverageGuardReport, EvidenceLookupInventoryError> {
    validate_discovered_surface_slice(report.surfaces(), covered)
}

fn validate_discovered_surface_slice(
    discovered: &[EvidenceLookupDiscoveredSurface],
    covered: &[CoveredEvidenceLookupSurface],
) -> Result<EvidenceLookupCoverageGuardReport, EvidenceLookupInventoryError> {
    for surface in discovered {
        if covered
            .iter()
            .any(|covered| covers_discovered_surface(covered, surface))
        {
            continue;
        }
        if !looks_evidence_lookup_shaped(surface.evidence()) {
            continue;
        }
        let kind = if surface.is_test_support() {
            EvidenceLookupInventoryErrorKind::ProductionShapedTestSupportUnclassified
        } else {
            EvidenceLookupInventoryErrorKind::UnclassifiedEvidenceLookupSurface
        };
        return Err(EvidenceLookupInventoryError::with_message(
            kind,
            surface.evidence().to_string(),
        ));
    }
    Ok(EvidenceLookupCoverageGuardReport::clean(
        discovered.len(),
        covered.len(),
        discovered.len(),
    ))
}

fn covers_discovered_surface(
    covered: &CoveredEvidenceLookupSurface,
    discovered: &EvidenceLookupDiscoveredSurface,
) -> bool {
    if covered.source_path() != discovered.source_path() {
        return false;
    }
    if discovered.source_path().ends_with(".rs") {
        return covered.row_scope() == EvidenceLookupInventoryRowScope::ConcreteSource;
    }
    true
}

fn catalog_discovery_status(
    source_path: &str,
    discovery_expectation: EvidenceLookupCatalogDiscoveryExpectation,
) -> EvidenceLookupCatalogRowDiscoveryStatus {
    let root = workspace_root().join(source_path);
    if !root.exists() {
        return EvidenceLookupCatalogRowDiscoveryStatus::MissingSource;
    }
    if discovery_expectation == EvidenceLookupCatalogDiscoveryExpectation::CatalogOnly {
        return EvidenceLookupCatalogRowDiscoveryStatus::ManualCatalogOnly;
    }
    if rust_files_below(&root)
        .iter()
        .any(|file| source_contains_lookup_shape(file))
    {
        return EvidenceLookupCatalogRowDiscoveryStatus::DiscoveredLookupShaped;
    }
    EvidenceLookupCatalogRowDiscoveryStatus::SourceExistsButNoLookupShape
}

fn source_contains_lookup_shape(file: &Path) -> bool {
    std::fs::read_to_string(file)
        .map(|source| looks_evidence_lookup_shaped(&source))
        .unwrap_or(false)
}

fn rust_files_below(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if root.is_file() {
        if root.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            files.push(root.to_path_buf());
        }
        return files;
    }
    let Ok(entries) = std::fs::read_dir(root) else {
        return files;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            files.extend(rust_files_below(&path));
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    files
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn looks_evidence_lookup_shaped(evidence: &str) -> bool {
    let normalized = evidence.to_ascii_lowercase();
    EVIDENCE_LOOKUP_SHAPES
        .iter()
        .any(|shape| normalized.contains(shape))
}

const EVIDENCE_LOOKUP_SHAPES: &[&str] = &[
    "evidence_identity",
    "evidence lookup",
    "evidence row",
    "evidence vector",
    "nearby",
    "receipt lookup",
    "row_for_stage",
    "stage index",
];
