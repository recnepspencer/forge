use forge_query::facade::runtime::{
    forge_query_domain, ForgeQueryAdmittedIntentPlan, ForgeQueryProjectionContractRequest,
};

fn admitted_plan() -> ForgeQueryAdmittedIntentPlan {
    todo!()
}

fn projection_contract() -> ForgeQueryProjectionContractRequest {
    todo!()
}

fn main() {
    let _ = forge_query_domain("worth.spatial")
        .for_admitted_intent_plan(&admitted_plan())
        .consumes_projection_contract("projection.contract", projection_contract())
        .review();
}
