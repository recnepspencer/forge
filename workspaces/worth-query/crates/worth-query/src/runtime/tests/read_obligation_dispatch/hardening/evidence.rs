use super::super::*;
use super::read_declarations::profile_read_declaration;
use crate::runtime::WorthQueryReadInvariantPackViolation;

#[test]
fn read_and_live_decision_traces_carry_obligation_dispatch_digest() {
    let mut read_workspace = workspace_with_read_obligation(
        "read-trace-obligation",
        "user",
        WorthQueryGraphObligationSupportPosture::supported(
            WorthQueryGraphObligationSupportLane::ReadFamily,
        ),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );
    let family = identity_read_family(&mut read_workspace, "tasks");
    let read_result = read_workspace
        .execute_read_family(&family)
        .expect("read-family execution should dispatch matching obligation");
    let read_digest = read_result
        .graph_obligation_envelope_digest()
        .expect("read-family dispatch should materialize envelope");
    assert_eq!(
        read_result.receipt().graph_obligation_envelope_digest(),
        Some(read_digest)
    );
    assert_eq!(
        read_result
            .graph_obligation_evidence()
            .and_then(|evidence| evidence.envelope_digest().map(str::to_string))
            .as_deref(),
        Some(read_digest)
    );
    assert_eq!(
        read_result
            .graph_obligation_evidence()
            .map(|evidence| evidence.selected_obligation_count()),
        Some(1)
    );
    assert_eq!(
        read_result
            .receipt()
            .decision_trace_envelope()
            .and_then(|trace| trace.graph_obligation_envelope_digest()),
        Some(read_digest)
    );

    let mut live_workspace = workspace_with_live_read_obligation("live-trace-obligation");
    let live = live_view(&mut live_workspace);
    let live_result = live_workspace
        .read_live_result(&live)
        .expect("live read should dispatch matching obligation");
    let live_digest = live_result
        .graph_obligation_envelope_digest()
        .expect("live-read dispatch should materialize envelope");
    assert_eq!(
        live_result.receipt().graph_obligation_envelope_digest(),
        Some(live_digest)
    );
    assert_eq!(
        live_result
            .graph_obligation_evidence()
            .and_then(|evidence| evidence.envelope_digest().map(str::to_string))
            .as_deref(),
        Some(live_digest)
    );
    assert_eq!(
        live_result
            .graph_obligation_evidence()
            .map(|evidence| evidence.selected_obligation_count()),
        Some(1)
    );
    assert_eq!(
        live_result
            .receipt()
            .decision_trace_envelope()
            .and_then(|trace| trace.graph_obligation_envelope_digest()),
        Some(live_digest)
    );
}

#[test]
fn compose_read_front_matches_reusable_family_obligation_evidence() {
    let mut compose_workspace = workspace_with_read_obligation(
        "compose-read-obligation",
        "user",
        WorthQueryGraphObligationSupportPosture::supported(
            WorthQueryGraphObligationSupportLane::ReadFamily,
        ),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );
    let composed = compose_workspace
        .compose_read(profile_read_declaration)
        .expect("compose_read should execute through read-family obligation dispatch");

    let mut family_workspace = workspace_with_read_obligation(
        "family-read-obligation",
        "user",
        WorthQueryGraphObligationSupportPosture::supported(
            WorthQueryGraphObligationSupportLane::ReadFamily,
        ),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );
    let family = family_workspace
        .define_read_family("profile-read", profile_read_declaration)
        .expect("family should define");
    let executed = family_workspace
        .execute_read_family(&family)
        .expect("family execution should dispatch");

    assert_eq!(
        composed
            .receipt()
            .graph_obligation_dispatch()
            .unwrap()
            .selection()
            .matched_obligation_count(),
        executed
            .receipt()
            .graph_obligation_dispatch()
            .unwrap()
            .selection()
            .matched_obligation_count()
    );
    assert_eq!(
        composed
            .receipt()
            .decision_trace_envelope()
            .and_then(|trace| trace.graph_obligation_envelope_digest()),
        composed.receipt().graph_obligation_envelope_digest()
    );
}

#[test]
fn invariant_pack_family_dispatches_after_admission_and_denies_before_dispatch() {
    let mut admitted_workspace = workspace_with_read_obligation(
        "invariant-admitted-obligation",
        "user",
        WorthQueryGraphObligationSupportPosture::supported(
            WorthQueryGraphObligationSupportLane::ReadFamily,
        ),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );
    let family = admitted_workspace
        .define_read_family_with_invariant_pack(
            "profile-read",
            "profile_budget",
            profile_read_declaration,
            |_context| Ok(()),
        )
        .expect("admitted invariant pack should define reusable family");
    let result = admitted_workspace
        .execute_read_family(&family)
        .expect("invariant-admitted family should execute and dispatch");
    assert_eq!(
        result
            .receipt()
            .graph_obligation_dispatch()
            .unwrap()
            .selection()
            .matched_obligation_count(),
        1
    );

    let mut denied_workspace = workspace_with_read_obligation(
        "invariant-denied-before-dispatch",
        "user",
        WorthQueryGraphObligationSupportPosture::unsupported(
            WorthQueryGraphObligationSupportLane::ReadFamily,
        ),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );
    let error = denied_workspace
        .define_read_family_with_invariant_pack(
            "profile-read",
            "profile_budget",
            profile_read_declaration,
            |_context| {
                Err(WorthQueryReadInvariantPackViolation::new(
                    "profile_budget",
                    "profile reads are disabled in this test domain",
                ))
            },
        )
        .expect_err("domain invariant denial should happen before obligation dispatch");
    assert!(matches!(
        error,
        WorthQueryRuntimeError::ReadCompositionDomainInvariantDenied(_)
    ));
}

#[test]
fn row_only_read_sugar_erases_evidence_but_result_helper_retains_it() {
    let mut result_workspace = workspace_with_live_read_obligation("live-result-evidence");
    let result_view = live_view(&mut result_workspace);
    let result = result_workspace
        .read_live_result(&result_view)
        .expect("result helper should execute");
    assert!(result.receipt().graph_obligation_dispatch().is_some());
    assert!(result.receipt().decision_trace_envelope().is_some());

    let mut row_workspace = workspace_with_live_read_obligation("live-row-sugar");
    let row_view = live_view(&mut row_workspace);
    let rows = row_workspace.read(&row_view);
    assert_eq!(rows, result.rows());
}
