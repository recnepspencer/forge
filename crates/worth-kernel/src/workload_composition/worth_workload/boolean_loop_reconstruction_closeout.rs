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

use crate::replay_undo_transaction_boundary::ReplayUndoTransactionBoundaryPacket;

use super::{
    AdmittedBooleanSplitReplayUndoBoundary, CompletedBooleanLoopReconstructionHandoff,
    CompletedBooleanLoopReconstructionProducts, CompletedBooleanSplitHandoff,
    WorkloadCompositionError,
};
use crate::workload_composition::current_touched_graph_readiness_handoff;

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
    pub(crate) fn complete_boolean_loop_reconstruction_from_admitted_replay_undo_boundary(
        &self,
        admitted_boundary: &AdmittedBooleanSplitReplayUndoBoundary,
        input: PlanarBooleanLoopReconstructionCloseoutInput<'_>,
    ) -> Result<CompletedBooleanLoopReconstructionHandoff, WorkloadCompositionError> {
        self.complete_boolean_loop_reconstruction_with_boundary_packet(
            input,
            Some(admitted_boundary.transaction_boundary_packet().clone()),
        )
    }

    fn complete_boolean_loop_reconstruction_with_boundary_packet(
        &self,
        input: PlanarBooleanLoopReconstructionCloseoutInput<'_>,
        replay_undo_transaction_boundary_packet: Option<ReplayUndoTransactionBoundaryPacket>,
    ) -> Result<CompletedBooleanLoopReconstructionHandoff, WorkloadCompositionError> {
        let batch_execution_cluster = self.admit_batch_execution_cluster()?;
        let downstream_consumption = batch_execution_cluster.admit_downstream_split_consumption(
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
            PlanarBooleanLoopReconstructionRequestInput::from_split_consumption_and_readiness(
                &loop_split_consumption,
                &current_touched_graph_readiness_handoff().map_err(|error| {
                    WorkloadCompositionError::LoopReconstructionCloseout(error.detail().to_string())
                })?,
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
        trace_loop_reconstruction_counts(
            source_provenance.source_loop_carriers().rows().len(),
            source_provenance.fragment_membership_map().rows().len(),
            source_provenance.overlap_chain_lineage_map().rows().len(),
            continuation_index.rows().len(),
            walk_candidate_assembly
                .closed_walk_candidates()
                .rows()
                .len(),
            walk_outcomes.rows().len(),
            candidate_boundary.loop_candidates().rows().len(),
            candidate_boundary.denied_loop_candidates().rows().len(),
            reconstructed_boundary.reconstructed_loops().rows().len(),
            reconstructed_boundary.born_loops().rows().len(),
            island_partition.rows().len(),
            role_boundary.role_outcomes().rows().len(),
            role_boundary.containment_evidence_postures().rows().len(),
            degenerate_boundary.outcomes().rows().len(),
            degenerate_boundary
                .counters()
                .admitted_for_identity_minting(),
            degenerate_boundary
                .counters()
                .tiny_cardinality_outcomes_emitted(),
            degenerate_boundary
                .counters()
                .self_touching_outcomes_emitted(),
            degenerate_boundary.counters().zero_area_outcomes_emitted(),
            degenerate_boundary
                .counters()
                .geometry_policy_required_outcomes_emitted(),
            degenerate_boundary
                .counters()
                .policy_required_outcomes_emitted(),
            identity_boundary.loop_identity_map().rows().len(),
            loop_ledger.rows().len(),
        );

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
                self.lookup_consumed_workload_handoff(),
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
            identity_boundary.loop_identity_map().clone(),
            identity_boundary.persistent_name_propagation_map().clone(),
            identity_boundary.subshape_signature_map().clone(),
            decision_log,
            loop_ledger,
        );

        Ok(CompletedBooleanLoopReconstructionHandoff::new(
            completed_workload,
            Some(products),
            loop_ledger_receipt,
            evidence_receipt,
            runtime_registration_proof,
            self.lookup_consumed_workload_handoff().clone(),
            replay_undo_transaction_boundary_packet,
        ))
    }
}

#[allow(clippy::too_many_arguments)]
fn trace_loop_reconstruction_counts(
    source_carriers: usize,
    fragment_memberships: usize,
    overlap_lineage: usize,
    continuation_rows: usize,
    closed_walk_candidates: usize,
    walk_outcomes: usize,
    loop_candidates: usize,
    denied_loop_candidates: usize,
    reconstructed_loops: usize,
    born_loops: usize,
    island_rows: usize,
    role_outcomes: usize,
    containment_postures: usize,
    degenerate_outcomes: usize,
    degenerate_admitted: usize,
    degenerate_tiny: usize,
    degenerate_self_touching: usize,
    degenerate_zero_area: usize,
    degenerate_geometry_policy: usize,
    degenerate_policy: usize,
    identity_rows: usize,
    ledger_rows: usize,
) {
    if std::env::var_os("WORTH_TRACE_LOOP_RECONSTRUCTION").is_none() {
        return;
    }
    eprintln!(
        "loop reconstruction counts: source_carriers={source_carriers} fragment_memberships={fragment_memberships} overlap_lineage={overlap_lineage} continuation_rows={continuation_rows} closed_walk_candidates={closed_walk_candidates} walk_outcomes={walk_outcomes} loop_candidates={loop_candidates} denied_loop_candidates={denied_loop_candidates} reconstructed_loops={reconstructed_loops} born_loops={born_loops} island_rows={island_rows} role_outcomes={role_outcomes} containment_postures={containment_postures} degenerate_outcomes={degenerate_outcomes} degenerate_admitted={degenerate_admitted} degenerate_tiny={degenerate_tiny} degenerate_self_touching={degenerate_self_touching} degenerate_zero_area={degenerate_zero_area} degenerate_geometry_policy={degenerate_geometry_policy} degenerate_policy={degenerate_policy} identity_rows={identity_rows} ledger_rows={ledger_rows}",
    );
}
