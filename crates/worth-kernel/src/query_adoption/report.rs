use std::collections::HashSet;

use super::classification::{
    WorthQueryAdoptionClassification, WorthQueryAdoptionInventoryRow, WorthQueryAuthorityCategory,
};
use super::registry::cross_crate_inventory_rows;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAdoptionInventoryReport {
    rows: Vec<WorthQueryAdoptionInventoryRow>,
    counters: WorthQueryAdoptionInventoryCounters,
}

impl WorthQueryAdoptionInventoryReport {
    pub fn cross_crate_reality_inventory() -> Result<Self, WorthQueryAdoptionInventoryError> {
        Self::from_rows(cross_crate_inventory_rows())
    }

    pub fn from_rows(
        rows: Vec<WorthQueryAdoptionInventoryRow>,
    ) -> Result<Self, WorthQueryAdoptionInventoryError> {
        validate_inventory_rows(&rows)?;
        let counters = WorthQueryAdoptionInventoryCounters::from_rows(&rows);
        Ok(Self { rows, counters })
    }

    pub fn rows(&self) -> &[WorthQueryAdoptionInventoryRow] {
        &self.rows
    }

    pub const fn counters(&self) -> &WorthQueryAdoptionInventoryCounters {
        &self.counters
    }

    pub fn require_source_set(&self, source_set: &str) -> Option<&WorthQueryAdoptionInventoryRow> {
        self.rows.iter().find(|row| row.source_set() == source_set)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryAdoptionInventoryCounters {
    audited_source_sets: usize,
    production_source_sets: usize,
    test_support_source_sets: usize,
    certification_only_source_sets: usize,
    explicit_residue_source_sets: usize,
    source_sets_with_forbidden_patterns: usize,
}

impl WorthQueryAdoptionInventoryCounters {
    fn from_rows(rows: &[WorthQueryAdoptionInventoryRow]) -> Self {
        Self {
            audited_source_sets: rows.len(),
            production_source_sets: count_rows(rows, WorthQueryAdoptionClassification::Production),
            test_support_source_sets: count_rows(
                rows,
                WorthQueryAdoptionClassification::TestSupport,
            ),
            certification_only_source_sets: count_rows(
                rows,
                WorthQueryAdoptionClassification::CertificationOnly,
            ),
            explicit_residue_source_sets: count_rows(
                rows,
                WorthQueryAdoptionClassification::ExplicitResidue,
            ),
            source_sets_with_forbidden_patterns: rows
                .iter()
                .filter(|row| row.forbidden_pattern().is_some())
                .count(),
        }
    }

    pub const fn audited_source_sets(&self) -> usize {
        self.audited_source_sets
    }

    pub const fn production_source_sets(&self) -> usize {
        self.production_source_sets
    }

    pub const fn test_support_source_sets(&self) -> usize {
        self.test_support_source_sets
    }

    pub const fn certification_only_source_sets(&self) -> usize {
        self.certification_only_source_sets
    }

    pub const fn explicit_residue_source_sets(&self) -> usize {
        self.explicit_residue_source_sets
    }

    pub const fn source_sets_with_forbidden_patterns(&self) -> usize {
        self.source_sets_with_forbidden_patterns
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryAdoptionInventoryErrorKind {
    DuplicateSourceSet,
    MissingResponsibility,
    MissingReplacementSurface,
    MissingResiduePattern,
    ClassificationAuthorityMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAdoptionInventoryError {
    kind: WorthQueryAdoptionInventoryErrorKind,
    source_set: &'static str,
}

impl WorthQueryAdoptionInventoryError {
    fn new(kind: WorthQueryAdoptionInventoryErrorKind, source_set: &'static str) -> Self {
        Self { kind, source_set }
    }

    pub const fn kind(&self) -> &WorthQueryAdoptionInventoryErrorKind {
        &self.kind
    }

    pub const fn source_set(&self) -> &'static str {
        self.source_set
    }
}

fn count_rows(
    rows: &[WorthQueryAdoptionInventoryRow],
    classification: WorthQueryAdoptionClassification,
) -> usize {
    rows.iter()
        .filter(|row| row.classification() == classification)
        .count()
}

fn validate_inventory_rows(
    rows: &[WorthQueryAdoptionInventoryRow],
) -> Result<(), WorthQueryAdoptionInventoryError> {
    let mut source_sets = HashSet::new();
    for row in rows {
        validate_row(row, &mut source_sets)?;
    }
    Ok(())
}

fn validate_row(
    row: &WorthQueryAdoptionInventoryRow,
    source_sets: &mut HashSet<&'static str>,
) -> Result<(), WorthQueryAdoptionInventoryError> {
    if !source_sets.insert(row.source_set()) {
        return Err(WorthQueryAdoptionInventoryError::new(
            WorthQueryAdoptionInventoryErrorKind::DuplicateSourceSet,
            row.source_set(),
        ));
    }
    if row.responsibility().is_empty() {
        return Err(row_error(
            WorthQueryAdoptionInventoryErrorKind::MissingResponsibility,
            row,
        ));
    }
    if row.replacement_surface().is_empty() {
        return Err(row_error(
            WorthQueryAdoptionInventoryErrorKind::MissingReplacementSurface,
            row,
        ));
    }
    if row.classification() == WorthQueryAdoptionClassification::ExplicitResidue
        && row.forbidden_pattern().is_none()
    {
        return Err(row_error(
            WorthQueryAdoptionInventoryErrorKind::MissingResiduePattern,
            row,
        ));
    }
    if !classification_matches_authority(row.classification(), row.authority_category()) {
        return Err(row_error(
            WorthQueryAdoptionInventoryErrorKind::ClassificationAuthorityMismatch,
            row,
        ));
    }
    Ok(())
}

fn classification_matches_authority(
    classification: WorthQueryAdoptionClassification,
    authority_category: WorthQueryAuthorityCategory,
) -> bool {
    match classification {
        WorthQueryAdoptionClassification::Production => matches!(
            authority_category,
            WorthQueryAuthorityCategory::Authoritative | WorthQueryAuthorityCategory::Derived
        ),
        WorthQueryAdoptionClassification::TestSupport => {
            authority_category == WorthQueryAuthorityCategory::TestSupportOnly
        }
        WorthQueryAdoptionClassification::CertificationOnly => {
            authority_category == WorthQueryAuthorityCategory::CertificationOnly
        }
        WorthQueryAdoptionClassification::ExplicitResidue => {
            authority_category == WorthQueryAuthorityCategory::Diagnostic
        }
    }
}

fn row_error(
    kind: WorthQueryAdoptionInventoryErrorKind,
    row: &WorthQueryAdoptionInventoryRow,
) -> WorthQueryAdoptionInventoryError {
    WorthQueryAdoptionInventoryError::new(kind, row.source_set())
}
