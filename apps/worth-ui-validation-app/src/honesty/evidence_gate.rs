use worth_ui_harness::facade::{HarnessEvidenceFamily, HarnessRunReceipt};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationAppEvidenceGate;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationAppEvidenceGateDenial {
    MissingRuntimeReceipt,
    MissingActivePlanObservation,
}

impl ValidationAppEvidenceGate {
    pub fn require_runtime_backed_receipt(
        receipt: &HarnessRunReceipt,
    ) -> Result<(), ValidationAppEvidenceGateDenial> {
        let completed_step_index = completed_receipt_step_index(receipt)?;
        reject_missing_runtime_receipt(receipt, completed_step_index)?;
        reject_missing_active_plan_observation(receipt, completed_step_index)?;
        Ok(())
    }
}

fn completed_receipt_step_index(
    receipt: &HarnessRunReceipt,
) -> Result<usize, ValidationAppEvidenceGateDenial> {
    receipt
        .completed_steps()
        .checked_sub(1)
        .ok_or(ValidationAppEvidenceGateDenial::MissingRuntimeReceipt)
}

fn reject_missing_runtime_receipt(
    receipt: &HarnessRunReceipt,
    completed_step_index: usize,
) -> Result<(), ValidationAppEvidenceGateDenial> {
    if receipt
        .evidence_ledger()
        .contains_family_at_step(completed_step_index, HarnessEvidenceFamily::RuntimeReceipt)
    {
        Ok(())
    } else {
        Err(ValidationAppEvidenceGateDenial::MissingRuntimeReceipt)
    }
}

fn reject_missing_active_plan_observation(
    receipt: &HarnessRunReceipt,
    completed_step_index: usize,
) -> Result<(), ValidationAppEvidenceGateDenial> {
    if receipt.evidence_ledger().contains_family_at_step(
        completed_step_index,
        HarnessEvidenceFamily::ActivePlanObservation,
    ) {
        Ok(())
    } else {
        Err(ValidationAppEvidenceGateDenial::MissingActivePlanObservation)
    }
}
