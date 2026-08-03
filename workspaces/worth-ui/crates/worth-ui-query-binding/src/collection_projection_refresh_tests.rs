use std::collections::BTreeSet;

use crate::{
    scalar_text_projection_fixture::{
        collection_projection_workspace, collection_projection_workspace_without_entity_lookup,
        insert_collection_status, update_identity, update_status,
    },
    UiCollectionProjectionBindingAdmission, UiCollectionProjectionBudget,
    UiCollectionProjectionChange, UiCollectionProjectionOpenOutcome,
    UiCollectionProjectionRefreshOutcome, UiPresentProjection, UiProjectionAvailability,
    UiProjectionFactStopKind, UiProjectionFieldRequirement, UiProjectionInputFactReference,
    UiProjectionInputPosture, UiProjectionInputSlot, UiProjectionInputTransitionStopKind,
    WorthUiCollectionResetReason, WorthUiQueryWorkspaceExt,
};

#[test]
fn refresh_translates_native_update_insert_remove_and_move_with_query_row_identity() {
    let mut workspace = collection_projection_workspace();
    let alpha = insert_collection_status(&mut workspace, "pulse.alpha", "Alpha");
    let bravo = insert_collection_status(&mut workspace, "pulse.bravo", "Bravo");
    let mut live = open(&mut workspace, 8);

    update_status(&mut workspace, bravo.clone(), "Bravo updated");
    let update = applied(&mut live, &mut workspace);
    assert_eq!(changed_values(update.fact()), ["Bravo updated"]);
    assert!(matches!(
        update.fact().changes(),
        [UiCollectionProjectionChange::Update { row }]
            if row.identity_for_reporting()
                == bravo.evidence_identity().terminal_projection_for_reporting()
    ));
    assert_changed_row_cost(&update, 1);

    let between = insert_collection_status(&mut workspace, "pulse.between", "Between");
    let insert = applied(&mut live, &mut workspace);
    assert_eq!(changed_values(insert.fact()), ["Between"]);
    assert!(insert.fact().changes().iter().any(|change| matches!(
        change,
        UiCollectionProjectionChange::Insert { row, .. }
            if row.identity_for_reporting()
                == between.evidence_identity().terminal_projection_for_reporting()
    )));
    assert_changed_row_cost(&insert, 1);

    workspace
        .delete(between.clone())
        .expect("delete inserted row");
    let remove = applied(&mut live, &mut workspace);
    assert!(changed_values(remove.fact()).is_empty());
    assert!(remove.fact().changes().iter().any(|change| matches!(
        change,
        UiCollectionProjectionChange::Remove { row, .. }
            if row.identity_for_reporting()
                == between.evidence_identity().terminal_projection_for_reporting()
    )));
    assert_changed_row_cost(&remove, 0);

    update_identity(&mut workspace, alpha.clone(), "pulse.zulu");
    let moved = applied(&mut live, &mut workspace);
    let actual_moves = moved
        .fact()
        .changes()
        .iter()
        .map(|change| match change {
            UiCollectionProjectionChange::Move { row, from, to } => {
                (row.identity_for_reporting().to_owned(), *from, *to)
            }
            other => panic!("identity reorder invented a non-move change: {other:?}"),
        })
        .collect::<BTreeSet<_>>();
    let expected_moves = BTreeSet::from([
        (
            alpha
                .evidence_identity()
                .terminal_projection_for_reporting()
                .to_owned(),
            0,
            1,
        ),
        (
            bravo
                .evidence_identity()
                .terminal_projection_for_reporting()
                .to_owned(),
            1,
            0,
        ),
    ]);
    assert_eq!(actual_moves, expected_moves);
    assert!(changed_values(moved.fact()).is_empty());
    assert_changed_row_cost(&moved, 0);
}

#[test]
fn changed_row_native_work_does_not_scale_with_collection_cardinality() {
    let mut small = collection_projection_workspace();
    let small_changed = insert_collection_status(&mut small, "pulse.0000", "Before");
    let mut small_live = open(&mut small, 2);
    update_status(&mut small, small_changed, "After");
    let small_receipt = applied(&mut small_live, &mut small);

    let mut large = collection_projection_workspace();
    let large_changed = insert_collection_status(&mut large, "pulse.0000", "Before");
    for row in 1..1_024 {
        insert_collection_status(
            &mut large,
            &format!("pulse.{row:04}"),
            &format!("Value {row}"),
        );
    }
    let mut large_live = open(&mut large, 1_024);
    update_status(&mut large, large_changed, "After");
    let large_receipt = applied(&mut large_live, &mut large);

    assert_changed_row_cost(&small_receipt, 1);
    assert_changed_row_cost(&large_receipt, 1);
    assert_eq!(
        small_receipt.fact().work(),
        large_receipt.fact().work(),
        "WUI native materialization must depend on changed rows, not collection cardinality"
    );
    assert_eq!(large_receipt.query_work().full_collection_scans(), 0);
    assert_eq!(large_receipt.query_work().unrelated_consumer_scans(), 0);
}

