#[allow(dead_code)]
mod graph_read_access_cost_model_support;
#[path = "support/mod.rs"]
mod support;

use worth_query::facade::runtime::{
    plan_admitted_graph_read_access_for_family, QueryPatchGroupKind,
    WorthQueryLiveGraphReadAccessPosture, WorthQueryLiveGraphReadMaintenanceBudget,
    WorthQueryNativeRow,
};

use graph_read_access_cost_model_support::{
    dense_traversal_family, simple_traversal_family, workspace,
};
use support::aspect_touch as touch;

#[test]
fn live_plan_preserves_one_shot_access_shape_and_required_index_digests() {
    let mut workspace = workspace("phase-thirteen-live-plan-parity");
    let family = simple_traversal_family(&mut workspace, "phase-thirteen-simple");
    let one_shot = match plan_admitted_graph_read_access_for_family(&family) {
        Ok(Some(plan)) => plan,
        Ok(None) => panic!("one-shot graph read did not admit"),
        Err(error) => panic!("one-shot graph read planning failed: {error:?}"),
    };

    let live_plan = workspace
        .plan_live_graph_read_access(&family, WorthQueryLiveGraphReadMaintenanceBudget::bounded())
        .expect("simple traversal should admit live maintenance");

    assert_eq!(
        live_plan.one_shot_access_shape_digest(),
        one_shot.admission().requirement_set().access_shape_digest()
    );
    assert_eq!(
        live_plan.required_index_digest(),
        one_shot.graph_index_support().requirement_set_digest()
    );
    assert_eq!(
        live_plan.posture(),
        &WorthQueryLiveGraphReadAccessPosture::AdmittedLiveIncrementalMaintenance
    );
    assert_eq!(
        live_plan
            .mutation_delta_scope()
            .affected_requirement_row_count(),
        one_shot.admission().requirement_set().rows().len()
    );
    assert!(!live_plan
        .mutation_delta_scope()
        .delta_scope_digest()
        .is_empty());
}

#[test]
fn live_planning_denies_one_shot_plan_that_requires_stronger_maintenance_support() {
    let mut workspace = workspace("phase-thirteen-live-plan-budget-denial");
    let family = simple_traversal_family(&mut workspace, "phase-thirteen-budget-denied");

    let denial = workspace
        .plan_live_graph_read_access(
            &family,
            WorthQueryLiveGraphReadMaintenanceBudget::strict_incremental(64, 16, 0),
        )
        .expect_err("one-shot safe read must not become live-maintained beyond budget");

    assert_eq!(
        denial.posture(),
        &WorthQueryLiveGraphReadAccessPosture::DeniedLiveMaintenanceBudget
    );
    assert!(!denial.digest().is_empty());
}

#[test]
fn dense_live_planning_denial_still_returns_live_specific_support_posture() {
    let mut workspace = workspace("phase-thirteen-live-plan-denial");
    let family = dense_traversal_family(&mut workspace, "phase-thirteen-dense");

    let denial = workspace
        .plan_live_graph_read_access(&family, WorthQueryLiveGraphReadMaintenanceBudget::bounded())
        .expect_err("dense broad read should not silently become live-maintained");

    assert!(matches!(
        denial.posture(),
        WorthQueryLiveGraphReadAccessPosture::DeniedLiveMaintenanceBudget
            | WorthQueryLiveGraphReadAccessPosture::DeniedLiveMaintenanceSupport
            | WorthQueryLiveGraphReadAccessPosture::LivePersistentIndexRequired
            | WorthQueryLiveGraphReadAccessPosture::LiveAsyncMaterializationRequired
            | WorthQueryLiveGraphReadAccessPosture::LiveStoreBackedCapabilityRequired
            | WorthQueryLiveGraphReadAccessPosture::LiveAccessCapabilityRegistrationRequired
    ));
    assert!(!denial.digest().is_empty());
}

#[test]
fn live_read_receipt_exposes_live_graph_access_counters() {
    let mut workspace = workspace("phase-thirteen-live-receipt");
    let live_view = workspace
        .live_view::<WorthQueryNativeRow>("tasks.table", |query| {
            query
                .from("Task")
                .select([
                    worth_query::facade::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    worth_query::facade::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                ])
                .order_by(
                    worth_query::facade::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                )
                .schema_basis("phase-thirteen-live-receipt")
        })
        .expect("live view should declare");

    let result = workspace
        .read_live_result(&live_view)
        .expect("live read should execute");
    let receipt = result
        .receipt()
        .live_graph_read_access()
        .expect("live read receipt should expose graph access proof");
    let counters = receipt.maintenance_counters();

    assert!(receipt.proves_no_caller_owned_n_plus_one());
    assert!(!receipt.mutation_delta_scope_digest().is_empty());
    assert_eq!(counters.per_result_neighbor_lookup_count(), 0);
    assert_eq!(counters.strategy_recompute_count(), 0);
    assert_eq!(counters.background_index_build_count(), 0);
    assert_eq!(counters.touched_edge_count(), 2);
    assert_eq!(counters.touched_frontier_count(), 1);
    assert_eq!(counters.affected_requirement_row_count(), 2);
    assert_eq!(counters.skipped_unaffected_requirement_count(), 0);
    assert_eq!(counters.index_update_count(), 2);
    assert_eq!(counters.live_view_update_count(), 0);
}

