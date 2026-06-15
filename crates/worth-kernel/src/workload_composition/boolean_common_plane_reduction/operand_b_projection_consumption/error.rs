use worth_spatial::facade::planar_boolean_common_plane::{
    PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind,
    PlanarBooleanCommonPlaneOperandSide,
};
use worth_spatial::facade::projection_workload::UnsupportedProjectionReasonCode;

use crate::workload_composition::boolean_common_plane_reduction::operand_projection_consumption_support::OperandProjectionSupportError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCommonPlaneOperandBProjectionConsumptionError {
    OperandProjectionWorkloadDenied {
        kind: UnsupportedProjectionReasonCode,
        human_reason: String,
    },
    RetainedOperandProjectionConsumptionDenied {
        kind: PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind,
        human_reason: &'static str,
    },
    OperandSideMismatch {
        expected_operand_side: PlanarBooleanCommonPlaneOperandSide,
        actual_operand_side: PlanarBooleanCommonPlaneOperandSide,
    },
    LocalFrameSelectionIdentityMismatch {
        expected_local_frame_selection_identity: String,
        actual_local_frame_selection_identity: String,
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
    ProjectionStageIdentityMismatch {
        expected_projection_stage_identity: String,
        actual_projection_stage_identity: String,
    },
    UpstreamSurfaceSupportIdentityMismatch {
        expected_upstream_surface_support_identity: String,
        actual_upstream_surface_support_identity: String,
    },
    CertifiedPlaneSupportIdentityMismatch {
        expected_certified_plane_support_identity: String,
        actual_certified_plane_support_identity: String,
    },
    ProjectionLocalBasisIdentityMismatch {
        expected_projection_local_basis_identity: String,
        actual_projection_local_basis_identity: String,
    },
    ProjectedEntityCountMismatch {
        expected_projected_entity_count: usize,
        actual_projected_entity_count: usize,
    },
}

impl PlanarBooleanCommonPlaneOperandBProjectionConsumptionError {
    pub fn human_reason(&self) -> &'static str {
        match self {
            Self::OperandProjectionWorkloadDenied { .. } => {
                "operand-B projection consumption must rebuild the right operand projection through the selected common-plane frame"
            }
            Self::RetainedOperandProjectionConsumptionDenied { human_reason, .. } => human_reason,
            Self::OperandSideMismatch { .. } => {
                "operand-B projection consumption must preserve the right operand side"
            }
            Self::LocalFrameSelectionIdentityMismatch { .. } => {
                "operand-B projection consumption must preserve the certified local-frame selection identity"
            }
            Self::SharedPlaneReceiptIdentityMismatch { .. } => {
                "operand-B projection consumption must preserve the certified shared-plane receipt identity"
            }
            Self::SharedPlaneIdentityMismatch { .. } => {
                "operand-B projection consumption must preserve the certified shared-plane identity"
            }
            Self::PlaneAgreementIdentityMismatch { .. } => {
                "operand-B projection consumption must preserve the certified plane-agreement identity"
            }
            Self::ProjectionStageIdentityMismatch { .. } => {
                "operand-B projection consumption must use the real right operand projection-stage identity"
            }
            Self::UpstreamSurfaceSupportIdentityMismatch { .. } => {
                "operand-B projection consumption must use the real right operand surface-support identity"
            }
            Self::CertifiedPlaneSupportIdentityMismatch { .. } => {
                "operand-B projection consumption must use the real right operand certified plane-support identity"
            }
            Self::ProjectionLocalBasisIdentityMismatch { .. } => {
                "operand-B projection consumption must use the real right operand projected local-basis identity"
            }
            Self::ProjectedEntityCountMismatch { .. } => {
                "operand-B projection consumption must preserve the real right operand projected topology count"
            }
        }
    }
}

impl From<OperandProjectionSupportError>
    for PlanarBooleanCommonPlaneOperandBProjectionConsumptionError
{
    fn from(error: OperandProjectionSupportError) -> Self {
        match error {
            OperandProjectionSupportError::OperandProjectionWorkloadDenied {
                kind,
                human_reason,
            } => Self::OperandProjectionWorkloadDenied { kind, human_reason },
            OperandProjectionSupportError::RetainedOperandProjectionConsumptionDenied {
                kind,
                human_reason,
            } => Self::RetainedOperandProjectionConsumptionDenied { kind, human_reason },
            OperandProjectionSupportError::OperandSideMismatch {
                expected_operand_side,
                actual_operand_side,
            } => Self::OperandSideMismatch {
                expected_operand_side,
                actual_operand_side,
            },
            OperandProjectionSupportError::LocalFrameSelectionIdentityMismatch {
                expected_local_frame_selection_identity,
                actual_local_frame_selection_identity,
            } => Self::LocalFrameSelectionIdentityMismatch {
                expected_local_frame_selection_identity,
                actual_local_frame_selection_identity,
            },
            OperandProjectionSupportError::SharedPlaneReceiptIdentityMismatch {
                expected_shared_plane_receipt_identity,
                actual_shared_plane_receipt_identity,
            } => Self::SharedPlaneReceiptIdentityMismatch {
                expected_shared_plane_receipt_identity,
                actual_shared_plane_receipt_identity,
            },
            OperandProjectionSupportError::SharedPlaneIdentityMismatch {
                expected_shared_plane_identity,
                actual_shared_plane_identity,
            } => Self::SharedPlaneIdentityMismatch {
                expected_shared_plane_identity,
                actual_shared_plane_identity,
            },
            OperandProjectionSupportError::PlaneAgreementIdentityMismatch {
                expected_plane_agreement_identity,
                actual_plane_agreement_identity,
            } => Self::PlaneAgreementIdentityMismatch {
                expected_plane_agreement_identity,
                actual_plane_agreement_identity,
            },
            OperandProjectionSupportError::ProjectionStageIdentityMismatch {
                expected_projection_stage_identity,
                actual_projection_stage_identity,
            } => Self::ProjectionStageIdentityMismatch {
                expected_projection_stage_identity,
                actual_projection_stage_identity,
            },
            OperandProjectionSupportError::UpstreamSurfaceSupportIdentityMismatch {
                expected_upstream_surface_support_identity,
                actual_upstream_surface_support_identity,
            } => Self::UpstreamSurfaceSupportIdentityMismatch {
                expected_upstream_surface_support_identity,
                actual_upstream_surface_support_identity,
            },
            OperandProjectionSupportError::CertifiedPlaneSupportIdentityMismatch {
                expected_certified_plane_support_identity,
                actual_certified_plane_support_identity,
            } => Self::CertifiedPlaneSupportIdentityMismatch {
                expected_certified_plane_support_identity,
                actual_certified_plane_support_identity,
            },
            OperandProjectionSupportError::ProjectionLocalBasisIdentityMismatch {
                expected_projection_local_basis_identity,
                actual_projection_local_basis_identity,
            } => Self::ProjectionLocalBasisIdentityMismatch {
                expected_projection_local_basis_identity,
                actual_projection_local_basis_identity,
            },
            OperandProjectionSupportError::ProjectedEntityCountMismatch {
                expected_projected_entity_count,
                actual_projected_entity_count,
            } => Self::ProjectedEntityCountMismatch {
                expected_projected_entity_count,
                actual_projected_entity_count,
            },
        }
    }
}
