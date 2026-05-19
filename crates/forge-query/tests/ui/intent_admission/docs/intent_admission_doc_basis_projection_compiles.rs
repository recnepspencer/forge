
use forge_query::facade::{
    forge_query_basis_observation_intent, forge_query_projection_consumption_intent,
    ForgeQueryIntentAdmissionDecision, ProjectionConsumptionDeclaration, RawBasisIntent,
};

fn basis_observation_paths() {
    let scoped_basis = forge_query_basis_observation_intent(RawBasisIntent::CurrentHead)
        .expect("basis observation authoring should build")
        .admit()
        .expect("basis observation should admit")
        .scope();
    let _ = scoped_basis.scoped_basis_digest();

    let basis_review = forge_query_basis_observation_intent(RawBasisIntent::CurrentHead)
        .expect("basis observation authoring should build")
        .review();
    let _ = basis_review.request();
    let _ = basis_review.eligibility();
    let _ = basis_review.decision();
    let _ = basis_review.decision_trace_envelope();
    let _ = basis_review.consumer_inspection();
    match basis_review.decision() {
        ForgeQueryIntentAdmissionDecision::Admitted(_) => {}
        ForgeQueryIntentAdmissionDecision::Advisory(_) => {}
        ForgeQueryIntentAdmissionDecision::Violation(_) => {}
    }
    let admitted = basis_review
        .admit()
        .expect("basis observation should admit");
    let _ = admitted.plan();
    let _ = admitted.scope();
}

fn projection_consumption_paths(declaration: ProjectionConsumptionDeclaration) {
    let contract = forge_query_projection_consumption_intent(declaration.clone())
        .expect("projection authoring should build")
        .admit()
        .expect("projection consumption should admit")
        .bind_contract();
    let _ = contract.contract_digest();

    let projection_review = forge_query_projection_consumption_intent(declaration)
        .expect("projection authoring should build")
        .review();
    let _ = projection_review.request();
    let _ = projection_review.eligibility();
    let _ = projection_review.decision();
    let _ = projection_review.decision_trace_envelope();
    let _ = projection_review.consumer_inspection();
    match projection_review.decision() {
        ForgeQueryIntentAdmissionDecision::Admitted(_) => {}
        ForgeQueryIntentAdmissionDecision::Advisory(_) => {}
        ForgeQueryIntentAdmissionDecision::Violation(_) => {}
    }
    let admitted = projection_review
        .admit()
        .expect("projection consumption should admit");
    let _ = admitted.plan();
    let _ = admitted.bind_contract();
}

fn main() {}
