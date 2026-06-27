use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, WorkloadEvidenceBooleanReceiptLookupProduct,
    WorkloadEvidenceLedgerError, WorkloadEvidenceRow, WorkloadEvidenceStage,
    WorkloadEvidenceStageLinkSet,
};

use super::super::stage_links::link_required_stages;
use super::counters::WorkloadEvidenceStageIndexCounters;
use super::identity::stage_index_identity;
use super::receipt_match::{match_boolean_receipt_lookup, match_boolean_row_lookup};
use super::validation::{build_stage_offsets, stage_index_counters};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadEvidenceStageIndexProduct {
    rows: Vec<WorkloadEvidenceRow>,
    stage_offsets: [Option<usize>; WorkloadEvidenceStage::STAGE_COUNT],
    counters: WorkloadEvidenceStageIndexCounters,
    index_identity: String,
}

impl WorkloadEvidenceStageIndexProduct {
    pub(crate) fn new(rows: Vec<WorkloadEvidenceRow>) -> Result<Self, WorkloadEvidenceLedgerError> {
        if rows.is_empty() {
            return Err(WorkloadEvidenceLedgerError::EmptyLedger);
        }
        if rows
            .iter()
            .any(|row| row.evidence_identity().trim().is_empty())
        {
            return Err(WorkloadEvidenceLedgerError::MissingEvidenceIdentity);
        }
        let stage_offsets = build_stage_offsets(&rows)?;
        let counters = stage_index_counters(&rows, &stage_offsets);
        let index_identity = stage_index_identity(&rows, counters);
        Ok(Self {
            rows,
            stage_offsets,
            counters,
            index_identity,
        })
    }

    pub fn index_identity(&self) -> &str {
        &self.index_identity
    }

    pub(crate) fn rows(&self) -> &[WorkloadEvidenceRow] {
        &self.rows
    }

    pub fn counters(&self) -> WorkloadEvidenceStageIndexCounters {
        self.counters
    }

    pub(crate) fn row_for_stage(
        &self,
        stage: WorkloadEvidenceStage,
    ) -> Option<&WorkloadEvidenceRow> {
        self.stage_offsets
            .get(stage.index_slot())
            .and_then(|offset| offset.map(|row_index| &self.rows[row_index]))
    }

    pub(crate) fn evidence_for_stage(&self, stage: WorkloadEvidenceStage) -> Option<&str> {
        self.row_for_stage(stage)
            .map(WorkloadEvidenceRow::evidence_identity)
    }

    pub fn missing_authority_stage(&self) -> Option<WorkloadEvidenceStage> {
        WorkloadEvidenceStage::AUTHORITY_STAGES
            .iter()
            .copied()
            .find(|stage| self.row_for_stage(*stage).is_none())
    }

    pub fn first_manual_authority_stage(&self) -> Option<WorkloadEvidenceStage> {
        WorkloadEvidenceStage::AUTHORITY_STAGES
            .iter()
            .copied()
            .find(|stage| {
                self.row_for_stage(*stage)
                    .is_some_and(|row| !row.is_receipt_backed())
            })
    }

    pub fn first_unadmitted_authority_stage(&self) -> Option<WorkloadEvidenceStage> {
        WorkloadEvidenceStage::AUTHORITY_STAGES
            .iter()
            .copied()
            .find(|stage| {
                self.row_for_stage(*stage)
                    .is_some_and(|row| row.is_receipt_backed() && !row.is_admitted())
            })
    }

    pub fn require_boolean_receipt<T: BooleanEvidenceReceipt + 'static>(
        &self,
        receipt: &T,
    ) -> Result<(), WorkloadEvidenceLedgerError> {
        self.require_boolean_receipt_lookup(receipt).map(|_| ())
    }

    pub fn require_boolean_receipt_lookup<T: BooleanEvidenceReceipt + 'static>(
        &self,
        receipt: &T,
    ) -> Result<WorkloadEvidenceBooleanReceiptLookupProduct, WorkloadEvidenceLedgerError> {
        match_boolean_receipt_lookup(self, receipt)
    }

    pub(crate) fn require_boolean_row_lookup(
        &self,
        stage: WorkloadEvidenceStage,
        evidence_identity: &str,
        support: crate::workload_platform::evidence_ledger::WorkloadEvidenceSupport,
        counters: crate::workload_platform::evidence_ledger::WorkloadEvidenceStageCounters,
    ) -> Result<WorkloadEvidenceBooleanReceiptLookupProduct, WorkloadEvidenceLedgerError> {
        match_boolean_row_lookup(self, stage, evidence_identity, support, counters)
    }

    pub fn link_required_stages(
        &self,
        stages: &[WorkloadEvidenceStage],
    ) -> Result<WorkloadEvidenceStageLinkSet, WorkloadEvidenceLedgerError> {
        link_required_stages(self, stages)
    }
}

#[cfg(test)]
mod tests;
