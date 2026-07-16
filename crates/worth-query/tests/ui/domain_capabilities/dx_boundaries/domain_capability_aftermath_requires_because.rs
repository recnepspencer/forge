#[path = "../support/installed_domain.rs"]
mod installed_domain;

use worth_query::facade::domain::WorthQueryProjectionContractRequest;
use worth_query::facade::runtime::WorthQueryAdmittedIntentPlan;

fn admitted_plan() -> WorthQueryAdmittedIntentPlan {
    todo!()
}

fn projection_contract() -> WorthQueryProjectionContractRequest {
    todo!()
}

fn main() {
    let installation = installed_domain::install("aftermath-requires-because");
    let _ = installation
        .contributions()
        .for_admitted_intent_plan(&admitted_plan()).expect("installed contribution authority must remain current")
        .consumes_projection_contract("projection.contract", projection_contract())
        .materialize();
}
