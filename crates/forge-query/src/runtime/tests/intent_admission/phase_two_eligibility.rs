use super::*;

#[test]
fn authoritative_runtime_floor_eligibility_carries_closed_pre_execution_facts() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "eligibility-authoritative-intent",
        "strategy.intent.reconcile",
        "1.0",
        "intent.reconcile.input.v1",
        json!({"entity": "task-1"}),
    );
    let request =
        crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(
            declaration,
        )
        .expect("authoritative request should build");
    let eligibility =
        crate::intent_admission::ForgeQueryIntentAdmissionEligibility::from_request(request);

    assert_eq!(
        eligibility.support_posture(),
        ForgeQueryIntentAdmissionSupportEligibility::Admitted
    );
    assert_eq!(
        eligibility.capability_posture(),
        ForgeQueryIntentAdmissionCapabilityEligibility::Admitted
    );
    assert_eq!(
        eligibility.policy_posture(),
        ForgeQueryIntentAdmissionPolicyEligibility::NotApplicableForRuntimeIntentFloor
    );
    assert_eq!(
        eligibility.basis_posture(),
        ForgeQueryIntentAdmissionBasisEligibility::NotApplicableForRuntimeIntentFloor
    );
    assert_eq!(
        eligibility.invariant_posture(),
        ForgeQueryIntentAdmissionInvariantEligibility::PreExecutionAuthorityRequired
    );
    assert_eq!(
        eligibility.projection_source_posture(),
        ForgeQueryIntentAdmissionProjectionSourceEligibility::NotApplicableForRuntimeIntentFloor
    );
    assert_eq!(
        eligibility.routing_support_posture(),
        ForgeQueryIntentAdmissionRoutingSupportEligibility::CoveredExecutionSeam(
            crate::facade::runtime::ForgeQueryIntentAdmissionExecutionSeam::BackendIntentAuthorityRoute
        )
    );
    assert_eq!(
        eligibility.source_lane_posture(),
        ForgeQueryIntentAdmissionSourceLaneEligibility::MatchesExpected(
            ForgeQueryIntentSourceLane::UserAuthored
        )
    );
    assert_eq!(
        eligibility.authority_lane_posture(),
        ForgeQueryIntentAdmissionAuthorityLaneEligibility::MatchesExpected(
            ForgeQueryAuthorityLane::AuthoritativeTruth
        )
    );
    assert_eq!(
        eligibility.pre_decision_posture(),
        ForgeQueryIntentAdmissionPreDecisionPosture::Admitted
    );
    assert_eq!(
        eligibility.admitted_execution_seam(),
        Some(
            crate::facade::runtime::ForgeQueryIntentAdmissionExecutionSeam::BackendIntentAuthorityRoute
        )
    );
}

#[test]
fn effect_runtime_floor_eligibility_carries_closed_pre_execution_facts() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "eligibility-effect-intent",
        "strategy.intent.reconcile",
        "1.0",
        "intent.reconcile.input.v1",
        json!({"entity": "task-1"}),
    )
    .with_source_lane(ForgeQueryIntentSourceLane::EffectTriggered);
    let request =
        crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::effect_runtime_entrypoint(
            declaration,
        )
        .expect("effect request should build");
    let eligibility =
        crate::intent_admission::ForgeQueryIntentAdmissionEligibility::from_request(request);

    assert_eq!(
        eligibility.support_posture(),
        ForgeQueryIntentAdmissionSupportEligibility::Admitted
    );
    assert_eq!(
        eligibility.capability_posture(),
        ForgeQueryIntentAdmissionCapabilityEligibility::Admitted
    );
    assert_eq!(
        eligibility.source_lane_posture(),
        ForgeQueryIntentAdmissionSourceLaneEligibility::MatchesExpected(
            ForgeQueryIntentSourceLane::EffectTriggered
        )
    );
    assert_eq!(
        eligibility.authority_lane_posture(),
        ForgeQueryIntentAdmissionAuthorityLaneEligibility::MatchesExpected(
            ForgeQueryAuthorityLane::AuthoritativeTruth
        )
    );
    assert_eq!(
        eligibility.admitted_execution_seam(),
        Some(
            crate::facade::runtime::ForgeQueryIntentAdmissionExecutionSeam::BackendIntentAuthorityRoute
        )
    );
}

