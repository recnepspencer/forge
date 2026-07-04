use std::fs;
use std::path::Path;

use topology::touched_graph_parity_closeout::{
    current_topology_invalidation_coverage_contributor,
    current_topology_read_family_coverage_contributor,
    current_topology_validator_invariant_coverage_contributor,
};
use worth_spatial::touched_graph_parity_closeout::{
    current_spatial_evidence_lookup_coverage_contributor,
    current_spatial_retained_surface_coverage_contributor,
};

use super::current::current_cross_family_coverage_inventory;
use super::row::{
    CrossFamilyCoverageFamilyKind as FamilyKind, CrossFamilyCoverageQuerySurfaceKind,
};
use crate::workload_composition::planner_owned_routing::{
    current_worth_touched_graph_conflict_public_facade_with_artifact_policy,
    WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy,
};
use crate::workload_composition::touched_graph_parity_closeout::family_contributors::{
    current_batch_admission_coverage_contributor,
    current_compiled_product_equivalence_coverage_contributor,
    current_compiled_product_reuse_coverage_contributor, current_conflict_coverage_contributor,
    current_derived_diagnostics_coverage_contributor, current_independence_coverage_contributor,
    current_public_proof_coverage_contributor, current_replay_coverage_contributor,
    current_reuse_family_contributor_catalog, current_undo_coverage_contributor,
};

#[test]
fn cross_family_coverage_inventory_is_scope_complete() {
    let inventory =
        current_cross_family_coverage_inventory().expect("cross-family coverage inventory");

    for family_kind in FamilyKind::ALL {
        assert!(
            inventory
                .rows()
                .iter()
                .any(|row| row.family_kind() == family_kind && row.ordinary_path_reachable()),
            "missing ordinary-path coverage row for {}",
            family_kind.as_str()
        );
    }
}

#[test]
fn coverage_rows_name_phase_one_cut_line_fields() {
    let inventory =
        current_cross_family_coverage_inventory().expect("cross-family coverage inventory");

    for row in inventory.rows() {
        assert!(!row.source_path().is_empty());
        assert!(!row.current_owner_crate().is_empty());
        assert!(!row.replacement_lane().is_empty());
        assert!(!row.selected_identity_fields_consumed().is_empty());
        assert!(!row.ordinary_path_live_caller_surface().is_empty());
        assert!(!row.ordinary_path_live_caller_path().is_empty());
        assert!(
            row.ordinary_path_reachable(),
            "ordinary-path reachability must be proven from a live caller for {}",
            row.current_surface()
        );
        match row.query_surface_kind() {
            CrossFamilyCoverageQuerySurfaceKind::NotQuery
            | CrossFamilyCoverageQuerySurfaceKind::SupportPosture
            | CrossFamilyCoverageQuerySurfaceKind::ConsumerResidue
            | CrossFamilyCoverageQuerySurfaceKind::BoundaryEnvelope => {}
        }
        assert!(
            !row.replacement_lane().contains("mixed") && !row.replacement_lane().contains("helper"),
            "replacement lane must be an exact path, not vague helper language: {}",
            row.replacement_lane()
        );
    }
}

#[test]
fn coverage_inventory_carries_one_live_authority_chain() {
    let inventory =
        current_cross_family_coverage_inventory().expect("cross-family coverage inventory");
    let public_facade = current_worth_touched_graph_conflict_public_facade_with_artifact_policy(
        WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy::MinimalOperationalTruth,
    )
    .expect("public facade");
    let public_proof = public_facade.public_proof();
    let diagnostics = public_facade.derived_diagnostics();

    assert_eq!(
        inventory.selected_route_identity_digest(),
        public_proof.selected_route_identity_digest()
    );
    assert_eq!(
        inventory.selected_route_identity_digest(),
        diagnostics.selected_route_identity_digest()
    );
    assert!(!inventory.inventory_digest().is_empty());
}

