use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanSplitNamedArtifactKind;
use crate::workload_platform::planar_boolean_events::PlanarBooleanLoopRole;
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanAdmittedReconstructedLoopSet, PlanarBooleanBornLoopSet,
    PlanarBooleanClosedWalkCandidateAssembly, PlanarBooleanClosedWalkCandidateSetInput,
    PlanarBooleanDegenerateLoopOutcome, PlanarBooleanDegenerateLoopOutcomeBoundary,
    PlanarBooleanDegenerateLoopOutcomeBoundaryInput, PlanarBooleanDegenerateLoopOutcomeKind,
    PlanarBooleanDegenerateLoopOutcomeSet, PlanarBooleanFragmentContinuationIndex,
    PlanarBooleanFragmentContinuationIndexInput, PlanarBooleanLoopCandidateBoundary,
    PlanarBooleanLoopCandidateBoundaryInput, PlanarBooleanLoopClassifiedProductKind,
    PlanarBooleanLoopContainmentEvidencePosture, PlanarBooleanLoopContainmentEvidencePostureKind,
    PlanarBooleanLoopDecisionLogInput, PlanarBooleanLoopIdentityBoundary,
    PlanarBooleanLoopIdentityMap, PlanarBooleanLoopIdentityMintingInput,
    PlanarBooleanLoopIslandPartition, PlanarBooleanLoopIslandPartitionInput,
    PlanarBooleanLoopNamingAuthoritySupport, PlanarBooleanLoopPersistentNamePropagationMap,
    PlanarBooleanLoopReconstructionLedgerInput, PlanarBooleanLoopReconstructionRequest,
    PlanarBooleanLoopRoleOutcome, PlanarBooleanLoopRoleOutcomeBoundary,
    PlanarBooleanLoopRoleOutcomeBoundaryInput, PlanarBooleanLoopRoleOutcomeKind,
    PlanarBooleanLoopRoleOutcomeSet, PlanarBooleanLoopSourceProvenanceBundle,
    PlanarBooleanLoopSubshapeSignatureMap, PlanarBooleanReconstructedLoopBoundary,
    PlanarBooleanReconstructedLoopBoundaryInput, PlanarBooleanSourceLoopSplitAttribution,
    PlanarBooleanSourceLoopSplitAttributionInput, PlanarBooleanSourceLoopSplitAttributionKind,
    PlanarBooleanSourceLoopSplitAttributionRow, PlanarBooleanWalkOutcomeSet,
    PlanarBooleanWalkOutcomeSetInput,
};

use super::{
    prepared_loop_reconstruction_subject, prepared_loop_reconstruction_subject_with_tag,
    LoopFixtureEntryOrder, PreparedLoopReconstructionSubject,
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

    pub(crate) fn ledger_input_with_identity_products<'a>(
        &'a self,
        decision_log: &'a crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopDecisionLog,
        identity_map: &'a crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopIdentityMap,
        persistent_name_map: &'a crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopPersistentNamePropagationMap,
        subshape_signature_map: &'a crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopSubshapeSignatureMap,
    ) -> PlanarBooleanLoopReconstructionLedgerInput<'a> {
        PlanarBooleanLoopReconstructionLedgerInput::from_decision_log_and_loop_products(
            &self.request,
            decision_log,
            identity_map,
            persistent_name_map,
            subshape_signature_map,
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
    prepared_phase_fourteen_subject_with_tag(order, "phase-14")
}

