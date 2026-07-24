use crate::{
    WorthUiCollectionAllocationEffect, WorthUiCollectionChangeKind,
    WorthUiCollectionContinuationPosture, WorthUiCollectionGraphEffect,
    WorthUiCollectionMeasurementEffect, WorthUiCollectionResetReason,
    WorthUiOperationLiveCloseOutcome, WorthUiOperationLiveRefreshDenial,
    WorthUiOperationLiveRefreshError, WorthUiOperationLiveRefreshOutcome,
    WorthUiQueryBindingSuccessionDenial,
};

mod cost;
mod fixture;
use fixture::LiveBindingFixture;

#[test]
fn unpublished_live_change_denies_succession_before_the_candidate_is_consumed() {
    let mut fixture = LiveBindingFixture::new("worth-ui-succession-staged-change");
    fixture.owner.update_measurement();
    let consequence = match fixture.refresh().unwrap() {
        WorthUiOperationLiveRefreshOutcome::Applied(consequence) => consequence,
        WorthUiOperationLiveRefreshOutcome::NoSemanticDelivery => {
            panic!("semantic measurement update produced no UI consequence")
        }
    };
    fixture
        .binding
        .admit_operation_live_change(consequence)
        .unwrap();
    let candidate = fixture.owner.binding_plan().prepare_downstream_state();

    let denial = match candidate.prepare_regional_succession(&fixture.binding, std::iter::empty()) {
        Err(denial) => denial,
        Ok(_) => panic!("unpublished active Query change cannot cross succession"),
    };

    assert_eq!(
        denial,
        WorthUiQueryBindingSuccessionDenial::UnpublishedLiveChanges
    );
    assert_eq!(
        fixture
            .binding
            .publish_staged_operation_live_changes()
            .published_change_count(),
        1
    );
    fixture.close();
}

#[test]
fn retained_binding_mints_and_publishes_one_ui_owned_live_change() {
    let mut fixture = LiveBindingFixture::new("worth-ui-operation-live");

    assert!(matches!(
        fixture.refresh().unwrap(),
        WorthUiOperationLiveRefreshOutcome::NoSemanticDelivery
    ));

    fixture.owner.update_measurement();
    let consequence = match fixture.refresh().unwrap() {
        WorthUiOperationLiveRefreshOutcome::Applied(consequence) => consequence,
        WorthUiOperationLiveRefreshOutcome::NoSemanticDelivery => {
            panic!("semantic measurement update produced no UI consequence")
        }
    };

    assert_incremental_update(&consequence);
    let before_publication = fixture
        .binding
        .operation_live_change_observation_for(&fixture.reference)
        .unwrap();
    assert_eq!(before_publication.staged_change_count(), 1);
    assert_eq!(before_publication.admitted_change_count(), 0);
    assert_eq!(before_publication.next_change_order(), 1);

    let second_refresh = fixture.refresh().unwrap_err();
    assert!(matches!(
        second_refresh,
        WorthUiOperationLiveRefreshError::Ui(WorthUiOperationLiveRefreshDenial::PublicationPending)
    ));

    let staging = fixture
        .binding
        .admit_operation_live_change(consequence)
        .unwrap();
    assert_eq!(staging.change_order(), 1);
    let publication = fixture.binding.publish_staged_operation_live_changes();
    assert_eq!(publication.published_change_count(), 1);

    let after_publication = fixture
        .binding
        .operation_live_change_observation_for(&fixture.reference)
        .unwrap();
    assert_eq!(after_publication.staged_change_count(), 0);
    assert_eq!(after_publication.admitted_change_count(), 1);
    fixture.close();
}

#[test]
fn equivalent_rows_from_foreign_query_worlds_never_alias_ui_sources() {
    let mut left = LiveBindingFixture::new("worth-ui-source-left");
    let mut right = LiveBindingFixture::new("worth-ui-source-right");

    left.owner.update_measurement();
    right.owner.update_measurement();
    let left_change = applied(left.refresh().unwrap());
    let right_change = applied(right.refresh().unwrap());

    assert_ne!(left_change.source(), right_change.source());
    let left_row = changed_row(&left_change);
    let right_row = changed_row(&right_change);
    assert_ne!(left_row, right_row);

    let swapped = right
        .binding
        .admit_operation_live_change(left_change)
        .expect_err("foreign binding must reject an equivalent-looking consequence");
    assert_eq!(
        swapped.denial(),
        crate::WorthUiCollectionChangeAdmissionDenial::ForeignInstalledReference
    );
    left.binding
        .admit_operation_live_change(swapped.into_consequence())
        .unwrap();
    right
        .binding
        .admit_operation_live_change(right_change)
        .unwrap();
    left.close();
    right.close();
}

