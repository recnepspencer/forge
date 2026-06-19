use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanClosedWalkCandidateAssembly, PlanarBooleanClosedWalkCandidateSetInput,
    PlanarBooleanDegenerateLoopOutcomeBoundary, PlanarBooleanDegenerateLoopOutcomeBoundaryInput,
    PlanarBooleanFragmentContinuationIndex, PlanarBooleanFragmentContinuationIndexInput,
    PlanarBooleanLoopCandidateBoundary, PlanarBooleanLoopCandidateBoundaryInput,
    PlanarBooleanLoopDecisionLogInput, PlanarBooleanLoopIdentityBoundary,
    PlanarBooleanLoopIdentityMintingInput, PlanarBooleanLoopIslandPartition,
    PlanarBooleanLoopIslandPartitionInput, PlanarBooleanLoopNamingAuthoritySupport,
    PlanarBooleanLoopReconstructionLedgerInput, PlanarBooleanLoopReconstructionRequest,
    PlanarBooleanLoopRoleOutcomeBoundary, PlanarBooleanLoopRoleOutcomeBoundaryInput,
    PlanarBooleanLoopSourceProvenanceBundle, PlanarBooleanReconstructedLoopBoundary,
    PlanarBooleanReconstructedLoopBoundaryInput, PlanarBooleanSourceLoopSplitAttribution,
    PlanarBooleanSourceLoopSplitAttributionInput, PlanarBooleanWalkOutcomeSet,
    PlanarBooleanWalkOutcomeSetInput,
};

use super::{
    prepared_loop_reconstruction_subject, LoopFixtureEntryOrder, PreparedLoopReconstructionSubject,
};

pub(crate) struct PreparedPhaseFourteenSubject {
    pub(crate) prepared: PreparedLoopReconstructionSubject,
    pub(crate) request: PlanarBooleanLoopReconstructionRequest,
    pub(crate) source_provenance: PlanarBooleanLoopSourceProvenanceBundle,
    pub(crate) continuation_index: PlanarBooleanFragmentContinuationIndex,
    pub(crate) walk_outcomes: PlanarBooleanWalkOutcomeSet,
    pub(crate) loop_candidate_boundary: PlanarBooleanLoopCandidateBoundary,
    pub(crate) reconstructed_boundary: PlanarBooleanReconstructedLoopBoundary,
    pub(crate) island_partition: PlanarBooleanLoopIslandPartition,
    pub(crate) split_attribution: PlanarBooleanSourceLoopSplitAttribution,
    pub(crate) role_boundary: PlanarBooleanLoopRoleOutcomeBoundary,
    pub(crate) degenerate_boundary: PlanarBooleanDegenerateLoopOutcomeBoundary,
    pub(crate) identity_boundary: PlanarBooleanLoopIdentityBoundary,
}

impl PreparedPhaseFourteenSubject {
    pub(crate) fn decision_log_input(&self) -> PlanarBooleanLoopDecisionLogInput<'_> {
        PlanarBooleanLoopDecisionLogInput::from_phase_thirteen_products(
            &self.request,
            &self.continuation_index,
            &self.walk_outcomes,
            self.loop_candidate_boundary.loop_candidates(),
            self.loop_candidate_boundary.denied_loop_candidates(),
            self.reconstructed_boundary.reconstructed_loops(),
            self.reconstructed_boundary.born_loops(),
            &self.island_partition,
            &self.split_attribution,
            self.role_boundary.role_outcomes(),
            self.degenerate_boundary.outcomes(),
            self.identity_boundary.loop_identity_map(),
            self.identity_boundary.persistent_name_propagation_map(),
            self.identity_boundary.subshape_signature_map(),
        )
    }

    pub(crate) fn ledger_input<'a>(
        &'a self,
        decision_log: &'a crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopDecisionLog,
    ) -> PlanarBooleanLoopReconstructionLedgerInput<'a> {
        PlanarBooleanLoopReconstructionLedgerInput::from_decision_log_and_loop_products(
            &self.request,
            decision_log,
            self.identity_boundary.loop_identity_map(),
            self.identity_boundary.persistent_name_propagation_map(),
            self.identity_boundary.subshape_signature_map(),
            self.reconstructed_boundary.reconstructed_loops(),
            self.reconstructed_boundary.born_loops(),
            &self.island_partition,
            &self.split_attribution,
            self.role_boundary.role_outcomes(),
            self.degenerate_boundary.outcomes(),
        )
    }
}