pub(crate) fn prepared_phase_fourteen_subject_with_tag(
    order: LoopFixtureEntryOrder,
    tag: &str,
) -> PreparedPhaseFourteenSubject {
    let prepared = if tag == "phase-14" {
        prepared_loop_reconstruction_subject(order)
    } else {
        prepared_loop_reconstruction_subject_with_tag(order, tag)
    };
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

pub(crate) fn admitted_phase_fourteen_identity_products(
    fixture: &PreparedPhaseFourteenSubject,
) -> (
    PlanarBooleanLoopIdentityMap,
    PlanarBooleanLoopPersistentNamePropagationMap,
    PlanarBooleanLoopSubshapeSignatureMap,
) {
    let reconstructed = fixture
        .reconstructed_boundary
        .reconstructed_loops()
        .rows()
        .first()
        .expect("phase fourteen fixture should reconstruct at least one loop");
    let source_carrier = fixture
        .source_provenance
        .source_loop_carriers()
        .rows()
        .first()
        .expect("phase fourteen fixture should expose one source carrier");
    let fragment_identities = fixture
        .prepared
        .naming
        .persistent_name_rows()
        .iter()
        .filter(|row| row.artifact_kind() == PlanarBooleanSplitNamedArtifactKind::SplitFragment)
        .take(2)
        .map(|row| row.artifact_identity().to_string())
        .collect::<Vec<_>>();
    let split_vertex_identities = fixture
        .prepared
        .naming
        .persistent_name_rows()
        .iter()
        .filter(|row| row.artifact_kind() == PlanarBooleanSplitNamedArtifactKind::SplitVertex)
        .take(1)
        .map(|row| row.artifact_identity().to_string())
        .collect::<Vec<_>>();
    let naming_support =
        PlanarBooleanLoopNamingAuthoritySupport::admit_from_split_receipt_and_provenance(
            &fixture.prepared.naming,
            &fixture.source_provenance,
            &PlanarBooleanSourceLoopSplitAttribution::new(
                "split-attribution:phase-fourteen-admitted".to_string(),
                fixture.request.request_identity().to_string(),
                vec![PlanarBooleanSourceLoopSplitAttributionRow::new(
                    "attribution:phase-fourteen-admitted".to_string(),
                    source_carrier.source_loop_identity().to_string(),
                    vec!["island:phase-fourteen-admitted".to_string()],
                    PlanarBooleanSourceLoopSplitAttributionKind::Preserved,
                )],
                fixture.split_attribution.counters(),
            ),
        )
        .expect("phase fourteen fixture should lower real naming support");
    let request_identity = fixture.request.request_identity().to_string();
    let loop_identity = reconstructed.reconstructed_loop_identity().to_string();
    let role_outcome_identity = "role-outcome:phase-fourteen-admitted".to_string();
    let containment_identity = "containment:phase-fourteen-admitted".to_string();
    let reconstructed_loops = PlanarBooleanAdmittedReconstructedLoopSet::new(
        format!("admitted-reconstructed-set:{request_identity}"),
        request_identity.clone(),
        vec![reconstructed.clone()],
    );
    let born_loops = PlanarBooleanBornLoopSet::new(
        format!("admitted-born-set:{request_identity}"),
        request_identity.clone(),
        Vec::new(),
    );
    let role_outcomes = PlanarBooleanLoopRoleOutcomeSet::new(
        format!("admitted-role-set:{request_identity}"),
        request_identity.clone(),
        vec![PlanarBooleanLoopRoleOutcome::new(
            role_outcome_identity.clone(),
            loop_identity.clone(),
            PlanarBooleanLoopClassifiedProductKind::ReconstructedLoop,
            vec!["island:phase-fourteen-admitted".to_string()],
            vec![source_carrier.source_loop_identity().to_string()],
            Some(PlanarBooleanLoopRole::OuterBoundary),
            PlanarBooleanLoopRoleOutcomeKind::PreservedSourceRole,
        )],
    );
    let degenerate_outcomes = PlanarBooleanDegenerateLoopOutcomeSet::new(
        format!("admitted-degenerate-set:{request_identity}"),
        request_identity,
        vec![PlanarBooleanDegenerateLoopOutcome::new(
            "degenerate-outcome:phase-fourteen-admitted".to_string(),
            loop_identity.clone(),
            PlanarBooleanLoopClassifiedProductKind::ReconstructedLoop,
            vec![source_carrier.source_loop_identity().to_string()],
            "local-frame:phase-fourteen-admitted".to_string(),
            "precision-basis:phase-fourteen-admitted".to_string(),
            fragment_identities,
            split_vertex_identities,
            Some(role_outcome_identity),
            Some(containment_identity.clone()),
            PlanarBooleanDegenerateLoopOutcomeKind::AdmittedForIdentityMinting,
            "fixture admits the reconstructed loop into phase fourteen identity products"
                .to_string(),
        )],
    );
    let split_attribution = PlanarBooleanSourceLoopSplitAttribution::new(
        "split-attribution:phase-fourteen-admitted".to_string(),
        fixture.request.request_identity().to_string(),
        vec![PlanarBooleanSourceLoopSplitAttributionRow::new(
            "attribution:phase-fourteen-admitted".to_string(),
            source_carrier.source_loop_identity().to_string(),
            vec!["island:phase-fourteen-admitted".to_string()],
            PlanarBooleanSourceLoopSplitAttributionKind::Preserved,
        )],
        fixture.split_attribution.counters(),
    );
    let _containment = PlanarBooleanLoopContainmentEvidencePosture::new(
        containment_identity,
        loop_identity,
        PlanarBooleanLoopClassifiedProductKind::ReconstructedLoop,
        vec!["island:phase-fourteen-admitted".to_string()],
        vec![source_carrier.source_loop_identity().to_string()],
        PlanarBooleanLoopContainmentEvidencePostureKind::PreservedSourceContainmentEvidence,
    );
    let boundary = PlanarBooleanLoopIdentityBoundary::mint(
        PlanarBooleanLoopIdentityMintingInput::from_phase_twelve_products_and_naming_support(
            &reconstructed_loops,
            &born_loops,
            &role_outcomes,
            &degenerate_outcomes,
            fixture.loop_candidate_boundary.denied_loop_candidates(),
            &naming_support,
            &split_attribution,
        ),
    )
    .expect(
        "phase fourteen fixture should mint admitted identity products with real naming support",
    );
    (
        boundary.loop_identity_map().clone(),
        boundary.persistent_name_propagation_map().clone(),
        boundary.subshape_signature_map().clone(),
    )
}
