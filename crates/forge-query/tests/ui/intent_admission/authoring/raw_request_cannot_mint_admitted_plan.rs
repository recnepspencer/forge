use forge_query::facade::{
    ForgeQueryAdmittedIntentPlan, ForgeQueryIntentAdmissionExecutionSeam,
    ForgeQueryIntentDeclaration, ForgeQueryIntentInput, ForgeQueryRawIntentAdmissionRequest,
};

fn main() {
    let request = ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(
        ForgeQueryIntentDeclaration::strategy_commit(
            "raw-to-plan-forbidden",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            ForgeQueryIntentInput::object([("entity", ForgeQueryIntentInput::string("task-1"))]),
        ),
    )
    .unwrap();
    let eligibility = forge_query::facade::ForgeQueryIntentAdmissionEligibility::from_request(
        request,
    );
    let _plan = ForgeQueryAdmittedIntentPlan::from_eligibility(
        eligibility,
        ForgeQueryIntentAdmissionExecutionSeam::BackendIntentAuthorityRoute,
    );
}
