use super::production_phase_four_closeout;

#[test]
fn first_vertical_slice_selection_preserves_phase_three_identity() {
    let closeout = production_phase_four_closeout();
    let selected = closeout.selected_slice();

    assert!(!selected.requirement_identity().is_empty());
    assert!(!selected.source_posture_row_digest().is_empty());
    assert!(!selected.source_requirement_record_digest().is_empty());
    assert!(!selected.query_family_digest_seed().is_empty());
    assert!(!selected.query_posture().is_empty());
    assert!(!selected.slice_digest().is_empty());
}

#[test]
fn selected_slice_does_not_use_toy_fixture_when_production_candidate_exists() {
    let closeout = production_phase_four_closeout();
    let selected = closeout.selected_slice();

    assert!(
        selected.source_attempt_digest().is_some()
            || selected.source_carried_gap_digest().is_some(),
        "production slice should be anchored in Phase 2 attempt or carried-gap evidence"
    );
    assert_ne!("test_family", selected.query_family_name().unwrap_or(""));
}