#[test]
fn mismatched_runtime_floor_eligibility_stops_as_phase_two_violation() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "eligibility-mismatch-intent",
        "strategy.intent.reconcile",
        "1.0",
        "intent.reconcile.input.v1",
        json!({"entity": "task-1"}),
    )
    .with_source_lane(ForgeQueryIntentSourceLane::EffectTriggered);
    let request =
        crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(
            declaration,
        )
        .expect("authoritative request should build");
    let eligibility = crate::intent_admission::ForgeQueryIntentAdmissionEligibility::from_request(
        request.clone(),
    );
    let decision = admit_runtime_intent_request(request);

    assert_eq!(
        eligibility.capability_posture(),
        ForgeQueryIntentAdmissionCapabilityEligibility::Violation {
            stage: "source-lane-admission",
            detail: "covered-runtime-entrypoint-rejects-effect-triggered-source-lane",
        }
    );
    assert_eq!(
        eligibility.source_lane_posture(),
        ForgeQueryIntentAdmissionSourceLaneEligibility::Mismatch {
            expected: ForgeQueryIntentSourceLane::UserAuthored,
            actual: ForgeQueryIntentSourceLane::EffectTriggered,
        }
    );
    assert_eq!(
        eligibility.pre_decision_posture(),
        ForgeQueryIntentAdmissionPreDecisionPosture::Violation {
            stage: "source-lane-admission",
            message: "covered-runtime-entrypoint-rejects-effect-triggered-source-lane",
        }
    );
    match decision {
        ForgeQueryIntentAdmissionDecision::Violation(violation) => {
            assert_eq!(violation.stage(), "source-lane-admission");
            assert_eq!(
                violation.message(),
                "covered-runtime-entrypoint-rejects-effect-triggered-source-lane"
            );
        }
        other => panic!("expected violation decision, got {other:?}"),
    }
}

#[test]
fn authority_lane_mismatch_stops_as_phase_two_violation() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "eligibility-authority-mismatch-intent",
        "strategy.intent.reconcile",
        "1.0",
        "intent.reconcile.input.v1",
        json!({"entity": "task-1"}),
    )
    .with_target_lane(ForgeQueryAuthorityLane::PreviewTruth);
    let request =
        crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(
            declaration,
        )
        .expect("authoritative request should build");
    let eligibility = crate::intent_admission::ForgeQueryIntentAdmissionEligibility::from_request(
        request.clone(),
    );
    let decision = admit_runtime_intent_request(request);

    assert_eq!(
        eligibility.capability_posture(),
        ForgeQueryIntentAdmissionCapabilityEligibility::Violation {
            stage: "authority-admission",
            detail: "covered-runtime-entrypoint-rejects-preview-truth-target",
        }
    );
    assert_eq!(
        eligibility.authority_lane_posture(),
        ForgeQueryIntentAdmissionAuthorityLaneEligibility::Mismatch {
            expected: ForgeQueryAuthorityLane::AuthoritativeTruth,
            actual: ForgeQueryAuthorityLane::PreviewTruth,
        }
    );
    assert_eq!(
        eligibility.pre_decision_posture(),
        ForgeQueryIntentAdmissionPreDecisionPosture::Violation {
            stage: "authority-admission",
            message: "covered-runtime-entrypoint-rejects-preview-truth-target",
        }
    );
    match decision {
        ForgeQueryIntentAdmissionDecision::Violation(violation) => {
            assert_eq!(violation.stage(), "authority-admission");
            assert_eq!(
                violation.message(),
                "covered-runtime-entrypoint-rejects-preview-truth-target"
            );
        }
        other => panic!("expected violation decision, got {other:?}"),
    }
}

