use super::{applied, LiveBindingFixture};

#[derive(Debug, Eq, PartialEq)]
struct PatchCost {
    ui: crate::WorthUiCollectionChangeCounters,
    query: crate::WorthUiCollectionQueryWorkInspection,
}

#[test]
fn equivalent_patches_ignore_unrelated_collection_width() {
    let narrow = update_first_row_cost("operation-live-cost-narrow", 8);
    let broad = update_first_row_cost("operation-live-cost-broad", 128);

    assert_eq!(narrow, broad);
    assert_eq!(narrow.ui.patch_operations_visited(), 1);
    assert_eq!(narrow.ui.patch_facts_reported(), 1);
    assert_eq!(narrow.ui.row_references_minted(), 1);
    assert_eq!(narrow.ui.graph_effects_minted(), 1);
    assert_eq!(narrow.ui.measurement_effects_minted(), 2);
    assert_eq!(narrow.ui.allocation_effects_minted(), 1);
    assert_eq!(narrow.query.operations_materialized(), 1);
    assert_eq!(narrow.query.native_facts_materialized(), 1);
    assert_eq!(narrow.query.full_collection_scans(), 0);
    assert_eq!(narrow.query.unrelated_consumer_scans(), 0);
}

#[test]
fn query_postures_construct_only_their_admitted_subsystems() {
    let query_free = crate::WorthUiQueryBindingPlan::default().prepare_downstream_state();
    let free_state = query_free.state_observation();
    assert!(!free_state.query_installed());
    assert_eq!(free_state.installed_reference_count(), 0);
    assert_eq!(free_state.settled_snapshot_count(), 0);
    assert_eq!(
        free_state.operation_live().subsystem_construction_count(),
        0
    );

    let mut snapshot_fixture =
        crate::certification::WorthUiInstalledQueryTestFixture::new("snapshot-only-cost");
    let snapshot_plan = snapshot_fixture.binding_plan();
    let mut snapshot = snapshot_plan.prepare_downstream_state();
    snapshot
        .admit_settled_snapshot(snapshot_fixture.settle_snapshot())
        .expect("snapshot-only posture retains one exact settlement");
    let snapshot_state = snapshot.state_observation();
    assert!(snapshot_state.query_installed());
    assert_eq!(snapshot_state.installed_reference_count(), 1);
    assert_eq!(snapshot_state.settled_snapshot_count(), 1);
    assert_eq!(
        snapshot_state
            .operation_live()
            .subsystem_construction_count(),
        0
    );

    let mut live = LiveBindingFixture::new("operation-live-cost-posture");
    let live_state = live.binding.state_observation();
    assert!(live_state.query_installed());
    assert_eq!(live_state.installed_reference_count(), 1);
    assert_eq!(live_state.settled_snapshot_count(), 0);
    assert_eq!(
        live_state.operation_live().subsystem_construction_count(),
        1
    );
    assert_eq!(live_state.operation_live().retained_resource_count(), 1);
    live.close();
}

fn update_first_row_cost(label: &str, width: usize) -> PatchCost {
    let identities = (0..width)
        .map(|index| format!("row-{index:03}"))
        .collect::<Vec<_>>();
    let identity_refs = identities.iter().map(String::as_str).collect::<Vec<_>>();
    let mut fixture = LiveBindingFixture::with_rows(label, &identity_refs, 4);
    fixture.owner.update_named_measurement("row-000");
    let consequence = applied(fixture.refresh().expect("real patch refresh succeeds"));
    let cost = PatchCost {
        ui: consequence.ui_counters(),
        query: consequence.query_work(),
    };
    fixture.admit_and_publish(consequence);
    fixture.close();
    cost
}