#[test]
fn refresh_preserves_live_continuation_changes_and_exposes_query_reset() {
    let mut workspace = collection_projection_workspace();
    let alpha = insert_collection_status(&mut workspace, "pulse.alpha", "Alpha");
    insert_collection_status(&mut workspace, "pulse.bravo", "Bravo");
    let mut live = open(&mut workspace, 1);

    workspace.delete(alpha).expect("delete continuation anchor");
    let shifted = applied(&mut live, &mut workspace);
    assert!(shifted
        .fact()
        .changes()
        .iter()
        .any(|change| matches!(change, UiCollectionProjectionChange::Remove { .. })));
    assert!(shifted
        .fact()
        .changes()
        .iter()
        .any(|change| matches!(change, UiCollectionProjectionChange::Insert { .. })));
    assert_eq!(changed_values(shifted.fact()), ["Bravo"]);
    assert!(present(shifted.fact()).continuation().is_none());

    let mut reset_workspace = collection_projection_workspace_without_entity_lookup();
    let row = insert_collection_status(&mut reset_workspace, "pulse.alpha", "Alpha");
    let mut reset_live = open(&mut reset_workspace, 1);
    update_status(&mut reset_workspace, row, "Updated");
    let reset = applied(&mut reset_live, &mut reset_workspace);
    assert!(matches!(
        reset.fact().changes(),
        [UiCollectionProjectionChange::ResetRequired {
            reason: WorthUiCollectionResetReason::UnsupportedIncrementalMeaning
        }]
    ));
    assert!(matches!(
        reset.fact().availability(),
        UiProjectionAvailability::Stopped(stop)
            if stop.kind() == UiProjectionFactStopKind::ResetRequired
    ));
}

#[test]
fn refresh_reports_empty_drain_without_inventing_semantic_delivery() {
    let mut workspace = collection_projection_workspace();
    insert_collection_status(&mut workspace, "pulse.alpha", "Alpha");
    let mut live = open(&mut workspace, 1);
    assert!(matches!(
        live.refresh(&mut workspace)
            .expect("an empty Query drain is not a refresh error"),
        UiCollectionProjectionRefreshOutcome::NoSemanticDelivery
    ));
}

#[test]
fn intent_input_catalog_applies_exact_query_patch_family_without_rebuilding_rows() {
    let mut workspace = collection_projection_workspace();
    let alpha = insert_collection_status(&mut workspace, "pulse.alpha", "Alpha");
    let bravo = insert_collection_status(&mut workspace, "pulse.bravo", "Bravo");
    let (mut live, snapshot) = open_with_snapshot(&mut workspace, 8);
    let slot = UiProjectionInputSlot::from_index(0).expect("slot zero is representable");
    let alpha_row = present(&snapshot).rows()[0].row().clone();
    let mut input = snapshot.intent_input_transition(slot).apply(None);
    assert_collection_input(&input, 2, 2, 0);
    let original_alpha = collection_input(&input)
        .current_option(&alpha_row)
        .expect("snapshot retains alpha");

    update_status(&mut workspace, bravo, "Bravo updated");
    let update = applied(&mut live, &mut workspace);
    let update_transition = update.fact().intent_input_transition(slot);
    let orphan = update_transition.apply(None);
    assert_eq!(
        collection_input(&orphan).posture(),
        UiProjectionInputPosture::TransitionStopped(
            UiProjectionInputTransitionStopKind::MissingPredecessor
        )
    );
    input = update_transition.apply(Some(&input));
    assert_collection_input(&input, 2, 0, 1);

    let between = insert_collection_status(&mut workspace, "pulse.between", "Between");
    let insert = applied(&mut live, &mut workspace);
    input = insert
        .fact()
        .intent_input_transition(slot)
        .apply(Some(&input));
    assert_collection_input(&input, 3, 0, insert.fact().changes().len());

    workspace.delete(between).expect("delete inserted row");
    let remove = applied(&mut live, &mut workspace);
    input = remove
        .fact()
        .intent_input_transition(slot)
        .apply(Some(&input));
    assert_collection_input(&input, 2, 0, remove.fact().changes().len());

    update_identity(&mut workspace, alpha, "pulse.zulu");
    let moved = applied(&mut live, &mut workspace);
    input = moved
        .fact()
        .intent_input_transition(slot)
        .apply(Some(&input));
    let moved_input = collection_input(&input);
    assert_collection_input(&input, 2, 0, 2);
    assert_eq!(moved_input.transition_work().node_copies(), 0);
    let current_alpha = moved_input
        .current_option(&alpha_row)
        .expect("move retains exact alpha identity");
    assert_eq!(
        current_alpha.identity_for_reporting(),
        original_alpha.identity_for_reporting()
    );
    assert_ne!(
        current_alpha.owner_revision(),
        original_alpha.owner_revision()
    );
}

