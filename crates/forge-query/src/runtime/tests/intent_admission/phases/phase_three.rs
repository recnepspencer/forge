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
fn read_admitted_decision_materializes_read_plan_and_handoff() {
    let runtime = read_runtime();
    let mut workspace =
        ForgeQueryWorkspace::new("phase-three-read", runtime).expect("workspace should build");
    let family = identity_read_family(&mut workspace, "tasks");
    let request =
        crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::read_family_entrypoint(
            crate::intent_admission::ForgeQueryReadExecutionIntentSeed::current_runtime(family),
        )
        .expect("read request should build");

    match admit_runtime_intent_request(request.clone()) {
        ForgeQueryIntentAdmissionDecision::Admitted(
            crate::intent_admission::ForgeQueryAdmittedIntentPlan::ReadExecution(plan),
        ) => {
            let handoff =
                crate::intent_admission::ForgeQueryReadExecutionHandoff::from_plan(plan.clone());
            assert_eq!(plan.family(), request.family());
            assert_eq!(plan.request_digest(), request.request_digest());
            assert_eq!(handoff.family(), request.family());
            assert_eq!(handoff.request_digest(), request.request_digest());
            assert_eq!(handoff.decision_digest(), plan.decision_digest());
        }
        other => panic!("expected read admitted plan, got {other:?}"),
    }
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

#[test]
fn basis_observation_admitted_decision_materializes_scoped_plan_without_handoff() {
    let request =
        crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::basis_observation_lane(
            crate::basis_lifecycle::RawBasisIntent::CurrentHead,
        )
        .expect("basis request should build");

    match admit_runtime_intent_request(request.clone()) {
        ForgeQueryIntentAdmissionDecision::Admitted(
            crate::intent_admission::ForgeQueryAdmittedIntentPlan::BasisObservation(plan),
        ) => {
            assert_eq!(plan.family(), request.family());
            assert_eq!(plan.request_digest(), request.request_digest());
            assert_eq!(plan.execution_seam(), None);
            let _scoped = plan.scope();
        }
        other => panic!("expected basis admitted plan, got {other:?}"),
    }
}

#[test]
fn projection_consumption_admitted_decision_materializes_contract_plan_without_handoff() {
    let declaration = crate::projection_consumption::declare_projection_consumption(
        crate::projection_consumption::ProjectionConsumptionSource::test_only(
            crate::projection_consumption::ProjectionSourceFamily::QueryReadReceipt,
            Some("query-digest"),
            Some("basis-digest"),
            Some("result-digest"),
            Some("shape-digest"),
            "query-read:test",
        ),
        crate::projection_consumption::ProjectionConsumptionBindingContext::test_only_with_projection_metadata(
            "shape-digest",
            "query-digest",
            "shape-digest",
            "projection.identity",
            "narrowed-shape-digest",
            "policy-digest",
            "tenant-schema-digest",
            vec!["field.visible".to_string()],
        ),
        crate::projection_consumption::ProjectMaterializedFacts::declare()
            .display_field("field.visible"),
    )
    .expect("projection declaration should build");
    let request =
        crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::projection_consumption(
            declaration,
        )
        .expect("projection request should build");

    match admit_runtime_intent_request(request.clone()) {
        ForgeQueryIntentAdmissionDecision::Admitted(
            crate::intent_admission::ForgeQueryAdmittedIntentPlan::ProjectionConsumption(plan),
        ) => {
            assert_eq!(plan.family(), request.family());
            assert_eq!(plan.request_digest(), request.request_digest());
            assert_eq!(plan.execution_seam(), None);
            let _contract = plan.bind_contract();
        }
        other => panic!("expected projection admitted plan, got {other:?}"),
    }
}
