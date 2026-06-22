use crate::workload_composition::{
    PlanarBooleanCommonPlaneReducedOperandPairRequest, WorkloadCatalogDeclarationReceipt,
    WorkloadCatalogSupportReceipt,
};

use super::identity::event_extraction_request_identity;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanEventExtractionRequest {
    reduced_operand_pair_request: PlanarBooleanCommonPlaneReducedOperandPairRequest,
    event_extraction_request_identity: String,
}

impl PlanarBooleanEventExtractionRequest {
    pub fn from_reduced_operand_pair(
        reduced_operand_pair_request: PlanarBooleanCommonPlaneReducedOperandPairRequest,
    ) -> Self {
        let request = Self {
            reduced_operand_pair_request,
            event_extraction_request_identity: String::new(),
        };
        Self {
            event_extraction_request_identity: event_extraction_request_identity(&request),
            ..request
        }
    }

    pub fn declaration(&self) -> &WorkloadCatalogDeclarationReceipt {
        self.reduced_operand_pair_request.declaration()
    }

    pub fn support(&self) -> &WorkloadCatalogSupportReceipt {
        self.reduced_operand_pair_request.support()
    }

    pub fn operand_pair_identity(&self) -> &str {
        self.reduced_operand_pair_request.operand_pair_identity()
    }

    pub fn common_plane_reduction_request_identity(&self) -> &str {
        self.reduced_operand_pair_request.request_identity()
    }

    pub fn shared_plane_identity(&self) -> &str {
        self.reduced_operand_pair_request.shared_plane_identity()
    }

    pub fn shared_plane_receipt_identity(&self) -> &str {
        self.reduced_operand_pair_request
            .shared_plane_receipt_identity()
    }

    pub fn plane_agreement_identity(&self) -> &str {
        self.reduced_operand_pair_request.plane_agreement_identity()
    }

    pub fn precision_agreement_identity(&self) -> &str {
        self.reduced_operand_pair_request
            .precision_agreement_identity()
    }

    pub fn local_frame_selection_identity(&self) -> &str {
        self.reduced_operand_pair_request
            .local_frame_selection_identity()
    }

    pub fn left_projection_identity(&self) -> &str {
        self.reduced_operand_pair_request.left_projection_identity()
    }

    pub fn right_projection_identity(&self) -> &str {
        self.reduced_operand_pair_request
            .right_projection_identity()
    }

    pub fn left_projection_stage_identity(&self) -> &str {
        self.reduced_operand_pair_request
            .left_projection_stage_identity()
    }

    pub fn right_projection_stage_identity(&self) -> &str {
        self.reduced_operand_pair_request
            .right_projection_stage_identity()
    }

    pub fn reduced_operand_pair_identity(&self) -> &str {
        self.reduced_operand_pair_request
            .reduced_operand_pair_identity()
    }

    pub fn source_left_operand_workload_identity(&self) -> &str {
        self.reduced_operand_pair_request
            .source_left_operand_workload_identity()
    }

    pub fn source_right_operand_workload_identity(&self) -> &str {
        self.reduced_operand_pair_request
            .source_right_operand_workload_identity()
    }

    pub fn reduced_operand_pair_request_identity(&self) -> &str {
        self.reduced_operand_pair_request
            .reduced_operand_pair_request_identity()
    }

    pub fn event_extraction_request_identity(&self) -> &str {
        &self.event_extraction_request_identity
    }

    pub fn reduced_operand_pair_request(
        &self,
    ) -> &PlanarBooleanCommonPlaneReducedOperandPairRequest {
        &self.reduced_operand_pair_request
    }
}
