#![allow(dead_code)]
#![allow(unused_imports)]

#[allow(dead_code)]
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_collinear_relations_support/mod.rs"]
mod collinear_relation_support;
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_loop_reconstruction_continuation_contract_support/mod.rs"]
#[allow(dead_code)]
mod continuation_contract_support;
#[allow(dead_code)]
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_edge_splitting_decision_log_support.rs"]
mod edge_splitting_decision_log_support;
#[allow(dead_code)]
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_edge_splitting_endpoint_boundary_support.rs"]
mod edge_splitting_endpoint_boundary_support;
#[allow(dead_code)]
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_edge_splitting_interval_subdivision_support.rs"]
mod edge_splitting_interval_subdivision_support;
#[allow(dead_code)]
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_edge_splitting_persistent_naming_support.rs"]
mod edge_splitting_persistent_naming_support;
#[allow(dead_code)]
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_edge_splitting_raw_schedule_support.rs"]
mod edge_splitting_raw_schedule_support;
#[allow(dead_code)]
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_edge_splitting_replay_parity_support.rs"]
mod edge_splitting_replay_parity_support;
#[allow(dead_code)]
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_edge_splitting_split_vertex_identity_support.rs"]
mod edge_splitting_split_vertex_identity_support;
#[allow(dead_code)]
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_edge_splitting_support.rs"]
mod edge_splitting_support;
#[allow(dead_code)]
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_event_ledger_support.rs"]
mod event_ledger_support;
#[allow(dead_code)]
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_event_extraction_metaboss_support/mod.rs"]
mod metaboss_support;
#[allow(dead_code)]
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_point_events_support/mod.rs"]
mod point_event_support;
#[allow(dead_code)]
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_event_predicate_binding_support.rs"]
mod predicate_binding_support;
#[allow(dead_code)]
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_common_plane_reduced_operand_pair_support.rs"]
mod reduced_pair_support;
#[allow(dead_code)]
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_loop_reconstruction_workload_evidence_support.rs"]
mod workload_evidence_support;

pub(super) use continuation_contract_support::assert_loop_reconstruction_continuation_contract_preserves_real_neighborhoods_and_ordering;
pub(super) use edge_splitting_replay_parity_support::{
    build_edge_split_replay_parity_subject, replay_parity_report, EdgeSplitReplayParitySubject,
};
pub(super) use metaboss_support::MetabossEventExtractionSubject;
pub(super) use workload_evidence_support::CertifiedLoopReplayCloseoutChain;

use worth_kernel::workload_composition::{
    CompletedBooleanLoopReconstructionHandoff, CompletedBooleanLoopReconstructionProducts,
};
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanEdgeSplitReplayParityReceipt, PlanarBooleanLoopReconstructionSplitConsumption,
    PlanarBooleanLoopReconstructionSplitConsumptionInput,
};
use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanClosedWalkCandidateAssembly, PlanarBooleanClosedWalkCandidateSetInput,
    PlanarBooleanDegenerateLoopOutcomeBoundary, PlanarBooleanDegenerateLoopOutcomeBoundaryInput,
    PlanarBooleanDegenerateLoopOutcomeSet, PlanarBooleanFragmentContinuationIndex,
    PlanarBooleanFragmentContinuationIndexInput, PlanarBooleanLoopCandidateBoundary,
    PlanarBooleanLoopCandidateBoundaryInput, PlanarBooleanLoopContainmentEvidencePostureSet,
    PlanarBooleanLoopIslandPartition, PlanarBooleanLoopIslandPartitionInput,
    PlanarBooleanLoopReconstructionRequest, PlanarBooleanLoopReconstructionRequestInput,
    PlanarBooleanLoopRoleOutcomeBoundary, PlanarBooleanLoopRoleOutcomeBoundaryInput,
    PlanarBooleanLoopRoleOutcomeSet, PlanarBooleanLoopSourceProvenanceBundle,
    PlanarBooleanLoopSourceProvenanceRecoveryInput, PlanarBooleanReconstructedLoopBoundary,
    PlanarBooleanReconstructedLoopBoundaryInput, PlanarBooleanSourceLoopSplitAttribution,
    PlanarBooleanSourceLoopSplitAttributionInput, PlanarBooleanWalkOutcomeSet,
    PlanarBooleanWalkOutcomeSetInput,
};

