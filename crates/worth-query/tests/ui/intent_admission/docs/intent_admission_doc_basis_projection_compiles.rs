
use worth_query::facade::{
    worth_query_basis_observation_intent, RawBasisIntent, WorthQueryIntentAdmissionDecision,
};

fn basis_observation_paths() {
    let scoped_basis = worth_query_basis_observation_intent(RawBasisIntent::CurrentHead)
        .expect("basis observation authoring should build")
        .admit()
        .expect("basis observation should admit")
        .scope();
    let _ = scoped_basis.scoped_basis_digest();

    let basis_review = worth_query_basis_observation_intent(RawBasisIntent::CurrentHead)
        .expect("basis observation authoring should build")
        .review();
    let _ = basis_review.request();
    let _ = basis_review.eligibility();
    let _ = basis_review.decision();
    let _ = basis_review.decision_trace_envelope();
    let _ = basis_review.consumer_inspection();
    match basis_review.decision() {
        WorthQueryIntentAdmissionDecision::Admitted(_) => {}
        WorthQueryIntentAdmissionDecision::Advisory(_) => {}
        WorthQueryIntentAdmissionDecision::Violation(_) => {}
    }
    let admitted = basis_review
        .admit()
        .expect("basis observation should admit");
    let _ = admitted.plan();
    let _ = admitted.scope();
}

fn main() {}
