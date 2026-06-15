#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCommonPlanePrecisionAgreementError {
    MissingBooleanDeclarationBoundary,
    PrecisionFactDigestMismatch {
        expected_precision_fact_digest: String,
        actual_precision_fact_digest: String,
    },
    LocalFrameFactDigestMismatch {
        expected_local_frame_fact_digest: String,
        actual_local_frame_fact_digest: String,
    },
    TopologyBasisIdentityMismatch {
        expected_topology_basis_identity: String,
        actual_topology_basis_identity: String,
    },
    MovementRotationPostureIdentityMismatch {
        expected_movement_rotation_posture_identity: String,
        actual_movement_rotation_posture_identity: String,
    },
}

impl PlanarBooleanCommonPlanePrecisionAgreementError {
    pub fn human_reason(&self) -> &'static str {
        match self {
            Self::MissingBooleanDeclarationBoundary => {
                "common-plane precision agreement requires the real 7.0 boolean declaration boundary"
            }
            Self::PrecisionFactDigestMismatch { .. } => {
                "precision agreement receipt must preserve the certified M7 precision fact digest"
            }
            Self::LocalFrameFactDigestMismatch { .. } => {
                "precision agreement receipt must preserve the certified M7 local-frame fact digest"
            }
            Self::TopologyBasisIdentityMismatch { .. } => {
                "precision agreement receipt must preserve the certified M7 topology basis identity"
            }
            Self::MovementRotationPostureIdentityMismatch { .. } => {
                "precision agreement receipt must preserve the certified M7 movement and rotation posture identity"
            }
        }
    }
}