#[test]
fn dropped_pre_admission_handoff_is_recoverable_from_its_binding_owner() {
    let mut fixture = LiveBindingFixture::new("worth-ui-dropped-handoff");
    fixture.owner.update_measurement();
    let consequence = applied(fixture.refresh().unwrap());
    drop(consequence);

    let pending = fixture
        .binding
        .operation_live_change_observation_for(&fixture.reference)
        .unwrap();
    assert_eq!(pending.staged_change_count(), 1);
    assert_eq!(pending.admitted_change_count(), 0);

    let retry = fixture
        .binding
        .retry_operation_live_change_handoff(&fixture.reference)
        .expect("binding owner retains the exact applied consequence");
    fixture.binding.admit_operation_live_change(retry).unwrap();
    assert_eq!(
        fixture
            .binding
            .publish_staged_operation_live_changes()
            .published_change_count(),
        1
    );
    fixture.close();
}

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
        .any(|effect| matches!(effect, WorthUiCollectionGraphEffect::Remove { from: 2 })));
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
        .any(|effect| matches!(effect, WorthUiCollectionGraphEffect::Remove { from: 0 })));
    fixture.admit_and_publish(consequence);
    fixture.close();
}

#[test]
fn wrong_workspace_stops_before_consuming_the_real_pending_delivery() {
    let mut subject = LiveBindingFixture::new("worth-ui-wrong-workspace-subject");
    let mut foreign = LiveBindingFixture::new("worth-ui-wrong-workspace-foreign");
    subject.owner.update_measurement();

    let stopped = subject
        .binding
        .refresh_operation_live(foreign.owner.refresh_request_for(&subject.reference))
        .expect_err("foreign Query workspace must stop");
    assert!(matches!(
        stopped,
        WorthUiOperationLiveRefreshError::Drain(_)
    ));
    let unchanged = subject
        .binding
        .operation_live_change_observation_for(&subject.reference)
        .unwrap();
    assert_eq!(unchanged.staged_change_count(), 0);
    assert_eq!(unchanged.admitted_change_count(), 0);

    let consequence = applied(subject.refresh().expect("correct workspace remains usable"));
    subject.admit_and_publish(consequence);
    subject.close();
    foreign.close();
}

#[test]
fn interrupted_real_close_retains_the_exact_lease_for_one_retry() {
    let mut fixture = LiveBindingFixture::with_failed_close("worth-ui-close-retry");
    let resource = fixture
        .binding
        .take_operation_live_resource(&fixture.reference)
        .expect("fixture retains its live resource");
    let stopped = match fixture.owner.close_resource(resource) {
        WorthUiOperationLiveCloseOutcome::Stopped(stop) => stop,
        WorthUiOperationLiveCloseOutcome::Closed(_) => {
            panic!("injected Query close failure must retain the resource")
        }
    };
    assert_eq!(stopped.counters().close_attempts, 1);
    assert_eq!(stopped.counters().close_completions, 0);
    assert_eq!(stopped.counters().owner_reinsertions, 1);
    let receipt = match fixture.owner.close_resource(stopped.into_resource()) {
        WorthUiOperationLiveCloseOutcome::Closed(receipt) => receipt,
        WorthUiOperationLiveCloseOutcome::Stopped(_) => {
            panic!("one injected failure must recover on retry")
        }
    };
    assert!(receipt.owner_terminal());
    assert_eq!(receipt.counters().close_attempts, 1);
    assert_eq!(receipt.counters().close_completions, 1);
}

fn assert_incremental_update(consequence: &crate::WorthUiCollectionChangeConsequence) {
    let WorthUiCollectionChangeKind::Incremental(incremental) = consequence.kind() else {
        panic!("ordinary value update must not become a reset")
    };
    assert!(matches!(
        incremental.graph(),
        [WorthUiCollectionGraphEffect::Update { .. }]
    ));
    assert!(matches!(
        incremental.measurement(),
        [
            WorthUiCollectionMeasurementEffect::RowChanged(_),
            WorthUiCollectionMeasurementEffect::ChangedNativeFacts { count: 1 }
        ]
    ));
    assert!(matches!(
        incremental.allocation(),
        [WorthUiCollectionAllocationEffect::RowPreservationCandidate(
            _
        )]
    ));
    assert_eq!(consequence.ui_counters().patch_operations_visited(), 1);
    assert_eq!(consequence.ui_counters().patch_facts_reported(), 1);
    assert_eq!(consequence.query_work().operations_materialized(), 1);
    // This patch borrows the already-produced native fact. Query therefore
    // reports no new fact materialization while WUI records the one fact
    // reported by the exact applied receipt without claiming to scan it.
    assert_eq!(consequence.query_work().native_facts_materialized(), 0);
    assert_eq!(consequence.query_work().full_collection_scans(), 0);
    assert_eq!(consequence.query_work().unrelated_consumer_scans(), 0);
}

fn applied(
    outcome: WorthUiOperationLiveRefreshOutcome,
) -> crate::WorthUiCollectionChangeConsequence {
    match outcome {
        WorthUiOperationLiveRefreshOutcome::Applied(consequence) => consequence,
        WorthUiOperationLiveRefreshOutcome::NoSemanticDelivery => {
            panic!("semantic update must produce a consequence")
        }
    }
}

fn changed_row(
    consequence: &crate::WorthUiCollectionChangeConsequence,
) -> &crate::WorthUiCollectionRowReference {
    let WorthUiCollectionChangeKind::Incremental(incremental) = consequence.kind() else {
        panic!("test update must remain incremental")
    };
    let [WorthUiCollectionGraphEffect::Update { row }] = incremental.graph() else {
        panic!("test update must produce one graph-row effect")
    };
    row
}
