use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt;

use super::error::PlanarBooleanCommonPlaneOperandBProjectionConsumptionError;
use super::identity::operand_b_projected_request_identity;
use crate::workload_composition::boolean_common_plane_reduction::operand_projection_consumption_support::{
    certify_projection_from_selected_frame, certify_projection_receipt, OperandProjectionRole,
};
use crate::workload_composition::{
    PlanarBooleanCommonPlaneLocalFrameSelectedRequest, WorkloadCatalogDeclarationReceipt,
    WorkloadCatalogSupportReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanCommonPlaneOperandBProjectedRequest {
    local_frame_selected_request: PlanarBooleanCommonPlaneLocalFrameSelectedRequest,
    projection_receipt: PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
    source_operand_workload_identity: String,
    operand_b_projection_identity: String,
}

impl PlanarBooleanCommonPlaneOperandBProjectedRequest {
    pub fn from_local_frame_selected_request(
        local_frame_selected_request: PlanarBooleanCommonPlaneLocalFrameSelectedRequest,
    ) -> Result<Self, PlanarBooleanCommonPlaneOperandBProjectionConsumptionError> {
        let (projection_receipt, source_operand_workload_identity) =
            certify_projection_from_selected_frame(
                &local_frame_selected_request,
                OperandProjectionRole::OperandB,
            )
            .map_err(PlanarBooleanCommonPlaneOperandBProjectionConsumptionError::from)?
            .into_parts();
        Self::from_certified_parts(
            local_frame_selected_request,
            projection_receipt,
            source_operand_workload_identity,
        )
    }

    pub fn from_parts(
        local_frame_selected_request: PlanarBooleanCommonPlaneLocalFrameSelectedRequest,
        projection_receipt: PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
    ) -> Result<Self, PlanarBooleanCommonPlaneOperandBProjectionConsumptionError> {
        let (projection_receipt, source_operand_workload_identity) = certify_projection_receipt(
            &local_frame_selected_request,
            OperandProjectionRole::OperandB,
            projection_receipt,
        )
        .map_err(PlanarBooleanCommonPlaneOperandBProjectionConsumptionError::from)?
        .into_parts();
        Self::from_certified_parts(
            local_frame_selected_request,
            projection_receipt,
            source_operand_workload_identity,
        )
    }

    pub fn declaration(&self) -> &WorkloadCatalogDeclarationReceipt {
        self.local_frame_selected_request.declaration()
    }

    pub fn support(&self) -> &WorkloadCatalogSupportReceipt {
        self.local_frame_selected_request.support()
    }

    pub fn operand_pair_identity(&self) -> &str {
        self.local_frame_selected_request.operand_pair_identity()
    }

    pub fn request_identity(&self) -> &str {
        self.local_frame_selected_request.request_identity()
    }

    pub fn scope_admission_identity(&self) -> &str {
        self.local_frame_selected_request.scope_admission_identity()
    }

    pub fn plane_agreement_identity(&self) -> &str {
        self.local_frame_selected_request.plane_agreement_identity()
    }

    pub fn posture_agreement_identity(&self) -> &str {
        self.local_frame_selected_request
            .posture_agreement_identity()
    }

    pub fn precision_agreement_identity(&self) -> &str {
        self.local_frame_selected_request
            .precision_agreement_identity()
    }

    pub fn shared_plane_identity(&self) -> &str {
        self.local_frame_selected_request.shared_plane_identity()
    }

    pub fn shared_plane_receipt_identity(&self) -> &str {
        self.local_frame_selected_request
            .shared_plane_receipt_identity()
    }

    pub fn local_frame_selection_identity(&self) -> &str {
        self.local_frame_selected_request
            .local_frame_selection_identity()
    }

    pub fn operand_b_projection_identity(&self) -> &str {
        &self.operand_b_projection_identity
    }

    pub fn source_operand_workload_identity(&self) -> &str {
        &self.source_operand_workload_identity
    }

    pub fn projection_receipt(
        &self,
    ) -> &PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt {
        &self.projection_receipt
    }

    pub fn local_frame_selected_request(
        &self,
    ) -> &PlanarBooleanCommonPlaneLocalFrameSelectedRequest {
        &self.local_frame_selected_request
    }
}

impl PlanarBooleanCommonPlaneOperandBProjectedRequest {
    fn from_certified_parts(
        local_frame_selected_request: PlanarBooleanCommonPlaneLocalFrameSelectedRequest,
        projection_receipt: PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
        source_operand_workload_identity: String,
    ) -> Result<Self, PlanarBooleanCommonPlaneOperandBProjectionConsumptionError> {
        let request = Self {
            source_operand_workload_identity,
            local_frame_selected_request,
            projection_receipt,
            operand_b_projection_identity: String::new(),
        };
        let operand_b_projection_identity = operand_b_projected_request_identity(&request);
        Ok(Self {
            operand_b_projection_identity,
            ..request
        })
    }
}
