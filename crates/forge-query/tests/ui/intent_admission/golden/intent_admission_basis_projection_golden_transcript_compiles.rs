
use forge_query::facade::{
    forge_query_basis_observation_intent, forge_query_projection_consumption_intent,
    ForgeQueryIntentAdmissionDecision, ProjectionConsumptionDeclaration, RawBasisIntent,
};

fn basis_observation_common_path() {
    let scoped_basis = forge_query_basis_observation_intent(RawBasisIntent::CurrentHead)
        .expect("basis observation authoring should build")
        .admit()
        .expect("basis observation should admit")
        .scope();
    let _ = scoped_basis.scoped_basis_digest();
}

fn basis_observation_advanced_path() {
    let review = forge_query_basis_observation_intent(RawBasisIntent::CurrentHead)
        .expect("basis observation authoring should build")
        .review();
    let _ = review.request();
    let _ = review.eligibility();
    let _ = review.decision();
    let _ = review.decision_trace_envelope();
    let _ = review.consumer_inspection();
    match review.decision() {
        ForgeQueryIntentAdmissionDecision::Admitted(_) => {}
        ForgeQueryIntentAdmissionDecision::Advisory(_) => {}
        ForgeQueryIntentAdmissionDecision::Violation(_) => {}
    }
    let admitted = review.admit().expect("basis observation should admit");
    let _ = admitted.plan();
    let _ = admitted.scope();
}

fn projection_consumption_common_path(declaration: ProjectionConsumptionDeclaration) {
    let contract = forge_query_projection_consumption_intent(declaration)
        .expect("projection authoring should build")
        .admit()
        .expect("projection consumption should admit")
        .bind_contract();
    let _ = contract.contract_digest();
}

fn projection_consumption_advanced_path(declaration: ProjectionConsumptionDeclaration) {
    let review = forge_query_projection_consumption_intent(declaration)
        .expect("projection authoring should build")
        .review();
    let _ = review.request();
    let _ = review.eligibility();
    let _ = review.decision();
    let _ = review.decision_trace_envelope();
    let _ = review.consumer_inspection();
    match review.decision() {
        ForgeQueryIntentAdmissionDecision::Admitted(_) => {}
        ForgeQueryIntentAdmissionDecision::Advisory(_) => {}
        ForgeQueryIntentAdmissionDecision::Violation(_) => {}
    }
    let admitted = review.admit().expect("projection consumption should admit");
    let _ = admitted.plan();
    let _ = admitted.bind_contract();
}

fn main() {}
