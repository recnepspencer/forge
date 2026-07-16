use worth_query::facade::foundation::{MaterializedProjectionContract, ProjectMaterializedFacts, ProjectionConsumptionBindingContext, ProjectionConsumptionSource};
use worth_query::facade::domain::WorthQueryProjectionContractRequest;
use worth_query::facade::runtime::WorthQueryAdmittedIntentPlan;

#[path = "../support/installed_domain.rs"]
mod installed_domain;

fn aftermath_common_lane(
    plan: &WorthQueryAdmittedIntentPlan,
    source: ProjectionConsumptionSource,
    binding: ProjectionConsumptionBindingContext,
    requested_facts: ProjectMaterializedFacts,
) -> MaterializedProjectionContract {
    installed_domain::install("aftermath-golden")
        .contributions()
        .for_admitted_intent_plan(plan).expect("installed contribution authority must remain current")
        .consumes_projection_contract(
            "projection.contract",
            WorthQueryProjectionContractRequest::new(source, binding, requested_facts),
        )
        .because("admitted plan aftermath should bind a stable projection contract")
        .materialize()
        .expect("aftermath common lane should materialize")
}

fn main() {}
