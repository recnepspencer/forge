use crate::workload_composition::planner_owned_routing::derived_diagnostics::current_worth_touched_graph_conflict_derived_diagnostic_projection_with_packet_loader;
use crate::workload_composition::planner_owned_routing::selected_route::current_worth_touched_graph_conflict_selected_route_packet;
use crate::workload_composition::planner_owned_routing::{
    current_public_closeout_consumer_residue_manifest,
    test_support::run_stack_heavy_planner_owned_routing_test,
    WorthTouchedGraphConflictPublicFacade, WorthTouchedGraphConflictPublicProofInspection,
};
use crate::workload_composition::{
    PlannerOwnedRoutingErrorKind, WorthTouchedGraphConflictArchitectureAlignmentReport,
    WorthTouchedGraphConflictMilestoneFifteenSeed, WorthTouchedGraphConflictResidueChain,
};

#[test]
fn public_facade_exports_inspection_without_authority_construction() {
    let public_proof_accessor: fn(
        &WorthTouchedGraphConflictPublicFacade,
    ) -> &WorthTouchedGraphConflictPublicProofInspection =
        WorthTouchedGraphConflictPublicFacade::public_proof;
    let selected_route_accessor: fn(&WorthTouchedGraphConflictPublicFacade) -> &str =
        WorthTouchedGraphConflictPublicFacade::selected_route_identity_digest;
    let public_proof_selected_route_accessor: fn(
        &WorthTouchedGraphConflictPublicProofInspection,
    ) -> &str = WorthTouchedGraphConflictPublicProofInspection::selected_route_identity_digest;
    let public_proof_selected_family_accessor: fn(
        &WorthTouchedGraphConflictPublicProofInspection,
    ) -> &str = WorthTouchedGraphConflictPublicProofInspection::selected_family_identity;
    let public_proof_selected_product_accessor: fn(
        &WorthTouchedGraphConflictPublicProofInspection,
    ) -> &str = WorthTouchedGraphConflictPublicProofInspection::selected_product_identity_digest;
    let public_proof_selected_witness_accessor: fn(
        &WorthTouchedGraphConflictPublicProofInspection,
    ) -> Option<&str> =
        WorthTouchedGraphConflictPublicProofInspection::selected_witness_identity_digest;
    let public_proof_closeout_accessor: fn(
        &WorthTouchedGraphConflictPublicProofInspection,
    ) -> &str = WorthTouchedGraphConflictPublicProofInspection::closeout_digest;
    let public_proof_firewall_accessor: fn(
        &WorthTouchedGraphConflictPublicProofInspection,
    ) -> &str = WorthTouchedGraphConflictPublicProofInspection::source_firewall_digest;
    let public_proof_deletion_accessor: fn(
        &WorthTouchedGraphConflictPublicProofInspection,
    ) -> &str = WorthTouchedGraphConflictPublicProofInspection::deletion_closeout_digest;
    let public_proof_residue_accessor: fn(
        &WorthTouchedGraphConflictPublicProofInspection,
    ) -> &WorthTouchedGraphConflictResidueChain =
        WorthTouchedGraphConflictPublicProofInspection::residue_chain;
    let public_proof_alignment_accessor: fn(
        &WorthTouchedGraphConflictPublicProofInspection,
    ) -> &WorthTouchedGraphConflictArchitectureAlignmentReport =
        WorthTouchedGraphConflictPublicProofInspection::architecture_alignment_report;
    let public_proof_seed_accessor: fn(
        &WorthTouchedGraphConflictPublicProofInspection,
    ) -> &WorthTouchedGraphConflictMilestoneFifteenSeed =
        WorthTouchedGraphConflictPublicProofInspection::milestone_fifteen_seed;

    let _ = (
        public_proof_accessor,
        selected_route_accessor,
        public_proof_selected_route_accessor,
        public_proof_selected_family_accessor,
        public_proof_selected_product_accessor,
        public_proof_selected_witness_accessor,
        public_proof_closeout_accessor,
        public_proof_firewall_accessor,
        public_proof_deletion_accessor,
        public_proof_residue_accessor,
        public_proof_alignment_accessor,
        public_proof_seed_accessor,
    );
}

#[test]
fn public_facade_rejects_support_wrapper_shortcuts() {
    run_stack_heavy_planner_owned_routing_test(|| {
        let error =
            current_worth_touched_graph_conflict_derived_diagnostic_projection_with_packet_loader(
                || {
                    Ok(
                        current_worth_touched_graph_conflict_selected_route_packet()?
                            .with_test_selected_reuse_basis_identity_digest_override(
                                "foreign-selected-reuse-basis",
                            ),
                    )
                },
            )
            .expect_err("planner-owned diagnostics must reject forged wrapper or local explainer packets before facade assembly");

        assert_eq!(
            error.kind(),
            PlannerOwnedRoutingErrorKind::CurrentProofUnavailable
        );
    });
}

#[test]
fn public_closeout_legacy_helpers_are_deleted_or_capped() {
    let legacy_residue_lane = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "src/workload_composition/public_closeout/compiled_product_consumer_cutover/residue_manifest.rs",
    );
    assert!(
        !legacy_residue_lane.exists(),
        "legacy public-closeout residue helper must not remain compiled beside planner-owned public facade"
    );

    let manifest = current_public_closeout_consumer_residue_manifest()
        .expect("planner-owned public-facade residue manifest should build");
    assert!(manifest.iter().all(|row| !row.source_path().is_empty()));
    assert!(manifest.iter().all(|row| !row.current_surface().is_empty()));
    assert!(manifest.iter().all(|row| !row.blocker().is_empty()));
    assert!(manifest.iter().all(|row| !row.removal_trigger().is_empty()));
    assert!(manifest.iter().all(|row| {
        row.source_path().starts_with(
            "crates/worth-kernel/src/workload_composition/planner_owned_routing/public_facade/",
        ) || row
            .source_path()
            .starts_with("crates/worth-kernel/src/workload_composition/public_closeout/")
    }));
}
