use worth_query::facade::runtime::{worth_query_domain, WorthQueryAdmittedIntentPlan, WorthQueryProjectionContractRequest};

fn admitted_plan() -> WorthQueryAdmittedIntentPlan {
    todo!()
}

fn projection_contract() -> WorthQueryProjectionContractRequest {
    todo!()
}

fn main() {
    let _ = worth_query_domain("worth.spatial")
        .for_admitted_intent_plan(&admitted_plan())
        .consumes_projection_contract("projection.contract", projection_contract())
        .review();
}
