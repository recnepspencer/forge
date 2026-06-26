use super::super::closeout::current_worth_graph_read_access_posture_matrix_closeout;
use super::{
    phase_two_closeout_with_attempts_for_tests, production_phase_three_closeout,
    required_or_denied_attempt_for_tests,
};

#[test]
fn denied_posture_resolved_from_phase_two_does_not_emit_execution_receipt() {
    let phase_two = phase_two_closeout_with_attempts_for_tests(
        vec![required_or_denied_attempt_for_tests(
            "denied-requirement",
            "denied",
            "unsupported_graph_index_support",
        )],
        Vec::new(),
    );
    let phase_three = current_worth_graph_read_access_posture_matrix_closeout(&phase_two)
        .expect("Phase 3 should resolve denied postures from Phase 2");
    let resolved = phase_three
        .posture_map()
        .resolved_postures()
        .first()
        .expect("test closeout should contain the denied posture");

    assert!(!resolved.claims_access_plan_consumption());
    assert!(!resolved.claims_graph_read_execution());
    assert!(!resolved.claims_graph_read_receipt());
}

#[test]
fn required_posture_resolved_from_phase_two_does_not_emit_execution_receipt() {
    let phase_two = phase_two_closeout_with_attempts_for_tests(
        vec![required_or_denied_attempt_for_tests(
            "required-requirement",
            "persistent_index_required",
            "required_persistent_index",
        )],
        Vec::new(),
    );
    let phase_three = current_worth_graph_read_access_posture_matrix_closeout(&phase_two)
        .expect("Phase 3 should resolve required postures from Phase 2");
    let resolved = phase_three
        .posture_map()
        .resolved_postures()
        .first()
        .expect("test closeout should contain the required posture");

    assert!(!resolved.claims_access_plan_consumption());
    assert!(!resolved.claims_graph_read_execution());
    assert!(!resolved.claims_graph_read_receipt());
}

#[test]
fn production_required_postures_do_not_emit_execution_receipts() {
    let phase_three = production_phase_three_closeout();

    for row in phase_three.posture_map().resolved_postures() {
        assert!(!row.claims_access_plan_consumption());
        assert!(!row.claims_graph_read_execution());
        assert!(!row.claims_graph_read_receipt());
    }
}
