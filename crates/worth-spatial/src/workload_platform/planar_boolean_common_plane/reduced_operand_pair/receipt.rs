use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt;

use super::denial::PlanarBooleanCommonPlaneReducedOperandPairDenial;
use super::identity::reduced_operand_pair_identity;
use super::ordering::PlanarBooleanCommonPlaneReducedOperandPairOrderingContract;
use super::validation::validate_reduced_operand_pair;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanCommonPlaneReducedOperandPairReceipt {
    left_projection_identity: String,
    right_projection_identity: String,
    shared_plane_receipt_identity: String,
    shared_plane_identity: String,
    plane_agreement_identity: String,
    local_frame_selection_identity: String,
    projection_local_basis_identity: String,
    left_projection_stage_identity: String,
    right_projection_stage_identity: String,
    ordering_contract: PlanarBooleanCommonPlaneReducedOperandPairOrderingContract,
    reduced_operand_pair_identity: String,
}

impl PlanarBooleanCommonPlaneReducedOperandPairReceipt {
    pub fn from_operand_projection_receipts(
        left: &PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
        right: &PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
    ) -> Result<Self, PlanarBooleanCommonPlaneReducedOperandPairDenial> {
        validate_reduced_operand_pair(left, right)?;
        let receipt = Self {
            left_projection_identity: left.operand_projection_consumption_identity().to_string(),
            right_projection_identity: right.operand_projection_consumption_identity().to_string(),
            shared_plane_receipt_identity: left.shared_plane_receipt_identity().to_string(),
            shared_plane_identity: left.shared_plane_identity().to_string(),
            plane_agreement_identity: left.plane_agreement_identity().to_string(),
            local_frame_selection_identity: left.local_frame_selection_identity().to_string(),
            projection_local_basis_identity: left.projection_local_basis_identity().to_string(),
            left_projection_stage_identity: left.projection_stage_identity().to_string(),
            right_projection_stage_identity: right.projection_stage_identity().to_string(),
            ordering_contract:
                PlanarBooleanCommonPlaneReducedOperandPairOrderingContract::semantic_left_right(),
            reduced_operand_pair_identity: String::new(),
        };
        Ok(Self {
            reduced_operand_pair_identity: reduced_operand_pair_identity(&receipt),
            ..receipt
        })
    }

    pub fn left_projection_identity(&self) -> &str {
        &self.left_projection_identity
    }

    pub fn right_projection_identity(&self) -> &str {
        &self.right_projection_identity
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

    pub fn local_frame_selection_identity(&self) -> &str {
        &self.local_frame_selection_identity
    }

    pub fn projection_local_basis_identity(&self) -> &str {
        &self.projection_local_basis_identity
    }

    pub fn left_projection_stage_identity(&self) -> &str {
        &self.left_projection_stage_identity
    }

    pub fn right_projection_stage_identity(&self) -> &str {
        &self.right_projection_stage_identity
    }

    pub fn ordering_contract(&self) -> PlanarBooleanCommonPlaneReducedOperandPairOrderingContract {
        self.ordering_contract
    }

    pub fn reduced_operand_pair_identity(&self) -> &str {
        &self.reduced_operand_pair_identity
    }
}
