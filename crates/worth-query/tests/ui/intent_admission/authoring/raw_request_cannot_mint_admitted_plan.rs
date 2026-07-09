use worth_query::facade::{
    WorthQueryAdmittedIntentPlan, WorthQueryIntentAdmissionExecutionSeam,
    WorthQueryIntentDeclaration, WorthQueryIntentInput, WorthQueryRawIntentAdmissionRequest,
};

fn main() {
    let request = WorthQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(
        WorthQueryIntentDeclaration::strategy_commit(
            "raw-to-plan-forbidden",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            WorthQueryIntentInput::object([("entity", WorthQueryIntentInput::string("task-1"))]),
        ),
    )
    .unwrap();
    let eligibility = worth_query::facade::WorthQueryIntentAdmissionEligibility::from_request(
        request,
    );
    let _plan = WorthQueryAdmittedIntentPlan::from_eligibility(
        eligibility,
        WorthQueryIntentAdmissionExecutionSeam::BackendIntentAuthorityRoute,
    );
}
