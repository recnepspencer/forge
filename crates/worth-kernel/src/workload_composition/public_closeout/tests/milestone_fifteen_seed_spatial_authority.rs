use super::*;

fn hostile_public_proof_input_with_foreign_spatial_reuse_decision(
    foreign_spatial_reuse_decision_identity_digest: &str,
) -> WorthTouchedGraphConflictAdmittedPublicProofInput {
    let packet = current_worth_touched_graph_conflict_selected_route_packet()
        .expect("current selected-route packet should build");
    let current_input = admit_worth_touched_graph_conflict_public_proof_input(&packet)
        .expect("current admitted public proof input should lower");
    WorthTouchedGraphConflictAdmittedPublicProofInput::from_parts(
        current_input.selected_route_packet_digest().to_string(),
        current_input.selected_route_identity_digest().to_string(),
        current_input
            .batch_admission_route_packet_identity()
            .to_string(),
        current_input
            .batch_admission_denial_witness_identity()
            .map(str::to_string),
        current_input.batch_admission_denial_witness_kind(),
        current_input
            .conflict_independence_route_packet_identity()
            .to_string(),
        current_input
            .conflict_independence_denial_witness_identity()
            .map(str::to_string),
        current_input.conflict_independence_denial_witness_kind(),
        current_input
            .replay_undo_route_packet_identity()
            .to_string(),
        current_input.replay_undo_route_family(),
        current_input.selected_family_identity().to_string(),
        current_input.selected_product_identity_digest().to_string(),
        current_input
            .compiled_product_reuse_route_packet_identity()
            .to_string(),
        current_input
            .topology_reuse_posture()
            .expect("current topology reuse posture"),
        current_input
            .spatial_reuse_posture()
            .expect("current spatial reuse posture"),
        current_input
            .selected_reuse_basis_identity_digest()
            .to_string(),
        current_input
            .selected_witness_identity_digest()
            .map(str::to_string),
        Some(foreign_spatial_reuse_decision_identity_digest.to_string()),
        current_input
            .rebuild_denial_identity_digest()
            .map(str::to_string),
        current_input
            .spatial_rebuild_denial_identity_digest()
            .map(str::to_string),
        current_input.spatial_selected_family_identity().to_string(),
        current_input
            .spatial_selected_product_identity_digest()
            .to_string(),
        current_input
            .spatial_equivalence_policy_identity_digest()
            .to_string(),
        current_input.topology_freshness_requirement_posture(),
        current_input.topology_rendered_output_comparison_posture(),
        current_input.spatial_freshness_requirement_posture(),
        current_input.spatial_rendered_output_comparison_posture(),
        current_input.topology_query_execution_count(),
        current_input.topology_row_scan_fallback_count(),
        current_input.topology_whole_view_fallback_count(),
        current_input.topology_repeated_rediscovery_denied_count(),
        current_input.spatial_receipt_proof_row_count(),
        current_input.spatial_non_ordinary_residue_row_count(),
    )
}

