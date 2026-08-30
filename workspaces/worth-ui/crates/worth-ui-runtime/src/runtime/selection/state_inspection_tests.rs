use super::state_test_fixture::*;
use super::*;

#[test]
fn interaction_drop_retains_its_latest_bounded_inspection_cause() {
    let owner = owner();
    let keys = (1..=2).map(key).collect::<Vec<_>>();
    let mut state = UiSelectionRuntimeState::new_session_restore_candidate();
    state
        .synchronize(registration(
            owner,
            UiSelectionPolicy::Multiple,
            keys.clone(),
            UiSelectionCatalogPosture::Complete,
        ))
        .unwrap();
    state
        .apply(
            owner,
            incarnation(),
            UiSelectionRequest::ToggleMultiple(keys[1]),
        )
        .unwrap();
    state
        .apply(
            owner,
            incarnation(),
            UiSelectionRequest::ToggleMultiple(keys[1]),
        )
        .unwrap();

    let inspection = state
        .last_drop()
        .expect("the owner retains the latest selection drop cause");
    assert_eq!(inspection.owner(), owner);
    assert_eq!(inspection.removed_count(), 1);
    assert_eq!(
        inspection.reason(),
        UiSelectionDropInspectionReason::Interaction
    );
}
