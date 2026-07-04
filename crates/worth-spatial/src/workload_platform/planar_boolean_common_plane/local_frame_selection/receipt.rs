use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt;
use crate::planar_contracts::contract_bundle::PlanarM7ReadinessReceipt;
use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, BooleanEvidenceReceiptSealed, BooleanEvidenceRowAuthority,
    BooleanEvidenceStageKind, WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};
use crate::workload_platform::projection_workload::common_plane_projection_local_basis_identity;

use super::denial::PlanarBooleanCommonPlaneLocalFrameSelectionDenial;
use super::validation::validate_local_frame_selection;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanCommonPlaneLocalFrameSelectionReceipt {
    local_frame_selection_receipt_identity: String,
    shared_plane_receipt_identity: String,
    shared_plane_identity: String,
    plane_agreement_identity: String,
    local_frame_fact_digest: String,
    frame_identity: String,
    precision_fact_digest: String,
    topology_basis_identity: String,
    movement_rotation_posture_identity: String,
}

impl PlanarBooleanCommonPlaneLocalFrameSelectionReceipt {
    pub fn from_shared_plane_identity_and_m7_readiness(
        shared_plane_receipt: &PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt,
        readiness: &PlanarM7ReadinessReceipt,
    ) -> Result<Self, PlanarBooleanCommonPlaneLocalFrameSelectionDenial> {
        let receipt = Self {
            local_frame_selection_receipt_identity: String::new(),
            shared_plane_receipt_identity: shared_plane_receipt
                .shared_plane_receipt_identity()
                .to_string(),
            shared_plane_identity: shared_plane_receipt.shared_plane_identity().to_string(),
            plane_agreement_identity: shared_plane_receipt.plane_agreement_identity().to_string(),
            local_frame_fact_digest: readiness.local_frame_receipt().fact_digest().to_string(),
            frame_identity: readiness
                .local_frame_receipt()
                .basis()
                .frame_identity()
                .to_string(),
            precision_fact_digest: readiness.precision_receipt().fact_digest().to_string(),
            topology_basis_identity: readiness.topology_basis_identity().to_string(),
            movement_rotation_posture_identity: readiness
                .movement_rotation_posture_identity()
                .to_string(),
        };
        validate_local_frame_selection(&receipt, readiness)?;
        Ok(Self {
            local_frame_selection_receipt_identity: local_frame_selection_receipt_identity(
                &receipt,
            ),
            ..receipt
        })
    }

    pub fn local_frame_selection_receipt_identity(&self) -> &str {
        &self.local_frame_selection_receipt_identity
    }

    pub fn shared_plane_receipt_identity(&self) -> &str {
        &self.shared_plane_receipt_identity
    }

    pub fn shared_plane_identity(&self) -> &str {
        &self.shared_plane_identity
    }

    pub fn plane_agreement_identity(&self) -> &str {
        &self.plane_agreement_identity
    }

    pub fn local_frame_fact_digest(&self) -> &str {
        &self.local_frame_fact_digest
    }

    pub fn frame_identity(&self) -> &str {
        &self.frame_identity
    }

    pub fn precision_fact_digest(&self) -> &str {
        &self.precision_fact_digest
    }

    pub fn topology_basis_identity(&self) -> &str {
        &self.topology_basis_identity
    }

    pub fn movement_rotation_posture_identity(&self) -> &str {
        &self.movement_rotation_posture_identity
    }

    pub fn projection_local_basis_identity(&self) -> String {
        common_plane_projection_local_basis_identity(self)
    }
}

impl BooleanEvidenceReceipt for PlanarBooleanCommonPlaneLocalFrameSelectionReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::LocalFrameSelection
    }

    fn evidence_identity(&self) -> &str {
        self.local_frame_selection_receipt_identity()
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_local_frame_selection()
    }
}

impl BooleanEvidenceReceiptSealed for PlanarBooleanCommonPlaneLocalFrameSelectionReceipt {}

impl BooleanEvidenceRowAuthority for PlanarBooleanCommonPlaneLocalFrameSelectionReceipt {}

fn local_frame_selection_receipt_identity(
    receipt: &PlanarBooleanCommonPlaneLocalFrameSelectionReceipt,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-common-plane-local-frame-selection".to_string(),
            format!(
                "shared-plane-receipt:{}",
                receipt.shared_plane_receipt_identity()
            ),
            format!("shared-plane:{}", receipt.shared_plane_identity()),
            format!("plane-agreement:{}", receipt.plane_agreement_identity()),
            format!("local-frame-fact:{}", receipt.local_frame_fact_digest()),
            format!("frame:{}", receipt.frame_identity()),
            format!("precision:{}", receipt.precision_fact_digest()),
            format!("topology:{}", receipt.topology_basis_identity()),
            format!(
                "movement-rotation:{}",
                receipt.movement_rotation_posture_identity()
            ),
        ],
    )
}

#[cfg(test)]
#[path = "receipt_test_support.rs"]
mod receipt_test_support;
#[cfg(test)]
pub(crate) use receipt_test_support::{readiness_receipt, shared_plane_identity_receipt};
#[cfg(test)]
#[path = "receipt_tests.rs"]
mod receipt_tests;
