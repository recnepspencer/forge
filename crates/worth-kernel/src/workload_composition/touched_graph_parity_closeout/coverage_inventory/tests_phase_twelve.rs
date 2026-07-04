use std::fs;
use std::path::Path;

use super::current::current_cross_family_coverage_inventory;
use super::row::CrossFamilyCoverageFamilyKind as FamilyKind;

#[test]
fn parity_closeout_new_lanes_exist_before_old_lane_split() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    for relative_path in [
        "src/workload_composition/touched_graph_parity_closeout/coverage_inventory",
        "src/workload_composition/touched_graph_parity_closeout/family_contributors",
        "../worth-topo/src/projection/touched_graph_parity_closeout/read_family",
        "../worth-topo/src/projection/touched_graph_parity_closeout/validator_invariant_family",
        "../worth-topo/src/projection/touched_graph_parity_closeout/invalidation_family",
        "../worth-spatial/src/workload_platform/touched_graph_parity_closeout/evidence_lookup_family",
    ] {
        assert!(
            manifest_dir.join(relative_path).exists(),
            "phase 12 requires the final closeout lane on disk before displaced-lane surgery: {relative_path}"
        );
    }
}

#[test]
fn displaced_closeout_lanes_stop_owning_ordinary_architecture_truth() {
    let inventory =
        current_cross_family_coverage_inventory().expect("cross-family coverage inventory");

    for family_kind in [
        FamilyKind::ReadRouting,
        FamilyKind::ValidatorInvariantRouting,
        FamilyKind::Invalidation,
        FamilyKind::EvidenceLookup,
        FamilyKind::RetainedSpatial,
    ] {
        let row = inventory
            .rows()
            .iter()
            .find(|row| row.family_kind() == family_kind)
            .expect("phase-12 family row should exist");
        assert!(
            row.replacement_lane().contains("touched_graph_parity_closeout"),
            "ordinary architecture truth must be claimed through the final parity closeout lane for {} via {}",
            family_kind.as_str(),
            row.replacement_lane()
        );
        assert!(
            !row.ordinary_path_live_caller_path()
                .contains("touched_graph_parity_closeout"),
            "live ordinary callers must still be proven from production seams rather than the parity audit lane for {}",
            family_kind.as_str()
        );
    }

    let planner_owned_routing_mod = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/workload_composition/planner_owned_routing/mod.rs"),
    )
    .expect("planner owned routing module should load");
    assert!(
        !planner_owned_routing_mod.contains("mod touched_graph_parity_closeout;"),
        "the displaced planner-owned touched-graph closeout file must not remain part of ordinary module ownership"
    );
}
