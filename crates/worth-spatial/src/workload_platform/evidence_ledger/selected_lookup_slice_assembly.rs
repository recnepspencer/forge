#![allow(dead_code)]

use super::{
    BooleanEvidenceRowAuthority, CompleteWorkloadEvidenceLedger,
    SpatialGeometryEvidenceTouchAuthority, WorkloadEvidenceLedger, WorkloadEvidenceLedgerError,
    WorkloadEvidenceRow,
};
use crate::workload_platform::evidence_lookup_input_admission::EvidenceLookupStageReceiptAdmission;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedLookupSliceLedger {
    ledger: CompleteWorkloadEvidenceLedger,
}

impl SelectedLookupSliceLedger {
    #[cfg(test)]
    pub(crate) fn from_complete_ledger(ledger: CompleteWorkloadEvidenceLedger) -> Self {
        Self { ledger }
    }

    pub(crate) fn complete_ledger(&self) -> &CompleteWorkloadEvidenceLedger {
        &self.ledger
    }

    pub fn counters(&self) -> super::WorkloadEvidenceCounters {
        self.ledger.counters()
    }
}

pub struct SelectedLookupSliceLedgerAssembly<'a> {
    authority: &'a SpatialGeometryEvidenceTouchAuthority,
    stage_receipt: &'a EvidenceLookupStageReceiptAdmission,
    additional_boolean_rows: Vec<WorkloadEvidenceRow>,
}

impl<'a> SelectedLookupSliceLedgerAssembly<'a> {
    pub fn from_touch_authority(
        authority: &'a SpatialGeometryEvidenceTouchAuthority,
        stage_receipt: &'a EvidenceLookupStageReceiptAdmission,
    ) -> Self {
        Self {
            authority,
            stage_receipt,
            additional_boolean_rows: Vec::new(),
        }
    }

    pub fn with_additional_boolean_receipt<T: BooleanEvidenceRowAuthority + 'static>(
        mut self,
        receipt: &'a T,
    ) -> Self {
        self.additional_boolean_rows
            .push(WorkloadEvidenceRow::from_boolean_evidence_receipt(receipt));
        self
    }

    pub fn assemble(self) -> Result<CompleteWorkloadEvidenceLedger, WorkloadEvidenceLedgerError> {
        Ok(self.assemble_complete_ledger()?)
    }

    pub fn assemble_selected_lookup_slice(
        self,
    ) -> Result<SelectedLookupSliceLedger, WorkloadEvidenceLedgerError> {
        if let Some(row) = self.additional_boolean_rows.first() {
            return Err(WorkloadEvidenceLedgerError::SelectedLookupSliceExceedsScope(row.stage()));
        }
        Ok(SelectedLookupSliceLedger {
            ledger: self.assemble_complete_ledger()?,
        })
    }

    fn assemble_complete_ledger(
        self,
    ) -> Result<CompleteWorkloadEvidenceLedger, WorkloadEvidenceLedgerError> {
        if self.stage_receipt.spatial_touch_digest() != self.authority.digest().as_str() {
            return Err(WorkloadEvidenceLedgerError::MismatchedBooleanStage(
                self.stage_receipt.stage(),
            ));
        }
        if self.stage_receipt.stage() != self.authority.evidence_stage() {
            return Err(WorkloadEvidenceLedgerError::MismatchedBooleanStage(
                self.stage_receipt.stage(),
            ));
        }
        if self.stage_receipt.stage_receipt_digest() != self.authority.evidence_identity() {
            return Err(WorkloadEvidenceLedgerError::MismatchedBooleanStage(
                self.stage_receipt.stage(),
            ));
        }

        let mut rows = self.authority.authority_rows().to_vec();
        rows.push(self.authority.selected_receipt_row());
        rows.extend(self.additional_boolean_rows);
        WorkloadEvidenceLedger::from_rows(rows)?.certify_complete()
    }
}
