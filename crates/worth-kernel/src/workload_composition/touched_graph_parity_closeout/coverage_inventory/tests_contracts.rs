use std::fs;
use std::path::Path;

use super::current::current_cross_family_coverage_inventory;

#[test]
fn parity_contract_vocabulary_is_single_source_of_closeout_meaning() {
    let kernel_row_mod =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "src/workload_composition/touched_graph_parity_closeout/coverage_inventory/row.rs",
        ))
        .expect("kernel coverage row module should load");
    let kernel_contributor_mod =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "src/workload_composition/touched_graph_parity_closeout/family_contributors/mod.rs",
        ))
        .expect("kernel family contributor module should load");
    let topo_contributor_mod = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../worth-topo/src/projection/touched_graph_parity_closeout/contributor.rs"),
    )
    .expect("topology parity contributor module should load");
    let spatial_contributor_mod = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../worth-spatial/src/workload_platform/touched_graph_parity_closeout/contributor.rs",
    ))
    .expect("spatial parity contributor module should load");

    assert!(
        !kernel_row_mod.contains("pub enum CrossFamilyCoverageFamilyKind"),
        "kernel coverage inventory must not define family-kind vocabulary locally",
    );
    assert!(
        !kernel_row_mod.contains("pub enum CrossFamilyCoverageQuerySurfaceKind"),
        "kernel coverage inventory must not define query-surface vocabulary locally",
    );
    assert!(
        !kernel_contributor_mod.contains("pub struct KernelTouchedGraphParityCoverageContributor"),
        "kernel family contributors must compile against the shared parity contributor type",
    );
    assert!(
        !topo_contributor_mod.contains("pub struct TopologyTouchedGraphParityCoverageContributor"),
        "topology family contributors must compile against the shared parity contributor type",
    );
    assert!(
        !spatial_contributor_mod
            .contains("pub struct SpatialTouchedGraphParityCoverageContributor"),
        "spatial family contributors must compile against the shared parity contributor type",
    );
}

#[test]
fn displaced_kernel_and_spatial_facades_do_not_reexport_phase_one_contributors() {
    let planner_owned_routing_mod = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/workload_composition/planner_owned_routing/mod.rs"),
    )
    .expect("planner-owned routing module should load");
    let spatial_facade_mod = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../worth-spatial/src/facade/mod.rs"),
    )
    .expect("spatial facade module should load");
    let spatial_planner_owned_routing_facade = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../worth-spatial/src/facade/planner_owned_routing/evidence_lookup_route.rs"),
    )
    .expect("spatial planner-owned-routing facade should load");

    assert!(
        !planner_owned_routing_mod.contains("_coverage_contributor"),
        "planner-owned routing module must not preserve phase-1 contributor aliases after cutover",
    );
    assert!(
        !spatial_facade_mod.contains("current_spatial_evidence_lookup_coverage_contributor"),
        "spatial facade must not re-export the phase-1 evidence-lookup contributor",
    );
    assert!(
        !spatial_planner_owned_routing_facade
            .contains("current_spatial_evidence_lookup_coverage_contributor"),
        "legacy spatial planner-owned-routing facade must not re-export phase-1 coverage authority",
    );
}

#[test]
fn coverage_inventory_names_distinct_live_sublanes_instead_of_proxy_rows() {
    let inventory =
        current_cross_family_coverage_inventory().expect("cross-family coverage inventory");
    let ordinary_surfaces = inventory
        .rows()
        .iter()
        .map(|row| row.current_surface())
        .collect::<Vec<_>>();

    for required_surface in [
        "current_replay_undo_boundary_route_authority::replay_scope_identity",
        "current_replay_undo_boundary_route_authority::undo_scope_identity",
        "current_worth_touched_graph_conflict_selected_route_packet::selected_conflict_plan_digests",
        "current_worth_touched_graph_conflict_selected_route_packet::independence_proof_digests",
        "current_worth_touched_graph_conflict_selected_route_packet::selected_batch_plan_digest",
        "current_worth_touched_graph_conflict_compiled_product_reuse_route_packet::selected_equivalence_policy_identity_digest",
        "current_worth_touched_graph_conflict_compiled_product_reuse_route_packet::selected_reuse_basis_identity_digest",
    ] {
        assert!(
            ordinary_surfaces.contains(&required_surface),
            "inventory must expose the live ordinary sublane {required_surface}",
        );
    }
}

#[test]
fn milestone_sixteen_spec_names_exact_displaced_and_future_paths() {
    let spec = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("_docs/worth/touched-graph-milestone-16-cross-family-parity-proof-residue-collapse-and-7.5-readiness.md"),
    )
    .expect("Milestone 16 spec should load");

    for required_path in [
        "crates/worth-kernel/src/workload_composition/planner_owned_routing_inventory/",
        "crates/worth-kernel/src/workload_composition/touched_graph_parity_closeout/coverage_inventory/",
        "crates/worth-topo/src/projection/touched_graph_parity_closeout/read_family/",
        "crates/worth-topo/src/projection/touched_graph_parity_closeout/validator_invariant_family/",
        "crates/worth-topo/src/projection/touched_graph_parity_closeout/invalidation_family/",
        "crates/worth-spatial/src/workload_platform/touched_graph_parity_closeout/evidence_lookup_family/",
        "crates/worth-kernel/src/workload_composition/touched_graph_parity_closeout/family_contributors/",
    ] {
        assert!(
            spec.contains(required_path),
            "Milestone 16 spec must name exact phase-one displaced or replacement paths: {required_path}"
        );
    }
}
