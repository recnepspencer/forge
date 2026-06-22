use topology::facade::{
    PlanarBooleanLoopOperatorClassificationMatrix, PlanarBooleanLoopValidatorRegistrationPlan,
};
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanEdgeSplitReplayParityReceipt, PlanarBooleanLoopReconstructionSplitConsumption,
    PlanarBooleanLoopReconstructionSplitConsumptionInput, PlanarBooleanOverlapEdgeChainSet,
    PlanarBooleanSplitChainValidationReceipt, PlanarBooleanSplitDecisionLogReceipt,
    PlanarBooleanSplitEdgeChainLedger, PlanarBooleanSplitEdgeFragmentSet,
    PlanarBooleanSplitPersistentNamingReceipt, PlanarBooleanSplitSourceEdgeCarrierSet,
    PlanarBooleanSplitVertexIdentitySet,
};
use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanClosedWalkCandidateAssembly, PlanarBooleanClosedWalkCandidateSetInput,
    PlanarBooleanDegenerateLoopOutcomeBoundary, PlanarBooleanDegenerateLoopOutcomeBoundaryInput,
    PlanarBooleanFragmentContinuationIndex, PlanarBooleanFragmentContinuationIndexInput,
    PlanarBooleanLoopCandidateBoundary, PlanarBooleanLoopCandidateBoundaryInput,
    PlanarBooleanLoopDecisionLog, PlanarBooleanLoopDecisionLogInput,
    PlanarBooleanLoopIdentityBoundary, PlanarBooleanLoopIdentityMintingInput,
    PlanarBooleanLoopNamingAuthoritySupport, PlanarBooleanLoopReconstructionEvidenceInput,
    PlanarBooleanLoopReconstructionEvidenceReceipt, PlanarBooleanLoopReconstructionLedger,
    PlanarBooleanLoopReconstructionLedgerInput, PlanarBooleanLoopReconstructionRequest,
    PlanarBooleanLoopReconstructionRequestInput, PlanarBooleanLoopRoleOutcomeBoundary,
    PlanarBooleanLoopRoleOutcomeBoundaryInput, PlanarBooleanLoopSourceProvenanceBundle,
    PlanarBooleanLoopSourceProvenanceRecoveryInput, PlanarBooleanReconstructedLoopBoundary,
    PlanarBooleanReconstructedLoopBoundaryInput, PlanarBooleanSourceLoopSplitAttribution,
    PlanarBooleanSourceLoopSplitAttributionInput, PlanarBooleanWalkOutcomeSet,
    PlanarBooleanWalkOutcomeSetInput,
};
use worth_spatial::facade::retained_replay_workload::ReplayReceiptSet;

use super::{
    CompletedBooleanLoopReconstructionHandoff, CompletedBooleanLoopReconstructionProducts,
    CompletedBooleanSplitHandoff, WorkloadCompositionError,
};

pub struct PlanarBooleanLoopReconstructionCloseoutInput<'a> {
    split_decision_log_receipt: &'a PlanarBooleanSplitDecisionLogReceipt,
    split_validation_receipt: &'a PlanarBooleanSplitChainValidationReceipt,
    split_persistent_naming_receipt: &'a PlanarBooleanSplitPersistentNamingReceipt,
    split_replay_parity_receipt: &'a PlanarBooleanEdgeSplitReplayParityReceipt,
    split_ledger: &'a PlanarBooleanSplitEdgeChainLedger,
    split_recovered_source_carriers: &'a PlanarBooleanSplitSourceEdgeCarrierSet,
    split_vertices: &'a PlanarBooleanSplitVertexIdentitySet,
    split_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
    overlap_chains: &'a PlanarBooleanOverlapEdgeChainSet,
    replay_receipts: &'a ReplayReceiptSet,
    operator_matrix: &'a PlanarBooleanLoopOperatorClassificationMatrix,
    validator_plan: &'a PlanarBooleanLoopValidatorRegistrationPlan,
}

impl<'a> PlanarBooleanLoopReconstructionCloseoutInput<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        split_decision_log_receipt: &'a PlanarBooleanSplitDecisionLogReceipt,
        split_validation_receipt: &'a PlanarBooleanSplitChainValidationReceipt,
        split_persistent_naming_receipt: &'a PlanarBooleanSplitPersistentNamingReceipt,
        split_replay_parity_receipt: &'a PlanarBooleanEdgeSplitReplayParityReceipt,
        split_ledger: &'a PlanarBooleanSplitEdgeChainLedger,
        split_recovered_source_carriers: &'a PlanarBooleanSplitSourceEdgeCarrierSet,
        split_vertices: &'a PlanarBooleanSplitVertexIdentitySet,
        split_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
        overlap_chains: &'a PlanarBooleanOverlapEdgeChainSet,
        replay_receipts: &'a ReplayReceiptSet,
        operator_matrix: &'a PlanarBooleanLoopOperatorClassificationMatrix,
        validator_plan: &'a PlanarBooleanLoopValidatorRegistrationPlan,
    ) -> Self {
        Self {
            split_decision_log_receipt,
            split_validation_receipt,
            split_persistent_naming_receipt,
            split_replay_parity_receipt,
            split_ledger,
            split_recovered_source_carriers,
            split_vertices,
            split_fragments,
            overlap_chains,
            replay_receipts,
            operator_matrix,
            validator_plan,
        }
    }
}

