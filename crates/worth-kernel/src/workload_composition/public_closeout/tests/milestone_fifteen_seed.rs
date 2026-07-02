use std::collections::BTreeSet;

use super::*;

#[test]
fn milestone_fifteen_seed_carries_product_identity_without_rediscovery() {
    let closeout = current_worth_touched_graph_conflict_public_closeout()
        .expect("current public closeout should publish from real proof products");
    let packet =
        crate::workload_composition::current_worth_touched_graph_conflict_selected_route_packet()
            .expect("current selected-route packet should build");
    let seed = current_worth_touched_graph_conflict_milestone_fifteen_seed()
        .expect("current Milestone 15 seed should derive from the canonical closeout");
    let topology_support =
        topology::certification::current_topology_milestone_fifteen_planner_seed_support()
            .expect("current topology Milestone 15 support should build");
    let spatial_support =
        worth_spatial::certification::current_spatial_milestone_fifteen_planner_seed_support()
            .expect("current spatial Milestone 15 support should build");
    let cutover =
        current_worth_workload_ordinary_consumer_cutover().expect("current cutover should build");
    let receipt = cutover.batch_execution_receipt();

    assert_eq!(
        seed.overlap_identity_digests(),
        receipt.overlap_identity_digests()
    );
    assert_eq!(
        seed.locality_footprint_digests(),
        receipt.locality_footprint_digests()
    );
    assert_eq!(
        seed.selected_conflict_plan_digests(),
        receipt.selected_conflict_plan_digests()
    );
    assert_eq!(
        seed.independence_proof_digests(),
        receipt.independence_proof_identities()
    );
    assert_eq!(
        seed.selected_batch_plan_digest(),
        receipt.selected_batch_plan_digest()
    );
    assert_eq!(
        seed.batch_execution_receipt_digest(),
        receipt.execution_receipt_digest()
    );
    assert_eq!(
        seed.source_firewall_digest(),
        current_worth_touched_graph_conflict_source_firewall_report()
            .expect("current firewall report")
            .report_digest()
    );
    assert_eq!(
        seed.topology_query_selected_equivalence_family_identity(),
        packet.selected_family_identity()
    );
    assert_eq!(
        seed.topology_query_selected_reuse_basis_identity_digest(),
        packet.selected_reuse_basis_identity_digest()
    );
    let planner_input = seed.as_planner_proof_input();
    assert_eq!(
        planner_input.selected_equivalence_family_identity(),
        Some(
            closeout
                .proof_chain()
                .topology_query_selected_equivalence_family_identity()
        )
    );
    assert_eq!(
        planner_input.reuse_basis_identity_digest(),
        Some(
            closeout
                .proof_chain()
                .topology_query_selected_reuse_basis_identity_digest()
        )
    );
    assert_eq!(
        planner_input.reuse_decision_identity_digest(),
        closeout
            .proof_chain()
            .topology_query_reuse_decision_identity_digest()
    );
    assert_eq!(
        planner_input.rebuild_denial_identity_digest(),
        closeout
            .proof_chain()
            .topology_query_rebuild_denial_identity_digest()
    );
    assert_eq!(
        planner_input.compiled_product_reuse_route_packet_identity(),
        seed.compiled_product_reuse_route_packet_identity()
    );
    assert_eq!(
        planner_input.spatial_reuse_decision_identity_digest(),
        seed.spatial_reuse_decision_identity_digest()
    );
    assert_eq!(
        planner_input.spatial_rebuild_denial_identity_digest(),
        seed.spatial_rebuild_denial_identity_digest()
    );
    assert_eq!(
        planner_input.spatial_selected_equivalence_family_identity(),
        seed.spatial_selected_equivalence_family_identity()
    );
    assert_eq!(
        planner_input.spatial_compiled_product_identity_digest(),
        seed.spatial_compiled_product_identity_digest()
    );
    assert_eq!(
        planner_input.spatial_equivalence_policy_identity_digest(),
        seed.spatial_equivalence_policy_identity_digest()
    );
    assert_eq!(
        planner_input.topology_freshness_requirement_posture(),
        topology_support.freshness_requirement_posture()
    );
    assert_eq!(
        planner_input.topology_rendered_output_comparison_posture(),
        topology_support.rendered_output_comparison_posture()
    );
    assert_eq!(
        planner_input.spatial_freshness_requirement_posture(),
        spatial_support.freshness_requirement_posture()
    );
    assert_eq!(
        planner_input.spatial_rendered_output_comparison_posture(),
        spatial_support.rendered_output_comparison_posture()
    );
    assert_eq!(
        planner_input.topology_query_execution_count(),
        topology_support.query_execution_count()
    );
    assert_eq!(
        planner_input.topology_row_scan_fallback_count(),
        topology_support.row_scan_fallback_count()
    );
    assert_eq!(
        planner_input.topology_whole_view_fallback_count(),
        topology_support.whole_view_fallback_count()
    );
    assert_eq!(
        planner_input.topology_repeated_rediscovery_denied_count(),
        topology_support.repeated_rediscovery_denied_count()
    );
    assert_eq!(
        planner_input.spatial_receipt_proof_row_count(),
        spatial_support.receipt_proof_row_count()
    );
    assert_eq!(
        planner_input.spatial_non_ordinary_residue_row_count(),
        spatial_support.non_ordinary_residue_row_count()
    );
    assert!(!seed.residue_digest().is_empty());
    assert!(!seed.seed_digest().is_empty());
}

