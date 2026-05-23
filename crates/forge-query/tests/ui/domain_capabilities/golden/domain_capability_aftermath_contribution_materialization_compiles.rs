use forge_query::facade::{
    ForgeQueryProjectionContractRequest, MaterializedProjectionContract, ProjectMaterializedFacts,
    ProjectionConsumptionBindingContext, ProjectionConsumptionSource,
};
use forge_query::facade::runtime::{forge_query_domain, ForgeQueryAdmittedIntentPlan};

fn aftermath_common_lane(
    plan: &ForgeQueryAdmittedIntentPlan,
    source: ProjectionConsumptionSource,
    binding: ProjectionConsumptionBindingContext,
    requested_facts: ProjectMaterializedFacts,
) -> MaterializedProjectionContract {
    forge_query_domain("worth.spatial")
        .for_admitted_intent_plan(plan)
        .consumes_projection_contract(
            "projection.contract",
            ForgeQueryProjectionContractRequest::new(source, binding, requested_facts),
        )
        .because("admitted plan aftermath should bind a stable projection contract")
        .materialize()
        .expect("aftermath common lane should materialize")
}

fn main() {}
