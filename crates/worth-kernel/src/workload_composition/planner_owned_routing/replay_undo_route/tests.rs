use schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoPlannerRouteFamily;
use schema::facade::platform::authority::replay_undo_semantic_graph::{
    admit_replay_scope_identity, ReplayScopeIdentityInput, ReplayUndoSemanticGraphEquivalenceBasis,
    ReplayUndoSemanticGraphTouchedSubject, ReplayUndoTransactionScopeClaim,
    ReplayUndoTransactionScopeKind,
};
use schema::facade::platform::authority::replay_undo_semantic_graph_internal::{
    admit_replay_undo_stage_index_identity, admit_spatial_evidence_lookup_prior_proof_identity,
};
use worth_spatial::facade::replay_undo_semantic_graph::current_boolean_split_spatial_boundary;

use super::current::{
    current_replay_undo_transaction_route_input_for_tests,
    current_replay_undo_transaction_route_packet,
    current_replay_undo_transaction_route_packet_with_input_override,
    current_replay_undo_undo_route_packet,
};
use crate::workload_composition::planner_owned_routing::{
    admitted_public_proof_input::current_worth_touched_graph_conflict_public_proof_input_with_packet_loader,
    current_worth_touched_graph_conflict_independence_route_packet,
    current_worth_touched_graph_conflict_selected_route_packet,
    selected_route::current_worth_touched_graph_conflict_selected_route_packet_with_route_loaders,
    PlannerOwnedRoutingErrorKind,
};
use crate::workload_composition::worth_workload::current_replay_undo_boundary_proof;

#[test]
fn transaction_and_undo_explanation_remain_distinct_planner_route_families() {
    let transaction_packet =
        current_replay_undo_transaction_route_packet().expect("transaction route packet");
    let undo_packet = current_replay_undo_undo_route_packet().expect("undo route packet");

    assert_eq!(
        transaction_packet.family(),
        ReplayUndoPlannerRouteFamily::Transaction
    );
    assert_eq!(undo_packet.family(), ReplayUndoPlannerRouteFamily::Undo);
    assert_ne!(
        transaction_packet.route_packet_identity(),
        undo_packet.route_packet_identity()
    );
}

#[test]
fn selected_route_and_public_proof_consume_the_same_replay_undo_route_product() {
    let route_packet =
        current_replay_undo_transaction_route_packet().expect("transaction route packet");
    let execution_proof = current_replay_undo_boundary_proof(
        &current_boolean_split_spatial_boundary().expect("current replay/undo split boundary"),
    )
    .expect("execution replay/undo boundary proof");
    let selected_route = current_worth_touched_graph_conflict_selected_route_packet()
        .expect("selected-route packet");
    let public_proof_input =
        crate::workload_composition::current_worth_touched_graph_conflict_public_proof_input()
            .expect("public proof input");

    assert_eq!(
        execution_proof.route_packet_identity(),
        route_packet.route_packet_identity()
    );
    assert_eq!(
        execution_proof.route_family(),
        ReplayUndoPlannerRouteFamily::Transaction
    );
    assert_eq!(
        selected_route.replay_undo_route_packet_identity(),
        route_packet.route_packet_identity()
    );
    assert_eq!(
        selected_route.replay_undo_route_family(),
        ReplayUndoPlannerRouteFamily::Transaction
    );
    assert_eq!(
        public_proof_input.replay_undo_route_packet_identity(),
        route_packet.route_packet_identity()
    );
    assert_eq!(
        public_proof_input.replay_undo_route_family(),
        ReplayUndoPlannerRouteFamily::Transaction
    );
}

#[test]
fn foreign_replay_scope_packets_deny_before_public_proof_assembly() {
    let current_input =
        current_replay_undo_transaction_route_input_for_tests().expect("current route input");
    let current_basis = current_input
        .replay_scope_identity()
        .equivalence_basis()
        .clone();
    let mut hostile_subjects = current_basis.touched_subjects().to_vec();
    hostile_subjects.push(
        ReplayUndoSemanticGraphTouchedSubject::TopologyRelationKind {
            relation_kind: "foreign-replay-route-proof".to_string(),
        },
    );
    let foreign_replay_scope = admit_replay_scope_identity(ReplayScopeIdentityInput::new(
        ReplayUndoSemanticGraphEquivalenceBasis::new(
            current_basis.locality_scope(),
            hostile_subjects,
            current_basis.prior_proof_identity().clone(),
            current_basis.stage_index_identity().cloned(),
        ),
    ));
    let error = current_worth_touched_graph_conflict_public_proof_input_with_packet_loader(|| {
        current_worth_touched_graph_conflict_selected_route_packet_with_route_loaders(
            topology::certification::current_topology_milestone_fifteen_planner_seed_support,
            worth_spatial::certification::current_spatial_milestone_fifteen_planner_seed_support,
            || {
                current_replay_undo_transaction_route_packet_with_input_override(|_| {
                    crate::replay_undo_transaction_boundary::ReplayUndoTransactionBoundaryInput::new(
                        current_input.touched_digest(),
                        admit_replay_undo_stage_index_identity(
                            current_input.stage_index_identity().digest(),
                        ),
                        current_input.invalidation_receipt_identity().clone(),
                        admit_spatial_evidence_lookup_prior_proof_identity(
                            current_input.evidence_lookup_receipt_identity().digest(),
                        ),
                        foreign_replay_scope.clone(),
                        current_input.undo_scope_identity().clone(),
                        current_input.support_posture().clone(),
                        current_input
                            .mutation_claims()
                            .iter()
                            .map(|claim| match claim.kind() {
                                ReplayUndoTransactionScopeKind::Replay => {
                                    ReplayUndoTransactionScopeClaim::new(
                                        ReplayUndoTransactionScopeKind::Replay,
                                        foreign_replay_scope.digest(),
                                    )
                                }
                                ReplayUndoTransactionScopeKind::Undo => claim.clone(),
                            })
                            .collect(),
                        current_input.counters().clone(),
                    )
                })
            },
            crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_batch_admission_route_packet,
            current_worth_touched_graph_conflict_independence_route_packet,
            crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_compiled_product_reuse_route_packet,
        )
    })
    .expect_err("foreign replay/undo packet should deny before public proof assembly");

    assert_eq!(
        error.kind(),
        PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport
    );
}
