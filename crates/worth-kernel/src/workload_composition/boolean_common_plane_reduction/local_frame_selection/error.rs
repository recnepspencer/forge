use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCommonPlaneLocalFrameSelectionError {
    MissingBooleanDeclarationBoundary,
    RetainedLocalFrameSelectionDenied {
        kind: PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind,
        human_reason: &'static str,
    },
    SharedPlaneReceiptIdentityMismatch {
        expected_shared_plane_receipt_identity: String,
        actual_shared_plane_receipt_identity: String,
    },
    SharedPlaneIdentityMismatch {
        expected_shared_plane_identity: String,
        actual_shared_plane_identity: String,
    },
    PlaneAgreementIdentityMismatch {
        expected_plane_agreement_identity: String,
        actual_plane_agreement_identity: String,
    },
    LocalFrameFactDigestMismatch {
        expected_local_frame_fact_digest: String,
        actual_local_frame_fact_digest: String,
    },
    FrameIdentityMismatch {
        expected_frame_identity: String,
        actual_frame_identity: String,
    },
    PrecisionFactDigestMismatch {
        expected_precision_fact_digest: String,
        actual_precision_fact_digest: String,
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

impl PlanarBooleanCommonPlaneLocalFrameSelectionError {
    pub fn human_reason(&self) -> &'static str {
        match self {
            Self::MissingBooleanDeclarationBoundary => {
                "local-frame selection requires the real 7.0 boolean declaration boundary"
            }
            Self::RetainedLocalFrameSelectionDenied { human_reason, .. } => human_reason,
            Self::SharedPlaneReceiptIdentityMismatch { .. } => {
                "local-frame selection receipt must preserve the certified shared-plane receipt identity"
            }
            Self::SharedPlaneIdentityMismatch { .. } => {
                "local-frame selection receipt must preserve the certified shared-plane identity"
            }
            Self::PlaneAgreementIdentityMismatch { .. } => {
                "local-frame selection receipt must preserve the certified plane-agreement identity"
            }
            Self::LocalFrameFactDigestMismatch { .. } => {
                "local-frame selection receipt must preserve the certified M7 local-frame fact digest"
            }
            Self::FrameIdentityMismatch { .. } => {
                "local-frame selection receipt must preserve the certified M7 frame identity"
            }
            Self::PrecisionFactDigestMismatch { .. } => {
                "local-frame selection receipt must preserve the certified M7 precision fact digest"
            }
            Self::TopologyBasisIdentityMismatch { .. } => {
                "local-frame selection receipt must preserve the certified M7 topology basis identity"
            }
            Self::MovementRotationPostureIdentityMismatch { .. } => {
                "local-frame selection receipt must preserve the certified M7 movement and rotation posture identity"
            }
        }
    }
}