#[derive(Clone, Copy)]
pub(super) enum ReplayBranch {
    Original,
    Replayed,
}

pub(super) struct RealLoopCandidateBoundaryProducts {
    pub(super) request: PlanarBooleanLoopReconstructionRequest,
    pub(super) source_provenance: PlanarBooleanLoopSourceProvenanceBundle,
    pub(super) continuation_index: PlanarBooleanFragmentContinuationIndex,
    pub(super) split_fragments:
        worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeFragmentSet,
    pub(super) walk_candidate_assembly: PlanarBooleanClosedWalkCandidateAssembly,
    pub(super) walk_outcomes: PlanarBooleanWalkOutcomeSet,
    pub(super) boundary: PlanarBooleanLoopCandidateBoundary,
}

pub(super) struct RealReconstructedLoopProducts {
    pub(super) candidate: RealLoopCandidateBoundaryProducts,
    pub(super) boundary: PlanarBooleanReconstructedLoopBoundary,
    pub(super) partition: PlanarBooleanLoopIslandPartition,
    pub(super) split_attribution: PlanarBooleanSourceLoopSplitAttribution,
}

pub(super) struct RealLoopRoleBoundaryProducts {
    pub(super) reconstructed: RealReconstructedLoopProducts,
    pub(super) role_outcomes: PlanarBooleanLoopRoleOutcomeSet,
    pub(super) containment_postures: PlanarBooleanLoopContainmentEvidencePostureSet,
}

pub(super) struct RealDegenerateLoopBoundaryProducts {
    pub(super) role_products: RealLoopRoleBoundaryProducts,
    pub(super) boundary: PlanarBooleanDegenerateLoopOutcomeBoundary,
    pub(super) outcomes: PlanarBooleanDegenerateLoopOutcomeSet,
}

pub(super) fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("loop-reconstruction-public-contracts".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("loop reconstruction public contract thread should spawn")
        .join()
        .expect("loop reconstruction public contract thread should finish");
}

pub(super) fn completed_loop_handoff(
    label: &'static str,
    branch: ReplayBranch,
) -> CompletedBooleanLoopReconstructionHandoff {
    let branch = match branch {
        ReplayBranch::Original => workload_evidence_support::ReplayBranch::Original,
        ReplayBranch::Replayed => workload_evidence_support::ReplayBranch::Replayed,
    };
    workload_evidence_support::certified_real_loop_handoff(label, branch)
        .expect("loop reconstruction public contract should certify through the real closeout seam")
}

pub(super) fn completed_loop_products(
    label: &'static str,
    branch: ReplayBranch,
) -> CompletedBooleanLoopReconstructionProducts {
    completed_loop_handoff(label, branch)
        .products()
        .cloned()
        .expect("real loop reconstruction handoff should retain canonical phase products")
}

pub(super) fn completed_loop_replay_closeout_chain(
    label: &'static str,
) -> CertifiedLoopReplayCloseoutChain {
    workload_evidence_support::certified_real_loop_replay_closeout_chain(label)
}

