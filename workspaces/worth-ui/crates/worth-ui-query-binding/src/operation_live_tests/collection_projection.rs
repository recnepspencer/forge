use super::{applied, changed_row, LiveBindingFixture};
use crate::{
    WorthUiCollectionAllocationEffect, WorthUiCollectionChangeConsequence,
    WorthUiCollectionChangeKind, WorthUiCollectionContinuationPosture,
    WorthUiCollectionGraphEffect, WorthUiCollectionResetReason, WorthUiCollectionRowReference,
    WorthUiOperationLiveRefreshError,
};

#[test]
fn real_window_change_translates_insert_remove_move_and_continuation() {
    let mut fixture = LiveBindingFixture::with_rows(
        "worth-ui-window-vocabulary",
        &["alpha", "charlie", "echo"],
        3,
    );

    fixture.owner.insert_named_measurement("bravo");
    let consequence = applied(fixture.refresh().unwrap());
    let WorthUiCollectionChangeKind::Incremental(incremental) = consequence.kind() else {
        panic!("bounded insertion must remain incremental")
    };
    assert!(incremental
        .graph()
        .iter()
        .any(|effect| matches!(effect, WorthUiCollectionGraphEffect::Insert { at: 1, .. })));
    assert!(incremental
        .graph()
        .iter()
        .any(|effect| matches!(effect, WorthUiCollectionGraphEffect::Remove { from: 2, .. })));
    assert!(incremental.graph().iter().any(|effect| matches!(
        effect,
        WorthUiCollectionGraphEffect::Move { from: 1, to: 2, .. }
    )));
    assert_eq!(
        consequence.inspection().continuation(),
        Some(WorthUiCollectionContinuationPosture::AdditionalLiveRows)
    );
    fixture.admit_and_publish(consequence);

    fixture.owner.remove_named_measurement("bravo");
    let consequence = applied(fixture.refresh().unwrap());
    assert_eq!(
        consequence.inspection().continuation(),
        Some(WorthUiCollectionContinuationPosture::Complete)
    );
    fixture.admit_and_publish(consequence);
    fixture.close();
}

#[test]
fn unsupported_real_lookup_mints_one_reset_and_preserves_prior_ui_truth() {
    let mut fixture = LiveBindingFixture::without_collection_entity_lookup("worth-ui-real-reset");
    fixture.owner.update_measurement();
    let consequence = applied(fixture.refresh().unwrap());
    let WorthUiCollectionChangeKind::Reset(reset) = consequence.kind() else {
        panic!("unsupported point lookup must become a typed reset")
    };
    assert_eq!(
        reset.reason(),
        WorthUiCollectionResetReason::UnsupportedIncrementalMeaning
    );
    assert!(reset.fresh_execution_required());
    assert_eq!(reset.maximum_replacement_rows(), 1);
    assert_eq!(
        fixture
            .binding
            .operation_live_change_observation_for(&fixture.reference)
            .unwrap()
            .admitted_change_count(),
        0
    );
    fixture.admit_and_publish(consequence);

    fixture.owner.update_measurement();
    let stopped = fixture
        .refresh()
        .expect_err("reset-pending Query consumer stops");
    assert!(matches!(
        stopped,
        WorthUiOperationLiveRefreshError::Delivery(_)
    ));
    let observation = fixture
        .binding
        .operation_live_change_observation_for(&fixture.reference)
        .unwrap();
    assert_eq!(observation.staged_change_count(), 0);
    assert_eq!(observation.admitted_change_count(), 1);
    fixture.close();
}

#[test]
fn real_tail_cursor_translates_window_shift_without_exposing_query_cursor() {
    let mut fixture = LiveBindingFixture::with_tail_rows(
        "worth-ui-real-window-shift",
        &["alpha", "bravo", "charlie", "delta"],
        2,
    );
    fixture.owner.remove_named_measurement("charlie");
    let consequence = applied(fixture.refresh().unwrap());
    let WorthUiCollectionChangeKind::Incremental(incremental) = consequence.kind() else {
        panic!("tail-anchor removal must remain incremental")
    };
    assert!(incremental.allocation().iter().any(|effect| matches!(
        effect,
        WorthUiCollectionAllocationEffect::WindowShift { .. }
    )));
    assert!(incremental
        .graph()
        .iter()
        .any(|effect| matches!(effect, WorthUiCollectionGraphEffect::Remove { from: 0, .. })));
    fixture.admit_and_publish(consequence);
    fixture.close();
}

#[test]
fn query_row_identity_survives_reorder_across_distinct_applied_patches() {
    let mut fixture = LiveBindingFixture::with_rows(
        "worth-ui-stable-query-row-identity",
        &["alpha", "bravo", "charlie"],
        3,
    );
    fixture.owner.update_named_measurement("bravo");
    let updated = applied(fixture.refresh().unwrap());
    let before_reorder = changed_row(&updated).clone();
    fixture.admit_and_publish(updated);

    fixture.owner.rename_measurement("bravo", "aardvark");
    let reordered = applied(fixture.refresh().unwrap());
    let after_reorder = reordered_row(&reordered, 1, 0);

    assert_eq!(before_reorder, *after_reorder);
    assert_eq!(
        before_reorder.identity_for_reporting(),
        after_reorder.identity_for_reporting()
    );
    fixture.admit_and_publish(reordered);
    fixture.close();
}

fn reordered_row(
    consequence: &WorthUiCollectionChangeConsequence,
    expected_from: usize,
    expected_to: usize,
) -> &WorthUiCollectionRowReference {
    let WorthUiCollectionChangeKind::Incremental(incremental) = consequence.kind() else {
        panic!("test reorder must remain incremental")
    };
    incremental
        .graph()
        .iter()
        .find_map(|effect| match effect {
            WorthUiCollectionGraphEffect::Move { row, from, to }
                if *from == expected_from && *to == expected_to =>
            {
                Some(row)
            }
            _ => None,
        })
        .expect("the renamed Query entity must carry its row identity through the move")
}
