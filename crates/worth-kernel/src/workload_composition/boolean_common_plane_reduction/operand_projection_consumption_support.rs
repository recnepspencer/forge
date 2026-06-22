use worth_spatial::facade::planar_boolean_common_plane::{
    PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind,
    PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
    PlanarBooleanCommonPlaneOperandSide,
};
use worth_spatial::facade::projection_workload::{
    LocalFrameBasis, ProjectedPlanarWorkload, ProjectionWorkload, UnsupportedProjectionReasonCode,
};

use crate::workload_composition::{
    BuiltWorkloadCatalogRecipe, PlanarBooleanCommonPlaneLocalFrameSelectedRequest,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum OperandProjectionRole {
    OperandA,
    OperandB,
}

impl OperandProjectionRole {
    pub(crate) fn side(self) -> PlanarBooleanCommonPlaneOperandSide {
        match self {
            Self::OperandA => PlanarBooleanCommonPlaneOperandSide::Left,
            Self::OperandB => PlanarBooleanCommonPlaneOperandSide::Right,
        }
    }

    fn operand_label(self) -> &'static str {
        match self {
            Self::OperandA => "left",
            Self::OperandB => "right",
        }
    }

    fn source_operand<'a>(
        self,
        local_frame_selected_request: &'a PlanarBooleanCommonPlaneLocalFrameSelectedRequest,
    ) -> &'a BuiltWorkloadCatalogRecipe {
        let reduction_request = local_frame_selected_request
            .shared_plane_identified_request()
            .precision_agreed_request()
            .posture_agreed_request()
            .plane_agreed_request()
            .admitted_request()
            .reduction_request();
        match self {
            Self::OperandA => reduction_request.left(),
            Self::OperandB => reduction_request.right(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum OperandProjectionSupportError {
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

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CertifiedOperandProjection {
    projection_receipt: PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
    projected_workload: ProjectedPlanarWorkload,
    source_operand_workload_identity: String,
}

impl CertifiedOperandProjection {
    pub(crate) fn into_parts(
        self,
    ) -> (
        PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
        ProjectedPlanarWorkload,
        String,
    ) {
        (
            self.projection_receipt,
            self.projected_workload,
            self.source_operand_workload_identity,
        )
    }
}

pub(crate) fn certify_projection_from_selected_frame(
    local_frame_selected_request: &PlanarBooleanCommonPlaneLocalFrameSelectedRequest,
    role: OperandProjectionRole,
) -> Result<CertifiedOperandProjection, OperandProjectionSupportError> {
    let projected = selected_frame_projection(local_frame_selected_request, role)?;
    let projection_receipt =
        PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt::from_local_frame_selection_and_projection_receipts(
            local_frame_selected_request.selection_receipt(),
            projected.receipts(),
            role.side(),
        )
        .map_err(|denial| {
            OperandProjectionSupportError::RetainedOperandProjectionConsumptionDenied {
                kind: denial.kind(),
                human_reason: denial.human_reason(),
            }
        })?;
    certify_projection_receipt(local_frame_selected_request, role, projection_receipt)
}

pub(crate) fn certify_projection_receipt(
    local_frame_selected_request: &PlanarBooleanCommonPlaneLocalFrameSelectedRequest,
    role: OperandProjectionRole,
    projection_receipt: PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
) -> Result<CertifiedOperandProjection, OperandProjectionSupportError> {
    let expected_projection = selected_frame_projection(local_frame_selected_request, role)?;
    let expected_projection_receipts = expected_projection.receipts();

    if projection_receipt.operand_side() != role.side() {
        return Err(OperandProjectionSupportError::OperandSideMismatch {
            expected_operand_side: role.side(),
            actual_operand_side: projection_receipt.operand_side(),
        });
    }
    if projection_receipt.local_frame_selection_identity()
        != local_frame_selected_request
            .selection_receipt()
            .local_frame_selection_receipt_identity()
    {
        return Err(
            OperandProjectionSupportError::LocalFrameSelectionIdentityMismatch {
                expected_local_frame_selection_identity: local_frame_selected_request
                    .selection_receipt()
                    .local_frame_selection_receipt_identity()
                    .to_string(),
                actual_local_frame_selection_identity: projection_receipt
                    .local_frame_selection_identity()
                    .to_string(),
            },
        );
    }
    if projection_receipt.shared_plane_receipt_identity()
        != local_frame_selected_request.shared_plane_receipt_identity()
    {
        return Err(
            OperandProjectionSupportError::SharedPlaneReceiptIdentityMismatch {
                expected_shared_plane_receipt_identity: local_frame_selected_request
                    .shared_plane_receipt_identity()
                    .to_string(),
                actual_shared_plane_receipt_identity: projection_receipt
                    .shared_plane_receipt_identity()
                    .to_string(),
            },
        );
    }
    if projection_receipt.shared_plane_identity()
        != local_frame_selected_request.shared_plane_identity()
    {
        return Err(OperandProjectionSupportError::SharedPlaneIdentityMismatch {
            expected_shared_plane_identity: local_frame_selected_request
                .shared_plane_identity()
                .to_string(),
            actual_shared_plane_identity: projection_receipt.shared_plane_identity().to_string(),
        });
    }
    if projection_receipt.plane_agreement_identity()
        != local_frame_selected_request
            .selection_receipt()
            .plane_agreement_identity()
    {
        return Err(
            OperandProjectionSupportError::PlaneAgreementIdentityMismatch {
                expected_plane_agreement_identity: local_frame_selected_request
                    .selection_receipt()
                    .plane_agreement_identity()
                    .to_string(),
                actual_plane_agreement_identity: projection_receipt
                    .plane_agreement_identity()
                    .to_string(),
            },
        );
    }
    if projection_receipt.projection_stage_identity()
        != expected_projection_receipts
            .stage_identity()
            .receipt_identity()
    {
        return Err(
            OperandProjectionSupportError::ProjectionStageIdentityMismatch {
                expected_projection_stage_identity: expected_projection_receipts
                    .stage_identity()
                    .receipt_identity()
                    .to_string(),
                actual_projection_stage_identity: projection_receipt
                    .projection_stage_identity()
                    .to_string(),
            },
        );
    }
    if projection_receipt.upstream_surface_support_identity()
        != expected_projection_receipts.upstream_surface_support_identity()
    {
        return Err(
            OperandProjectionSupportError::UpstreamSurfaceSupportIdentityMismatch {
                expected_upstream_surface_support_identity: expected_projection_receipts
                    .upstream_surface_support_identity()
                    .to_string(),
                actual_upstream_surface_support_identity: projection_receipt
                    .upstream_surface_support_identity()
                    .to_string(),
            },
        );
    }
    if projection_receipt.certified_plane_support_identity()
        != expected_projection_receipts.certified_plane_support_identity()
    {
        return Err(
            OperandProjectionSupportError::CertifiedPlaneSupportIdentityMismatch {
                expected_certified_plane_support_identity: expected_projection_receipts
                    .certified_plane_support_identity()
                    .to_string(),
                actual_certified_plane_support_identity: projection_receipt
                    .certified_plane_support_identity()
                    .to_string(),
            },
        );
    }
    if projection_receipt.projection_local_basis_identity()
        != local_frame_selected_request
            .selection_receipt()
            .projection_local_basis_identity()
    {
        return Err(
            OperandProjectionSupportError::ProjectionLocalBasisIdentityMismatch {
                expected_projection_local_basis_identity: local_frame_selected_request
                    .selection_receipt()
                    .projection_local_basis_identity(),
                actual_projection_local_basis_identity: projection_receipt
                    .projection_local_basis_identity()
                    .to_string(),
            },
        );
    }
    if projection_receipt.projected_entity_count()
        != expected_projection_receipts
            .counters()
            .projected_topology_entities()
    {
        return Err(
            OperandProjectionSupportError::ProjectedEntityCountMismatch {
                expected_projected_entity_count: expected_projection_receipts
                    .counters()
                    .projected_topology_entities(),
                actual_projected_entity_count: projection_receipt.projected_entity_count(),
            },
        );
    }

    Ok(CertifiedOperandProjection {
        source_operand_workload_identity: role
            .source_operand(local_frame_selected_request)
            .workload()
            .response()
            .identity()
            .receipt_identity(),
        projected_workload: expected_projection,
        projection_receipt,
    })
}

fn selected_frame_projection(
    local_frame_selected_request: &PlanarBooleanCommonPlaneLocalFrameSelectedRequest,
    role: OperandProjectionRole,
) -> Result<ProjectedPlanarWorkload, OperandProjectionSupportError> {
    let operand = role.source_operand(local_frame_selected_request);

    ProjectionWorkload::for_certified_surface_support(operand.surface_support().clone())
        .declared(format!(
            "project {} operand through selected common-plane frame for {}",
            role.operand_label(),
            local_frame_selected_request.request_identity()
        ))
        .with_local_frame(LocalFrameBasis::from_common_plane_selection(
            local_frame_selected_request.selection_receipt(),
        ))
        .project()
        .map_err(
            |denial| OperandProjectionSupportError::OperandProjectionWorkloadDenied {
                kind: denial.reason_code(),
                human_reason: denial.human_reason().to_string(),
            },
        )
}
