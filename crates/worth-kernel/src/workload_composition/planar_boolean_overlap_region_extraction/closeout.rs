use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityReadinessInput;
use topology::facade::{
    PlanarBooleanOverlapOperatorClassificationMatrix,
    PlanarBooleanOverlapValidatorRegistrationPlan,
    TopologyMilestoneSevenFiveOverlapReadinessConsumer,
};
use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    ComparePlanarBooleanOverlapRegionReplayParity, PlanarBooleanOverlapRegionEvidenceInput,
    PlanarBooleanOverlapRegionEvidenceReceipt, PlanarBooleanOverlapRegionExtractionRequest,
    PlanarBooleanOverlapRegionLedgerAssemblyBundle, PlanarBooleanOverlapRegionLedgerReceipt,
    PlanarBooleanOverlapRegionReplayParityInput,
};
use worth_spatial::facade::retained_replay_workload::ReplayReceiptSet;

use crate::workload_composition::{
    CompletedBooleanLoopReconstructionHandoff, WorkloadCompositionError,
};

use super::{
    CompletedPlanarBooleanOverlapRegionExtractionHandoff,
    PlanarBooleanOverlapRegionAntiTheatreFenceProof,
    PlanarBooleanOverlapRegionPublicContractFenceProof, PlanarBooleanOverlapReplayCertifiedPeer,
    PlanarBooleanOverlapRuntimeRegistrationProof,
};

pub struct PlanarBooleanOverlapRegionCloseoutInput<'a> {
    readiness: &'a TouchedGraphParityReadinessInput,
    readiness_consumer: &'a TopologyMilestoneSevenFiveOverlapReadinessConsumer,
    overlap_request: &'a PlanarBooleanOverlapRegionExtractionRequest,
    overlap_ledger_bundle: &'a PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    replayed_loop_handoff: &'a CompletedBooleanLoopReconstructionHandoff,
    replay_receipts: &'a ReplayReceiptSet,
    operator_matrix: &'a PlanarBooleanOverlapOperatorClassificationMatrix,
    validator_plan: &'a PlanarBooleanOverlapValidatorRegistrationPlan,
}

impl<'a> PlanarBooleanOverlapRegionCloseoutInput<'a> {
    pub fn new(
        readiness: &'a TouchedGraphParityReadinessInput,
        readiness_consumer: &'a TopologyMilestoneSevenFiveOverlapReadinessConsumer,
        overlap_request: &'a PlanarBooleanOverlapRegionExtractionRequest,
        overlap_ledger_bundle: &'a PlanarBooleanOverlapRegionLedgerAssemblyBundle,
        replayed_loop_handoff: &'a CompletedBooleanLoopReconstructionHandoff,
        replay_receipts: &'a ReplayReceiptSet,
        operator_matrix: &'a PlanarBooleanOverlapOperatorClassificationMatrix,
        validator_plan: &'a PlanarBooleanOverlapValidatorRegistrationPlan,
    ) -> Self {
        Self {
            readiness,
            readiness_consumer,
            overlap_request,
            overlap_ledger_bundle,
            replayed_loop_handoff,
            replay_receipts,
            operator_matrix,
            validator_plan,
        }
    }
}