#[test]
fn live_mutation_delivery_carries_graph_read_maintenance_receipt() {
    let mut workspace = workspace("phase-thirteen-live-mutation-maintenance");
    let live_view = workspace
        .live_view::<WorthQueryNativeRow>("tasks.table", |query| {
            query
                .from("Task")
                .select([
                    worth_query::facade::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    worth_query::facade::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                ])
                .order_by(
                    worth_query::facade::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                )
                .schema_basis("phase-thirteen-live-mutation-maintenance")
        })
        .expect("live view should declare");

    workspace
        .insert("Task", |task| {
            task.set_aspect(touch("identity.id"), authored_text("task-maintained"))
                .set_aspect(touch("title.value"), authored_text("Maintained task"))
        })
        .expect("insert should execute through the public runtime");

    let patches = workspace.observe(&live_view);
    let delivery = patches
        .query_delivery_batches
        .first()
        .expect("projected insert should deliver one live batch");
    let maintenance = delivery
        .live_graph_read_maintenance()
        .expect("relational live delivery should carry graph read maintenance proof");
    let counters = maintenance.maintenance_counters();

    assert_eq!(delivery.patch_group_width(), 2);
    assert_eq!(
        delivery.patch_group_kind(),
        QueryPatchGroupKind::CollectionMembershipPatchGroup
    );
    assert!(!maintenance.digest().is_empty());
    assert!(!maintenance.maintenance_delta_for_reporting().is_empty());
    assert!(!delivery.delivery_cause_for_reporting().is_empty());
    assert!(!maintenance.live_access_plan_digest().is_empty());
    assert!(!maintenance.mutation_delta_scope_digest().is_empty());
    assert_eq!(counters.mutation_delta_count(), 1);
    assert_eq!(counters.affected_requirement_row_count(), 2);
    assert_eq!(counters.touched_edge_count(), 2);
    assert_eq!(counters.touched_frontier_count(), 1);
    assert_eq!(counters.skipped_unaffected_requirement_count(), 0);
    assert_eq!(counters.index_update_count(), 2);
    assert_eq!(counters.live_view_update_count(), 1);
    assert_eq!(counters.per_result_neighbor_lookup_count(), 0);
    assert_eq!(counters.strategy_recompute_count(), 0);
    assert_eq!(counters.background_index_build_count(), 0);
}

#[test]
fn live_maintenance_receipt_tracks_projected_updates_without_hidden_overdelivery() {
    let mut workspace = workspace("phase-thirteen-live-update-maintenance");
    let live_view = workspace
        .live_view::<WorthQueryNativeRow>("tasks.update.table", |query| {
            query
                .from("Task")
                .select([
                    worth_query::facade::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    worth_query::facade::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                ])
                .order_by(
                    worth_query::facade::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                )
                .schema_basis("phase-thirteen-live-update-maintenance")
        })
        .expect("live view should declare");
    let seed = workspace
        .insert("Task", |task| {
            task.set_aspect(touch("identity.id"), authored_text("task-updated"))
                .set_aspect(touch("title.value"), authored_text("Original title"))
                .set_aspect(touch("description.value"), authored_text("hidden"))
        })
        .expect("seed insert should execute");
    let entity_identity = seed.deltas()[0].entity_identity().clone();
    let _ = workspace.observe(&live_view);

    workspace
        .update(entity_identity.clone(), |task| {
            task.set_aspect(touch("title.value"), authored_text("Updated title"))
        })
        .expect("projected update should execute");
    let projected = workspace.observe(&live_view);
    let projected_maintenance = projected.query_delivery_batches[0]
        .live_graph_read_maintenance()
        .expect("projected update should carry maintenance proof");
    let projected_counters = projected_maintenance.maintenance_counters();

    workspace
        .update(entity_identity, |task| {
            task.set_aspect(touch("description.value"), authored_text("hidden again"))
        })
        .expect("hidden update should execute");
    let hidden_only = workspace.observe(&live_view);

    assert_eq!(
        projected.query_delivery_batches[0].patch_group_kind(),
        QueryPatchGroupKind::DetailFieldPatchGroup
    );
    assert_eq!(
        projected_maintenance
            .maintenance_counters()
            .per_result_neighbor_lookup_count(),
        0
    );
    assert_eq!(projected_counters.mutation_delta_count(), 1);
    assert_eq!(projected_counters.affected_requirement_row_count(), 2);
    assert_eq!(projected_counters.touched_edge_count(), 1);
    assert_eq!(projected_counters.touched_frontier_count(), 1);
    assert_eq!(projected_counters.skipped_unaffected_requirement_count(), 1);
    assert_eq!(projected_counters.index_update_count(), 1);
    assert_eq!(projected_counters.live_view_update_count(), 1);
    assert_eq!(projected_counters.strategy_recompute_count(), 0);
    assert_eq!(projected_counters.background_index_build_count(), 0);
    assert!(hidden_only.query_delivery_batches.is_empty());
}

fn authored_text(value: impl Into<String>) -> worth_query::facade::WorthQueryAuthoredAspectValue {
    worth_query::facade::WorthQueryAuthoredAspectValue::string(value)
}
