use worth_spatial::facade::planar_boolean_common_plane::{
    PlanarBooleanCommonPlaneOperandSide, PlanarBooleanCommonPlaneReducedOperandPairDenialKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCommonPlaneReducedOperandPairAssemblyError {
    SpatialReducedOperandPairDenied {
        kind: PlanarBooleanCommonPlaneReducedOperandPairDenialKind,
        human_reason: &'static str,
    },
    LeftOperandProjectionIdentityMismatch {
        expected_left_projection_identity: String,
        actual_left_projection_identity: String,
    },
    RightOperandProjectionIdentityMismatch {
        expected_right_projection_identity: String,
        actual_right_projection_identity: String,
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
    LocalFrameSelectionIdentityMismatch {
        expected_local_frame_selection_identity: String,
        actual_local_frame_selection_identity: String,
    },
    ProjectionLocalBasisIdentityMismatch {
        expected_projection_local_basis_identity: String,
        actual_projection_local_basis_identity: String,
    },
    LeftProjectionStageIdentityMismatch {
        expected_left_projection_stage_identity: String,
        actual_left_projection_stage_identity: String,
    },
    RightProjectionStageIdentityMismatch {
        expected_right_projection_stage_identity: String,
        actual_right_projection_stage_identity: String,
    },
    OrderingContractMismatch {
        expected_first_slot_side: PlanarBooleanCommonPlaneOperandSide,
        actual_first_slot_side: PlanarBooleanCommonPlaneOperandSide,
    },
}

impl PlanarBooleanCommonPlaneReducedOperandPairAssemblyError {
    pub fn human_reason(&self) -> &'static str {
        match self {
            Self::SpatialReducedOperandPairDenied { human_reason, .. } => human_reason,
            Self::LeftOperandProjectionIdentityMismatch { .. } => {
                "reduced operand-pair assembly must preserve the certified left operand projection identity"
            }
            Self::RightOperandProjectionIdentityMismatch { .. } => {
                "reduced operand-pair assembly must preserve the certified right operand projection identity"
            }
            Self::SharedPlaneReceiptIdentityMismatch { .. } => {
                "reduced operand-pair assembly must preserve one shared-plane receipt identity across both operands"
            }
            Self::SharedPlaneIdentityMismatch { .. } => {
                "reduced operand-pair assembly must preserve one shared-plane identity across both operands"
            }
            Self::PlaneAgreementIdentityMismatch { .. } => {
                "reduced operand-pair assembly must preserve one plane-agreement identity across both operands"
            }
            Self::LocalFrameSelectionIdentityMismatch { .. } => {
                "reduced operand-pair assembly must preserve one local-frame selection identity across both operands"
            }
            Self::ProjectionLocalBasisIdentityMismatch { .. } => {
                "reduced operand-pair assembly must preserve one projected local-basis identity across both operands"
            }
            Self::LeftProjectionStageIdentityMismatch { .. } => {
                "reduced operand-pair assembly must preserve the certified left projection-stage identity"
            }
            Self::RightProjectionStageIdentityMismatch { .. } => {
                "reduced operand-pair assembly must preserve the certified right projection-stage identity"
            }
            Self::OrderingContractMismatch { .. } => {
                "reduced operand-pair assembly must preserve the semantic left-to-right ordering contract"
            }
        }
    }
}
