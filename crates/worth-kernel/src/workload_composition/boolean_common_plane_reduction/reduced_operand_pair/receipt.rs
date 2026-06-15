use worth_spatial::facade::planar_boolean_common_plane::{
    PlanarBooleanCommonPlaneReducedOperandPairOrderingContract,
    PlanarBooleanCommonPlaneReducedOperandPairReceipt,
};

use super::error::PlanarBooleanCommonPlaneReducedOperandPairAssemblyError;
use super::identity::reduced_operand_pair_request_identity;
use crate::workload_composition::{
    PlanarBooleanCommonPlaneOperandAProjectedRequest,
    PlanarBooleanCommonPlaneOperandBProjectedRequest, WorkloadCatalogDeclarationReceipt,
    WorkloadCatalogSupportReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanCommonPlaneReducedOperandPairRequest {
    operand_a_projected_request: PlanarBooleanCommonPlaneOperandAProjectedRequest,
    operand_b_projected_request: PlanarBooleanCommonPlaneOperandBProjectedRequest,
    reduced_pair_receipt: PlanarBooleanCommonPlaneReducedOperandPairReceipt,
    source_left_operand_workload_identity: String,
    source_right_operand_workload_identity: String,
    reduced_operand_pair_request_identity: String,
}

impl PlanarBooleanCommonPlaneReducedOperandPairRequest {
    pub fn from_operand_projection_requests(
        operand_a_projected_request: PlanarBooleanCommonPlaneOperandAProjectedRequest,
        operand_b_projected_request: PlanarBooleanCommonPlaneOperandBProjectedRequest,
    ) -> Result<Self, PlanarBooleanCommonPlaneReducedOperandPairAssemblyError> {
        let reduced_pair_receipt =
            PlanarBooleanCommonPlaneReducedOperandPairReceipt::from_operand_projection_receipts(
                operand_a_projected_request.projection_receipt(),
                operand_b_projected_request.projection_receipt(),
            )
            .map_err(spatial_denial_to_error)?;
        Ok(Self::from_certified_parts(
            operand_a_projected_request,
            operand_b_projected_request,
            reduced_pair_receipt,
        ))
    }

    pub fn from_parts(
        operand_a_projected_request: PlanarBooleanCommonPlaneOperandAProjectedRequest,
        operand_b_projected_request: PlanarBooleanCommonPlaneOperandBProjectedRequest,
        reduced_pair_receipt: PlanarBooleanCommonPlaneReducedOperandPairReceipt,
    ) -> Result<Self, PlanarBooleanCommonPlaneReducedOperandPairAssemblyError> {
        if operand_a_projected_request
            .projection_receipt()
            .operand_projection_consumption_identity()
            != reduced_pair_receipt.left_projection_identity()
        {
            return Err(
                PlanarBooleanCommonPlaneReducedOperandPairAssemblyError::LeftOperandProjectionIdentityMismatch {
                    expected_left_projection_identity: operand_a_projected_request
                        .projection_receipt()
                        .operand_projection_consumption_identity()
                        .to_string(),
                    actual_left_projection_identity: reduced_pair_receipt
                        .left_projection_identity()
                        .to_string(),
                },
            );
        }
        if operand_b_projected_request
            .projection_receipt()
            .operand_projection_consumption_identity()
            != reduced_pair_receipt.right_projection_identity()
        {
            return Err(
                PlanarBooleanCommonPlaneReducedOperandPairAssemblyError::RightOperandProjectionIdentityMismatch {
                    expected_right_projection_identity: operand_b_projected_request
                        .projection_receipt()
                        .operand_projection_consumption_identity()
                        .to_string(),
                    actual_right_projection_identity: reduced_pair_receipt
                        .right_projection_identity()
                        .to_string(),
                },
            );
        }
        if operand_a_projected_request
            .projection_receipt()
            .shared_plane_receipt_identity()
            != reduced_pair_receipt.shared_plane_receipt_identity()
        {
            return Err(
                PlanarBooleanCommonPlaneReducedOperandPairAssemblyError::SharedPlaneReceiptIdentityMismatch {
                    expected_shared_plane_receipt_identity: operand_a_projected_request
                        .projection_receipt()
                        .shared_plane_receipt_identity()
                        .to_string(),
                    actual_shared_plane_receipt_identity: reduced_pair_receipt
                        .shared_plane_receipt_identity()
                        .to_string(),
                },
            );
        }
        if operand_a_projected_request
            .projection_receipt()
            .shared_plane_identity()
            != reduced_pair_receipt.shared_plane_identity()
        {
            return Err(
                PlanarBooleanCommonPlaneReducedOperandPairAssemblyError::SharedPlaneIdentityMismatch {
                    expected_shared_plane_identity: operand_a_projected_request
                        .projection_receipt()
                        .shared_plane_identity()
                        .to_string(),
                    actual_shared_plane_identity: reduced_pair_receipt
                        .shared_plane_identity()
                        .to_string(),
                },
            );
        }
        if operand_a_projected_request
            .projection_receipt()
            .plane_agreement_identity()
            != reduced_pair_receipt.plane_agreement_identity()
        {
            return Err(
                PlanarBooleanCommonPlaneReducedOperandPairAssemblyError::PlaneAgreementIdentityMismatch {
                    expected_plane_agreement_identity: operand_a_projected_request
                        .projection_receipt()
                        .plane_agreement_identity()
                        .to_string(),
                    actual_plane_agreement_identity: reduced_pair_receipt
                        .plane_agreement_identity()
                        .to_string(),
                },
            );
        }
        if operand_a_projected_request
            .projection_receipt()
            .local_frame_selection_identity()
            != reduced_pair_receipt.local_frame_selection_identity()
        {
            return Err(
                PlanarBooleanCommonPlaneReducedOperandPairAssemblyError::LocalFrameSelectionIdentityMismatch {
                    expected_local_frame_selection_identity: operand_a_projected_request
                        .projection_receipt()
                        .local_frame_selection_identity()
                        .to_string(),
                    actual_local_frame_selection_identity: reduced_pair_receipt
                        .local_frame_selection_identity()
                        .to_string(),
                },
            );
        }
        if operand_a_projected_request
            .projection_receipt()
            .projection_local_basis_identity()
            != reduced_pair_receipt.projection_local_basis_identity()
        {
            return Err(
                PlanarBooleanCommonPlaneReducedOperandPairAssemblyError::ProjectionLocalBasisIdentityMismatch {
                    expected_projection_local_basis_identity: operand_a_projected_request
                        .projection_receipt()
                        .projection_local_basis_identity()
                        .to_string(),
                    actual_projection_local_basis_identity: reduced_pair_receipt
                        .projection_local_basis_identity()
                        .to_string(),
                },
            );
        }
        if operand_a_projected_request
            .projection_receipt()
            .projection_stage_identity()
            != reduced_pair_receipt.left_projection_stage_identity()
        {
            return Err(
                PlanarBooleanCommonPlaneReducedOperandPairAssemblyError::LeftProjectionStageIdentityMismatch {
                    expected_left_projection_stage_identity: operand_a_projected_request
                        .projection_receipt()
                        .projection_stage_identity()
                        .to_string(),
                    actual_left_projection_stage_identity: reduced_pair_receipt
                        .left_projection_stage_identity()
                        .to_string(),
                },
            );
        }
        if operand_b_projected_request
            .projection_receipt()
            .projection_stage_identity()
            != reduced_pair_receipt.right_projection_stage_identity()
        {
            return Err(
                PlanarBooleanCommonPlaneReducedOperandPairAssemblyError::RightProjectionStageIdentityMismatch {
                    expected_right_projection_stage_identity: operand_b_projected_request
                        .projection_receipt()
                        .projection_stage_identity()
                        .to_string(),
                    actual_right_projection_stage_identity: reduced_pair_receipt
                        .right_projection_stage_identity()
                        .to_string(),
                },
            );
        }
        if reduced_pair_receipt.ordering_contract()
            != PlanarBooleanCommonPlaneReducedOperandPairOrderingContract::semantic_left_right()
        {
            return Err(
                PlanarBooleanCommonPlaneReducedOperandPairAssemblyError::OrderingContractMismatch {
                    expected_first_slot_side:
                        PlanarBooleanCommonPlaneReducedOperandPairOrderingContract::semantic_left_right()
                            .first_slot_side(),
                    actual_first_slot_side: reduced_pair_receipt
                        .ordering_contract()
                        .first_slot_side(),
                },
            );
        }
        Ok(Self::from_certified_parts(
            operand_a_projected_request,
            operand_b_projected_request,
            reduced_pair_receipt,
        ))
    }

    fn from_certified_parts(
        operand_a_projected_request: PlanarBooleanCommonPlaneOperandAProjectedRequest,
        operand_b_projected_request: PlanarBooleanCommonPlaneOperandBProjectedRequest,
        reduced_pair_receipt: PlanarBooleanCommonPlaneReducedOperandPairReceipt,
    ) -> Self {
        let request = Self {
            source_left_operand_workload_identity: operand_a_projected_request
                .source_operand_workload_identity()
                .to_string(),
            source_right_operand_workload_identity: operand_b_projected_request
                .source_operand_workload_identity()
                .to_string(),
            operand_a_projected_request,
            operand_b_projected_request,
            reduced_pair_receipt,
            reduced_operand_pair_request_identity: String::new(),
        };
        Self {
            reduced_operand_pair_request_identity: reduced_operand_pair_request_identity(&request),
            ..request
        }
    }

    pub fn declaration(&self) -> &WorkloadCatalogDeclarationReceipt {
        self.operand_a_projected_request.declaration()
    }

    pub fn support(&self) -> &WorkloadCatalogSupportReceipt {
        self.operand_a_projected_request.support()
    }

    pub fn operand_pair_identity(&self) -> &str {
        self.operand_a_projected_request.operand_pair_identity()
    }

    pub fn request_identity(&self) -> &str {
        self.operand_a_projected_request.request_identity()
    }

    pub fn shared_plane_identity(&self) -> &str {
        self.reduced_pair_receipt.shared_plane_identity()
    }

    pub fn shared_plane_receipt_identity(&self) -> &str {
        self.reduced_pair_receipt.shared_plane_receipt_identity()
    }

    pub fn plane_agreement_identity(&self) -> &str {
        self.reduced_pair_receipt.plane_agreement_identity()
    }

    pub fn local_frame_selection_identity(&self) -> &str {
        self.reduced_pair_receipt.local_frame_selection_identity()
    }

    pub fn left_projection_stage_identity(&self) -> &str {
        self.reduced_pair_receipt.left_projection_stage_identity()
    }

    pub fn right_projection_stage_identity(&self) -> &str {
        self.reduced_pair_receipt.right_projection_stage_identity()
    }

    pub fn reduced_operand_pair_identity(&self) -> &str {
        self.reduced_pair_receipt.reduced_operand_pair_identity()
    }

    pub fn ordering_contract(&self) -> PlanarBooleanCommonPlaneReducedOperandPairOrderingContract {
        self.reduced_pair_receipt.ordering_contract()
    }

    pub fn source_left_operand_workload_identity(&self) -> &str {
        &self.source_left_operand_workload_identity
    }

    pub fn source_right_operand_workload_identity(&self) -> &str {
        &self.source_right_operand_workload_identity
    }

    pub fn reduced_operand_pair_request_identity(&self) -> &str {
        &self.reduced_operand_pair_request_identity
    }

    pub fn reduced_pair_receipt(&self) -> &PlanarBooleanCommonPlaneReducedOperandPairReceipt {
        &self.reduced_pair_receipt
    }
}

fn spatial_denial_to_error(
    denial: worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneReducedOperandPairDenial,
) -> PlanarBooleanCommonPlaneReducedOperandPairAssemblyError {
    PlanarBooleanCommonPlaneReducedOperandPairAssemblyError::SpatialReducedOperandPairDenied {
        kind: denial.kind(),
        human_reason: denial.human_reason(),
    }
}
