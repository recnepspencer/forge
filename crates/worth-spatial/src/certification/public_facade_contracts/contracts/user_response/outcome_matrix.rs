use worth_spatial::facade::user_response::{
    WorthDeniedCause, WorthIntegrityMismatchCause, WorthNoOptionsCause, WorthPolicyDecision,
    WorthUnsupportedCause, WorthUserOutcomeCauseKind, WorthUserOutcomeKind,
};

use super::contract_subject::{
    admitted_response, denied_movement_response, dirty_input_response, integrity_mismatch_response,
    policy_required_response, predicate_uncertain_response, unsupported_input_response,
};

#[test]
fn worth_user_outcome_classifies_admitted_policy_unsupported_denied_uncertain_integrity_and_no_options(
) {
    let responses = vec![
        admitted_response("user-response-matrix-admitted"),
        policy_required_response("user-response-matrix-policy"),
        unsupported_input_response("user-response-matrix-unsupported"),
        denied_movement_response("user-response-matrix-denied"),
        predicate_uncertain_response("user-response-matrix-predicate"),
        integrity_mismatch_response("user-response-matrix-integrity"),
        dirty_input_response("user-response-matrix-no-options"),
    ];

    assert_one_kind(&responses, WorthUserOutcomeKind::Admitted);
    assert_one_kind(&responses, WorthUserOutcomeKind::PolicyRequired);
    assert_one_kind(&responses, WorthUserOutcomeKind::Unsupported);
    assert_one_kind(&responses, WorthUserOutcomeKind::Denied);
    assert_one_kind(&responses, WorthUserOutcomeKind::PredicateUncertain);
    assert_one_kind(&responses, WorthUserOutcomeKind::IntegrityMismatch);
    assert_one_kind(&responses, WorthUserOutcomeKind::NoOptions);

    for response in &responses {
        assert!(!response.evidence().digest().is_empty());
        assert!(!response.evidence().source_identity().is_empty());
        assert!(response.human_response().summary().contains(' '));
        assert_eq!(
            response.stage_identity().upstream_receipt(),
            response.evidence().source_identity()
        );
    }

    assert_cause(
        &responses,
        WorthUserOutcomeKind::Unsupported,
        WorthUserOutcomeCauseKind::UnsupportedInput,
    );
    assert_unsupported_cause(&responses, WorthUnsupportedCause::UnsupportedInput);
    assert_cause(
        &responses,
        WorthUserOutcomeKind::Denied,
        WorthUserOutcomeCauseKind::DeniedMovementOrRotation,
    );
    assert_denied_cause(&responses, WorthDeniedCause::DeniedMovementOrRotation);
    assert_cause(
        &responses,
        WorthUserOutcomeKind::PredicateUncertain,
        WorthUserOutcomeCauseKind::PredicateUncertain,
    );
    assert_cause(
        &responses,
        WorthUserOutcomeKind::IntegrityMismatch,
        WorthUserOutcomeCauseKind::IntegrityMismatch,
    );
    assert_integrity_cause(
        &responses,
        WorthIntegrityMismatchCause::RetainedReplayProjectionDrift,
    );
    assert_cause(
        &responses,
        WorthUserOutcomeKind::NoOptions,
        WorthUserOutcomeCauseKind::DirtyInput,
    );
    assert_no_options_cause(&responses, WorthNoOptionsCause::DirtyInput);

    let policy = responses
        .iter()
        .find(|response| response.outcome().kind() == WorthUserOutcomeKind::PolicyRequired)
        .expect("policy response");
    assert_eq!(
        policy.outcome().choices(),
        &[
            WorthPolicyDecision::treat_candidate_as_inside_face(),
            WorthPolicyDecision::treat_candidate_as_outside_face(),
            WorthPolicyDecision::pause_for_manual_inspection(),
        ]
    );

    for response in responses
        .iter()
        .filter(|response| response.outcome().kind() != WorthUserOutcomeKind::PolicyRequired)
    {
        assert!(response.outcome().choices().is_empty());
    }
}

fn assert_no_options_cause(
    responses: &[worth_spatial::facade::user_response::WorthUserResponseReceipt],
    cause: WorthNoOptionsCause,
) {
    let response = response_by_kind(responses, WorthUserOutcomeKind::NoOptions);
    assert_eq!(
        response
            .outcome()
            .cause()
            .and_then(|cause| cause.no_options_cause()),
        Some(cause)
    );
}

fn assert_unsupported_cause(
    responses: &[worth_spatial::facade::user_response::WorthUserResponseReceipt],
    cause: WorthUnsupportedCause,
) {
    let response = response_by_kind(responses, WorthUserOutcomeKind::Unsupported);
    assert_eq!(
        response
            .outcome()
            .cause()
            .and_then(|cause| cause.unsupported_cause()),
        Some(cause)
    );
}

fn assert_denied_cause(
    responses: &[worth_spatial::facade::user_response::WorthUserResponseReceipt],
    cause: WorthDeniedCause,
) {
    let response = response_by_kind(responses, WorthUserOutcomeKind::Denied);
    assert_eq!(
        response
            .outcome()
            .cause()
            .and_then(|cause| cause.denied_cause()),
        Some(cause)
    );
}

fn assert_integrity_cause(
    responses: &[worth_spatial::facade::user_response::WorthUserResponseReceipt],
    cause: WorthIntegrityMismatchCause,
) {
    let response = response_by_kind(responses, WorthUserOutcomeKind::IntegrityMismatch);
    assert_eq!(
        response
            .outcome()
            .cause()
            .and_then(|cause| cause.integrity_mismatch_cause()),
        Some(cause)
    );
}

fn assert_one_kind(
    responses: &[worth_spatial::facade::user_response::WorthUserResponseReceipt],
    kind: WorthUserOutcomeKind,
) {
    assert_eq!(
        responses
            .iter()
            .filter(|response| response.outcome().kind() == kind)
            .count(),
        1
    );
}

fn assert_cause(
    responses: &[worth_spatial::facade::user_response::WorthUserResponseReceipt],
    kind: WorthUserOutcomeKind,
    cause: WorthUserOutcomeCauseKind,
) {
    let response = response_by_kind(responses, kind);
    assert_eq!(
        response.outcome().cause().map(|cause| cause.kind()),
        Some(cause)
    );
    assert!(response
        .outcome()
        .cause()
        .expect("cause")
        .human_reason()
        .contains(' '));
}

fn response_by_kind(
    responses: &[worth_spatial::facade::user_response::WorthUserResponseReceipt],
    kind: WorthUserOutcomeKind,
) -> &worth_spatial::facade::user_response::WorthUserResponseReceipt {
    responses
        .iter()
        .find(|response| response.outcome().kind() == kind)
        .expect("response kind")
}
