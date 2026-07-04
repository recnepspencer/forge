use super::builder::build_representative_selected_route_parity_path;
use super::consumer_step::RepresentativeSelectedRouteConsumerKind;
use super::current::current_representative_selected_route_parity_path;
use super::path::RepresentativeSelectedRouteParityPathErrorKind;
use crate::workload_composition::planner_owned_routing::{
    current_replay_undo_transaction_route_packet,
    current_worth_touched_graph_conflict_compiled_product_reuse_route_packet,
    current_worth_touched_graph_conflict_public_facade,
    current_worth_touched_graph_conflict_selected_route_packet,
};
use topology::facade::current_topology_query_backed_consumer_cutover;
use worth_spatial::facade::evidence_lookup_route::current_evidence_lookup_route_packet;

#[test]
fn representative_selected_route_path_preserves_one_authority_chain() {
    let path = current_representative_selected_route_parity_path()
        .expect("representative selected-route parity path");
    let query_step = path.query_backed_read();
    let evidence_step = path.evidence_lookup();
    let replay_step = path.replay_or_conflict();
    let reuse_step = path.compiled_product_reuse();
    let public_proof_step = path.public_proof_step();
    let diagnostic_step = path.diagnostic_step();

    assert_eq!(
        path.selected_route_identity_digest(),
        path.public_proof().selected_route_identity_digest()
    );
    assert_eq!(
        path.selected_family_identity(),
        path.public_proof().selected_family_identity()
    );
    assert_eq!(
        path.selected_product_identity_digest(),
        path.public_proof().selected_product_identity_digest()
    );
    assert_eq!(
        path.selected_witness_identity_digest(),
        path.public_proof().selected_witness_identity_digest()
    );
    assert_eq!(
        path.selected_witness_identity_digest(),
        path.derived_diagnostics()
            .selected_witness_identity_digest()
    );
    assert_eq!(
        path.residue_digest(),
        path.public_proof().residue_chain().residue_digest()
    );
    assert_eq!(
        path.source_firewall_digest(),
        path.public_proof().source_firewall_digest()
    );
    assert_eq!(
        query_step.cutover().closeout_digest(),
        path.public_proof()
            .milestone_fifteen_seed()
            .topology_query_backed_consumer_cutover_digest()
    );
    assert_eq!(
        query_step.selected_family_row().row_digest(),
        path.public_proof()
            .milestone_fifteen_seed()
            .topology_query_public_read_family_row_digest()
    );
    assert_eq!(
        query_step
            .selected_family_row()
            .selected_equivalence_family_identity(),
        Some(path.selected_family_identity())
    );
    assert_eq!(
        query_step
            .selected_family_row()
            .compiled_product_identity_digest(),
        Some(path.selected_product_identity_digest())
    );
    assert_eq!(
        query_step
            .selected_family_row()
            .selected_reuse_basis_identity_digest(),
        Some(
            path.public_proof()
                .milestone_fifteen_seed()
                .topology_query_selected_reuse_basis_identity_digest()
        )
    );
    assert_eq!(
        query_step.evidence_lookup_query_boundary_support_digest(),
        path.public_proof()
            .milestone_fifteen_seed()
            .evidence_lookup_query_boundary_support_digest()
    );
    assert_eq!(
        evidence_step.public_closeout_digest(),
        path.public_proof()
            .milestone_fifteen_seed()
            .evidence_lookup_public_closeout_digest()
    );
    assert_eq!(
        evidence_step
            .packet()
            .selected_equivalence_family_identity(),
        path.authority().spatial_selected_family_identity()
    );
    assert_eq!(
        evidence_step.packet().compiled_product_identity_digest(),
        path.authority().spatial_selected_product_identity_digest()
    );
    assert!(!evidence_step
        .packet()
        .selected_reuse_basis_identity_digest()
        .is_empty());
    assert_eq!(
        evidence_step.packet().query_support_digest(),
        path.authority().evidence_lookup_query_support_digest()
    );
    assert!(!evidence_step.packet().route_packet_digest().is_empty());
    assert!(!replay_step.route_authority_digest().is_empty());
    assert_eq!(
        replay_step.route_packet_identity(),
        path.authority().replay_undo_route_packet_identity()
    );
    assert_eq!(
        replay_step.family(),
        path.authority().replay_undo_route_family()
    );
    assert_eq!(
        reuse_step.packet().selected_reuse_basis_identity_digest(),
        path.public_proof()
            .milestone_fifteen_seed()
            .topology_query_selected_reuse_basis_identity_digest()
    );
    assert_eq!(
        reuse_step.packet().packet_identity(),
        path.authority()
            .compiled_product_reuse_route_packet_identity()
    );
    assert_eq!(
        reuse_step.packet().selected_family_identity(),
        path.selected_family_identity()
    );
    assert_eq!(
        reuse_step.packet().selected_product_identity_digest(),
        path.authority().spatial_selected_product_identity_digest()
    );
    assert_eq!(
        public_proof_step.inspection().proof_chain_digest(),
        path.public_proof().proof_chain_digest()
    );
    assert_eq!(
        diagnostic_step
            .projection()
            .selected_route_identity_digest(),
        path.selected_route_identity_digest()
    );
    assert_eq!(
        diagnostic_step.projection().selected_family_identity(),
        path.selected_family_identity()
    );
    assert_eq!(
        diagnostic_step
            .projection()
            .selected_product_identity_digest(),
        path.selected_product_identity_digest()
    );

    let consumer_kinds: Vec<_> = path.consumers().iter().map(|step| step.kind()).collect();
    assert!(consumer_kinds.contains(&RepresentativeSelectedRouteConsumerKind::QueryBackedRead));
    assert!(consumer_kinds.contains(&RepresentativeSelectedRouteConsumerKind::EvidenceLookup));
    assert!(consumer_kinds.contains(&RepresentativeSelectedRouteConsumerKind::ReplayOrConflict));
    assert!(consumer_kinds.contains(&RepresentativeSelectedRouteConsumerKind::CompiledProductReuse));
    assert!(consumer_kinds.contains(&RepresentativeSelectedRouteConsumerKind::PublicProof));
    assert!(consumer_kinds.contains(&RepresentativeSelectedRouteConsumerKind::Diagnostic));
}

