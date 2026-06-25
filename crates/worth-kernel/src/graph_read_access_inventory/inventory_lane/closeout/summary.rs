use super::super::bypass_audit::WorthGraphReadBypassAdoptionReport;
use super::super::coverage::WorthGraphReadAccessCoverageGuardReport;
use super::super::inventory_error::{
    WorthGraphReadAccessInventoryError, WorthGraphReadAccessInventoryErrorKind,
};
use super::super::row::{WorthGraphReadAccessClassification, WorthGraphReadAccessInventoryRow};
use super::super::scope::{WorthGraphReadAccessScopePlanReport, WorthGraphReadAccessScopeReport};
use super::super::seed::WorthGraphReadAccessInventorySeed;
use super::deleted_source_report::WorthGraphReadDeletedSourceReport;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessInventoryCloseout {
    seed: WorthGraphReadAccessInventorySeed,
    guard_report: WorthGraphReadAccessCoverageGuardReport,
    bypass_adoption_report: WorthGraphReadBypassAdoptionReport,
    deleted_source_report: WorthGraphReadDeletedSourceReport,
    scope_report: WorthGraphReadAccessScopeReport,
    scope_plan_report: WorthGraphReadAccessScopePlanReport,
    rows: Vec<WorthGraphReadAccessInventoryRow>,
    counters: WorthGraphReadAccessInventoryCloseoutCounters,
}

impl WorthGraphReadAccessInventoryCloseout {
    pub(crate) fn from_admitted_rows(
        seed: WorthGraphReadAccessInventorySeed,
        guard_report: WorthGraphReadAccessCoverageGuardReport,
        bypass_adoption_report: WorthGraphReadBypassAdoptionReport,
        rows: Vec<WorthGraphReadAccessInventoryRow>,
    ) -> Result<Self, WorthGraphReadAccessInventoryError> {
        if rows.is_empty() {
            return Err(WorthGraphReadAccessInventoryError::new(
                WorthGraphReadAccessInventoryErrorKind::EmptyInventoryRows,
            ));
        }
        let deleted_source_report = WorthGraphReadDeletedSourceReport::from_rows(&rows)?;
        let counters = WorthGraphReadAccessInventoryCloseoutCounters::from_rows(&rows);
        let scope_report = WorthGraphReadAccessScopeReport::from_rows(&rows);
        let scope_plan_report = WorthGraphReadAccessScopePlanReport::from_rows(&rows);
        Ok(Self {
            seed,
            guard_report,
            bypass_adoption_report,
            deleted_source_report,
            scope_report,
            scope_plan_report,
            rows,
            counters,
        })
    }

    pub const fn seed(&self) -> &WorthGraphReadAccessInventorySeed {
        &self.seed
    }

    pub const fn closeout_owner(&self) -> WorthGraphReadAccessCloseoutOwner {
        WorthGraphReadAccessCloseoutOwner::WorthKernel
    }

    pub const fn guard_report(&self) -> &WorthGraphReadAccessCoverageGuardReport {
        &self.guard_report
    }

    pub const fn graph_read_bypass_adoption_report(&self) -> &WorthGraphReadBypassAdoptionReport {
        &self.bypass_adoption_report
    }

    pub const fn deleted_source_report(&self) -> &WorthGraphReadDeletedSourceReport {
        &self.deleted_source_report
    }

    pub const fn scope_report(&self) -> &WorthGraphReadAccessScopeReport {
        &self.scope_report
    }

    pub const fn scope_plan_report(&self) -> &WorthGraphReadAccessScopePlanReport {
        &self.scope_plan_report
    }

    pub fn rows(&self) -> &[WorthGraphReadAccessInventoryRow] {
        &self.rows
    }

    pub const fn counters(&self) -> &WorthGraphReadAccessInventoryCloseoutCounters {
        &self.counters
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessCloseoutOwner {
    WorthKernel,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthGraphReadAccessInventoryCloseoutCounters {
    total_row_count: usize,
    declaration_candidate_count: usize,
    deletion_target_count: usize,
    capped_residue_count: usize,
    certification_only_count: usize,
    capability_gap_count: usize,
    out_of_scope_count: usize,
}

impl WorthGraphReadAccessInventoryCloseoutCounters {
    pub(crate) fn from_rows(rows: &[WorthGraphReadAccessInventoryRow]) -> Self {
        Self {
            total_row_count: rows.len(),
            declaration_candidate_count: count_classification(
                rows,
                WorthGraphReadAccessClassification::QueryDeclarationCandidate,
            ),
            deletion_target_count: count_classification(
                rows,
                WorthGraphReadAccessClassification::DeletionTarget,
            ),
            capped_residue_count: count_classification(
                rows,
                WorthGraphReadAccessClassification::CappedResidue,
            ),
            certification_only_count: count_classification(
                rows,
                WorthGraphReadAccessClassification::CertificationOnlySupport,
            ),
            capability_gap_count: count_classification(
                rows,
                WorthGraphReadAccessClassification::QueryAccessCapabilityGap,
            ),
            out_of_scope_count: count_classification(
                rows,
                WorthGraphReadAccessClassification::OutOfScopeNonGraphRead,
            ),
        }
    }

    pub const fn total_row_count(&self) -> usize {
        self.total_row_count
    }

    pub const fn declaration_candidate_count(&self) -> usize {
        self.declaration_candidate_count
    }

    pub const fn deletion_target_count(&self) -> usize {
        self.deletion_target_count
    }

    pub const fn capped_residue_count(&self) -> usize {
        self.capped_residue_count
    }

    pub const fn certification_only_count(&self) -> usize {
        self.certification_only_count
    }

    pub const fn capability_gap_count(&self) -> usize {
        self.capability_gap_count
    }

    pub const fn out_of_scope_count(&self) -> usize {
        self.out_of_scope_count
    }
}

fn count_classification(
    rows: &[WorthGraphReadAccessInventoryRow],
    classification: WorthGraphReadAccessClassification,
) -> usize {
    rows.iter()
        .filter(|row| row.classification() == classification)
        .count()
}