pub(crate) fn prepared_phase_fourteen_subject(
    order: LoopFixtureEntryOrder,
) -> PreparedPhaseFourteenSubject {
    let prepared = prepared_loop_reconstruction_subject(order);
    let request = prepared.admit_loop_request();
    let source_provenance = PlanarBooleanLoopSourceProvenanceBundle::recover(
        crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopSourceProvenanceRecoveryInput::from_request_and_split_support(
            &request,
            prepared.split_ledger_result.ledger(),
            prepared.split_ledger_result.receipt(),
            &prepared.recovered_source_carriers,
            &prepared.fragments,
            &prepared.overlap_chains,
        ),
    )
    .expect("phase fourteen test support should recover source provenance");
    let continuation_index = PlanarBooleanFragmentContinuationIndex::admit(
        PlanarBooleanFragmentContinuationIndexInput::from_request_and_provenance(
            &request,
            &source_provenance,
            &prepared.vertices,
            &prepared.fragments,
            &prepared.overlap_chains,
        ),
    )
    .expect("phase fourteen test support should admit continuation index");
    let walk_assembly = PlanarBooleanClosedWalkCandidateAssembly::assemble(
        PlanarBooleanClosedWalkCandidateSetInput::from_continuation_index(&continuation_index),
    );
    let walk_outcomes = PlanarBooleanWalkOutcomeSet::classify(
        PlanarBooleanWalkOutcomeSetInput::from_closed_walk_candidates(
            walk_assembly.closed_walk_candidates(),
            walk_assembly.fragment_consumption_proof(),
        ),
    );
    let loop_candidate_boundary = PlanarBooleanLoopCandidateBoundary::promote(
        PlanarBooleanLoopCandidateBoundaryInput::from_walk_outcomes(&walk_outcomes),
    );
    let reconstructed_boundary = PlanarBooleanReconstructedLoopBoundary::admit(
        PlanarBooleanReconstructedLoopBoundaryInput::from_loop_candidates_and_provenance(
            loop_candidate_boundary.loop_candidates(),
            &source_provenance,
        ),
    )
    .expect("phase fourteen test support should reconstruct loops");
    let island_partition = PlanarBooleanLoopIslandPartition::partition(
        PlanarBooleanLoopIslandPartitionInput::from_reconstructed_loop_boundary(
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
            &prepared.fragments,
        ),
    );
    let naming_support =
        PlanarBooleanLoopNamingAuthoritySupport::admit_from_split_receipt_and_provenance(
            &prepared.naming,
            &source_provenance,
            &split_attribution,
        )
        .expect("phase fourteen test support should admit naming authority");
    let identity_boundary = PlanarBooleanLoopIdentityBoundary::mint(
        PlanarBooleanLoopIdentityMintingInput::from_phase_twelve_products_and_naming_support(
            reconstructed_boundary.reconstructed_loops(),
            reconstructed_boundary.born_loops(),
            role_boundary.role_outcomes(),
            degenerate_boundary.outcomes(),
            loop_candidate_boundary.denied_loop_candidates(),
            &naming_support,
            &split_attribution,
        ),
    )
    .expect("phase fourteen test support should mint loop identities");

    PreparedPhaseFourteenSubject {
        prepared,
        request,
        source_provenance,
        continuation_index,
        walk_outcomes,
        loop_candidate_boundary,
        reconstructed_boundary,
        island_partition,
        split_attribution,
        role_boundary,
        degenerate_boundary,
        identity_boundary,
    }
}