impl CompletedBooleanLoopReconstructionHandoff {
    pub fn complete_planar_boolean_overlap_region_extraction(
        &self,
        input: PlanarBooleanOverlapRegionCloseoutInput<'_>,
    ) -> Result<CompletedPlanarBooleanOverlapRegionExtractionHandoff, WorkloadCompositionError>
    {
        self.require_boolean_loop_reconstruction()?;
        if input
            .overlap_request
            .readiness_loop_ledger_binding()
            .loop_ledger_receipt_identity()
            != self.loop_ledger_receipt().receipt_identity()
        {
            return Err(WorkloadCompositionError::OverlapRegionCloseout(
                "overlap closeout requires the request readiness binding to consume the completed loop-ledger receipt".to_string(),
            ));
        }
        let replay_certified_peer =
            PlanarBooleanOverlapReplayCertifiedPeer::certify_from_loop_handoffs(
                self,
                input.replayed_loop_handoff,
                input.readiness_consumer,
                input.replay_receipts,
            )
            .map_err(|denial| {
                WorkloadCompositionError::OverlapRegionCloseout(format!("{denial:?}"))
            })?;
        let evidence_receipt = PlanarBooleanOverlapRegionEvidenceReceipt::admit(
            PlanarBooleanOverlapRegionEvidenceInput::from_readiness_and_request_and_ledger(
                input.readiness,
                input.readiness_consumer,
                input.overlap_request,
                input.overlap_ledger_bundle.receipt(),
                replay_certified_peer.replay_receipts(),
            ),
        )
        .map_err(|denial| WorkloadCompositionError::OverlapRegionCloseout(format!("{denial:?}")))?;
        let replayed_evidence_receipt = PlanarBooleanOverlapRegionEvidenceReceipt::admit(
            PlanarBooleanOverlapRegionEvidenceInput::from_readiness_and_request_and_ledger(
                input.readiness,
                input.readiness_consumer,
                replay_certified_peer.replayed_overlap_request(),
                replay_certified_peer.replayed_overlap_ledger_receipt(),
                replay_certified_peer.replay_receipts(),
            ),
        )
        .map_err(|denial| WorkloadCompositionError::OverlapRegionCloseout(format!("{denial:?}")))?;
        let replay_parity_receipt = ComparePlanarBooleanOverlapRegionReplayParity::compare(
            PlanarBooleanOverlapRegionReplayParityInput::admit_from_ledger_and_evidence(
                input.overlap_ledger_bundle.receipt(),
                replay_certified_peer.replayed_overlap_ledger_receipt(),
                &evidence_receipt,
                &replayed_evidence_receipt,
                replay_certified_peer.replay_receipts(),
            )
            .map_err(|denial| {
                WorkloadCompositionError::OverlapRegionCloseout(format!("{denial:?}"))
            })?,
        )
        .map_err(|denial| WorkloadCompositionError::OverlapRegionCloseout(format!("{denial:?}")))?;
        let checkpoint_parity_receipt = replay_parity_receipt.checkpoint_receipt().clone();
        let completed_workload = self
            .completed_workload()
            .with_completed_planar_boolean_overlap_region_extraction(&evidence_receipt)?;
        let runtime_registration_proof = PlanarBooleanOverlapRuntimeRegistrationProof::certify(
            input.overlap_ledger_bundle.receipt(),
            &evidence_receipt,
            &completed_workload,
            input.operator_matrix,
            input.validator_plan,
        )?;
        let public_contract_fence_proof =
            PlanarBooleanOverlapRegionPublicContractFenceProof::certify(
                input.overlap_ledger_bundle.receipt(),
                &evidence_receipt,
                &runtime_registration_proof,
                &completed_workload,
            )
            .map_err(|denial| {
                WorkloadCompositionError::OverlapRegionCloseout(format!("{denial:?}"))
            })?;
        let anti_theatre_fence_proof = PlanarBooleanOverlapRegionAntiTheatreFenceProof::certify(
            &evidence_receipt,
            &public_contract_fence_proof,
        )
        .map_err(|denial| WorkloadCompositionError::OverlapRegionCloseout(format!("{denial:?}")))?;

        Ok(CompletedPlanarBooleanOverlapRegionExtractionHandoff::new(
            completed_workload,
            input.overlap_ledger_bundle.clone(),
            input.overlap_ledger_bundle.receipt().clone(),
            evidence_receipt,
            replay_parity_receipt,
            checkpoint_parity_receipt,
            runtime_registration_proof,
            public_contract_fence_proof,
            anti_theatre_fence_proof,
        ))
    }
}
