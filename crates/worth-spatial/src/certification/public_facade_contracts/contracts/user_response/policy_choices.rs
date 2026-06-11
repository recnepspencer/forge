use worth_spatial::facade::user_response::WorthUserOutcomeKind;

use super::contract_subject::{dirty_input_response, policy_required_response};

#[test]
fn user_response_offers_choices_only_for_policy_required_outcomes() {
    let policy = policy_required_response("user-response-policy-choices");
    assert_eq!(
        policy.outcome().kind(),
        WorthUserOutcomeKind::PolicyRequired
    );
    assert_eq!(policy.outcome().choices().len(), 3);
    assert!(policy
        .outcome()
        .choices()
        .iter()
        .all(|decision| decision.label().contains(' ')));

    let no_options = dirty_input_response("user-response-no-policy-choices");
    assert_eq!(no_options.outcome().kind(), WorthUserOutcomeKind::NoOptions);
    assert!(no_options.outcome().choices().is_empty());
    assert!(no_options.outcome().cause().is_some());
}