#[test]
fn deferred_neighbor_eligibility_stops_as_phase_two_advisory() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "eligibility-deferred-read",
        "strategy.intent.read",
        "1.0",
        "intent.read.input.v1",
        json!({"entity": "task-1"}),
    );
    let request = crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::deferred_neighbor(
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadNeighborDeferred,
        declaration,
    )
    .expect("deferred neighbor request should build");
    let eligibility = crate::intent_admission::ForgeQueryIntentAdmissionEligibility::from_request(
        request.clone(),
    );
    let decision = admit_runtime_intent_request(request);

    assert_eq!(
        eligibility.support_posture(),
        ForgeQueryIntentAdmissionSupportEligibility::Deferred(
            "read-execution-neighbor-deferred-until-covered"
        )
    );
    assert_eq!(
        eligibility.routing_support_posture(),
        ForgeQueryIntentAdmissionRoutingSupportEligibility::DeferredNeighbor(
            "read-execution-neighbor-deferred-until-covered"
        )
    );
    assert_eq!(
        eligibility.pre_decision_posture(),
        ForgeQueryIntentAdmissionPreDecisionPosture::Deferred {
            stage: "support-deferred",
            message: "read-execution-neighbor-deferred-until-covered",
        }
    );
    match decision {
        ForgeQueryIntentAdmissionDecision::Advisory(advisory) => {
            assert_eq!(advisory.stage(), "support-deferred");
            assert_eq!(
                advisory.message(),
                "read-execution-neighbor-deferred-until-covered"
            );
        }
        other => panic!("expected advisory decision, got {other:?}"),
    }
}

#[test]
fn eligibility_digest_changes_when_pre_decision_fact_meaning_changes() {
    let source_lane_violation = crate::intent_admission::ForgeQueryIntentAdmissionEligibility::from_request(
        crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(
            ForgeQueryIntentDeclaration::strategy_commit(
                "eligibility-source-digest-intent",
                "strategy.intent.reconcile",
                "1.0",
                "intent.reconcile.input.v1",
                json!({"entity": "task-1"}),
            )
            .with_source_lane(ForgeQueryIntentSourceLane::EffectTriggered),
        )
        .expect("authoritative request should build"),
    );
    let authority_lane_violation = crate::intent_admission::ForgeQueryIntentAdmissionEligibility::from_request(
        crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(
            ForgeQueryIntentDeclaration::strategy_commit(
                "eligibility-authority-digest-intent",
                "strategy.intent.reconcile",
                "1.0",
                "intent.reconcile.input.v1",
                json!({"entity": "task-1"}),
            )
            .with_target_lane(ForgeQueryAuthorityLane::PreviewTruth),
        )
        .expect("authoritative request should build"),
    );

    assert_ne!(
        source_lane_violation.eligibility_digest(),
        authority_lane_violation.eligibility_digest()
    );
}

#[test]
fn basis_observation_eligibility_admits_without_runtime_handoff() {
    let request =
        crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::basis_observation_lane(
            crate::basis_lifecycle::RawBasisIntent::CurrentHead,
        )
        .expect("basis observation request should build");
    let eligibility =
        crate::intent_admission::ForgeQueryIntentAdmissionEligibility::from_request(request);

    assert_eq!(
        eligibility.support_posture(),
        ForgeQueryIntentAdmissionSupportEligibility::Admitted
    );
    assert_eq!(
        eligibility.basis_posture(),
        ForgeQueryIntentAdmissionBasisEligibility::ObservationLifecycleAdmitted
    );
    assert_eq!(
        eligibility.routing_support_posture(),
        ForgeQueryIntentAdmissionRoutingSupportEligibility::NoExecutionHandoff(
            "basis-observation-admitted-plan-scopes-to-lower-runtime-evidence-without-query-execution-handoff"
        )
    );
    assert_eq!(eligibility.admitted_execution_seam(), None);
}

#[test]
fn projection_consumption_warning_eligibility_stays_admitted_without_execution_handoff() {
    let declaration = crate::projection_consumption::declare_projection_consumption(
        crate::projection_consumption::ProjectionConsumptionSource::test_only(
            crate::projection_consumption::ProjectionSourceFamily::QueryContextExecution,
            Some("query-digest"),
            Some("basis-digest"),
            Some("result-digest"),
            Some("shape-digest"),
            "query-context:test",
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
    let eligibility =
        crate::intent_admission::ForgeQueryIntentAdmissionEligibility::from_request(request);

    assert_eq!(
        eligibility.projection_source_posture(),
        ForgeQueryIntentAdmissionProjectionSourceEligibility::ProjectionConsumptionAdmittedWithWarnings(
            "projection-consumption-warning-bearing-admission"
        )
    );
    assert_eq!(eligibility.admitted_execution_seam(), None);
}
