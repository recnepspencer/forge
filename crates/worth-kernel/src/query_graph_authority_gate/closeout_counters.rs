use std::collections::HashSet;

use super::closeout_types::{
    disposition_for, WorthGraphAuthorityCloseoutBypassClass,
    WorthGraphAuthorityCloseoutBypassEvidence, WorthGraphAuthorityCloseoutDisposition,
    WorthGraphAuthorityCloseoutMatrixRow, WorthGraphAuthorityDeletionClassCloseoutEvidence,
    WorthGraphAuthorityPublicFacadeProof,
};
use super::gate_report_types::WorthGraphAuthorityGateReport;
use super::types::{WorthGraphAuthorityDeletionTarget, WorthLowerAuthorityPromotionCase};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphAuthorityCloseoutCounters {
    audited_sources_covered: usize,
    inventory_matrix_rows: usize,
    deletion_target_classes: usize,
    deleted_surfaces: usize,
    collapsed_canonical_query_proofs: usize,
    collapsed_split_ledger_receipts: usize,
    collapsed_loop_ledger_receipts: usize,
    certification_only_boundaries: usize,
    explicit_residue_rows: usize,
    query_capability_gaps: usize,
    lower_authority_rejection_fixtures: usize,
    rejected_bypass_classes: usize,
    public_facade_proofs: usize,
    deletion_line_removal_classes: usize,
    deletion_removal_ledger_rows: usize,
    deletion_affected_source_files: usize,
    deletion_affected_source_lines: usize,
}

impl WorthGraphAuthorityCloseoutCounters {
    pub fn audited_sources_covered(&self) -> usize {
        self.audited_sources_covered
    }
    pub fn inventory_matrix_rows(&self) -> usize {
        self.inventory_matrix_rows
    }
    pub fn deletion_target_classes(&self) -> usize {
        self.deletion_target_classes
    }
    pub fn deleted_surfaces(&self) -> usize {
        self.deleted_surfaces
    }
    pub fn collapsed_canonical_query_proofs(&self) -> usize {
        self.collapsed_canonical_query_proofs
    }
    pub fn collapsed_split_ledger_receipts(&self) -> usize {
        self.collapsed_split_ledger_receipts
    }
    pub fn collapsed_loop_ledger_receipts(&self) -> usize {
        self.collapsed_loop_ledger_receipts
    }
    pub fn certification_only_boundaries(&self) -> usize {
        self.certification_only_boundaries
    }
    pub fn explicit_residue_rows(&self) -> usize {
        self.explicit_residue_rows
    }
    pub fn query_capability_gaps(&self) -> usize {
        self.query_capability_gaps
    }
    pub fn lower_authority_rejection_fixtures(&self) -> usize {
        self.lower_authority_rejection_fixtures
    }
    pub fn rejected_bypass_classes(&self) -> usize {
        self.rejected_bypass_classes
    }
    pub fn public_facade_proofs(&self) -> usize {
        self.public_facade_proofs
    }
    pub fn deletion_line_removal_classes(&self) -> usize {
        self.deletion_line_removal_classes
    }
    pub fn deletion_removal_ledger_rows(&self) -> usize {
        self.deletion_removal_ledger_rows
    }
    pub fn deletion_affected_source_files(&self) -> usize {
        self.deletion_affected_source_files
    }
    pub fn deletion_affected_source_lines(&self) -> usize {
        self.deletion_affected_source_lines
    }
    pub fn zero_silent_covered_lane_bypass(&self) -> bool {
        self.lower_authority_rejection_fixtures == WorthLowerAuthorityPromotionCase::ALL.len()
            && self.rejected_bypass_classes == WorthGraphAuthorityCloseoutBypassClass::ALL.len()
            && self.public_facade_proofs == WorthGraphAuthorityPublicFacadeProof::ALL.len()
            && self.deletion_line_removal_classes == self.deletion_target_classes
    }
}

