#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_loop_reconstruction_continuation_contract_support/mod.rs"]
mod continuation_contract_support;
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_edge_splitting_decision_log_support.rs"]
mod edge_splitting_decision_log_support;
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_edge_splitting_endpoint_boundary_support.rs"]
mod edge_splitting_endpoint_boundary_support;
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_edge_splitting_interval_subdivision_support.rs"]
mod edge_splitting_interval_subdivision_support;
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_edge_splitting_persistent_naming_support.rs"]
mod edge_splitting_persistent_naming_support;
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_edge_splitting_raw_schedule_support.rs"]
mod edge_splitting_raw_schedule_support;
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_edge_splitting_replay_parity_support.rs"]
mod edge_splitting_replay_parity_support;
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_edge_splitting_split_vertex_identity_support.rs"]
mod edge_splitting_split_vertex_identity_support;
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_edge_splitting_support.rs"]
mod edge_splitting_support;
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_event_extraction_metaboss_support/mod.rs"]
mod metaboss_support;
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_loop_reconstruction_workload_evidence_support.rs"]
mod workload_evidence_support;

pub(super) use continuation_contract_support::assert_loop_reconstruction_continuation_contract_preserves_real_neighborhoods_and_ordering;
pub(super) use workload_evidence_support::{
    build_edge_split_replay_parity_subject, CertifiedLoopReplayCloseoutChain,
    MetabossEventExtractionSubject,
};

use worth_kernel::workload_composition::{
    CompletedBooleanLoopReconstructionHandoff, CompletedBooleanLoopReconstructionProducts,
};
use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanDegenerateLoopOutcomeSet, PlanarBooleanLoopIslandPartition,
    PlanarBooleanLoopRoleOutcomeSet, PlanarBooleanReconstructedLoopBoundary,
    PlanarBooleanSourceLoopSplitAttribution,
};

#[derive(Clone, Copy)]
pub(super) enum ReplayBranch {
    Original,
    Replayed,
}

pub(super) struct RealReconstructedLoopProducts {
    pub(super) boundary: PlanarBooleanReconstructedLoopBoundary,
    pub(super) partition: PlanarBooleanLoopIslandPartition,
    pub(super) split_attribution: PlanarBooleanSourceLoopSplitAttribution,
}

pub(super) struct RealLoopRoleBoundaryProducts {
    pub(super) reconstructed: RealReconstructedLoopProducts,
    pub(super) role_outcomes: PlanarBooleanLoopRoleOutcomeSet,
}

pub(super) struct RealDegenerateLoopBoundaryProducts {
    pub(super) role_products: RealLoopRoleBoundaryProducts,
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