pub(super) fn real_loop_candidate_boundary(
    subject: &MetabossEventExtractionSubject,
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_parity_receipt: &PlanarBooleanEdgeSplitReplayParityReceipt,
    branch: ReplayBranch,
) -> RealLoopCandidateBoundaryProducts {
    let completed_split_handoff =
        continuation_contract_support::completed_split_handoff_for(subject, replay_subject);
    let (
        decision_log_receipt,
        validation,
        naming,
        ledger_result,
        vertices,
        fragments,
        chains,
        split_request,
    ) = match branch {
        ReplayBranch::Original => (
            replay_subject.original_decision_log.receipt(),
            &replay_subject.original_products.validation,
            &replay_subject.original_products.naming,
            &replay_subject.original_ledger,
            &replay_subject.original_products.vertices,
            &replay_subject.original_products.fragments,
            &replay_subject.original_products.chains,
            &replay_subject.original_products.request,
        ),
        ReplayBranch::Replayed => (
            replay_subject.replayed_decision_log.receipt(),
            &replay_subject.replayed_products.validation,
            &replay_subject.replayed_products.naming,
            &replay_subject.replayed_ledger,
            &replay_subject.replayed_products.vertices,
            &replay_subject.replayed_products.fragments,
            &replay_subject.replayed_products.chains,
            &replay_subject.replayed_products.request,
        ),
    };
    let downstream_consumption = completed_split_handoff
        .admit_downstream_split_consumption(
            decision_log_receipt,
            validation,
            naming,
            replay_parity_receipt,
        )
        .expect("real split evidence should admit downstream split consumption");
    let loop_split_consumption = PlanarBooleanLoopReconstructionSplitConsumption::admit(
        PlanarBooleanLoopReconstructionSplitConsumptionInput::from_downstream_split_consumption(
            &downstream_consumption,
        ),
    )
    .expect("loop reconstruction should consume the real downstream split product");
    let loop_request = PlanarBooleanLoopReconstructionRequest::admit(
        PlanarBooleanLoopReconstructionRequestInput::from_split_consumption(
            &loop_split_consumption,
        ),
    )
    .expect("loop reconstruction request should admit from the real loop split consumption");
    let recovered_source_carriers =
        continuation_contract_support::recovered_source_carriers(subject, split_request);
    let source_provenance = PlanarBooleanLoopSourceProvenanceBundle::recover(
        PlanarBooleanLoopSourceProvenanceRecoveryInput::from_request_and_split_support(
            &loop_request,
            ledger_result.ledger(),
            ledger_result.receipt(),
            &recovered_source_carriers,
            fragments,
            chains,
        ),
    )
    .expect("loop source provenance should recover from the real split chain");
    let continuation_index = PlanarBooleanFragmentContinuationIndex::admit(
        PlanarBooleanFragmentContinuationIndexInput::from_request_and_provenance(
            &loop_request,
            &source_provenance,
            vertices,
            fragments,
            chains,
        ),
    )
    .expect("continuation index should admit from real loop reconstruction support");
    let walk_candidate_assembly = PlanarBooleanClosedWalkCandidateAssembly::assemble(
        PlanarBooleanClosedWalkCandidateSetInput::from_continuation_index(&continuation_index),
    );
    let walk_outcomes = PlanarBooleanWalkOutcomeSet::classify(
        PlanarBooleanWalkOutcomeSetInput::from_closed_walk_candidates(
            walk_candidate_assembly.closed_walk_candidates(),
            walk_candidate_assembly.fragment_consumption_proof(),
        ),
    );
    let boundary = PlanarBooleanLoopCandidateBoundary::promote(
        PlanarBooleanLoopCandidateBoundaryInput::from_walk_outcomes(&walk_outcomes),
    );

    RealLoopCandidateBoundaryProducts {
        request: loop_request,
        source_provenance,
        continuation_index,
        split_fragments: fragments.clone(),
        walk_candidate_assembly,
        walk_outcomes,
        boundary,
    }
}

pub(super) fn real_reconstructed_loop_products(
    subject: &MetabossEventExtractionSubject,
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_parity_receipt: &PlanarBooleanEdgeSplitReplayParityReceipt,
    branch: ReplayBranch,
) -> RealReconstructedLoopProducts {
    let reconstructed =
        real_loop_candidate_boundary(subject, replay_subject, replay_parity_receipt, branch);
    let loop_boundary = PlanarBooleanReconstructedLoopBoundary::admit(
        PlanarBooleanReconstructedLoopBoundaryInput::from_loop_candidates_and_provenance(
            reconstructed.boundary.loop_candidates(),
            &reconstructed.source_provenance,
        ),
    )
    .expect("real loop candidates should reconstruct through the public phase-ten boundary");
    let partition = PlanarBooleanLoopIslandPartition::partition(
        PlanarBooleanLoopIslandPartitionInput::from_reconstructed_loop_boundary(
            loop_boundary.reconstructed_loops(),
            loop_boundary.born_loops(),
        ),
    );
    let split_attribution = PlanarBooleanSourceLoopSplitAttribution::attribute(
        PlanarBooleanSourceLoopSplitAttributionInput::from_island_partition(&partition),
    );
    RealReconstructedLoopProducts {
        candidate: reconstructed,
        boundary: loop_boundary,
        partition,
        split_attribution,
    }
}

