use crate::workload_platform::planar_boolean_common_plane::{
    PlanarBooleanCommonPlaneLocalFrameSelectionReceipt, PlanarBooleanCommonPlaneOperandSide,
};
use crate::workload_platform::projection_workload::ProjectionReceiptSet;

use super::denial::PlanarBooleanCommonPlaneOperandProjectionConsumptionDenial;
use super::identity::operand_projection_consumption_identity;
use super::validation::validate_operand_projection_consumption;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt {
    operand_side: PlanarBooleanCommonPlaneOperandSide,
    local_frame_selection_identity: String,
    shared_plane_receipt_identity: String,
    shared_plane_identity: String,
    plane_agreement_identity: String,
    projection_stage_identity: String,
    upstream_surface_support_identity: String,
    certified_plane_support_identity: String,
    projection_local_basis_identity: String,
    projected_entity_count: usize,
    operand_projection_consumption_identity: String,
}

impl PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt {
    pub fn from_local_frame_selection_and_projection_receipts(
        selection: &PlanarBooleanCommonPlaneLocalFrameSelectionReceipt,
        receipts: &ProjectionReceiptSet,
        operand_side: PlanarBooleanCommonPlaneOperandSide,
    ) -> Result<Self, PlanarBooleanCommonPlaneOperandProjectionConsumptionDenial> {
        let receipt = Self {
            operand_side,
            local_frame_selection_identity: selection
                .local_frame_selection_receipt_identity()
                .to_string(),
            shared_plane_receipt_identity: selection.shared_plane_receipt_identity().to_string(),
            shared_plane_identity: selection.shared_plane_identity().to_string(),
            plane_agreement_identity: selection.plane_agreement_identity().to_string(),
            projection_stage_identity: receipts.stage_identity().receipt_identity().to_string(),
            upstream_surface_support_identity: receipts
                .upstream_surface_support_identity()
                .to_string(),
            certified_plane_support_identity: receipts
                .certified_plane_support_identity()
                .to_string(),
            projection_local_basis_identity: receipts
                .local_frame_receipt()
                .local_basis_identity()
                .to_string(),
            projected_entity_count: receipts.counters().projected_topology_entities(),
            operand_projection_consumption_identity: String::new(),
        };
        if receipt.projection_local_basis_identity() != selection.projection_local_basis_identity()
        {
            return Err(PlanarBooleanCommonPlaneOperandProjectionConsumptionDenial::new(
                super::denial::PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind::ProjectionLocalBasisSelectionMismatch,
                "operand projection consumption must use the projection basis derived from the selected common-plane frame",
            ));
        }
        validate_operand_projection_consumption(&receipt)?;
        Ok(Self {
            operand_projection_consumption_identity: operand_projection_consumption_identity(
                &receipt,
            ),
            ..receipt
        })
    }

    pub fn operand_side(&self) -> PlanarBooleanCommonPlaneOperandSide {
        self.operand_side
    }

    pub fn local_frame_selection_identity(&self) -> &str {
        &self.local_frame_selection_identity
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

    pub fn projection_stage_identity(&self) -> &str {
        &self.projection_stage_identity
    }

    pub fn upstream_surface_support_identity(&self) -> &str {
        &self.upstream_surface_support_identity
    }

    pub fn certified_plane_support_identity(&self) -> &str {
        &self.certified_plane_support_identity
    }

    pub fn projection_local_basis_identity(&self) -> &str {
        &self.projection_local_basis_identity
    }

    pub fn projected_entity_count(&self) -> usize {
        self.projected_entity_count
    }

    pub fn operand_projection_consumption_identity(&self) -> &str {
        &self.operand_projection_consumption_identity
    }
}
