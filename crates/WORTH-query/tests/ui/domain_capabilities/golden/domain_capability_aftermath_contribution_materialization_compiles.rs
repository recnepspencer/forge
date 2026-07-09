use worth_query::facade::{
    WorthQueryProjectionContractRequest, MaterializedProjectionContract, ProjectMaterializedFacts,
    ProjectionConsumptionBindingContext, ProjectionConsumptionSource,
};
use worth_query::facade::runtime::{worth_query_domain, WorthQueryAdmittedIntentPlan};

fn aftermath_common_lane(
    plan: &WorthQueryAdmittedIntentPlan,
    source: ProjectionConsumptionSource,
    binding: ProjectionConsumptionBindingContext,
    requested_facts: ProjectMaterializedFacts,
) -> MaterializedProjectionContract {
    worth_query_domain("worth.spatial")
        .for_admitted_intent_plan(plan)
        .consumes_projection_contract(
            "projection.contract",
            WorthQueryProjectionContractRequest::new(source, binding, requested_facts),
        )
        .because("admitted plan aftermath should bind a stable projection contract")
        .materialize()
        .expect("aftermath common lane should materialize")
}

fn main() {}
