use super::*;

#[test]
fn closeout_binds_full_conflict_authority_chain() {
    let closeout = current_worth_touched_graph_conflict_public_closeout()
        .expect("current public closeout should publish from real proof products");
    let selected_route_packet =
        crate::workload_composition::current_worth_touched_graph_conflict_selected_route_packet()
            .expect("current selected-route packet should build");
    let cutover =
        current_worth_workload_ordinary_consumer_cutover().expect("current cutover should build");

    assert_eq!(
        closeout.proof_chain().selected_batch_plan_digest(),
        closeout
            .milestone_fifteen_seed()
            .selected_batch_plan_digest()
    );
    assert_eq!(
        closeout.proof_chain().batch_execution_receipt_digest(),
        closeout
            .milestone_fifteen_seed()
            .batch_execution_receipt_digest()
    );
    assert_eq!(
        closeout.proof_chain().selected_route_packet_digest(),
        selected_route_packet.packet_digest()
    );
    assert_eq!(
        closeout.proof_chain().selected_route_identity_digest(),
        selected_route_packet.selected_route_identity_digest()
    );
    assert_eq!(
        closeout.proof_chain().selected_conflict_plan_digests(),
        cutover
            .batch_execution_receipt()
            .selected_conflict_plan_digests()
    );
    assert_eq!(
        closeout.proof_chain().independence_proof_digests(),
        cutover
            .batch_execution_receipt()
            .independence_proof_identities()
    );
    let route_authority_digests = cutover
        .rows()
        .iter()
        .filter(|row| {
            row.posture()
                == crate::workload_composition::worth_workload::WorthWorkloadOrdinaryConsumerCutoverPosture::SelectedPlanDrivenOrdinaryConsumer
        })
        .map(|row| {
            row.selected_plan_witness()
                .expect("selected-plan ordinary consumer rows should carry a bound receipt witness")
                .route_authority_digest()
        })
        .collect::<Vec<_>>();
    assert_eq!(route_authority_digests.len(), 3);
    assert_ne!(route_authority_digests[0], route_authority_digests[1]);
    assert_ne!(route_authority_digests[1], route_authority_digests[2]);
    for row in cutover.rows().iter().filter(|row| {
        row.posture()
            == crate::workload_composition::worth_workload::WorthWorkloadOrdinaryConsumerCutoverPosture::SelectedPlanDrivenOrdinaryConsumer
    }) {
        assert_eq!(
            row.selected_plan_witness()
                .expect("selected-plan ordinary consumer rows should carry a bound receipt witness")
                .batch_execution_receipt_digest(),
            closeout.proof_chain().batch_execution_receipt_digest()
        );
        assert!(!row
            .selected_plan_witness()
            .expect("selected-plan ordinary consumer rows should carry a bound receipt witness")
            .route_lineage_digest()
            .is_empty());
        assert!(!row
            .selected_plan_witness()
            .expect("selected-plan ordinary consumer rows should carry a bound receipt witness")
            .route_authority_digest()
            .is_empty());
    }
    let replay_undo_row = cutover
        .rows()
        .iter()
        .find(|row| row.surface_name() == "admit_boolean_split_replay_undo_boundary")
        .expect("replay/undo selected-plan row should exist");
    let replay_undo_witness = replay_undo_row
        .selected_plan_witness()
        .expect("replay/undo selected-plan row should carry a bound proof witness");
    assert!(replay_undo_witness
        .replay_undo_boundary_proof_digest()
        .is_some_and(|value| !value.is_empty()));
    assert!(replay_undo_witness
        .transaction_packet_identity()
        .is_some_and(|value| !value.is_empty()));
    assert!(replay_undo_witness
        .replay_scope_identity()
        .is_some_and(|value| !value.is_empty()));
    assert!(replay_undo_witness
        .undo_scope_identity()
        .is_some_and(|value| !value.is_empty()));
    assert_eq!(
        closeout.proof_chain().replay_undo_boundary_proof_digests(),
        &[replay_undo_witness
            .replay_undo_boundary_proof_digest()
            .expect("replay/undo proof digest should survive into public closeout")
            .to_string()]
    );
    assert_eq!(
        closeout.proof_chain().transaction_packet_identities(),
        &[replay_undo_witness
            .transaction_packet_identity()
            .expect("replay/undo packet identity should survive into public closeout")
            .to_string()]
    );
    assert_eq!(
        closeout.proof_chain().replay_scope_identities(),
        &[replay_undo_witness
            .replay_scope_identity()
            .expect("replay scope identity should survive into public closeout")
            .to_string()]
    );
    assert_eq!(
        closeout.proof_chain().undo_scope_identities(),
        &[replay_undo_witness
            .undo_scope_identity()
            .expect("undo scope identity should survive into public closeout")
            .to_string()]
    );
    assert_eq!(
        closeout.source_firewall_digest(),
        current_worth_touched_graph_conflict_source_firewall_report()
            .expect("current firewall report")
            .report_digest()
    );
    assert_eq!(
        closeout.deletion_closeout_digest(),
        current_worth_touched_graph_conflict_deletion_closeout()
            .expect("current deletion closeout")
            .closeout_digest()
    );
    let spatial_route_packet =
        worth_spatial::facade::planner_owned_routing::evidence_lookup_route::current_evidence_lookup_route_packet()
            .expect("current planner-owned evidence lookup route packet");
    let spatial_route_projection_markers =
        crate::workload_composition::planner_owned_routing::SpatialRouteProjectionMarkers::from_route_packet(
            &spatial_route_packet,
        );
    assert_eq!(
        closeout
            .proof_chain()
            .evidence_lookup_public_closeout_digest(),
        spatial_route_projection_markers.evidence_lookup_public_closeout_digest()
    );
    assert_eq!(
        closeout
            .proof_chain()
            .evidence_lookup_family_coverage_digest(),
        spatial_route_projection_markers.evidence_lookup_family_coverage_digest()
    );
    assert_eq!(
        closeout
            .proof_chain()
            .evidence_lookup_query_surface_matrix_digest(),
        spatial_route_projection_markers.evidence_lookup_query_surface_matrix_digest()
    );
    assert_eq!(
        closeout
            .proof_chain()
            .evidence_lookup_query_consumer_kit_digest(),
        spatial_route_projection_markers.evidence_lookup_query_consumer_kit_digest()
    );
    assert_eq!(
        closeout
            .proof_chain()
            .evidence_lookup_query_boundary_support_digest(),
        spatial_route_projection_markers.evidence_lookup_query_boundary_support_digest()
    );
    let topology_query_backed_cutover = current_topology_query_backed_consumer_cutover()
        .expect("current topology query-backed cutover");
    assert_eq!(
        closeout
            .proof_chain()
            .topology_query_backed_consumer_cutover_digest(),
        topology_query_backed_cutover.closeout_digest()
    );
    assert_eq!(
        closeout
            .proof_chain()
            .topology_query_handle_identity_digest(),
        topology_query_backed_cutover.handle_identity_digest()
    );
    assert_eq!(
        closeout
            .proof_chain()
            .topology_query_operating_context_identity_digest(),
        topology_query_backed_cutover.operating_context_identity_digest()
    );
    assert_eq!(
        closeout
            .proof_chain()
            .topology_query_support_snapshot_digest(),
        topology_query_backed_cutover.support_snapshot_digest()
    );
    let loop_cycle_row = topology_query_backed_cutover
        .family_rows()
        .iter()
        .find(|row| row.request_family() == TopologyReadRequestFamily::LoopCycleNeighborhood)
        .expect("loop-cycle family row");
    assert_eq!(
        closeout
            .proof_chain()
            .topology_query_compiled_product_identity_digest(),
        loop_cycle_row
            .compiled_product_identity_digest()
            .expect("loop-cycle row should carry a compiled-product identity")
    );
    assert_eq!(
        closeout
            .proof_chain()
            .topology_query_equivalence_policy_identity_digest(),
        loop_cycle_row
            .equivalence_policy_identity_digest()
            .expect("loop-cycle row should carry an equivalence-policy identity")
    );
    assert_eq!(
        closeout
            .proof_chain()
            .topology_query_public_read_family_row_digest(),
        loop_cycle_row.row_digest()
    );
    assert_eq!(
        closeout
            .proof_chain()
            .topology_query_selected_equivalence_family_identity(),
        loop_cycle_row
            .selected_equivalence_family_identity()
            .expect("loop-cycle row should carry a selected equivalence family")
    );
    assert_eq!(
        closeout
            .proof_chain()
            .topology_query_selected_equivalence_basis_identity_digest(),
        loop_cycle_row
            .selected_equivalence_basis_identity_digest()
            .expect("loop-cycle row should carry a selected equivalence basis")
    );
    assert_eq!(
        closeout
            .proof_chain()
            .topology_query_selected_compatibility_basis_identity_digest(),
        loop_cycle_row
            .selected_compatibility_basis_identity_digest()
            .expect("loop-cycle row should carry a selected compatibility basis")
    );
    assert_eq!(
        closeout
            .proof_chain()
            .topology_query_selected_reuse_basis_identity_digest(),
        loop_cycle_row
            .selected_reuse_basis_identity_digest()
            .expect("loop-cycle row should carry a selected reuse basis")
    );
    assert_eq!(
        closeout
            .proof_chain()
            .topology_query_reuse_decision_identity_digest(),
        loop_cycle_row.reuse_decision_identity_digest()
    );
    assert_eq!(
        closeout
            .proof_chain()
            .topology_query_rebuild_denial_identity_digest(),
        loop_cycle_row.rebuild_denial_identity_digest()
    );
    assert!(!closeout.closeout_digest().is_empty());
    assert!(closeout
        .architecture_alignment_report()
        .milestone_fifteen_ready());
}

#[test]
fn public_closeout_rejects_foreign_admitted_public_proof_input() {
    let components = current_public_closeout_components().expect("current closeout components");
    let hostile_public_proof_input = hostile_public_proof_input_with_foreign_reuse_basis(
        "foreign-topology-selected-reuse-basis",
    );

    let error = publish_from_parts(
        components.input().expect("current closeout input"),
        components.cutover(),
        components.selected_route_packet(),
        &hostile_public_proof_input,
    )
    .expect_err("public closeout must reject admitted public proof input that disagrees with the selected-route packet");

    assert_eq!(
        error.kind(),
        WorthTouchedGraphConflictPublicCloseoutErrorKind::IncompleteProofChain
    );
    assert!(error.detail().contains(
        "Milestone 15 planner proof input must preserve selected-route packet topology authority"
    ));
}