fn open(
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
    rows: u32,
) -> crate::UiLiveCollectionProjection {
    open_with_snapshot(workspace, rows).0
}

fn open_with_snapshot(
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
    rows: u32,
) -> (
    crate::UiLiveCollectionProjection,
    crate::UiCollectionProjectionFactReceipt,
) {
    let installed = workspace.worth_ui().expect("Worth UI domain installed");
    let registration = crate::UiCollectionProjectionRegistration::text(
        installed
            .projection_view("support.collection.status")
            .expect("installed collection view"),
        UiProjectionFieldRequirement::declared("identity.id").expect("row field"),
        [UiProjectionFieldRequirement::declared("status").expect("selected field")],
        false,
        true,
    )
    .expect("collection registration");
    let UiCollectionProjectionBindingAdmission::Ready(binding) = registration.admit(workspace)
    else {
        panic!("canonical collection registration must admit");
    };
    let budget =
        UiCollectionProjectionBudget::new(rows, 64, 1, 1_048_576).expect("collection budget");
    let UiCollectionProjectionOpenOutcome::Opened(opened) = binding.open(budget, workspace) else {
        panic!("canonical collection binding must open");
    };
    opened.into_parts()
}

fn applied(
    live: &mut crate::UiLiveCollectionProjection,
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
) -> crate::UiCollectionProjectionRefreshReceipt {
    match live.refresh(workspace).expect("Query refresh pipeline") {
        UiCollectionProjectionRefreshOutcome::Applied(receipt) => receipt,
        UiCollectionProjectionRefreshOutcome::NoSemanticDelivery => {
            panic!("mutation must have a semantic collection effect")
        }
    }
}

fn changed_values(fact: &crate::UiCollectionProjectionFactReceipt) -> Vec<&str> {
    present(fact)
        .rows()
        .iter()
        .flat_map(|row| row.selected_values().iter())
        .map(crate::UiNativeTextValue::as_str)
        .collect()
}

fn present(fact: &crate::UiCollectionProjectionFactReceipt) -> &crate::UiCollectionProjectionValue {
    match fact.availability() {
        UiProjectionAvailability::Present(UiPresentProjection::Current(value)) => value,
        other => panic!("refresh fact was not current: {other:?}"),
    }
}

fn assert_changed_row_cost(
    receipt: &crate::UiCollectionProjectionRefreshReceipt,
    changed_rows: usize,
) {
    assert_eq!(receipt.fact().work().rows_visited(), changed_rows);
    assert_eq!(receipt.fact().work().selected_key_accesses(), changed_rows);
    assert_eq!(receipt.fact().work().indexed_row_lookups(), changed_rows);
    assert_eq!(
        receipt.fact().work().native_values_materialized(),
        changed_rows
    );
    assert_eq!(receipt.fact().work().unrelated_width_scans(), 0);
    assert_eq!(receipt.query_work().full_collection_scans(), 0);
    assert_eq!(receipt.query_work().unrelated_consumer_scans(), 0);
}

fn collection_input(
    input: &UiProjectionInputFactReference,
) -> &crate::UiCollectionProjectionInputFact {
    let UiProjectionInputFactReference::Collection(collection) = input else {
        panic!("collection projection retains collection input shape")
    };
    collection
}

fn assert_collection_input(
    input: &UiProjectionInputFactReference,
    rows: usize,
    replaced_rows: usize,
    change_operations: usize,
) {
    let input = collection_input(input);
    assert_eq!(input.posture(), UiProjectionInputPosture::Current);
    assert_eq!(input.row_count(), rows);
    assert_eq!(input.transition_work().replaced_rows(), replaced_rows);
    assert_eq!(
        input.transition_work().change_operations(),
        change_operations
    );
}
