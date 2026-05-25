use forge_query::facade::{
    ProjectMaterializedFacts, ProjectionConsumptionBindingContext, ProjectionConsumptionSource,
};
use forge_query::facade::runtime::{
    ForgeQueryAdmittedPlanAftermathDraft, ForgeQueryAdmittedPlanBoundContributionTarget,
};

fn main() {
    let _ = ForgeQueryAdmittedPlanAftermathDraft {
        domain: "worth.spatial".to_string(),
        target: unsafe { std::mem::zeroed::<ForgeQueryAdmittedPlanBoundContributionTarget>() },
        posture: unsafe { std::mem::zeroed() },
        semantic_code: "projection.contract".to_string(),
        source: unsafe { std::mem::zeroed::<ProjectionConsumptionSource>() },
        binding: unsafe { std::mem::zeroed::<ProjectionConsumptionBindingContext>() },
        requested_facts: unsafe { std::mem::zeroed::<ProjectMaterializedFacts>() },
    };
}
