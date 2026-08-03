use worth_ui_query_binding::{
    UiCollectionProjectionWorkCounters, WorthUiCollectionQueryWorkInspection,
};

use super::super::support::{CollectionProjectionWorld, WorldPosture};

#[test]
fn real_collection_work_scales_with_selected_and_changed_rows_not_cardinality() {
    let (empty, empty_fact) = CollectionProjectionWorld::open(0, 1, WorldPosture::Complete, false);
    assert_initial_work(empty_fact.work(), 0);
    empty.close();

    let one = changed_row_world(1, 1);
    assert_changed_work(one.0, one.1, 1);

    let medium = changed_row_world(1_024, 64);
    assert_changed_work(medium.0, medium.1, 64);
}

#[test]
#[ignore = "closure-only 65,536-row Query stress world; invoke explicitly"]
fn closure_stress_65536_rows_preserves_changed_row_cost() {
    let medium = changed_row_world(1_024, 64);
    let large = changed_row_world(65_536, 64);
    assert_changed_work(large.0, large.1, 64);
    assert_eq!(
        medium, large,
        "64 changed rows must cost the same at 1,024 and 65,536 rows"
    );
}

fn changed_row_world(
    cardinality: usize,
    changed_rows: usize,
) -> (
    UiCollectionProjectionWorkCounters,
    WorthUiCollectionQueryWorkInspection,
) {
    let selected_rows = cardinality.min(512);
    let (mut world, initial) = CollectionProjectionWorld::open(
        cardinality,
        selected_rows as u32,
        WorldPosture::Complete,
        false,
    );
    assert_eq!(
        world.cardinality(),
        cardinality,
        "the Query-owned seed receipt must account for the full collection"
    );
    assert_initial_work(initial.work(), selected_rows);
    let changed = world.update_first(changed_rows);
    let receipt = world.refresh_receipt();
    world
        .expected()
        .assert_fact_rows(receipt.fact(), &world.expected().selected(&changed));
    let work = receipt.fact().work();
    let query = *receipt.query_work();
    world.close();
    (work, query)
}

fn assert_initial_work(work: UiCollectionProjectionWorkCounters, selected_rows: usize) {
    assert_eq!(work.rows_visited(), selected_rows);
    assert_eq!(work.selected_key_accesses(), selected_rows);
    assert_eq!(work.indexed_row_lookups(), selected_rows);
    assert_eq!(work.native_values_materialized(), selected_rows);
    assert_eq!(work.unrelated_width_scans(), 0);
}

fn assert_changed_work(
    work: UiCollectionProjectionWorkCounters,
    query: WorthUiCollectionQueryWorkInspection,
    changed_rows: usize,
) {
    assert_eq!(work.rows_visited(), changed_rows);
    assert_eq!(work.selected_key_accesses(), changed_rows);
    assert_eq!(work.indexed_row_lookups(), changed_rows);
    assert_eq!(work.native_values_materialized(), changed_rows);
    assert_eq!(work.unrelated_width_scans(), 0);
    assert_eq!(query.native_facts_materialized(), changed_rows);
    assert_eq!(query.entity_point_lookups(), changed_rows);
    assert_eq!(query.full_collection_scans(), 0);
    assert_eq!(query.unrelated_consumer_scans(), 0);
}