#[test]
fn coverage_inventory_derives_rows_from_live_family_contributors() {
    let inventory =
        current_cross_family_coverage_inventory().expect("cross-family coverage inventory");
    let expected_surfaces = [
        current_topology_read_family_coverage_contributor()
            .expect("topology read-family contributor")
            .current_surface(),
        current_topology_validator_invariant_coverage_contributor()
            .expect("topology validator contributor")
            .current_surface(),
        current_topology_invalidation_coverage_contributor()
            .expect("topology invalidation contributor")
            .current_surface(),
        current_spatial_evidence_lookup_coverage_contributor()
            .expect("spatial evidence contributor")
            .current_surface(),
        current_spatial_retained_surface_coverage_contributor()
            .expect("spatial retained contributor")
            .current_surface(),
        current_replay_coverage_contributor()
            .expect("kernel replay contributor")
            .current_surface(),
        current_undo_coverage_contributor()
            .expect("kernel undo contributor")
            .current_surface(),
        current_conflict_coverage_contributor()
            .expect("kernel conflict contributor")
            .current_surface(),
        current_independence_coverage_contributor()
            .expect("kernel independence contributor")
            .current_surface(),
        current_batch_admission_coverage_contributor()
            .expect("kernel batch contributor")
            .current_surface(),
        current_compiled_product_equivalence_coverage_contributor()
            .expect("kernel equivalence contributor")
            .current_surface(),
        current_compiled_product_reuse_coverage_contributor()
            .expect("kernel reuse contributor")
            .current_surface(),
        current_public_proof_coverage_contributor()
            .expect("kernel public-proof contributor")
            .current_surface(),
        current_derived_diagnostics_coverage_contributor()
            .expect("kernel diagnostics contributor")
            .current_surface(),
    ];

    for surface in expected_surfaces {
        assert!(
            inventory
                .rows()
                .iter()
                .any(|row| row.current_surface() == surface),
            "inventory must be derived from the live contributor surface {surface}"
        );
    }
}

#[test]
fn coverage_inventory_names_distinct_live_sublanes_instead_of_proxy_rows() {
    let inventory =
        current_cross_family_coverage_inventory().expect("cross-family coverage inventory");
    let catalog = current_reuse_family_contributor_catalog().expect("reuse-family catalog");

    for row in catalog.rows() {
        assert!(
            inventory
                .rows()
                .iter()
                .any(|inventory_row| inventory_row.current_surface()
                    == row.coverage_contributor().current_surface()),
            "inventory must preserve the catalog-backed reuse sublane {}",
            row.kind().as_str()
        );
    }
}

#[test]
fn public_facing_coverage_rows_name_real_ordinary_consumers() {
    let inventory =
        current_cross_family_coverage_inventory().expect("cross-family coverage inventory");

    let public_proof_row = inventory
        .rows()
        .iter()
        .find(|row| row.family_kind() == FamilyKind::PublicProof)
        .expect("public-proof family row should exist");
    let diagnostics_row = inventory
        .rows()
        .iter()
        .find(|row| row.family_kind() == FamilyKind::DerivedDiagnostics)
        .expect("derived-diagnostics family row should exist");

    assert_eq!(
        public_proof_row.ordinary_path_live_caller_path(),
        "crates/worth-kernel/src/workload_composition/worth_workload/ordinary_consumer_sweep/closeout.rs"
    );
    assert_eq!(
        public_proof_row.ordinary_path_live_caller_surface(),
        "current_worth_workload_ordinary_consumer_sweep_closeout"
    );
    assert_eq!(
        diagnostics_row.ordinary_path_live_caller_path(),
        "crates/worth-kernel/src/workload_composition/worth_workload/ordinary_consumer_sweep/closeout.rs"
    );
    assert_eq!(
        diagnostics_row.ordinary_path_live_caller_surface(),
        "current_worth_workload_ordinary_consumer_sweep_closeout"
    );

    assert_eq!(
        public_proof_row.source_path(),
        "crates/worth-kernel/src/workload_composition/planner_owned_routing/public_facade/current.rs"
    );
    assert_eq!(
        public_proof_row.current_surface(),
        "public_proof_inspection"
    );
    assert_eq!(
        diagnostics_row.source_path(),
        "crates/worth-kernel/src/workload_composition/planner_owned_routing/derived_diagnostics/current.rs"
    );
    assert_eq!(
        diagnostics_row.current_surface(),
        "current_worth_touched_graph_conflict_derived_diagnostic_projection_with_artifact_policy"
    );
}

