use std::collections::BTreeSet;

use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::catalog::covered_evidence_lookup_surfaces;
use super::coverage::{
    validate_catalog_rows_against_current_sources, validate_current_evidence_lookup_surfaces,
    EvidenceLookupCatalogValidationReport, EvidenceLookupCoverageGuardReport,
};
use super::error::{EvidenceLookupInventoryError, EvidenceLookupInventoryErrorKind};
use super::row::{
    EvidenceLookupAuthorityKind, EvidenceLookupDisposition, EvidenceLookupInventoryRow,
    EvidenceLookupInventoryRowBuilder, EvidenceLookupInventoryRowScope, EvidenceLookupQuerySurface,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EvidenceLookupInventoryCloseoutCounters {
    covered_surface_count: usize,
    discovered_surface_count: usize,
    classified_row_count: usize,
    migrate_row_count: usize,
    delete_row_count: usize,
    cap_row_count: usize,
    certification_only_row_count: usize,
    query_gap_row_count: usize,
    query_surface_row_count: usize,
    broad_scan_row_count: usize,
    raw_vector_row_count: usize,
    public_exposure_row_count: usize,
    concrete_source_row_count: usize,
    family_summary_row_count: usize,
}

impl EvidenceLookupInventoryCloseoutCounters {
    fn from_rows(
        guard_report: &EvidenceLookupCoverageGuardReport,
        rows: &[EvidenceLookupInventoryRow],
    ) -> Self {
        let mut counters = Self {
            covered_surface_count: guard_report.covered_surface_count(),
            discovered_surface_count: guard_report.discovered_surface_count(),
            classified_row_count: rows.len(),
            ..Self::default()
        };
        for row in rows {
            counters.count_row(row);
        }
        counters
    }

    fn count_row(&mut self, row: &EvidenceLookupInventoryRow) {
        match row.disposition() {
            EvidenceLookupDisposition::Migrate => self.migrate_row_count += 1,
            EvidenceLookupDisposition::Delete => self.delete_row_count += 1,
            EvidenceLookupDisposition::Cap => self.cap_row_count += 1,
            EvidenceLookupDisposition::CertificationOnly => {
                self.certification_only_row_count += 1;
            }
            EvidenceLookupDisposition::QueryGap => self.query_gap_row_count += 1,
        }
        if row.query_surface() != EvidenceLookupQuerySurface::NotQuery {
            self.query_surface_row_count += 1;
        }
        match row.authority_kind() {
            EvidenceLookupAuthorityKind::RawEvidenceVectorAccess => {
                self.raw_vector_row_count += 1;
            }
            EvidenceLookupAuthorityKind::BroadReceiptScan => {
                self.broad_scan_row_count += 1;
            }
            EvidenceLookupAuthorityKind::PublicEvidenceRowExposure => {
                self.public_exposure_row_count += 1;
            }
            _ => {}
        }
        match row.row_scope() {
            EvidenceLookupInventoryRowScope::ConcreteSource => self.concrete_source_row_count += 1,
            EvidenceLookupInventoryRowScope::FamilySummary => self.family_summary_row_count += 1,
        }
    }

    pub const fn covered_surface_count(&self) -> usize {
        self.covered_surface_count
    }

    pub const fn discovered_surface_count(&self) -> usize {
        self.discovered_surface_count
    }

    pub const fn classified_row_count(&self) -> usize {
        self.classified_row_count
    }

    pub const fn migrate_row_count(&self) -> usize {
        self.migrate_row_count
    }

    pub const fn delete_row_count(&self) -> usize {
        self.delete_row_count
    }

    pub const fn cap_row_count(&self) -> usize {
        self.cap_row_count
    }

    pub const fn certification_only_row_count(&self) -> usize {
        self.certification_only_row_count
    }

    pub const fn query_gap_row_count(&self) -> usize {
        self.query_gap_row_count
    }

    pub const fn query_surface_row_count(&self) -> usize {
        self.query_surface_row_count
    }

    pub const fn broad_scan_row_count(&self) -> usize {
        self.broad_scan_row_count
    }

    pub const fn raw_vector_row_count(&self) -> usize {
        self.raw_vector_row_count
    }

    pub const fn public_exposure_row_count(&self) -> usize {
        self.public_exposure_row_count
    }

    pub const fn concrete_source_row_count(&self) -> usize {
        self.concrete_source_row_count
    }

    pub const fn family_summary_row_count(&self) -> usize {
        self.family_summary_row_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupInventoryCloseout {
    guard_report: EvidenceLookupCoverageGuardReport,
    rows: Vec<EvidenceLookupInventoryRow>,
    counters: EvidenceLookupInventoryCloseoutCounters,
    catalog_validation_report: EvidenceLookupCatalogValidationReport,
    closeout_digest: String,
}

impl EvidenceLookupInventoryCloseout {
    fn from_rows(
        guard_report: EvidenceLookupCoverageGuardReport,
        catalog_validation_report: EvidenceLookupCatalogValidationReport,
        rows: Vec<EvidenceLookupInventoryRow>,
    ) -> Result<Self, EvidenceLookupInventoryError> {
        if rows.is_empty() {
            return Err(error(EvidenceLookupInventoryErrorKind::EmptyInventoryRows));
        }
        if rows.len() != guard_report.covered_surface_count() {
            return Err(error(
                EvidenceLookupInventoryErrorKind::ClassifiedRowCountMismatch,
            ));
        }
        let counters = EvidenceLookupInventoryCloseoutCounters::from_rows(&guard_report, &rows);
        let closeout_digest = closeout_digest(&guard_report, &rows, &counters);
        Ok(Self {
            guard_report,
            rows,
            counters,
            catalog_validation_report,
            closeout_digest,
        })
    }

    pub fn rows(&self) -> &[EvidenceLookupInventoryRow] {
        &self.rows
    }

    pub const fn counters(&self) -> &EvidenceLookupInventoryCloseoutCounters {
        &self.counters
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub const fn claims_lookup_execution_authority(&self) -> bool {
        false
    }

    pub const fn claims_later_milestone_completion(&self) -> bool {
        false
    }

    pub fn explain(&self) -> EvidenceLookupInventoryExplanation<'_> {
        EvidenceLookupInventoryExplanation { closeout: self }
    }

    pub const fn guard_report(&self) -> &EvidenceLookupCoverageGuardReport {
        &self.guard_report
    }

    pub const fn catalog_validation_report(&self) -> &EvidenceLookupCatalogValidationReport {
        &self.catalog_validation_report
    }
}

pub struct EvidenceLookupInventoryExplanation<'a> {
    closeout: &'a EvidenceLookupInventoryCloseout,
}

impl EvidenceLookupInventoryExplanation<'_> {
    pub fn assert_no_unclassified_surfaces(&self) -> Result<(), EvidenceLookupInventoryError> {
        if self.closeout.counters.covered_surface_count
            < self.closeout.counters.discovered_surface_count
        {
            return Err(error(
                EvidenceLookupInventoryErrorKind::UnclassifiedEvidenceLookupSurface,
            ));
        }
        Ok(())
    }

    pub fn assert_no_keep_dispositions(&self) -> Result<(), EvidenceLookupInventoryError> {
        Ok(())
    }

    pub fn assert_query_rows_are_non_lookup_authority(
        &self,
    ) -> Result<(), EvidenceLookupInventoryError> {
        for row in &self.closeout.rows {
            if row.authority_kind() == EvidenceLookupAuthorityKind::QueryLookingLocalProof
                && row.query_surface() == EvidenceLookupQuerySurface::NotQuery
            {
                return Err(error(
                    EvidenceLookupInventoryErrorKind::QuerySurfaceRequired,
                ));
            }
            if row.authority_kind() != EvidenceLookupAuthorityKind::QueryLookingLocalProof
                && row.query_surface() != EvidenceLookupQuerySurface::NotQuery
            {
                return Err(error(
                    EvidenceLookupInventoryErrorKind::QuerySurfaceCannotMintLookupAuthority,
                ));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EvidenceLookupInventoryCollector {
    guard_report: EvidenceLookupCoverageGuardReport,
    rows: Vec<EvidenceLookupInventoryRow>,
    identities: BTreeSet<EvidenceLookupInventoryRowIdentity>,
}

impl EvidenceLookupInventoryCollector {
    pub(crate) fn with_guard_report(guard_report: EvidenceLookupCoverageGuardReport) -> Self {
        Self {
            guard_report,
            rows: Vec::new(),
            identities: BTreeSet::new(),
        }
    }

    pub(crate) fn admit_row(
        mut self,
        builder: EvidenceLookupInventoryRowBuilder,
    ) -> Result<Self, EvidenceLookupInventoryError> {
        let row = builder.build()?;
        let identity = EvidenceLookupInventoryRowIdentity::from_row(&row);
        if !self.identities.insert(identity) {
            return Err(error(
                EvidenceLookupInventoryErrorKind::DuplicateInventoryRowIdentity,
            ));
        }
        self.rows.push(row);
        Ok(self)
    }

    pub(crate) fn closeout(
        self,
        catalog_validation_report: EvidenceLookupCatalogValidationReport,
    ) -> Result<EvidenceLookupInventoryCloseout, EvidenceLookupInventoryError> {
        EvidenceLookupInventoryCloseout::from_rows(
            self.guard_report,
            catalog_validation_report,
            self.rows,
        )
    }

    #[cfg(test)]
    pub(crate) fn closeout_without_catalog_validation(
        self,
    ) -> Result<EvidenceLookupInventoryCloseout, EvidenceLookupInventoryError> {
        EvidenceLookupInventoryCloseout::from_rows(
            self.guard_report,
            EvidenceLookupCatalogValidationReport::empty(),
            self.rows,
        )
    }
}

pub fn current_evidence_lookup_inventory(
) -> Result<EvidenceLookupInventoryCloseout, EvidenceLookupInventoryError> {
    let covered_surfaces = covered_evidence_lookup_surfaces();
    let guard_report = validate_current_evidence_lookup_surfaces(&covered_surfaces)?;
    let catalog_validation_report =
        validate_catalog_rows_against_current_sources(&covered_surfaces)?;
    let mut collector = EvidenceLookupInventoryCollector::with_guard_report(guard_report);
    for surface in &covered_surfaces {
        collector = collector.admit_row(surface.into_row_builder())?;
    }
    collector.closeout(catalog_validation_report)
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct EvidenceLookupInventoryRowIdentity {
    source_path: String,
    surface_name: String,
}

impl EvidenceLookupInventoryRowIdentity {
    fn from_row(row: &EvidenceLookupInventoryRow) -> Self {
        Self {
            source_path: row.source_path().to_string(),
            surface_name: row.surface_name().to_string(),
        }
    }
}

fn closeout_digest(
    guard_report: &EvidenceLookupCoverageGuardReport,
    rows: &[EvidenceLookupInventoryRow],
    counters: &EvidenceLookupInventoryCloseoutCounters,
) -> String {
    let mut parts = vec![
        "evidence-lookup-inventory-closeout".to_string(),
        format!("covered:{}", guard_report.covered_surface_count()),
        format!("discovered:{}", guard_report.discovered_surface_count()),
        format!("classified:{}", counters.classified_row_count()),
        format!("concrete:{}", counters.concrete_source_row_count()),
        format!("family:{}", counters.family_summary_row_count()),
    ];
    for row in rows {
        parts.push(format!(
            concat!(
                "{}:{}:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}:",
                "{}:{:?}:{:?}:{:?}:{:?}"
            ),
            row.source_path(),
            row.surface_name(),
            row.owner(),
            row.current_caller(),
            row.authority_kind(),
            row.disposition(),
            row.replacement_phase(),
            row.blocker(),
            row.removal_trigger(),
            row.certification_posture(),
            row.cost_posture(),
            row.query_surface(),
            row.row_scope()
        ));
    }
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

const fn error(kind: EvidenceLookupInventoryErrorKind) -> EvidenceLookupInventoryError {
    EvidenceLookupInventoryError::new(kind)
}
