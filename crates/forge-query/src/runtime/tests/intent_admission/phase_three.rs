use super::*;

#[test]
fn authoritative_admitted_decision_materializes_family_specific_plan_and_handoff() {
    let request = crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(
        ForgeQueryIntentDeclaration::strategy_commit(
            "phase-three-authoritative-intent",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            json!({"entity": "task-1"}),
        ),
    )
    .expect("authoritative request should build");

    match admit_runtime_intent_request(request.clone()) {
        ForgeQueryIntentAdmissionDecision::Admitted(
            crate::intent_admission::ForgeQueryAdmittedIntentPlan::Authoritative(plan),
        ) => {
            let handoff = ForgeQueryAuthoritativeIntentExecutionHandoff::from_plan(plan.clone());
            assert_eq!(plan.family(), request.family());
            assert_eq!(plan.request_digest(), request.request_digest());
            assert_eq!(handoff.family(), request.family());
            assert_eq!(handoff.request_digest(), request.request_digest());
            assert_eq!(handoff.decision_digest(), plan.decision_digest());
        }
        other => panic!("expected authoritative admitted plan, got {other:?}"),
    }
}

#[test]
fn effect_admitted_decision_materializes_family_specific_plan_and_handoff() {
    let request =
        crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::effect_runtime_entrypoint(
            ForgeQueryIntentDeclaration::strategy_commit(
                "phase-three-effect-intent",
                "strategy.intent.reconcile",
                "1.0",
                "intent.reconcile.input.v1",
                json!({"entity": "task-1"}),
            )
            .with_source_lane(ForgeQueryIntentSourceLane::EffectTriggered),
        )
        .expect("effect request should build");

    match admit_runtime_intent_request(request.clone()) {
        ForgeQueryIntentAdmissionDecision::Admitted(
            crate::intent_admission::ForgeQueryAdmittedIntentPlan::EffectTriggered(plan),
        ) => {
            let handoff = ForgeQueryEffectTriggeredIntentExecutionHandoff::from_plan(plan.clone());
            assert_eq!(plan.family(), request.family());
            assert_eq!(plan.request_digest(), request.request_digest());
            assert_eq!(handoff.family(), request.family());
            assert_eq!(handoff.request_digest(), request.request_digest());
            assert_eq!(handoff.decision_digest(), plan.decision_digest());
        }
        other => panic!("expected effect admitted plan, got {other:?}"),
    }
}

#[test]
fn advisory_decision_materializes_explicit_stop_artifact() {
    let request = crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::deferred_neighbor(
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadNeighborDeferred,
        ForgeQueryIntentDeclaration::strategy_commit(
            "phase-three-advisory-stop",
            "strategy.intent.read",
            "1.0",
            "intent.read.input.v1",
            json!({"entity": "task-1"}),
        ),
    )
    .expect("deferred request should build");

    let stop = admit_runtime_intent_request(request.clone())
        .into_non_admitted_stop()
        .expect("deferred decision should stop");
    assert!(matches!(stop, ForgeQueryIntentNonAdmittedStop::Advisory(_)));
    assert_eq!(stop.family(), request.family());
    assert_eq!(stop.request_digest(), request.request_digest());
    assert_eq!(stop.stage(), "support-deferred");
    assert!(!stop.stop_digest().is_empty());
}

#[test]
fn violation_decision_materializes_explicit_stop_artifact() {
    let request = crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(
        ForgeQueryIntentDeclaration::strategy_commit(
            "phase-three-violation-stop",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            json!({"entity": "task-1"}),
        )
        .with_target_lane(ForgeQueryAuthorityLane::PreviewTruth),
    )
    .expect("request should build");

    let stop = admit_runtime_intent_request(request.clone())
        .into_non_admitted_stop()
        .expect("violation decision should stop");
    assert!(matches!(
        stop,
        ForgeQueryIntentNonAdmittedStop::Violation(_)
    ));
    assert_eq!(stop.family(), request.family());
    assert_eq!(stop.request_digest(), request.request_digest());
    assert_eq!(stop.stage(), "authority-admission");
    assert!(!stop.stop_digest().is_empty());
}