#[test]
fn representative_selected_route_path_rejects_local_reassembly() {
    let selected_route_packet = current_worth_touched_graph_conflict_selected_route_packet()
        .expect("selected route packet")
        .with_test_selected_reuse_basis_identity_digest_override(
            "phase-4-local-helper-reassembled-selected-reuse-basis",
        );
    let public_facade =
        current_worth_touched_graph_conflict_public_facade().expect("public facade");
    let query_cutover = current_topology_query_backed_consumer_cutover().expect("query cutover");
    let evidence_route = current_evidence_lookup_route_packet().expect("evidence route");
    let replay_route = current_replay_undo_transaction_route_packet().expect("replay route");
    let reuse_route = current_worth_touched_graph_conflict_compiled_product_reuse_route_packet()
        .expect("reuse route");

    let error = build_representative_selected_route_parity_path(
        selected_route_packet,
        public_facade,
        query_cutover,
        evidence_route,
        replay_route,
        reuse_route,
    )
    .expect_err("local helper reassembly should be rejected");

    assert_eq!(
        error.kind(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedReuseIdentity
    );
}

#[test]
fn representative_selected_route_path_rejects_foreign_public_proof_witness_identity() {
    let selected_route_packet = current_worth_touched_graph_conflict_selected_route_packet()
        .expect("selected route packet");
    let public_facade = current_worth_touched_graph_conflict_public_facade()
        .expect("public facade")
        .with_test_public_proof_witness_identity_override(Some("foreign-public-proof-witness"));
    let query_cutover = current_topology_query_backed_consumer_cutover().expect("query cutover");
    let evidence_route = current_evidence_lookup_route_packet().expect("evidence route");
    let replay_route = current_replay_undo_transaction_route_packet().expect("replay route");
    let reuse_route = current_worth_touched_graph_conflict_compiled_product_reuse_route_packet()
        .expect("reuse route");

    let error = build_representative_selected_route_parity_path(
        selected_route_packet,
        public_facade,
        query_cutover,
        evidence_route,
        replay_route,
        reuse_route,
    )
    .expect_err("representative path must reject a foreign public-proof witness identity");

    assert_eq!(
        error.kind(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedSelectedWitnessIdentity
    );
}