fn hostile_public_proof_input_with_foreign_spatial_rebuild_denial(
    foreign_spatial_rebuild_denial_identity_digest: &str,
) -> WorthTouchedGraphConflictAdmittedPublicProofInput {
    let packet = current_worth_touched_graph_conflict_selected_route_packet()
        .expect("current selected-route packet should build");
    let current_input = admit_worth_touched_graph_conflict_public_proof_input(&packet)
        .expect("current admitted public proof input should lower");
    WorthTouchedGraphConflictAdmittedPublicProofInput::from_parts(
        current_input.selected_route_packet_digest().to_string(),
        current_input.selected_route_identity_digest().to_string(),
        current_input
            .batch_admission_route_packet_identity()
            .to_string(),
        current_input
            .batch_admission_denial_witness_identity()
            .map(str::to_string),
        current_input.batch_admission_denial_witness_kind(),
        current_input
            .conflict_independence_route_packet_identity()
            .to_string(),
        current_input
            .conflict_independence_denial_witness_identity()
            .map(str::to_string),
        current_input.conflict_independence_denial_witness_kind(),
        current_input
            .replay_undo_route_packet_identity()
            .to_string(),
        current_input.replay_undo_route_family(),
        current_input.selected_family_identity().to_string(),
        current_input.selected_product_identity_digest().to_string(),
        current_input
            .compiled_product_reuse_route_packet_identity()
            .to_string(),
        current_input
            .topology_reuse_posture()
            .expect("current topology reuse posture"),
        current_input
            .spatial_reuse_posture()
            .expect("current spatial reuse posture"),
        current_input
            .selected_reuse_basis_identity_digest()
            .to_string(),
        current_input
            .selected_witness_identity_digest()
            .map(str::to_string),
        current_input
            .spatial_reuse_decision_identity_digest()
            .map(str::to_string),
        current_input
            .rebuild_denial_identity_digest()
            .map(str::to_string),
        Some(foreign_spatial_rebuild_denial_identity_digest.to_string()),
        current_input.spatial_selected_family_identity().to_string(),
        current_input
            .spatial_selected_product_identity_digest()
            .to_string(),
        current_input
            .spatial_equivalence_policy_identity_digest()
            .to_string(),
        current_input.topology_freshness_requirement_posture(),
        current_input.topology_rendered_output_comparison_posture(),
        current_input.spatial_freshness_requirement_posture(),
        current_input.spatial_rendered_output_comparison_posture(),
        current_input.topology_query_execution_count(),
        current_input.topology_row_scan_fallback_count(),
        current_input.topology_whole_view_fallback_count(),
        current_input.topology_repeated_rediscovery_denied_count(),
        current_input.spatial_receipt_proof_row_count(),
        current_input.spatial_non_ordinary_residue_row_count(),
    )
}

fn hostile_public_proof_input_with_foreign_reuse_route_packet_identity(
    foreign_compiled_product_reuse_route_packet_identity: &str,
) -> WorthTouchedGraphConflictAdmittedPublicProofInput {
    let packet = current_worth_touched_graph_conflict_selected_route_packet()
        .expect("current selected-route packet should build");
    let current_input = admit_worth_touched_graph_conflict_public_proof_input(&packet)
        .expect("current admitted public proof input should lower");
    WorthTouchedGraphConflictAdmittedPublicProofInput::from_parts(
        current_input.selected_route_packet_digest().to_string(),
        current_input.selected_route_identity_digest().to_string(),
        current_input
            .batch_admission_route_packet_identity()
            .to_string(),
        current_input
            .batch_admission_denial_witness_identity()
            .map(str::to_string),
        current_input.batch_admission_denial_witness_kind(),
        current_input
            .conflict_independence_route_packet_identity()
            .to_string(),
        current_input
            .conflict_independence_denial_witness_identity()
            .map(str::to_string),
        current_input.conflict_independence_denial_witness_kind(),
        current_input
            .replay_undo_route_packet_identity()
            .to_string(),
        current_input.replay_undo_route_family(),
        current_input.selected_family_identity().to_string(),
        current_input.selected_product_identity_digest().to_string(),
        foreign_compiled_product_reuse_route_packet_identity.to_string(),
        current_input
            .topology_reuse_posture()
            .expect("current topology reuse posture"),
        current_input
            .spatial_reuse_posture()
            .expect("current spatial reuse posture"),
        current_input
            .selected_reuse_basis_identity_digest()
            .to_string(),
        current_input
            .selected_witness_identity_digest()
            .map(str::to_string),
        current_input
            .spatial_reuse_decision_identity_digest()
            .map(str::to_string),
        current_input
            .rebuild_denial_identity_digest()
            .map(str::to_string),
        current_input
            .spatial_rebuild_denial_identity_digest()
            .map(str::to_string),
        current_input.spatial_selected_family_identity().to_string(),
        current_input
            .spatial_selected_product_identity_digest()
            .to_string(),
        current_input
            .spatial_equivalence_policy_identity_digest()
            .to_string(),
        current_input.topology_freshness_requirement_posture(),
        current_input.topology_rendered_output_comparison_posture(),
        current_input.spatial_freshness_requirement_posture(),
        current_input.spatial_rendered_output_comparison_posture(),
        current_input.topology_query_execution_count(),
        current_input.topology_row_scan_fallback_count(),
        current_input.topology_whole_view_fallback_count(),
        current_input.topology_repeated_rediscovery_denied_count(),
        current_input.spatial_receipt_proof_row_count(),
        current_input.spatial_non_ordinary_residue_row_count(),
    )
}

