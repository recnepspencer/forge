use super::current::current_cross_family_coverage_inventory;
use super::row::CrossFamilyCoverageFamilyKind as FamilyKind;

#[test]
fn conflict_family_inventory_rows_use_selected_route_and_public_proof_concurrency_authority() {
    let inventory =
        current_cross_family_coverage_inventory().expect("cross-family coverage inventory");

    let conflict = inventory
        .rows()
        .iter()
        .find(|row| {
            row.family_kind() == FamilyKind::ConflictIndependenceBatchAdmission
                && row.current_surface()
                    == "current_worth_touched_graph_conflict_selected_route_packet::selected_conflict_plan_digests"
        })
        .expect("conflict row");
    let independence = inventory
        .rows()
        .iter()
        .find(|row| {
            row.family_kind() == FamilyKind::ConflictIndependenceBatchAdmission
                && row.current_surface()
                    == "current_worth_touched_graph_conflict_selected_route_packet::independence_proof_digests"
        })
        .expect("independence row");
    let batch = inventory
        .rows()
        .iter()
        .find(|row| {
            row.family_kind() == FamilyKind::ConflictIndependenceBatchAdmission
                && row.current_surface()
                    == "current_worth_touched_graph_conflict_selected_route_packet::selected_batch_plan_digest"
        })
        .expect("batch row");

    for row in [conflict, independence] {
        assert_eq!(
            row.ordinary_path_live_caller_surface(),
            "current_worth_touched_graph_conflict_public_facade"
        );
        assert_eq!(
            row.ordinary_path_live_caller_path(),
            "crates/worth-kernel/src/workload_composition/planner_owned_routing/public_facade/current.rs"
        );
    }

    assert_eq!(
        batch.ordinary_path_live_caller_surface(),
        "current_worth_touched_graph_conflict_selected_route_packet"
    );
    assert_eq!(
        batch.ordinary_path_live_caller_path(),
        "crates/worth-kernel/src/workload_composition/planner_owned_routing/selected_route/current.rs"
    );
}
