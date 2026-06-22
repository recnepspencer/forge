use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::planar_contracts::contract_bundle::PlanarM7ReadinessReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanCommonPlanePrecisionAgreementReceipt {
    precision_agreement_receipt_identity: String,
    precision_fact_digest: String,
    local_frame_fact_digest: String,
    topology_basis_identity: String,
    movement_rotation_posture_identity: String,
}

impl PlanarBooleanCommonPlanePrecisionAgreementReceipt {
    pub fn from_m7_readiness(readiness: &PlanarM7ReadinessReceipt) -> Self {
        Self::from_certified_parts(
            readiness.precision_fact_digest(),
            readiness.local_frame_fact_digest(),
            readiness.topology_basis_identity(),
            readiness.movement_rotation_posture_identity(),
        )
    }

    pub fn from_certified_parts(
        precision_fact_digest: impl Into<String>,
        local_frame_fact_digest: impl Into<String>,
        topology_basis_identity: impl Into<String>,
        movement_rotation_posture_identity: impl Into<String>,
    ) -> Self {
        let precision_fact_digest = precision_fact_digest.into();
        let local_frame_fact_digest = local_frame_fact_digest.into();
        let topology_basis_identity = topology_basis_identity.into();
        let movement_rotation_posture_identity = movement_rotation_posture_identity.into();
        let precision_agreement_receipt_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "planar-boolean-common-plane-precision-agreement".to_string(),
                format!("precision:{precision_fact_digest}"),
                format!("local-frame:{local_frame_fact_digest}"),
                format!("topology:{topology_basis_identity}"),
                format!("movement-rotation:{movement_rotation_posture_identity}"),
            ],
        );

        Self {
            precision_agreement_receipt_identity,
            precision_fact_digest,
            local_frame_fact_digest,
            topology_basis_identity,
            movement_rotation_posture_identity,
        }
    }

    pub fn precision_agreement_receipt_identity(&self) -> &str {
        &self.precision_agreement_receipt_identity
    }

    pub fn precision_fact_digest(&self) -> &str {
        &self.precision_fact_digest
    }

    pub fn local_frame_fact_digest(&self) -> &str {
        &self.local_frame_fact_digest
    }

    pub fn topology_basis_identity(&self) -> &str {
        &self.topology_basis_identity
    }

    pub fn movement_rotation_posture_identity(&self) -> &str {
        &self.movement_rotation_posture_identity
    }
}