impl CompletedBooleanSplitHandoff {
    pub fn complete_boolean_loop_reconstruction(
        &self,
        input: PlanarBooleanLoopReconstructionCloseoutInput<'_>,
    ) -> Result<CompletedBooleanLoopReconstructionHandoff, WorkloadCompositionError> {
        let downstream_consumption = self.admit_downstream_split_consumption(
            input.split_decision_log_receipt,
            input.split_validation_receipt,
            input.split_persistent_naming_receipt,
            input.split_replay_parity_receipt,
        )?;
        let loop_split_consumption = PlanarBooleanLoopReconstructionSplitConsumption::admit(
            PlanarBooleanLoopReconstructionSplitConsumptionInput::from_downstream_split_consumption(
                &downstream_consumption,
            ),
        )
        .map_err(|denial| {
            WorkloadCompositionError::LoopReconstructionCloseout(denial.human_reason().to_string())
        })?;
        let loop_request = PlanarBooleanLoopReconstructionRequest::admit(
            PlanarBooleanLoopReconstructionRequestInput::from_split_consumption(
                &loop_split_consumption,
            ),
        )
        .map_err(|denial| {
            WorkloadCompositionError::LoopReconstructionCloseout(denial.human_reason().to_string())
        })?;
        let source_provenance = PlanarBooleanLoopSourceProvenanceBundle::recover(
            PlanarBooleanLoopSourceProvenanceRecoveryInput::from_request_and_split_support(
                &loop_request,
                input.split_ledger,
                self.split_ledger_receipt(),
                input.split_recovered_source_carriers,
                input.split_fragments,
                input.overlap_chains,
            ),
        )
        .map_err(|denial| {
            WorkloadCompositionError::LoopReconstructionCloseout(denial.human_reason().to_string())
        })?;
        let continuation_index = PlanarBooleanFragmentContinuationIndex::admit(
            PlanarBooleanFragmentContinuationIndexInput::from_request_and_provenance(
                &loop_request,
                &source_provenance,
                input.split_vertices,
                input.split_fragments,
                input.overlap_chains,
            ),
        )
        .map_err(|denial| {
            WorkloadCompositionError::LoopReconstructionCloseout(denial.human_reason().to_string())
        })?;
        let walk_candidate_assembly = PlanarBooleanClosedWalkCandidateAssembly::assemble(
            PlanarBooleanClosedWalkCandidateSetInput::from_continuation_index(&continuation_index),
        );
        let walk_outcomes = PlanarBooleanWalkOutcomeSet::classify(
            PlanarBooleanWalkOutcomeSetInput::from_closed_walk_candidates(
                walk_candidate_assembly.closed_walk_candidates(),
                walk_candidate_assembly.fragment_consumption_proof(),
            ),
        );
        let candidate_boundary = PlanarBooleanLoopCandidateBoundary::promote(
            PlanarBooleanLoopCandidateBoundaryInput::from_walk_outcomes(&walk_outcomes),
        );
        let reconstructed_boundary = PlanarBooleanReconstructedLoopBoundary::admit(
            PlanarBooleanReconstructedLoopBoundaryInput::from_loop_candidates_and_provenance(
                candidate_boundary.loop_candidates(),
                &source_provenance,
            ),
        )
        .map_err(|denial| {
            WorkloadCompositionError::LoopReconstructionCloseout(denial.human_reason().to_string())
        })?;
        let island_partition =
            worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanLoopIslandPartition::partition(
                worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanLoopIslandPartitionInput::from_reconstructed_loop_boundary(
                    reconstructed_boundary.reconstructed_loops(),
                    reconstructed_boundary.born_loops(),
                ),
            );
        let split_attribution = PlanarBooleanSourceLoopSplitAttribution::attribute(
            PlanarBooleanSourceLoopSplitAttributionInput::from_island_partition(&island_partition),
        );
        let role_boundary = PlanarBooleanLoopRoleOutcomeBoundary::classify(
            PlanarBooleanLoopRoleOutcomeBoundaryInput::from_reconstructed_loop_products_and_provenance(
                &reconstructed_boundary,
                &island_partition,
                &split_attribution,
                &source_provenance,
            ),
        );
        let degenerate_boundary = PlanarBooleanDegenerateLoopOutcomeBoundary::classify(
            PlanarBooleanDegenerateLoopOutcomeBoundaryInput::from_reconstructed_products_and_role_evidence(
                reconstructed_boundary.reconstructed_loops(),
                reconstructed_boundary.born_loops(),
                role_boundary.role_outcomes(),
                role_boundary.containment_evidence_postures(),
                source_provenance.source_loop_carriers(),
                input.split_fragments,
            ),
        );
        let naming_support =
            PlanarBooleanLoopNamingAuthoritySupport::admit_from_split_receipt_and_provenance(
                input.split_persistent_naming_receipt,
                &source_provenance,
                &split_attribution,
            )
            .map_err(|denial| {
                WorkloadCompositionError::LoopReconstructionCloseout(
                    denial.human_reason().to_string(),
                )
            })?;
        let identity_boundary = PlanarBooleanLoopIdentityBoundary::mint(
            PlanarBooleanLoopIdentityMintingInput::from_phase_twelve_products_and_naming_support(
                reconstructed_boundary.reconstructed_loops(),
                reconstructed_boundary.born_loops(),
                role_boundary.role_outcomes(),
                degenerate_boundary.outcomes(),
                candidate_boundary.denied_loop_candidates(),
                &naming_support,
                &split_attribution,
            ),
        )
        .map_err(|denial| {
            WorkloadCompositionError::LoopReconstructionCloseout(denial.human_reason().to_string())
        })?;
        let decision_log = PlanarBooleanLoopDecisionLog::record(
            PlanarBooleanLoopDecisionLogInput::from_phase_thirteen_products(
                &loop_request,
                &continuation_index,
                &walk_outcomes,
                candidate_boundary.loop_candidates(),
                candidate_boundary.denied_loop_candidates(),
                reconstructed_boundary.reconstructed_loops(),
                reconstructed_boundary.born_loops(),
                &island_partition,
                &split_attribution,
                role_boundary.role_outcomes(),
                degenerate_boundary.outcomes(),
                identity_boundary.loop_identity_map(),
                identity_boundary.persistent_name_propagation_map(),
                identity_boundary.subshape_signature_map(),
            ),
        )
        .map_err(|denial| {
            WorkloadCompositionError::LoopReconstructionCloseout(denial.human_reason().to_string())
        })?;
        let (loop_ledger, loop_ledger_receipt) = PlanarBooleanLoopReconstructionLedger::assemble(
            PlanarBooleanLoopReconstructionLedgerInput::from_decision_log_and_loop_products(
                &loop_request,
                &decision_log,
                identity_boundary.loop_identity_map(),
                identity_boundary.persistent_name_propagation_map(),
                identity_boundary.subshape_signature_map(),
                reconstructed_boundary.reconstructed_loops(),
                reconstructed_boundary.born_loops(),
                &island_partition,
                &split_attribution,
                role_boundary.role_outcomes(),
                degenerate_boundary.outcomes(),
            ),
        )
        .map_err(|denial| {
            WorkloadCompositionError::LoopReconstructionCloseout(denial.human_reason().to_string())
        })?;

        let evidence_receipt = PlanarBooleanLoopReconstructionEvidenceReceipt::admit(
            PlanarBooleanLoopReconstructionEvidenceInput::from_phase_sixteen_products(
                &reconstructed_boundary,
                &island_partition,
                &split_attribution,
                role_boundary.role_outcomes(),
                degenerate_boundary.outcomes(),
                &decision_log,
                &loop_ledger_receipt,
                input.replay_receipts,
            ),
        );
        let completed_handoff = self
            .completed_workload()
            .with_completed_boolean_loop_reconstruction(
                &loop_ledger_receipt,
                &evidence_receipt,
                input.operator_matrix,
                input.validator_plan,
            )?;
        let completed_workload = completed_handoff.completed_workload().clone();
        let runtime_registration_proof = completed_handoff.runtime_registration_proof().clone();
        let products = CompletedBooleanLoopReconstructionProducts::new(
            downstream_consumption,
            loop_split_consumption,
            loop_request,
            source_provenance,
            input.split_fragments.clone(),
            continuation_index,
            walk_candidate_assembly,
            walk_outcomes,
            candidate_boundary,
            reconstructed_boundary,
            island_partition,
            split_attribution,
            role_boundary.role_outcomes().clone(),
            role_boundary.containment_evidence_postures().clone(),
            degenerate_boundary.clone(),
            degenerate_boundary.outcomes().clone(),
            decision_log,
            loop_ledger,
        );

        Ok(CompletedBooleanLoopReconstructionHandoff::new(
            completed_workload,
            Some(products),
            loop_ledger_receipt,
            evidence_receipt,
            runtime_registration_proof,
        ))
    }
}
