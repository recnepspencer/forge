use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanEdgeSplitRequest;
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanEventLedgerReceipt, PlanarBooleanSegmentPairEnumerationReceipt,
};

use super::{
    require_boolean_evidence, WorkloadCompositionError, WorkloadStageRequirement, WorthWorkload,
};
use crate::workload_composition::{
    PlanarBooleanBlockerEvidenceReceipt, PlanarBooleanCommonPlaneLocalFrameSelectedRequest,
    PlanarBooleanCommonPlaneOperandAProjectedRequest,
    PlanarBooleanCommonPlaneOperandBProjectedRequest,
    PlanarBooleanCommonPlanePrecisionAgreedRequest,
    PlanarBooleanCommonPlaneReducedOperandPairRequest,
    PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest, PlanarBooleanDeclarationReceipt,
    PlanarBooleanEventExtractionRequest, PlanarBooleanOperandPairConstructionReceipt,
    PlanarBooleanSupportReceipt,
};

impl WorthWorkload {
    pub fn require_boolean_declaration_entry(
        &self,
        declaration: &PlanarBooleanDeclarationReceipt,
    ) -> Result<(), WorkloadCompositionError> {
        require_boolean_evidence(
            &self.evidence_ledger,
            declaration,
            WorkloadStageRequirement::BooleanDeclarationEntry,
        )
    }

    pub fn require_boolean_route_plan(
        &self,
        route_plan: &PlanarBooleanSupportReceipt,
    ) -> Result<(), WorkloadCompositionError> {
        require_boolean_evidence(
            &self.evidence_ledger,
            route_plan,
            WorkloadStageRequirement::BooleanRoutePlan,
        )
    }

    pub fn require_boolean_operand_pair_construction(
        &self,
        construction: &PlanarBooleanOperandPairConstructionReceipt,
    ) -> Result<(), WorkloadCompositionError> {
        require_boolean_evidence(
            &self.evidence_ledger,
            construction,
            WorkloadStageRequirement::BooleanOperandPairConstruction,
        )
    }

    pub fn require_boolean_blocker_provenance(
        &self,
        blocker: &PlanarBooleanBlockerEvidenceReceipt,
    ) -> Result<(), WorkloadCompositionError> {
        require_boolean_evidence(
            &self.evidence_ledger,
            blocker,
            WorkloadStageRequirement::BooleanBlockerProvenance,
        )
    }

    pub fn require_boolean_shared_plane_identity(
        &self,
        shared_plane_identity: &PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest,
    ) -> Result<(), WorkloadCompositionError> {
        require_boolean_evidence(
            &self.evidence_ledger,
            shared_plane_identity,
            WorkloadStageRequirement::BooleanSharedPlaneIdentity,
        )
    }

    pub fn require_boolean_precision_agreement(
        &self,
        precision_agreement: &PlanarBooleanCommonPlanePrecisionAgreedRequest,
    ) -> Result<(), WorkloadCompositionError> {
        require_boolean_evidence(
            &self.evidence_ledger,
            precision_agreement,
            WorkloadStageRequirement::BooleanPrecisionAgreement,
        )
    }

    pub fn require_boolean_local_frame_selection(
        &self,
        local_frame_selection: &PlanarBooleanCommonPlaneLocalFrameSelectedRequest,
    ) -> Result<(), WorkloadCompositionError> {
        require_boolean_evidence(
            &self.evidence_ledger,
            local_frame_selection,
            WorkloadStageRequirement::BooleanLocalFrameSelection,
        )
    }

    pub fn require_boolean_operand_a_projection_consumption(
        &self,
        operand_a_projection: &PlanarBooleanCommonPlaneOperandAProjectedRequest,
    ) -> Result<(), WorkloadCompositionError> {
        require_boolean_evidence(
            &self.evidence_ledger,
            operand_a_projection,
            WorkloadStageRequirement::BooleanOperandAProjectionConsumption,
        )
    }

    pub fn require_boolean_operand_b_projection_consumption(
        &self,
        operand_b_projection: &PlanarBooleanCommonPlaneOperandBProjectedRequest,
    ) -> Result<(), WorkloadCompositionError> {
        require_boolean_evidence(
            &self.evidence_ledger,
            operand_b_projection,
            WorkloadStageRequirement::BooleanOperandBProjectionConsumption,
        )
    }

    pub fn require_boolean_reduced_operand_pair(
        &self,
        reduced_operand_pair: &PlanarBooleanCommonPlaneReducedOperandPairRequest,
    ) -> Result<(), WorkloadCompositionError> {
        require_boolean_evidence(
            &self.evidence_ledger,
            reduced_operand_pair,
            WorkloadStageRequirement::BooleanReducedOperandPair,
        )
    }

    pub fn require_boolean_event_extraction_request(
        &self,
        event_extraction_request: &PlanarBooleanEventExtractionRequest,
    ) -> Result<(), WorkloadCompositionError> {
        require_boolean_evidence(
            &self.evidence_ledger,
            event_extraction_request,
            WorkloadStageRequirement::BooleanEventExtractionRequest,
        )
    }

    pub fn require_boolean_segment_pair_enumeration(
        &self,
        segment_pair_enumeration: &PlanarBooleanSegmentPairEnumerationReceipt,
    ) -> Result<(), WorkloadCompositionError> {
        require_boolean_evidence(
            &self.evidence_ledger,
            segment_pair_enumeration,
            WorkloadStageRequirement::BooleanSegmentPairEnumeration,
        )
    }

    pub fn require_boolean_event_ledger(
        &self,
        event_ledger: &PlanarBooleanEventLedgerReceipt,
    ) -> Result<(), WorkloadCompositionError> {
        require_boolean_evidence(
            &self.evidence_ledger,
            event_ledger,
            WorkloadStageRequirement::BooleanEventLedger,
        )
    }

    pub fn require_boolean_split(
        &self,
        split_request: &PlanarBooleanEdgeSplitRequest,
    ) -> Result<(), WorkloadCompositionError> {
        require_boolean_evidence(
            &self.evidence_ledger,
            split_request,
            WorkloadStageRequirement::BooleanSplit,
        )
    }
}
