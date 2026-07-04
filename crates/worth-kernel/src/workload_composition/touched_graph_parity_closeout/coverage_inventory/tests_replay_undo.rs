use super::current::current_cross_family_coverage_inventory;
use super::row::CrossFamilyCoverageFamilyKind as FamilyKind;

#[test]
fn replay_and_undo_inventory_rows_use_carried_boundary_route_authority() {
    let inventory =
        current_cross_family_coverage_inventory().expect("cross-family coverage inventory");
    let replay_row = inventory
        .rows()
        .iter()
        .find(|row| {
            row.family_kind() == FamilyKind::ReplayUndo
                && row.current_surface()
                    == "current_replay_undo_boundary_route_authority::replay_scope_identity"
        })
        .expect("replay inventory row should exist");
    let undo_row = inventory
        .rows()
        .iter()
        .find(|row| {
            row.family_kind() == FamilyKind::ReplayUndo
                && row.current_surface()
                    == "current_replay_undo_boundary_route_authority::undo_scope_identity"
        })
        .expect("undo inventory row should exist");

    for row in [replay_row, undo_row] {
        assert_eq!(
            row.source_path(),
            "crates/worth-kernel/src/workload_composition/planner_owned_routing/ordinary_consumer_authority/replay_undo_route_authority.rs"
        );
        assert_eq!(
            row.ordinary_path_live_caller_surface(),
            "admit_boolean_split_replay_undo_boundary"
        );
        assert_eq!(
            row.ordinary_path_live_caller_path(),
            "crates/worth-kernel/src/workload_composition/worth_workload/replay_undo_boundary/boolean_split_boundary_admission.rs"
        );
    }
}
