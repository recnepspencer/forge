use super::support::*;

#[test]
fn ordinary_write_does_not_accidentally_select_configured_policy_gate() {
    let mut runtime = supported_policy_gate_runtime("requires-policy-context");

    let receipt = runtime
        .write(task_insert_command("ordinary-write"))
        .expect("ordinary write should still execute");
    let dispatch = receipt
        .obligation_dispatch()
        .expect("descriptor-backed write keeps selection counters");

    assert_eq!(dispatch.selection().matched_obligation_count(), 0);
    assert!(dispatch.policy_gate().is_none());
    assert!(dispatch.envelope().is_none());
    assert_eq!(
        dispatch
            .selection()
            .counters()
            .registration_full_scan_count(),
        0
    );
}

#[test]
fn policy_context_without_selected_gate_does_not_materialize_gate_evidence() {
    let policy_context = policy_context("no-selected-gate", false);
    let mut runtime = complete_backend_from_parts_builder()
        .graph_obligation(policy_gate_registration_for_collection(
            "other-collection-gate",
            "Other",
            WorthQueryGraphObligationSupportPosture::supported(
                WorthQueryGraphObligationSupportLane::ScalarMutation,
            ),
        ))
        .build_backend_from_parts()
        .build()
        .expect("runtime should build with unmatched policy gate");

    let receipt = runtime
        .write_with_policy_context(task_insert_command("no-selected-gate"), policy_context)
        .expect("policy context without selected gate should not block write");
    let dispatch = receipt
        .obligation_dispatch()
        .expect("descriptor-backed write should keep selection evidence");

    assert_eq!(dispatch.selection().matched_obligation_count(), 0);
    assert!(dispatch.policy_gate().is_none());
    assert!(dispatch.envelope().is_none());
    assert!(
        dispatch
            .selection()
            .counters()
            .attempted_bucket_lookup_count()
            > 0
    );
    assert_eq!(
        dispatch
            .selection()
            .counters()
            .registration_full_scan_count(),
        0
    );
}
