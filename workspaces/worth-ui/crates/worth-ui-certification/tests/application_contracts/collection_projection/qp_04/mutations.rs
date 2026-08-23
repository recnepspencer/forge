use std::collections::BTreeMap;

use worth_query::facade::runtime::WorthQueryEvidenceIdentityKey;
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
    let initial_order = world.identities().to_vec();
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
            if row.query_identity().operational_key() == changed[0]
    ));

    let inserted_identity = world.insert("pulse.00000a", "Inserted");
    let inserted = world.refresh();
    world.expected().assert_fact_rows(
        &inserted,
        &world
            .expected()
            .selected(std::slice::from_ref(&inserted_identity)),
    );
    assert_eq!(
        exact_changes(&inserted),
        vec![
            ExactChange::Insert(inserted_identity, 1),
            ExactChange::Move(initial_order[1], 1, 2),
            ExactChange::Move(initial_order[2], 2, 3),
        ]
    );

    let stable_identity = world.reorder(0, "pulse.zzzzz");
    let reordered = world.refresh();
    world
        .expected()
        .assert_fact_rows(&reordered, &BTreeMap::new());
    assert_eq!(stable_identity, initial_order[0]);
    assert_eq!(
        exact_changes(&reordered),
        vec![
            ExactChange::Move(inserted_identity, 1, 0),
            ExactChange::Move(initial_order[1], 2, 1),
            ExactChange::Move(initial_order[2], 3, 2),
            ExactChange::Move(initial_order[0], 0, 3),
        ]
    );

    let removed_identity = world.remove(3);
    let removed = world.refresh();
    world
        .expected()
        .assert_fact_rows(&removed, &BTreeMap::new());
    assert_eq!(removed_identity, inserted_identity);
    assert_eq!(
        exact_changes(&removed),
        vec![
            ExactChange::Remove(inserted_identity, 0),
            ExactChange::Move(initial_order[1], 1, 0),
            ExactChange::Move(initial_order[2], 2, 1),
            ExactChange::Move(initial_order[0], 3, 2),
        ]
    );
    world.close();
}

#[test]
pub(crate) fn continuation_completion_and_explicit_reset_are_preserved() {
    let (mut world, initial) = CollectionProjectionWorld::open(2, 1, WorldPosture::Complete, false);
    let initial_order = world.identities().to_vec();
    assert!(present(&initial).continuation().is_some());
    world.remove(0);
    let completed = world.refresh();
    assert!(present(&completed).continuation().is_none());
    assert_eq!(
        exact_changes(&completed),
        vec![
            ExactChange::Remove(initial_order[0], 0),
            ExactChange::Insert(initial_order[1], 0),
        ]
    );
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

#[derive(Debug, Eq, PartialEq)]
enum ExactChange {
    Insert(WorthQueryEvidenceIdentityKey, usize),
    Remove(WorthQueryEvidenceIdentityKey, usize),
    Move(WorthQueryEvidenceIdentityKey, usize, usize),
    Regroup(
        WorthQueryEvidenceIdentityKey,
        Option<Box<[String]>>,
        Option<Box<[String]>>,
    ),
    Update(WorthQueryEvidenceIdentityKey),
    WindowShift,
    ResetRequired(WorthUiCollectionResetReason),
}

fn exact_changes(
    fact: &worth_ui_query_binding::UiCollectionProjectionFactReceipt,
) -> Vec<ExactChange> {
    fact.changes()
        .iter()
        .map(|change| match change {
            UiCollectionProjectionChange::Insert { row, at } => {
                ExactChange::Insert(row.query_identity().operational_key(), *at)
            }
            UiCollectionProjectionChange::Remove { row, from } => {
                ExactChange::Remove(row.query_identity().operational_key(), *from)
            }
            UiCollectionProjectionChange::Move { row, from, to } => {
                ExactChange::Move(row.query_identity().operational_key(), *from, *to)
            }
            UiCollectionProjectionChange::Regroup { row, from, to } => ExactChange::Regroup(
                row.query_identity().operational_key(),
                from.clone(),
                to.clone(),
            ),
            UiCollectionProjectionChange::Update { row } => {
                ExactChange::Update(row.query_identity().operational_key())
            }
            UiCollectionProjectionChange::WindowShift => ExactChange::WindowShift,
            UiCollectionProjectionChange::ResetRequired { reason } => {
                ExactChange::ResetRequired(*reason)
            }
        })
        .collect()
}
