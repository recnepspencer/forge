use topology::facade::{
    PlanarBooleanLoopOperatorClassificationMatrix, PlanarBooleanLoopValidatorRegistrationPlan,
};
use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeChainLedgerReceipt;
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanEventLedgerReceipt, PlanarBooleanSegmentPairEnumerationReceipt,
};
use worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanLoopReconstructionLedgerReceipt;
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceBooleanReceiptLookupProduct, WorkloadEvidenceStage,
};

use super::{
    require_boolean_evidence, require_evidence_stage, CompletedBooleanLoopReconstructionHandoff,
    CompletedBooleanSplitHandoff, PlanarBooleanLoopRuntimeRegistrationProof,
    WorkloadCompositionError, WorkloadStageRequirement, WorthWorkload, WorthWorkloadParts,
};
use crate::workload_composition::boolean_evidence_requirement::map_boolean_ledger_error;
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
        require_evidence_stage(
            &self.evidence_ledger,
            WorkloadEvidenceStage::BooleanDeclarationEntry,
            declaration.query_declaration_digest(),
        )
    }

    pub fn require_boolean_route_plan(
        &self,
        route_plan: &PlanarBooleanSupportReceipt,
    ) -> Result<(), WorkloadCompositionError> {
        require_evidence_stage(
            &self.evidence_ledger,
            WorkloadEvidenceStage::BooleanRoutePlan,
            route_plan.query_support_digest(),
        )
    }

    pub fn require_boolean_operand_pair_construction(
        &self,
        construction: &PlanarBooleanOperandPairConstructionReceipt,
    ) -> Result<(), WorkloadCompositionError> {
        require_evidence_stage(
            &self.evidence_ledger,
            WorkloadEvidenceStage::BooleanOperandPairConstruction,
            construction.construction_digest(),
        )
    }

    pub fn require_boolean_blocker_provenance(
        &self,
        blocker: &PlanarBooleanBlockerEvidenceReceipt,
    ) -> Result<(), WorkloadCompositionError> {
        require_evidence_stage(
            &self.evidence_ledger,
            WorkloadEvidenceStage::BooleanBlockerProvenance,
            blocker.blocker_digest(),
        )
    }

    pub fn require_boolean_shared_plane_identity(
        &self,
        shared_plane_identity: &PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest,
    ) -> Result<(), WorkloadCompositionError> {
        require_evidence_stage(
            &self.evidence_ledger,
            WorkloadEvidenceStage::BooleanSharedPlaneIdentity,
            shared_plane_identity.shared_plane_identified_request_identity(),
        )
    }

    pub fn require_boolean_precision_agreement(
        &self,
        precision_agreement: &PlanarBooleanCommonPlanePrecisionAgreedRequest,
    ) -> Result<(), WorkloadCompositionError> {
        require_evidence_stage(
            &self.evidence_ledger,
            WorkloadEvidenceStage::BooleanPrecisionAgreement,
            precision_agreement.precision_agreement_identity(),
        )
    }

    pub fn require_boolean_local_frame_selection(
        &self,
        local_frame_selection: &PlanarBooleanCommonPlaneLocalFrameSelectedRequest,
    ) -> Result<(), WorkloadCompositionError> {
        require_evidence_stage(
            &self.evidence_ledger,
            WorkloadEvidenceStage::BooleanLocalFrameSelection,
            local_frame_selection.local_frame_selection_identity(),
        )
    }

    pub fn require_boolean_operand_a_projection_consumption(
        &self,
        operand_a_projection: &PlanarBooleanCommonPlaneOperandAProjectedRequest,
    ) -> Result<(), WorkloadCompositionError> {
        require_evidence_stage(
            &self.evidence_ledger,
            WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
            operand_a_projection.operand_a_projection_identity(),
        )
    }

    pub fn require_boolean_operand_b_projection_consumption(
        &self,
        operand_b_projection: &PlanarBooleanCommonPlaneOperandBProjectedRequest,
    ) -> Result<(), WorkloadCompositionError> {
        require_evidence_stage(
            &self.evidence_ledger,
            WorkloadEvidenceStage::BooleanOperandBProjectionConsumption,
            operand_b_projection.operand_b_projection_identity(),
        )
    }

    pub fn require_boolean_reduced_operand_pair(
        &self,
        reduced_operand_pair: &PlanarBooleanCommonPlaneReducedOperandPairRequest,
    ) -> Result<(), WorkloadCompositionError> {
        require_evidence_stage(
            &self.evidence_ledger,
            WorkloadEvidenceStage::BooleanReducedOperandPair,
            reduced_operand_pair.reduced_operand_pair_request_identity(),
        )
    }

    pub fn require_boolean_event_extraction_request(
        &self,
        event_extraction_request: &PlanarBooleanEventExtractionRequest,
    ) -> Result<(), WorkloadCompositionError> {
        require_evidence_stage(
            &self.evidence_ledger,
            WorkloadEvidenceStage::BooleanEventExtractionRequest,
            event_extraction_request.event_extraction_request_identity(),
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
        split_ledger: &PlanarBooleanSplitEdgeChainLedgerReceipt,
    ) -> Result<(), WorkloadCompositionError> {
        self.require_boolean_split_lookup(split_ledger).map(|_| ())
    }

    pub fn require_boolean_split_lookup(
        &self,
        split_ledger: &PlanarBooleanSplitEdgeChainLedgerReceipt,
    ) -> Result<WorkloadEvidenceBooleanReceiptLookupProduct, WorkloadCompositionError> {
        self.evidence_ledger
            .require_boolean_receipt_lookup(split_ledger)
            .map_err(|error| {
                map_boolean_ledger_error(error, WorkloadStageRequirement::BooleanSplit)
            })
    }

    pub fn require_boolean_loop_reconstruction(
        &self,
        loop_ledger: &PlanarBooleanLoopReconstructionLedgerReceipt,
    ) -> Result<(), WorkloadCompositionError> {
        self.require_boolean_loop_reconstruction_lookup(loop_ledger)
            .map(|_| ())
    }

    pub fn require_boolean_loop_reconstruction_lookup(
        &self,
        loop_ledger: &PlanarBooleanLoopReconstructionLedgerReceipt,
    ) -> Result<WorkloadEvidenceBooleanReceiptLookupProduct, WorkloadCompositionError> {
        self.evidence_ledger
            .require_boolean_receipt_lookup(loop_ledger)
            .map_err(|error| {
                map_boolean_ledger_error(error, WorkloadStageRequirement::BooleanLoopReconstruction)
            })
    }

    pub fn with_completed_boolean_split_ledger(
        &self,
        split_ledger: &PlanarBooleanSplitEdgeChainLedgerReceipt,
    ) -> Result<Self, WorkloadCompositionError> {
        let evidence_ledger = self
            .evidence_ledger
            .with_boolean_evidence_receipt(split_ledger)
            .map_err(|error| {
                map_boolean_ledger_error(error, WorkloadStageRequirement::BooleanSplit)
            })?;

        Self::compose(WorthWorkloadParts {
            topology: self.topology.clone(),
            geometry_binding: self.geometry_binding.clone(),
            surface_support: self.surface_support.clone(),
            projection: self.projection.clone(),
            transform: self.transform.clone(),
            retained_replay: self.retained_replay.clone(),
            diagnostics: self.diagnostics.clone(),
            response: self.response.clone(),
            evidence_ledger,
        })
    }

    pub fn complete_boolean_split_handoff(
        &self,
        split_ledger: &PlanarBooleanSplitEdgeChainLedgerReceipt,
    ) -> Result<CompletedBooleanSplitHandoff, WorkloadCompositionError> {
        let completed_workload = self.with_completed_boolean_split_ledger(split_ledger)?;
        Ok(CompletedBooleanSplitHandoff::new(
            completed_workload,
            split_ledger.clone(),
        ))
    }

    pub fn with_completed_boolean_loop_reconstruction(
        &self,
        loop_ledger: &PlanarBooleanLoopReconstructionLedgerReceipt,
        evidence_receipt: &worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanLoopReconstructionEvidenceReceipt,
        operator_matrix: &PlanarBooleanLoopOperatorClassificationMatrix,
        validator_plan: &PlanarBooleanLoopValidatorRegistrationPlan,
    ) -> Result<CompletedBooleanLoopReconstructionHandoff, WorkloadCompositionError> {
        let evidence_ledger = self
            .evidence_ledger
            .with_boolean_evidence_receipt(loop_ledger)
            .map_err(|error| {
                map_boolean_ledger_error(error, WorkloadStageRequirement::BooleanLoopReconstruction)
            })?;

        let completed_workload = Self::compose(WorthWorkloadParts {
            topology: self.topology.clone(),
            geometry_binding: self.geometry_binding.clone(),
            surface_support: self.surface_support.clone(),
            projection: self.projection.clone(),
            transform: self.transform.clone(),
            retained_replay: self.retained_replay.clone(),
            diagnostics: self.diagnostics.clone(),
            response: self.response.clone(),
            evidence_ledger,
        })?;
        let runtime_registration_proof = PlanarBooleanLoopRuntimeRegistrationProof::certify(
            loop_ledger,
            &completed_workload,
            operator_matrix,
            validator_plan,
        )?;
        Ok(CompletedBooleanLoopReconstructionHandoff::new(
            completed_workload,
            None,
            loop_ledger.clone(),
            evidence_receipt.clone(),
            runtime_registration_proof,
        ))
    }
}
