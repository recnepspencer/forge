use std::fs;
use std::path::Path;

use super::{
    current_worth_touched_graph_conflict_derived_diagnostic_projection,
    current_worth_touched_graph_conflict_derived_diagnostic_projection_with_artifact_policy,
    WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy,
};
use crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_selected_route_packet;
use crate::workload_composition::planner_owned_routing::test_support::run_stack_heavy_planner_owned_routing_test;

#[test]
fn derived_diagnostics_localize_exact_route_and_mismatch_locus() {
    run_stack_heavy_planner_owned_routing_test(|| {
        let selected_route_packet = current_worth_touched_graph_conflict_selected_route_packet()
            .expect(
                "current selected-route packet should lower from the real planner-owned route seam",
            );
        let diagnostic_projection = current_worth_touched_graph_conflict_derived_diagnostic_projection()
            .expect("current derived diagnostic projection should lower from the real selected-route packet");
        let rich_localization = diagnostic_projection
            .rich_localization()
            .expect("rich localization should remain available by default");

        assert_eq!(
            diagnostic_projection.selected_route_identity_digest(),
            selected_route_packet.selected_route_identity_digest()
        );
        assert_eq!(
            diagnostic_projection.selected_family_identity(),
            selected_route_packet.selected_family_identity()
        );
        assert_eq!(
            diagnostic_projection.selected_product_identity_digest(),
            selected_route_packet.selected_product_identity_digest()
        );
        assert_eq!(
            diagnostic_projection.selected_witness_identity_digest(),
            selected_route_packet.selected_witness_identity_digest()
        );
        assert_eq!(
            diagnostic_projection.topology_reuse_posture(),
            selected_route_packet.topology_reuse_posture()
        );
        assert_eq!(
            diagnostic_projection.spatial_reuse_posture(),
            selected_route_packet.spatial_reuse_posture()
        );
        assert_eq!(
            rich_localization.touched_closure_digest(),
            selected_route_packet.touched_closure_digest()
        );
        assert_eq!(
            rich_localization.selected_plan_digest(),
            selected_route_packet.selected_plan_digest()
        );
        assert_eq!(
            rich_localization.touched_semantic_family_key(),
            selected_route_packet.touched_semantic_family_key()
        );
        assert_eq!(
            rich_localization.touched_aspect_count(),
            selected_route_packet.touched_aspect_count()
        );
        assert_eq!(
            rich_localization.touched_scope_count(),
            selected_route_packet.touched_scope_count()
        );
        assert_eq!(
            rich_localization.selected_row_family_identities(),
            selected_route_packet.selected_row_family_identities()
        );
        assert_eq!(
            rich_localization.compiled_product_reuse_route_packet_identity(),
            Some(selected_route_packet.compiled_product_reuse_route_packet_identity())
        );
        assert_eq!(
            rich_localization.batch_admission_denial_witness_identity(),
            selected_route_packet.batch_admission_denial_witness_identity()
        );
        assert_eq!(
            rich_localization.conflict_independence_denial_witness_identity(),
            selected_route_packet.conflict_independence_denial_witness_identity()
        );
    });
}

#[test]
fn artifact_policy_can_suppress_rich_diagnostics_without_losing_operational_truth() {
    run_stack_heavy_planner_owned_routing_test(|| {
        let selected_route_packet = current_worth_touched_graph_conflict_selected_route_packet()
            .expect("current selected-route packet should lower");
        let minimal_projection =
            current_worth_touched_graph_conflict_derived_diagnostic_projection_with_artifact_policy(
                WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy::MinimalOperationalTruth,
            )
            .expect("minimal diagnostic projection should lower");
        let rich_projection = current_worth_touched_graph_conflict_derived_diagnostic_projection()
            .expect("rich diagnostic projection should lower");

        assert_eq!(
            minimal_projection.artifact_policy(),
            WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy::MinimalOperationalTruth
        );
        assert!(minimal_projection.rich_localization().is_none());
        assert_eq!(
            minimal_projection.selected_route_identity_digest(),
            selected_route_packet.selected_route_identity_digest()
        );
        assert_eq!(
            minimal_projection.selected_family_identity(),
            rich_projection.selected_family_identity()
        );
        assert_eq!(
            minimal_projection.selected_product_identity_digest(),
            rich_projection.selected_product_identity_digest()
        );
        assert_eq!(
            minimal_projection.selected_witness_identity_digest(),
            rich_projection.selected_witness_identity_digest()
        );
        assert_eq!(
            minimal_projection.batch_admission_denial_witness_identity_digest(),
            rich_projection.batch_admission_denial_witness_identity_digest()
        );
        assert_eq!(
            minimal_projection.batch_admission_denial_witness_kind(),
            rich_projection.batch_admission_denial_witness_kind()
        );
        assert_eq!(
            minimal_projection.conflict_independence_denial_witness_identity_digest(),
            rich_projection.conflict_independence_denial_witness_identity_digest()
        );
        assert_eq!(
            minimal_projection.conflict_independence_denial_witness_kind(),
            rich_projection.conflict_independence_denial_witness_kind()
        );
        assert_eq!(
            minimal_projection.rebuild_denial_identity_digest(),
            rich_projection.rebuild_denial_identity_digest()
        );
        assert_eq!(
            minimal_projection.spatial_rebuild_denial_identity_digest(),
            rich_projection.spatial_rebuild_denial_identity_digest()
        );
        assert!(rich_projection.rich_localization().is_some());
    });
}

#[test]
fn derived_diagnostics_do_not_reload_topology_local_invalidation_input() {
    let source = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/workload_composition/planner_owned_routing/derived_diagnostics/current.rs"),
    )
    .expect("derived diagnostics current source should load");

    assert!(
        !source.contains("current_topology_invalidation_route_input"),
        "planner-owned diagnostics must lower rich localization from the selected-route packet instead of reloading topology-local invalidation input",
    );
}