pub(super) fn real_loop_role_boundary_products(
    subject: &MetabossEventExtractionSubject,
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_parity_receipt: &PlanarBooleanEdgeSplitReplayParityReceipt,
    branch: ReplayBranch,
) -> RealLoopRoleBoundaryProducts {
    let reconstructed =
        real_reconstructed_loop_products(subject, replay_subject, replay_parity_receipt, branch);
    let role_boundary = PlanarBooleanLoopRoleOutcomeBoundary::classify(
        PlanarBooleanLoopRoleOutcomeBoundaryInput::from_reconstructed_loop_products_and_provenance(
            &reconstructed.boundary,
            &reconstructed.partition,
            &reconstructed.split_attribution,
            &reconstructed.candidate.source_provenance,
        ),
    );
    RealLoopRoleBoundaryProducts {
        reconstructed,
        role_outcomes: role_boundary.role_outcomes().clone(),
        containment_postures: role_boundary.containment_evidence_postures().clone(),
    }
}

pub(super) fn real_degenerate_loop_boundary_products(
    subject: &MetabossEventExtractionSubject,
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_parity_receipt: &PlanarBooleanEdgeSplitReplayParityReceipt,
    branch: ReplayBranch,
) -> RealDegenerateLoopBoundaryProducts {
    let role_products =
        real_loop_role_boundary_products(subject, replay_subject, replay_parity_receipt, branch);
    let boundary = PlanarBooleanDegenerateLoopOutcomeBoundary::classify(
        PlanarBooleanDegenerateLoopOutcomeBoundaryInput::from_reconstructed_products_and_role_evidence(
            role_products.reconstructed.boundary.reconstructed_loops(),
            role_products.reconstructed.boundary.born_loops(),
            &role_products.role_outcomes,
            &role_products.containment_postures,
            role_products
                .reconstructed
                .candidate
                .source_provenance
                .source_loop_carriers(),
            &role_products.reconstructed.candidate.split_fragments,
        ),
    );
    RealDegenerateLoopBoundaryProducts {
        role_products,
        boundary: boundary.clone(),
        outcomes: boundary.outcomes().clone(),
    }
}

pub(super) fn assert_honest_promotion_partition(
    products: &CompletedBooleanLoopReconstructionProducts,
) {
    assert!(
        !products
            .walk_candidate_assembly()
            .closed_walk_candidates()
            .rows()
            .is_empty(),
        "real metaboss subject should assemble at least one closed-walk candidate"
    );
    assert_eq!(
        products
            .walk_candidate_assembly()
            .closed_walk_candidates()
            .counters()
            .walk_candidates_assembled(),
        products
            .walk_candidate_assembly()
            .closed_walk_candidates()
            .rows()
            .len()
    );
    assert_eq!(
        products.walk_outcomes().counters().walks_classified(),
        products.walk_outcomes().rows().len()
    );
    assert_eq!(
        products
            .candidate_boundary()
            .counters()
            .closed_walks_considered(),
        products.walk_outcomes().closed_rows().count()
    );
    assert_eq!(
        products
            .candidate_boundary()
            .counters()
            .closed_walks_considered(),
        products.candidate_boundary().loop_candidates().rows().len()
            + products
                .candidate_boundary()
                .denied_loop_candidates()
                .rows()
                .len()
    );

    for loop_candidate in products.candidate_boundary().loop_candidates().rows() {
        let walk_row = products
            .walk_outcomes()
            .rows()
            .iter()
            .find(|row| row.walk_outcome_identity() == loop_candidate.walk_outcome_identity())
            .expect("every admitted loop candidate should point back to a classified walk outcome");
        assert_eq!(
            walk_row.source_loop_identity(),
            loop_candidate.source_loop_identity()
        );
        assert_eq!(
            walk_row.fragment_identities(),
            loop_candidate.fragment_identities()
        );
        assert_eq!(walk_row.source_face_identities().len(), 1);
        assert_eq!(walk_row.local_frame_identities().len(), 1);
        assert_eq!(walk_row.precision_basis_identities().len(), 1);
        assert!(walk_row.fragment_identities().len() >= 2);
    }

    for denied_candidate in products
        .candidate_boundary()
        .denied_loop_candidates()
        .rows()
    {
        let walk_row = products
            .walk_outcomes()
            .rows()
            .iter()
            .find(|row| row.walk_outcome_identity() == denied_candidate.walk_outcome_identity())
            .expect("every denied loop candidate should point back to a classified walk outcome");
        assert_eq!(
            walk_row.source_loop_identity(),
            denied_candidate.source_loop_identity()
        );
        assert_eq!(
            walk_row.fragment_identities(),
            denied_candidate.fragment_identities()
        );
    }
}