pub(crate) fn closeout_counters(
    gate: &WorthGraphAuthorityGateReport,
    matrix: &[WorthGraphAuthorityCloseoutMatrixRow],
    bypass_evidence: &[WorthGraphAuthorityCloseoutBypassEvidence],
    deletion_class_evidence: &[WorthGraphAuthorityDeletionClassCloseoutEvidence],
) -> WorthGraphAuthorityCloseoutCounters {
    let mut deletion_target_classes = HashSet::new();
    let mut disposition_counts = CloseoutDispositionCounts::default();
    for row in gate.deletion_ledger() {
        let target = row.deletion_target();
        if target != WorthGraphAuthorityDeletionTarget::None {
            deletion_target_classes.insert(target);
        }
        disposition_counts.record(disposition_for(row.action(), target));
    }
    WorthGraphAuthorityCloseoutCounters {
        audited_sources_covered: gate.counters().audited_sources(),
        inventory_matrix_rows: matrix.len(),
        deletion_target_classes: deletion_target_classes.len(),
        deleted_surfaces: disposition_counts.deleted_surfaces,
        collapsed_canonical_query_proofs: disposition_counts.collapsed_canonical_query_proofs,
        collapsed_split_ledger_receipts: disposition_counts.collapsed_split_ledger_receipts,
        collapsed_loop_ledger_receipts: disposition_counts.collapsed_loop_ledger_receipts,
        certification_only_boundaries: disposition_counts.certification_only_boundaries,
        explicit_residue_rows: disposition_counts.explicit_residue_rows,
        query_capability_gaps: disposition_counts.query_capability_gaps,
        lower_authority_rejection_fixtures: gate.lower_authority_guard_plan().len(),
        rejected_bypass_classes: bypass_evidence.len(),
        public_facade_proofs: matrix
            .iter()
            .map(|row| row.public_facade_evidence().proof())
            .collect::<HashSet<_>>()
            .len(),
        deletion_line_removal_classes: deletion_class_evidence.len(),
        deletion_removal_ledger_rows: sum_removal_rows(deletion_class_evidence),
        deletion_affected_source_files: sum_source_files(deletion_class_evidence),
        deletion_affected_source_lines: sum_source_lines(deletion_class_evidence),
    }
}

#[derive(Default)]
struct CloseoutDispositionCounts {
    deleted_surfaces: usize,
    collapsed_canonical_query_proofs: usize,
    collapsed_split_ledger_receipts: usize,
    collapsed_loop_ledger_receipts: usize,
    certification_only_boundaries: usize,
    explicit_residue_rows: usize,
    query_capability_gaps: usize,
}

impl CloseoutDispositionCounts {
    fn record(&mut self, disposition: WorthGraphAuthorityCloseoutDisposition) {
        match disposition {
            WorthGraphAuthorityCloseoutDisposition::DeletedSurface => self.deleted_surfaces += 1,
            WorthGraphAuthorityCloseoutDisposition::CollapsedCanonicalQueryProof => {
                self.collapsed_canonical_query_proofs += 1;
            }
            WorthGraphAuthorityCloseoutDisposition::CollapsedSplitLedgerReceipt => {
                self.collapsed_split_ledger_receipts += 1;
            }
            WorthGraphAuthorityCloseoutDisposition::CollapsedLoopLedgerReceipt => {
                self.collapsed_loop_ledger_receipts += 1;
            }
            WorthGraphAuthorityCloseoutDisposition::CertificationOnlyBoundary => {
                self.certification_only_boundaries += 1;
            }
            WorthGraphAuthorityCloseoutDisposition::ExplicitResidue => {
                self.explicit_residue_rows += 1;
            }
            WorthGraphAuthorityCloseoutDisposition::QueryCapabilityGap => {
                self.query_capability_gaps += 1;
            }
            WorthGraphAuthorityCloseoutDisposition::PublicFacadeStatusOnly => {}
        }
    }
}

fn sum_removal_rows(evidence: &[WorthGraphAuthorityDeletionClassCloseoutEvidence]) -> usize {
    evidence.iter().map(|row| row.removal_ledger_rows()).sum()
}

fn sum_source_files(evidence: &[WorthGraphAuthorityDeletionClassCloseoutEvidence]) -> usize {
    evidence.iter().map(|row| row.affected_source_files()).sum()
}

fn sum_source_lines(evidence: &[WorthGraphAuthorityDeletionClassCloseoutEvidence]) -> usize {
    evidence.iter().map(|row| row.affected_source_lines()).sum()
}