#[test]
fn topology_phase_one_rows_name_live_ordinary_surfaces_instead_of_proxy_inputs() {
    let inventory =
        current_cross_family_coverage_inventory().expect("cross-family coverage inventory");
    let invalidation_row = inventory
        .rows()
        .iter()
        .find(|row| row.family_kind() == FamilyKind::Invalidation)
        .expect("invalidation family row should exist");
    let validator_row = inventory
        .rows()
        .iter()
        .find(|row| row.family_kind() == FamilyKind::ValidatorInvariantRouting)
        .expect("validator/invariant family row should exist");

    assert_eq!(
        invalidation_row.current_surface(),
        "current_topology_invalidation_route_input"
    );
    assert_eq!(
        validator_row.current_surface(),
        "current_topology_validator_invariant_milestone_nine_closeout"
    );
    assert_ne!(
        validator_row.current_surface(),
        "current_topology_invalidation_route_input::selected_rows"
    );
    assert_ne!(
        validator_row.current_surface(),
        "current_topology_validator_invariant_operator_cutover_closeout"
    );
    assert_ne!(
        validator_row.current_surface(),
        "current_topology_validator_invariant_selection_closeout_for_declared_touch"
    );
    assert_eq!(
        validator_row.source_path(),
        "crates/worth-topo/src/validator_invariant_catalog/milestone_nine_closeout/current.rs"
    );
}

#[test]
fn workload_composition_public_facade_does_not_export_legacy_phase_one_inventory() {
    let workload_composition_mod = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src/workload_composition/mod.rs"),
    )
    .expect("workload composition module should load");

    assert!(
        !workload_composition_mod.contains("pub use planner_owned_routing_inventory::{"),
        "workload-composition public facade must cut over phase-1 inventory authority to cross-family coverage only",
    );
}

#[test]
fn topology_and_spatial_rows_prove_reachability_from_live_ordinary_callers() {
    let inventory =
        current_cross_family_coverage_inventory().expect("cross-family coverage inventory");
    let validator_row = inventory
        .rows()
        .iter()
        .find(|row| row.family_kind() == FamilyKind::ValidatorInvariantRouting)
        .expect("validator/invariant family row should exist");
    let invalidation_row = inventory
        .rows()
        .iter()
        .find(|row| row.family_kind() == FamilyKind::Invalidation)
        .expect("invalidation family row should exist");
    let spatial_row = inventory
        .rows()
        .iter()
        .find(|row| row.family_kind() == FamilyKind::EvidenceLookup)
        .expect("evidence-lookup family row should exist");
    let retained_spatial_row = inventory
        .rows()
        .iter()
        .find(|row| row.family_kind() == FamilyKind::RetainedSpatial)
        .expect("retained-spatial family row should exist");

    assert_eq!(
        validator_row.source_path(),
        "crates/worth-topo/src/validator_invariant_catalog/milestone_nine_closeout/current.rs"
    );
    assert_eq!(
        invalidation_row.source_path(),
        "crates/worth-topo/src/projection/planner_owned_routing/invalidation_route/route_input.rs"
    );
    assert_eq!(
        spatial_row.source_path(),
        "crates/worth-spatial/src/workload_platform/planner_owned_routing/evidence_lookup_route/current.rs"
    );
    assert_eq!(
        retained_spatial_row.source_path(),
        "crates/worth-spatial/src/workload_platform/planner_owned_routing/public_closeout_route/current.rs"
    );
    assert_eq!(
        validator_row.ordinary_path_live_caller_path(),
        "crates/worth-topo/src/validator_invariant_catalog/milestone_nine_closeout/current.rs"
    );
    assert_eq!(
        validator_row.ordinary_path_live_caller_surface(),
        "current_topology_validator_invariant_milestone_nine_closeout"
    );
    assert_eq!(
        invalidation_row.ordinary_path_live_caller_path(),
        "crates/worth-kernel/src/workload_composition/planner_owned_routing/selected_route/current.rs"
    );
    assert_eq!(
        invalidation_row.ordinary_path_live_caller_surface(),
        "current_worth_touched_graph_conflict_selected_route_packet"
    );
    assert_eq!(
        spatial_row.ordinary_path_live_caller_path(),
        "crates/worth-kernel/src/workload_composition/planner_owned_routing/selected_route/current.rs"
    );
    assert_eq!(
        spatial_row.ordinary_path_live_caller_surface(),
        "current_worth_touched_graph_conflict_selected_route_packet"
    );
    assert_eq!(
        retained_spatial_row.ordinary_path_live_caller_path(),
        "crates/worth-spatial/src/workload_platform/planner_owned_routing/public_closeout_route/current.rs"
    );
    assert_eq!(
        retained_spatial_row.ordinary_path_live_caller_surface(),
        "current_evidence_lookup_public_closeout"
    );
}

