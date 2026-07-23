use worth_query::facade::domain;

use crate::{WorthUiCollectionAllocationPolicy, WorthUiCollectionPatchConsequences};

#[path = "collection_delivery_tests/mounted_graph.rs"]
mod mounted_graph;
#[path = "collection_delivery_tests/query_fixture.rs"]
mod query_fixture;

#[test]
fn admitted_query_patch_drives_ui_consequences_without_ui_query_authority() {
    let (mut workspace, entity) = query_fixture::workspace_with_measurement();
    let baseline = query_fixture::bound_collection(&mut workspace);
    let breadth = domain::WorthQueryCollectionWindowBreadth::new(1, 0, 0, 1).unwrap();
    let admitted_window = baseline
        .declare_window(baseline.beginning_cursor(), breadth)
        .unwrap();
    let baseline_window = baseline.resolve_window(admitted_window).unwrap();
    let mut mounted = mounted_graph::MountedUiCollection::from_query_window(&baseline_window);
    let mut consumer =
        domain::WorthQueryCollectionConsumerWindow::from_bound(baseline, baseline_window).unwrap();

    let lease = query_fixture::managed_lease(&mut workspace);
    assert!(lease.drain(&mut workspace).unwrap().delivery().is_empty());

    query_fixture::update_measurement(&mut workspace, entity.clone());
    let delivery = lease.drain(&mut workspace).unwrap();
    let delta = lease.consumer_invalidation_delta(delivery).unwrap();
    let admitted = match lease.admit_consumer_invalidation_delta(delta, &workspace) {
        Ok(admitted) => admitted,
        Err(stop) => panic!(
            "Worth UI collection invalidation did not readmit: {:?}",
            stop.kind()
        ),
    };
    consumer.bind_shared_target(&admitted, &workspace).unwrap();

    let fresh = query_fixture::bound_collection(&mut workspace);
    let fresh_admission = fresh
        .declare_window(fresh.beginning_cursor(), breadth)
        .unwrap();
    let fresh_window = fresh.resolve_window(fresh_admission).unwrap();
    let patch = match consumer.plan_patch(&admitted, &workspace) {
        domain::WorthQueryCollectionDeliveryOutcome::Patch(patch) => patch,
        domain::WorthQueryCollectionDeliveryOutcome::NoDelivery(denial) => {
            panic!("Worth UI mutation produced no patch: {:?}", denial.kind())
        }
    };
    assert!(patch.operations().iter().any(|operation| matches!(
        operation,
        domain::WorthQueryCollectionPatchOperation::Update { row }
            if row.entity_identity() == &entity
    )));
    assert_eq!(patch.facts().len(), 1);
    let receipt = consumer.apply_patch(patch).unwrap();
    assert_eq!(
        consumer
            .rows()
            .iter()
            .map(|row| row.entity_identity())
            .collect::<Vec<_>>(),
        fresh_window
            .rows()
            .iter()
            .map(|row| row.entity_identity())
            .collect::<Vec<_>>()
    );

    assert_ui_handoff(&receipt, &mut mounted, &fresh_window, &entity);
}

fn assert_ui_handoff(
    receipt: &domain::WorthQueryCollectionPatchApplicationReceipt,
    mounted: &mut mounted_graph::MountedUiCollection,
    fresh: &domain::WorthQueryBoundCollectionWindow,
    entity: &worth_query::facade::foundation::WorthQueryEntityIdentity,
) {
    let consequences = WorthUiCollectionPatchConsequences::from_query_receipt(
        receipt,
        WorthUiCollectionAllocationPolicy::PreserveMounted,
    );
    mounted.apply(&consequences);
    mounted.assert_fresh_parity(fresh);
    assert!(matches!(
        consequences.graph_mutations(),
        [crate::WorthUiCollectionGraphMutation::Update { row }]
            if row.entity_identity() == entity
    ));
    assert_eq!(
        consequences.measurement_invalidation_plan(),
        [crate::WorthUiCollectionMeasurementInvalidation::Row(
            entity.clone()
        )]
    );
    assert_eq!(consequences.graph_value_touches(), 1);
    assert_eq!(consequences.native_fact_touches(), 1);
    assert_eq!(consequences.measurement_invalidations(), 1);
    assert_eq!(consequences.mounted_identity_preservations(), 1);
    assert!(!consequences.reset_required());
}
