use std::collections::{BTreeMap, BTreeSet};

use worth_ui_query_binding::{
    UiCollectionCompleteness, UiCollectionProjectionChange, UiProjectionAvailability,
    UiProjectionFactStopKind, WorthUiCollectionResetReason,
};

use super::super::{
    oracle::present,
    support::{CollectionProjectionWorld, WorldPosture},
};

#[test]
pub(crate) fn complete_partial_and_continuation_postures_come_from_real_query_results() {
    let (empty, empty_fact) = CollectionProjectionWorld::open(0, 1, WorldPosture::Complete, false);
    assert_eq!(
        present(&empty_fact).completeness(),
        UiCollectionCompleteness::Complete
    );
    assert!(present(&empty_fact).rows().is_empty());
    assert!(present(&empty_fact).continuation().is_none());
    empty.close();

    let (partial, partial_fact) =
        CollectionProjectionWorld::open(1, 1, WorldPosture::Partial, false);
    assert_eq!(
        present(&partial_fact).completeness(),
        UiCollectionCompleteness::Partial
    );
    partial.close();

    let (required, required_fact) =
        CollectionProjectionWorld::open(1, 1, WorldPosture::Partial, true);
    assert!(matches!(
        required_fact.availability(),
        UiProjectionAvailability::Stopped(stop)
            if stop.kind() == UiProjectionFactStopKind::PayloadShapeMismatch
    ));
    required.close();

    let (continued, continued_fact) =
        CollectionProjectionWorld::open(2, 1, WorldPosture::Complete, false);
    assert!(present(&continued_fact).continuation().is_some());
    continued.close();
}

#[test]
fn exact_insert_update_reorder_and_remove_keep_query_row_identity() {
    let (mut world, initial) = CollectionProjectionWorld::open(3, 4, WorldPosture::Complete, false);
    let initial_expected = world.expected().selected(world.identities());
    world
        .expected()
        .assert_fact_rows(&initial, &initial_expected);

    let changed = world.update_first(1);
    let updated = world.refresh();
    world
        .expected()
        .assert_fact_rows(&updated, &world.expected().selected(&changed));
    assert!(matches!(
        updated.changes(),
        [UiCollectionProjectionChange::Update { row }]
            if row.reporting_projection().as_str() == changed[0]
    ));

    let inserted_identity = world.insert("pulse.00000a", "Inserted");
    let inserted = world.refresh();
    world.expected().assert_fact_rows(
        &inserted,
        &world
            .expected()
            .selected(std::slice::from_ref(&inserted_identity)),
    );
    assert!(inserted.changes().iter().any(|change| matches!(
        change,
        UiCollectionProjectionChange::Insert { row, .. }
            if row.reporting_projection().as_str() == inserted_identity
    )));

    let stable_identity = world.reorder(0, "pulse.zzzzz");
    let reordered = world.refresh();
    world
        .expected()
        .assert_fact_rows(&reordered, &BTreeMap::new());
    assert_stable_move(&reordered, &stable_identity);

    let removed_identity = world.remove(3);
    let removed = world.refresh();
    world
        .expected()
        .assert_fact_rows(&removed, &BTreeMap::new());
    assert_removed(&removed, &removed_identity);
    world.close();
}

fn assert_stable_move(
    fact: &worth_ui_query_binding::UiCollectionProjectionFactReceipt,
    stable_identity: &str,
) {
    let moved = fact
        .changes()
        .iter()
        .filter_map(|change| match change {
            UiCollectionProjectionChange::Move { row, .. } => {
                Some(row.reporting_projection().as_str().to_owned())
            }
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    assert!(moved.contains(stable_identity));
}

fn assert_removed(
    fact: &worth_ui_query_binding::UiCollectionProjectionFactReceipt,
    removed_identity: &str,
) {
    assert!(fact.changes().iter().any(|change| matches!(
        change,
        UiCollectionProjectionChange::Remove { row, .. }
            if row.reporting_projection().as_str() == removed_identity
    )));
}

#[test]
pub(crate) fn continuation_completion_and_explicit_reset_are_preserved() {
    let (mut world, initial) = CollectionProjectionWorld::open(2, 1, WorldPosture::Complete, false);
    assert!(present(&initial).continuation().is_some());
    world.remove(0);
    let completed = world.refresh();
    assert!(present(&completed).continuation().is_none());
    assert!(completed
        .changes()
        .iter()
        .any(|change| matches!(change, UiCollectionProjectionChange::Remove { .. })));
    assert!(completed
        .changes()
        .iter()
        .any(|change| matches!(change, UiCollectionProjectionChange::Insert { .. })));
    world.close();

    let (mut reset_world, _) =
        CollectionProjectionWorld::open(1, 1, WorldPosture::ResetOnly, false);
    reset_world.update_first(1);
    let reset = reset_world.refresh();
    assert!(matches!(
        reset.changes(),
        [UiCollectionProjectionChange::ResetRequired {
            reason: WorthUiCollectionResetReason::UnsupportedIncrementalMeaning
        }]
    ));
    assert!(matches!(
        reset.availability(),
        UiProjectionAvailability::Stopped(stop)
            if stop.kind() == UiProjectionFactStopKind::ResetRequired
    ));
    reset_world.close();
}