#[test]
fn cross_family_coverage_inventory_rejects_hidden_second_ontologies() {
    let inventory =
        current_cross_family_coverage_inventory().expect("cross-family coverage inventory");
    for row in inventory.rows() {
        assert_ne!(
            row.ordinary_path_live_caller_surface(),
            "current_worth_touched_graph_conflict_public_facade_ordinary_coverage_proof",
            "phase-1 inventory must not hide public consumers behind the deleted public-facade coverage helper"
        );
        assert_ne!(
            row.ordinary_path_live_caller_surface(),
            "current_topology_validator_invariant_ordinary_coverage_proof",
            "phase-1 inventory must not hide validator consumers behind the deleted invalidation-route coverage helper"
        );
        assert!(
            !row.replacement_lane().contains("touched_graph_parity_closeout"),
            "phase-1 inventory must derive replacement lanes from production-owned claim seams, not parity-closeout helpers: {}",
            row.replacement_lane()
        );
        assert!(
            !row.ordinary_path_live_caller_path().contains("touched_graph_parity_closeout"),
            "phase-1 inventory must prove ordinary callers from production-owned seams, not parity-closeout helpers: {}",
            row.ordinary_path_live_caller_path()
        );
    }
}

#[test]
fn live_caller_proof_is_derived_from_production_owned_typed_seams() {
    let inventory =
        current_cross_family_coverage_inventory().expect("cross-family coverage inventory");

    for row in inventory.rows() {
        assert!(
            !row.ordinary_path_live_caller_surface().contains("coverage_proof"),
            "live caller proof must come from production contributors rather than a coverage helper for {} via {}",
            row.current_surface(),
            row.ordinary_path_live_caller_surface()
        );
        assert!(
            !row.ordinary_path_live_caller_path()
                .contains("coverage_inventory"),
            "live caller proof must not originate from the phase-1 inventory lane for {}",
            row.current_surface()
        );
    }
}

#[test]
fn phase_one_ordinary_coverage_proof_helpers_are_deleted() {
    let selected_route_current = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/workload_composition/planner_owned_routing/selected_route/current.rs"),
    )
    .expect("selected-route current source should load");
    let public_facade_current = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/workload_composition/planner_owned_routing/public_facade/current.rs"),
    )
    .expect("public-facade current source should load");
    let topology_invalidation_proof = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../worth-topo/src/projection/planner_owned_routing/invalidation_route/current_route.rs"),
    )
    .expect("topology invalidation route source should load");

    for forbidden_symbol in [
        "current_worth_touched_graph_conflict_selected_route_ordinary_coverage_proof",
        "SelectedRouteOrdinaryCoverageProof",
        "current_worth_touched_graph_conflict_public_facade_ordinary_coverage_proof",
        "PublicFacadeOrdinaryCoverageProof",
        "current_topology_validator_invariant_ordinary_coverage_proof",
        "TopologyValidatorInvariantOrdinaryCoverageProof",
    ] {
        assert!(
            !selected_route_current.contains(forbidden_symbol)
                && !public_facade_current.contains(forbidden_symbol)
                && !topology_invalidation_proof.contains(forbidden_symbol),
            "phase-1 coverage must not rely on deleted ordinary-coverage proof helper symbol {forbidden_symbol}"
        );
    }
}
