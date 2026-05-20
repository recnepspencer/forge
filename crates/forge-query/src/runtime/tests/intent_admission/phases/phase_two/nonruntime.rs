use super::*;

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
