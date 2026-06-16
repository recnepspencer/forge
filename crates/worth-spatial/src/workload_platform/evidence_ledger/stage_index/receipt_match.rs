use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, WorkloadEvidenceLedgerError, WorkloadEvidenceSupport,
};

use super::product::WorkloadEvidenceStageIndexProduct;

pub(crate) fn match_boolean_receipt(
    product: &WorkloadEvidenceStageIndexProduct,
    receipt: &impl BooleanEvidenceReceipt,
) -> Result<(), WorkloadEvidenceLedgerError> {
    let stage = receipt.boolean_stage().evidence_stage();
    let row = product
        .row_for_stage(stage)
        .ok_or(WorkloadEvidenceLedgerError::MissingBooleanStage(stage))?;
    if !row.is_receipt_backed() {
        return Err(WorkloadEvidenceLedgerError::ManualBooleanStage(stage));
    }
    if !row.counters().has_receipt_backed_counter_for_stage(stage) {
        return Err(WorkloadEvidenceLedgerError::CounterlessBooleanStage(stage));
    }
    if row.evidence_identity() != receipt.evidence_identity() {
        return Err(WorkloadEvidenceLedgerError::MismatchedBooleanStage(stage));
    }
    if row.support() != receipt.evidence_support() {
        return Err(match receipt.evidence_support() {
            WorkloadEvidenceSupport::Manual => {
                WorkloadEvidenceLedgerError::ManualBooleanStage(stage)
            }
            WorkloadEvidenceSupport::Admitted
            | WorkloadEvidenceSupport::Unsupported
            | WorkloadEvidenceSupport::Blocked => {
                WorkloadEvidenceLedgerError::UnsupportedBooleanStage(stage)
            }
        });
    }
    Ok(())
}
