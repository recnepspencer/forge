
use worth_query::facade::foundation::{worth_query_basis_observation_intent, basis_lifecycle};
use worth_query::facade::runtime::WorthQueryIntentAdmissionDecision;

fn basis_observation_common_path() {
    let scoped_basis = worth_query_basis_observation_intent(basis_lifecycle().current_head())
        .expect("basis observation authoring should build")
        .admit()
        .expect("basis observation should admit")
        .scope();
    let _ = scoped_basis.scoped_basis_digest();
}

fn basis_observation_advanced_path() {
    let review = worth_query_basis_observation_intent(basis_lifecycle().current_head())
        .expect("basis observation authoring should build")
        .review();
    let _ = review.request();
    let _ = review.eligibility();
    let _ = review.decision();
    let _ = review.decision_trace_envelope();
    let _ = review.consumer_inspection();
    match review.decision() {
        WorthQueryIntentAdmissionDecision::Admitted(_) => {}
        WorthQueryIntentAdmissionDecision::Advisory(_) => {}
        WorthQueryIntentAdmissionDecision::Violation(_) => {}
    }
    let admitted = review.admit().expect("basis observation should admit");
    let _ = admitted.plan();
    let _ = admitted.scope();
}

fn main() {}
