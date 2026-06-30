use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanClosedWalkCandidateAssembly, PlanarBooleanClosedWalkCandidateSetInput,
    PlanarBooleanFragmentContinuationIndex, PlanarBooleanFragmentContinuationIndexInput,
    PlanarBooleanLoopCandidateBoundary, PlanarBooleanLoopCandidateBoundaryInput,
    PlanarBooleanLoopSourceProvenanceBundle, PlanarBooleanLoopSourceProvenanceRecoveryInput,
    PlanarBooleanWalkOutcomeSet, PlanarBooleanWalkOutcomeSetInput,
};

use super::continuation_contract_support::{
    completed_split_handoff_for, recovered_source_carriers,
};
use super::edge_splitting_replay_parity_support::{
    build_edge_split_replay_parity_subject, replay_parity_report,
};
use super::metaboss_support::MetabossEventExtractionSubject;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanLoopReconstructionSplitConsumption,
    PlanarBooleanLoopReconstructionSplitConsumptionInput,
};

pub(crate) fn assert_loop_reconstruction_candidate_contract_preserves_real_promotion_boundary() {
    let subject =
        MetabossEventExtractionSubject::certify("phase7.4 public loop candidate contract");
    let replay_subject = build_edge_split_replay_parity_subject(&subject);
    let replay_report = replay_parity_report(&replay_subject);
    let original_boundary = real_loop_candidate_boundary(
        &subject,
        &replay_subject,
        replay_report.receipt(),
        ReplayBranch::Original,
    );
    let replayed_boundary = real_loop_candidate_boundary(
        &subject,
        &replay_subject,
        replay_report.receipt(),
        ReplayBranch::Replayed,
    );

    assert_eq!(
        original_boundary
            .walk_candidate_assembly
            .closed_walk_candidates()
            .rows(),
        replayed_boundary
            .walk_candidate_assembly
            .closed_walk_candidates()
            .rows()
    );
    assert_eq!(
        original_boundary
            .walk_candidate_assembly
            .fragment_consumption_proof(),
        replayed_boundary
            .walk_candidate_assembly
            .fragment_consumption_proof()
    );
    assert_eq!(
        original_boundary.walk_outcomes.rows(),
        replayed_boundary.walk_outcomes.rows()
    );
    assert_eq!(
        original_boundary.boundary.loop_candidates().rows(),
        replayed_boundary.boundary.loop_candidates().rows()
    );
    assert_eq!(
        original_boundary.boundary.denied_loop_candidates().rows(),
        replayed_boundary.boundary.denied_loop_candidates().rows()
    );

    assert_honest_promotion_partition(&original_boundary);
    assert_honest_promotion_partition(&replayed_boundary);
}

enum ReplayBranch {
    Original,
    Replayed,
}

struct RealLoopCandidateBoundaryProducts {
    walk_candidate_assembly: PlanarBooleanClosedWalkCandidateAssembly,
    walk_outcomes: PlanarBooleanWalkOutcomeSet,
    boundary: PlanarBooleanLoopCandidateBoundary,
}

fn real_loop_candidate_boundary(
    subject: &MetabossEventExtractionSubject,
    replay_subject: &super::edge_splitting_replay_parity_support::EdgeSplitReplayParitySubject,
    replay_parity_receipt: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanEdgeSplitReplayParityReceipt,
    branch: ReplayBranch,
) -> RealLoopCandidateBoundaryProducts {
    let completed_split_handoff = completed_split_handoff_for(subject, replay_subject);
    let (decision_log, validation, naming, ledger, vertices, fragments, chains, request) =
        match branch {
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
        .admit_batch_execution_cluster()
        .expect("real split evidence should admit batch execution cluster")
        .admit_downstream_split_consumption(decision_log, validation, naming, replay_parity_receipt)
        .expect("real split evidence should admit downstream split consumption");
    let loop_split_consumption = PlanarBooleanLoopReconstructionSplitConsumption::admit(
        PlanarBooleanLoopReconstructionSplitConsumptionInput::from_downstream_split_consumption(
            &downstream_consumption,
        ),
    )
    .expect("loop reconstruction should consume the real downstream split product");
    let loop_request = worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanLoopReconstructionRequest::admit(
        worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanLoopReconstructionRequestInput::from_split_consumption(
            &loop_split_consumption,
        ),
    )
    .expect("loop reconstruction request should admit from the real loop split consumption");
    let recovered_source_carriers = recovered_source_carriers(subject, request);
    let source_provenance = PlanarBooleanLoopSourceProvenanceBundle::recover(
        PlanarBooleanLoopSourceProvenanceRecoveryInput::from_request_and_split_support(
            &loop_request,
            ledger.ledger(),
            ledger.receipt(),
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
        walk_candidate_assembly,
        walk_outcomes,
        boundary,
    }
}

fn assert_honest_promotion_partition(products: &RealLoopCandidateBoundaryProducts) {
    assert!(
        !products
            .walk_candidate_assembly
            .closed_walk_candidates()
            .rows()
            .is_empty(),
        "real metaboss subject should assemble at least one closed-walk candidate"
    );
    assert_eq!(
        products
            .walk_candidate_assembly
            .closed_walk_candidates()
            .counters()
            .walk_candidates_assembled(),
        products
            .walk_candidate_assembly
            .closed_walk_candidates()
            .rows()
            .len()
    );
    assert_eq!(
        products.walk_outcomes.counters().walks_classified(),
        products.walk_outcomes.rows().len()
    );
    assert_eq!(
        products.boundary.counters().closed_walks_considered(),
        products.walk_outcomes.closed_rows().count()
    );
    assert_eq!(
        products.boundary.counters().closed_walks_considered(),
        products.boundary.loop_candidates().rows().len()
            + products.boundary.denied_loop_candidates().rows().len()
    );

    for loop_candidate in products.boundary.loop_candidates().rows() {
        let walk_row = products
            .walk_outcomes
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

    for denied_candidate in products.boundary.denied_loop_candidates().rows() {
        let walk_row = products
            .walk_outcomes
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
