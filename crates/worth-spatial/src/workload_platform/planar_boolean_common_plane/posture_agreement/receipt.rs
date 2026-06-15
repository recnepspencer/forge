use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::PlanarBooleanCommonPlanePostureWitness;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanCommonPlanePostureAgreementReceipt {
    agreement_identity: String,
    shared_posture_identity: String,
    left_witness: PlanarBooleanCommonPlanePostureWitness,
    right_witness: PlanarBooleanCommonPlanePostureWitness,
}

impl PlanarBooleanCommonPlanePostureAgreementReceipt {
    pub(crate) fn new(
        declaration: &str,
        left_witness: PlanarBooleanCommonPlanePostureWitness,
        right_witness: PlanarBooleanCommonPlanePostureWitness,
    ) -> Self {
        let shared_posture_identity = left_witness.semantic_posture_identity().to_string();
        let agreement_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "planar-boolean-common-plane-posture-agreement".to_string(),
                format!("declaration:{declaration}"),
                format!("shared-posture:{shared_posture_identity}"),
                format!(
                    "left-projected-workload:{}",
                    left_witness.projected_workload_identity()
                ),
                format!(
                    "right-projected-workload:{}",
                    right_witness.projected_workload_identity()
                ),
                format!(
                    "left-transform-stage:{}",
                    left_witness.transform_stage_identity()
                ),
                format!(
                    "right-transform-stage:{}",
                    right_witness.transform_stage_identity()
                ),
            ],
        );
        Self {
            agreement_identity,
            shared_posture_identity,
            left_witness,
            right_witness,
        }
    }

    pub fn agreement_identity(&self) -> &str {
        &self.agreement_identity
    }

    pub fn shared_posture_identity(&self) -> &str {
        &self.shared_posture_identity
    }

    pub fn left_witness(&self) -> &PlanarBooleanCommonPlanePostureWitness {
        &self.left_witness
    }

    pub fn right_witness(&self) -> &PlanarBooleanCommonPlanePostureWitness {
        &self.right_witness
    }
}
