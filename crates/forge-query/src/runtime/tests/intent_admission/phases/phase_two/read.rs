use super::*;

#[test]
fn read_execution_eligibility_admits_runtime_current_route() {
    let runtime = read_runtime();
    let mut workspace = ForgeQueryWorkspace::new("read-intent-eligibility", runtime)
        .expect("workspace should build");
    let family = identity_read_family(&mut workspace, "tasks");
    let request =
        crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::read_family_entrypoint(
            crate::intent_admission::ForgeQueryReadExecutionIntentSeed::current_runtime(family),
        )
        .expect("read request should build");
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
        eligibility.basis_posture(),
        ForgeQueryIntentAdmissionBasisEligibility::ReadExecutionCurrentRuntimeAdmitted
    );
    assert_eq!(
        eligibility.routing_support_posture(),
        ForgeQueryIntentAdmissionRoutingSupportEligibility::CoveredExecutionSeam(
            crate::facade::runtime::ForgeQueryIntentAdmissionExecutionSeam::QueryRuntimeReadExecutionRoute
        )
    );
    assert_eq!(
        eligibility.pre_decision_posture(),
        ForgeQueryIntentAdmissionPreDecisionPosture::Admitted
    );
}

#[test]
fn read_execution_basis_context_mismatch_stops_as_phase_two_violation() {
    let runtime = read_runtime();
    let mut workspace = ForgeQueryWorkspace::new("read-intent-basis-mismatch", runtime)
        .expect("workspace should build");
    let family = identity_read_family(&mut workspace, "tasks");
    let mut unrelated_workspace = ForgeQueryWorkspace::new("read-intent-unrelated", read_runtime())
        .expect("workspace should build");
    let unrelated_family = profile_read_family(&mut unrelated_workspace, "other-tasks");
    let mismatched_context = current_context_for_family(&unrelated_family, "snapshot:mismatch");
    let request =
        crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::read_family_in_basis_context_entrypoint(
            crate::intent_admission::ForgeQueryReadExecutionIntentSeed::in_basis_context(
                family,
                mismatched_context,
            ),
        )
        .expect("basis-context request should build");
    let eligibility = crate::intent_admission::ForgeQueryIntentAdmissionEligibility::from_request(
        request.clone(),
    );
    let decision = admit_runtime_intent_request(request);

    assert_eq!(
        eligibility.capability_posture(),
        ForgeQueryIntentAdmissionCapabilityEligibility::Violation {
            stage: "read-basis-context-admission",
            detail: "read-basis-context query digest does not match requested read family",
        }
    );
    assert_eq!(
        eligibility.basis_posture(),
        ForgeQueryIntentAdmissionBasisEligibility::ReadExecutionBasisContextViolation(
            "read-basis-context query digest does not match requested read family"
        )
    );
    match decision {
        ForgeQueryIntentAdmissionDecision::Violation(violation) => {
            assert_eq!(violation.stage(), "read-basis-context-admission");
            assert_eq!(
                violation.message(),
                "read-basis-context query digest does not match requested read family"
            );
        }
        other => panic!("expected read basis mismatch violation, got {other:?}"),
    }
}
