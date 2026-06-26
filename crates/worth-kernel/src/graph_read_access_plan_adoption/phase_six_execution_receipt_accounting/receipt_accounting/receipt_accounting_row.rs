use crate::graph_read_access_plan_adoption::{
    WorthGraphReadAccessSliceReceiptProjection, WorthGraphReadAccessSpatialDensePostureOutcome,
    WorthGraphReadAccessSpatialDensePostureProjection,
};

use super::super::stable_digest;
use super::receipt_identity::{
    WorthGraphReadAccessReceiptIdentity, WorthGraphReadAccessReceiptIdentityInput,
};
use super::WorthGraphReadAccessReceiptStatus;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessReceiptAccountingRow {
    receipt_identity: WorthGraphReadAccessReceiptIdentity,
    status: WorthGraphReadAccessReceiptStatus,
    source_projection_digest: String,
    row_digest: String,
}

impl WorthGraphReadAccessReceiptAccountingRow {
    pub(crate) fn from_phase_four_receipt(
        receipt: &WorthGraphReadAccessSliceReceiptProjection,
    ) -> Self {
        let status = if receipt.claims_graph_read_receipt() {
            WorthGraphReadAccessReceiptStatus::ExecutedThroughQueryReceipt
        } else {
            WorthGraphReadAccessReceiptStatus::AdmittedPlanRequiresExecutionReceipt
        };
        Self::new(
            WorthGraphReadAccessReceiptIdentity::from_input(
                WorthGraphReadAccessReceiptIdentityInput {
                    source_kind: "phase_four_vertical_slice".to_string(),
                    source_projection_digest: receipt.projection_digest().to_string(),
                    read_family_identity_digest: receipt
                        .declared_read_family_identity_digest()
                        .map(str::to_string),
                    requirement_row_digest: receipt.requirement_row_digest().map(str::to_string),
                    query_family_digest_seed: receipt
                        .executed_read_family_digest()
                        .or_else(|| receipt.declared_read_family_identity_digest())
                        .unwrap_or("none")
                        .to_string(),
                    query_posture: receipt.status().as_str().to_string(),
                    touched_authority_digest: receipt.selected_slice_digest().to_string(),
                    execution_basis: receipt.execution_basis().to_string(),
                    policy_narrowing_digest: None,
                    plan_digest: receipt.admitted_plan_digest().map(str::to_string),
                    receipt_digest: receipt.plan_consumption_digest().map(str::to_string),
                    execution_counter_digest: receipt.plan_consumption_digest().map(str::to_string),
                },
            ),
            status,
        )
    }

    pub(crate) fn from_spatial_dense_projection(
        projection: &WorthGraphReadAccessSpatialDensePostureProjection,
    ) -> Self {
        let status = receipt_status_for_projection(projection);
        Self::new(
            WorthGraphReadAccessReceiptIdentity::from_input(
                WorthGraphReadAccessReceiptIdentityInput {
                    source_kind: projection.slice_kind().as_str().to_string(),
                    source_projection_digest: projection.projection_digest().to_string(),
                    read_family_identity_digest: projection
                        .read_family_identity_digest()
                        .map(str::to_string),
                    requirement_row_digest: projection.requirement_row_digest().map(str::to_string),
                    query_family_digest_seed: projection.query_family_digest_seed().to_string(),
                    query_posture: projection.query_posture().to_string(),
                    touched_authority_digest: projection
                        .read_family_identity_digest()
                        .or_else(|| projection.requirement_row_digest())
                        .unwrap_or_else(|| projection.source_posture_row_digest())
                        .to_string(),
                    execution_basis: "phase_five_posture_projection".to_string(),
                    policy_narrowing_digest: None,
                    plan_digest: projection.query_plan_digest().map(str::to_string),
                    receipt_digest: projection.query_receipt_digest().map(str::to_string),
                    execution_counter_digest: projection
                        .execution_counter_digest()
                        .map(str::to_string),
                },
            ),
            status,
        )
    }

    fn new(
        receipt_identity: WorthGraphReadAccessReceiptIdentity,
        status: WorthGraphReadAccessReceiptStatus,
    ) -> Self {
        let source_projection_digest = receipt_identity.source_projection_digest().to_string();
        let row_digest = stable_digest(&[
            "worth_graph_read_access_receipt_accounting_row_v1".to_string(),
            format!("identity:{}", receipt_identity.identity_digest()),
            format!("status:{}", status.as_str()),
        ]);
        Self {
            receipt_identity,
            status,
            source_projection_digest,
            row_digest,
        }
    }

    pub const fn receipt_identity(&self) -> &WorthGraphReadAccessReceiptIdentity {
        &self.receipt_identity
    }

    pub const fn status(&self) -> WorthGraphReadAccessReceiptStatus {
        self.status
    }

    pub fn source_projection_digest(&self) -> &str {
        &self.source_projection_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

fn receipt_status_for_projection(
    projection: &WorthGraphReadAccessSpatialDensePostureProjection,
) -> WorthGraphReadAccessReceiptStatus {
    match projection.outcome() {
        WorthGraphReadAccessSpatialDensePostureOutcome::ExecutedThroughQueryReceipt => {
            WorthGraphReadAccessReceiptStatus::ExecutedThroughQueryReceipt
        }
        WorthGraphReadAccessSpatialDensePostureOutcome::AdmittedPlanRequiresExecutionReceipt => {
            WorthGraphReadAccessReceiptStatus::AdmittedPlanRequiresExecutionReceipt
        }
        WorthGraphReadAccessSpatialDensePostureOutcome::RequiredQueryPosture => {
            WorthGraphReadAccessReceiptStatus::RequiredQueryPostureNoReceipt
        }
        WorthGraphReadAccessSpatialDensePostureOutcome::DeniedByQueryPosture => {
            WorthGraphReadAccessReceiptStatus::DeniedByQueryPostureNoReceipt
        }
        WorthGraphReadAccessSpatialDensePostureOutcome::CarriedCapabilityGap => {
            WorthGraphReadAccessReceiptStatus::CarriedCapabilityGapNoReceipt
        }
    }
}

#[cfg(test)]
mod adversarial_receipt_accounting_row {
    use super::*;

    impl WorthGraphReadAccessReceiptAccountingRow {
        pub(crate) fn with_status_for_tests(
            &self,
            status: WorthGraphReadAccessReceiptStatus,
        ) -> Self {
            Self::new(self.receipt_identity.clone(), status)
        }
    }
}