#[test]
fn milestone_fifteen_seed_rejects_foreign_topology_planner_support_before_seed_construction() {
    let components = current_public_closeout_components().expect("current closeout components");
    let packet =
        crate::workload_composition::current_worth_touched_graph_conflict_selected_route_packet()
            .expect("current selected-route packet should build");
    let hostile_public_proof_input = hostile_public_proof_input_with_foreign_reuse_basis(
        "foreign-topology-selected-reuse-basis",
    );

    let error = WorthTouchedGraphConflictMilestoneFifteenSeed::from_selected_route_packet(
        &packet,
        components.residue_chain().residue_digest(),
        packet.source_firewall_digest(),
        hostile_public_proof_input,
    )
    .expect_err("foreign admitted public proof input must fail before seed construction");

    assert_eq!(
        error.kind(),
        WorthTouchedGraphConflictPublicCloseoutErrorKind::IncompleteProofChain
    );
    assert!(error.detail().contains(
        "Milestone 15 planner proof input must preserve selected-route packet reuse authority"
    ));
}

#[test]
fn public_closeout_residue_manifest_matches_live_residue_chain() {
    let components = current_public_closeout_components().expect("current closeout components");
    let live_rows = components.residue_chain();
    let manifest =
        current_public_closeout_consumer_residue_manifest().expect("public residue manifest");

    assert_eq!(
        manifest
            .iter()
            .map(manifest_row_key)
            .collect::<BTreeSet<_>>(),
        live_rows
            .rows()
            .iter()
            .map(live_residue_row_key)
            .collect::<BTreeSet<_>>()
    );
}

fn manifest_row_key(
    row: &crate::workload_composition::public_closeout::PublicCloseoutConsumerResidueRow,
) -> (String, String, String, String, String, String, String) {
    (
        row.source_path().to_string(),
        row.current_surface().to_string(),
        match row.owner() {
            PublicCloseoutConsumerResidueOwner::WorthKernel => "worth-kernel",
            PublicCloseoutConsumerResidueOwner::WorthTopo => "worth-topo",
            PublicCloseoutConsumerResidueOwner::WorthSpatial => "worth-spatial",
            PublicCloseoutConsumerResidueOwner::ForgeQuery => "forge-query",
        }
        .to_string(),
        match row.disposition() {
            PublicCloseoutConsumerResidueDisposition::ExplicitResidue => "explicit-residue",
        }
        .to_string(),
        row.blocker().to_string(),
        row.removal_trigger().to_string(),
        match row.boundary_posture() {
            PublicCloseoutConsumerResidueBoundaryPosture::QueryProofAccompanimentOnly => {
                "query-proof-accompaniment-only"
            }
            PublicCloseoutConsumerResidueBoundaryPosture::ReplayUndoCloseoutOnly => {
                "replay-undo-closeout-only"
            }
            PublicCloseoutConsumerResidueBoundaryPosture::CoveredOrdinaryConsumerDependency => {
                "covered-ordinary-consumer-dependency"
            }
        }
        .to_string(),
    )
}

fn live_residue_row_key(
    row: &WorthTouchedGraphConflictResidueRow,
) -> (String, String, String, String, String, String, String) {
    (
        "crates/worth-kernel/src/workload_composition/public_closeout/residue_chain.rs".to_string(),
        row.surface_name().to_string(),
        row.owner().to_string(),
        "explicit-residue".to_string(),
        row.blocker().to_string(),
        row.removal_trigger().to_string(),
        match row.boundary_posture() {
            WorthTouchedGraphConflictResidueBoundaryPosture::QueryProofAccompanimentOnly => {
                "query-proof-accompaniment-only"
            }
            WorthTouchedGraphConflictResidueBoundaryPosture::ReplayUndoCloseoutOnly => {
                "replay-undo-closeout-only"
            }
            WorthTouchedGraphConflictResidueBoundaryPosture::CoveredOrdinaryConsumerDependency => {
                "covered-ordinary-consumer-dependency"
            }
        }
        .to_string(),
    )
}