fn hostile_public_proof_input_with_foreign_spatial_route_tuple(
    foreign_spatial_selected_family_identity: &str,
    foreign_spatial_selected_product_identity_digest: &str,
    foreign_spatial_equivalence_policy_identity_digest: &str,
) -> WorthTouchedGraphConflictAdmittedPublicProofInput {
    let packet = current_worth_touched_graph_conflict_selected_route_packet()
        .expect("current selected-route packet should build");
    let current_input = admit_worth_touched_graph_conflict_public_proof_input(&packet)
        .expect("current admitted public proof input should lower");
    WorthTouchedGraphConflictAdmittedPublicProofInput::from_parts(
        current_input.selected_route_packet_digest().to_string(),
        current_input.selected_route_identity_digest().to_string(),
        current_input
            .batch_admission_route_packet_identity()
            .to_string(),
        current_input
            .batch_admission_denial_witness_identity()
            .map(str::to_string),
        current_input.batch_admission_denial_witness_kind(),
        current_input
            .conflict_independence_route_packet_identity()
            .to_string(),
        current_input
            .conflict_independence_denial_witness_identity()
            .map(str::to_string),
        current_input.conflict_independence_denial_witness_kind(),
        current_input
            .replay_undo_route_packet_identity()
            .to_string(),
        current_input.replay_undo_route_family(),
        current_input.selected_family_identity().to_string(),
        current_input.selected_product_identity_digest().to_string(),
        current_input
            .compiled_product_reuse_route_packet_identity()
            .to_string(),
        current_input
            .topology_reuse_posture()
            .expect("current topology reuse posture"),
        current_input
            .spatial_reuse_posture()
            .expect("current spatial reuse posture"),
        current_input
            .selected_reuse_basis_identity_digest()
            .to_string(),
        current_input
            .selected_witness_identity_digest()
            .map(str::to_string),
        current_input
            .spatial_reuse_decision_identity_digest()
            .map(str::to_string),
        current_input
            .rebuild_denial_identity_digest()
            .map(str::to_string),
        current_input
            .spatial_rebuild_denial_identity_digest()
            .map(str::to_string),
        foreign_spatial_selected_family_identity.to_string(),
        foreign_spatial_selected_product_identity_digest.to_string(),
        foreign_spatial_equivalence_policy_identity_digest.to_string(),
        current_input.topology_freshness_requirement_posture(),
        current_input.topology_rendered_output_comparison_posture(),
        current_input.spatial_freshness_requirement_posture(),
        current_input.spatial_rendered_output_comparison_posture(),
        current_input.topology_query_execution_count(),
        current_input.topology_row_scan_fallback_count(),
        current_input.topology_whole_view_fallback_count(),
        current_input.topology_repeated_rediscovery_denied_count(),
        current_input.spatial_receipt_proof_row_count(),
        current_input.spatial_non_ordinary_residue_row_count(),
    )
}

