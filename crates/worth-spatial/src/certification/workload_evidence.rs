//! Certification-only accessors for spatial workload evidence ledgers.
//!
//! The ordinary facade exposes typed stage-index and receipt lookup products for
//! production composition. This module exists for cross-crate public contract
//! tests that must inspect or perturb evidence rows without making raw ledger
//! rows part of the production API.

use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, CompleteWorkloadEvidenceLedger, WorkloadEvidenceBacking,
    WorkloadEvidenceLedger, WorkloadEvidenceLedgerError, WorkloadEvidenceRow,
    WorkloadEvidenceStage, WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertifiedWorkloadEvidenceStageSnapshot {
    stage: WorkloadEvidenceStage,
    evidence_identity: String,
    backing: WorkloadEvidenceBacking,
    support: WorkloadEvidenceSupport,
    counters: WorkloadEvidenceStageCounters,
}

impl CertifiedWorkloadEvidenceStageSnapshot {
    fn from_row(row: &WorkloadEvidenceRow) -> Self {
        Self {
            stage: row.stage(),
            evidence_identity: row.evidence_identity().to_string(),
            backing: row.backing(),
            support: row.support(),
            counters: row.counters(),
        }
    }

    pub fn stage(&self) -> WorkloadEvidenceStage {
        self.stage
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn backing(&self) -> WorkloadEvidenceBacking {
        self.backing
    }

    pub fn support(&self) -> WorkloadEvidenceSupport {
        self.support
    }

    pub fn counters(&self) -> WorkloadEvidenceStageCounters {
        self.counters
    }

    pub fn is_receipt_backed(&self) -> bool {
        self.backing == WorkloadEvidenceBacking::Receipt
    }

    pub fn is_admitted(&self) -> bool {
        self.support == WorkloadEvidenceSupport::Admitted
    }
}

pub fn complete_ledger_stage_snapshot(
    ledger: &CompleteWorkloadEvidenceLedger,
    stage: WorkloadEvidenceStage,
) -> Option<CertifiedWorkloadEvidenceStageSnapshot> {
    ledger
        .row_for_stage(stage)
        .map(CertifiedWorkloadEvidenceStageSnapshot::from_row)
}

pub fn complete_ledger_stage_snapshots(
    ledger: &CompleteWorkloadEvidenceLedger,
) -> Vec<CertifiedWorkloadEvidenceStageSnapshot> {
    ledger
        .rows()
        .iter()
        .map(CertifiedWorkloadEvidenceStageSnapshot::from_row)
        .collect()
}

pub fn ledger_stage_snapshot(
    ledger: &WorkloadEvidenceLedger,
    stage: WorkloadEvidenceStage,
) -> Option<CertifiedWorkloadEvidenceStageSnapshot> {
    ledger
        .row_for_stage(stage)
        .map(CertifiedWorkloadEvidenceStageSnapshot::from_row)
}

pub fn matched_boolean_receipt_snapshot<T: BooleanEvidenceReceipt + 'static>(
    ledger: &CompleteWorkloadEvidenceLedger,
    receipt: &T,
) -> Result<CertifiedWorkloadEvidenceStageSnapshot, WorkloadEvidenceLedgerError> {
    ledger.require_boolean_receipt_lookup(receipt)?;
    ledger
        .row_for_stage(receipt.boolean_stage().evidence_stage())
        .map(CertifiedWorkloadEvidenceStageSnapshot::from_row)
        .ok_or(WorkloadEvidenceLedgerError::MissingBooleanStage(
            receipt.boolean_stage().evidence_stage(),
        ))
}

pub fn complete_ledger_with_additional_rows(
    ledger: &CompleteWorkloadEvidenceLedger,
    additional_rows: Vec<WorkloadEvidenceRow>,
) -> Result<CompleteWorkloadEvidenceLedger, WorkloadEvidenceLedgerError> {
    let mut rows = ledger.rows().to_vec();
    rows.extend(additional_rows);
    WorkloadEvidenceLedger::from_rows(rows)?.certify_complete()
}

/// Builds a receipt-backed row only for certification tests that exercise
/// malformed stage state without exposing receipt authority through the facade.
pub fn certification_only_admitted_stage_row(
    stage: WorkloadEvidenceStage,
    evidence_identity: impl Into<String>,
    counters: WorkloadEvidenceStageCounters,
) -> WorkloadEvidenceRow {
    WorkloadEvidenceRow::certification_only_admitted(stage, evidence_identity, counters)
}

/// Builds an unsupported receipt-backed row only for certification tests that
/// need to prove production lookups reject weaker evidence posture.
pub fn certification_only_unsupported_stage_row(
    stage: WorkloadEvidenceStage,
    evidence_identity: impl Into<String>,
    counters: WorkloadEvidenceStageCounters,
) -> WorkloadEvidenceRow {
    WorkloadEvidenceRow::certification_only_with_support(
        stage,
        evidence_identity,
        WorkloadEvidenceSupport::Unsupported,
        counters,
    )
}

pub fn ledger_with_manual_stage_substitution(
    ledger: &CompleteWorkloadEvidenceLedger,
    stage: WorkloadEvidenceStage,
) -> Result<WorkloadEvidenceLedger, WorkloadEvidenceLedgerError> {
    let rows = ledger
        .rows()
        .iter()
        .map(|row| {
            if row.stage() == stage {
                WorkloadEvidenceRow::new(stage, row.evidence_identity())
            } else {
                row.clone()
            }
        })
        .collect();
    WorkloadEvidenceLedger::from_rows(rows)
}
