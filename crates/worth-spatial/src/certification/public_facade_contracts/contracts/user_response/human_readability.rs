use worth_spatial::facade::planar_predicates::{
    PlanarPredicateAuthorityFactError, PlanarPredicateEvaluationFailureKind,
};
use worth_spatial::facade::user_response::{
    HumanReadableResponse, HumanReadableResponseError, WorthUserResponseSource,
};

use super::contract_subject::{
    policy_required_response, unsupported_input_response, user_response,
};

#[test]
fn worth_user_outcome_rejects_machine_token_only_messages() {
    let error = HumanReadableResponse::new("signed-area-policy-required-before-overlap-imprint")
        .expect_err("machine token must not satisfy human response text");
    assert_eq!(error, HumanReadableResponseError::MachineTokenOnly);

    let policy = policy_required_response("user-response-readable-policy");
    assert_eq!(
        policy.human_response().summary(),
        "Signed area needs a user policy decision before overlap imprint."
    );
    assert!(policy.human_response().summary().contains(' '));

    let unsupported = unsupported_input_response("user-response-readable-unsupported");
    assert!(unsupported
        .human_response()
        .summary()
        .contains("unsupported"));
    assert!(unsupported.human_response().summary().contains(' '));
}

#[test]
fn user_response_constructor_rewrites_machine_token_source_messages() {
    let error = PlanarPredicateAuthorityFactError::PredicateEvaluation {
        kind: PlanarPredicateEvaluationFailureKind::CertifiedPredicateMathFailure,
        reason: "signed-area-policy-required-before-overlap-imprint".to_string(),
    };
    let response = user_response(WorthUserResponseSource::from_predicate_authority_error(
        &error,
    ));

    assert_ne!(
        response.human_response().summary(),
        "signed-area-policy-required-before-overlap-imprint"
    );
    assert!(response.human_response().summary().contains(' '));
    assert!(response
        .human_response()
        .summary()
        .contains("product-facing explanation"));
}