#[test]
fn milestone_fifteen_seed_rejects_foreign_spatial_reuse_witness_before_seed_construction() {
    let components = current_public_closeout_components().expect("current closeout components");
    let packet = current_worth_touched_graph_conflict_selected_route_packet()
        .expect("current selected-route packet should build");
    let hostile_public_proof_input = hostile_public_proof_input_with_foreign_spatial_reuse_decision(
        "foreign-spatial-reuse-decision",
    );

    let error = WorthTouchedGraphConflictMilestoneFifteenSeed::from_selected_route_packet(
        &packet,
        components.residue_chain().residue_digest(),
        packet.source_firewall_digest(),
        hostile_public_proof_input,
    )
    .expect_err("foreign spatial reuse witness must fail before seed construction");

    assert_eq!(
        error.kind(),
        WorthTouchedGraphConflictPublicCloseoutErrorKind::IncompleteProofChain
    );
    assert!(error.detail().contains(
        "Milestone 15 planner proof input must preserve selected-route packet reuse authority"
    ));
}

#[test]
fn milestone_fifteen_seed_rejects_foreign_spatial_rebuild_denial_before_seed_construction() {
    let components = current_public_closeout_components().expect("current closeout components");
    let packet = current_worth_touched_graph_conflict_selected_route_packet()
        .expect("current selected-route packet should build");
    let hostile_public_proof_input = hostile_public_proof_input_with_foreign_spatial_rebuild_denial(
        "foreign-spatial-rebuild-denial",
    );

    let error = WorthTouchedGraphConflictMilestoneFifteenSeed::from_selected_route_packet(
        &packet,
        components.residue_chain().residue_digest(),
        packet.source_firewall_digest(),
        hostile_public_proof_input,
    )
    .expect_err("foreign spatial rebuild denial witness must fail before seed construction");

    assert_eq!(
        error.kind(),
        WorthTouchedGraphConflictPublicCloseoutErrorKind::IncompleteProofChain
    );
    assert!(error.detail().contains(
        "Milestone 15 planner proof input must preserve selected-route packet reuse authority"
    ));
}

#[test]
fn milestone_fifteen_seed_rejects_foreign_reuse_route_packet_identity_before_seed_construction() {
    let components = current_public_closeout_components().expect("current closeout components");
    let packet = current_worth_touched_graph_conflict_selected_route_packet()
        .expect("current selected-route packet should build");
    let hostile_public_proof_input =
        hostile_public_proof_input_with_foreign_reuse_route_packet_identity(
            "foreign-compiled-product-reuse-route-packet",
        );

    let error = WorthTouchedGraphConflictMilestoneFifteenSeed::from_selected_route_packet(
        &packet,
        components.residue_chain().residue_digest(),
        packet.source_firewall_digest(),
        hostile_public_proof_input,
    )
    .expect_err("foreign reuse route packet identity must fail before seed construction");

    assert_eq!(
        error.kind(),
        WorthTouchedGraphConflictPublicCloseoutErrorKind::IncompleteProofChain
    );
    assert!(error.detail().contains(
        "Milestone 15 planner proof input must preserve selected-route packet reuse authority"
    ));
}

#[test]
fn milestone_fifteen_seed_rejects_foreign_spatial_route_tuple_before_seed_construction() {
    let components = current_public_closeout_components().expect("current closeout components");
    let packet = current_worth_touched_graph_conflict_selected_route_packet()
        .expect("current selected-route packet should build");
    let hostile_public_proof_input = hostile_public_proof_input_with_foreign_spatial_route_tuple(
        "foreign-spatial-selected-family",
        "foreign-spatial-selected-product",
        "foreign-spatial-equivalence-policy",
    );

    let error = WorthTouchedGraphConflictMilestoneFifteenSeed::from_selected_route_packet(
        &packet,
        components.residue_chain().residue_digest(),
        packet.source_firewall_digest(),
        hostile_public_proof_input,
    )
    .expect_err("foreign spatial route tuple must fail before seed construction");

    assert_eq!(
        error.kind(),
        WorthTouchedGraphConflictPublicCloseoutErrorKind::IncompleteProofChain
    );
    assert!(error.detail().contains(
        "Milestone 15 planner proof input must preserve selected-route packet reuse authority"
    ));
}
