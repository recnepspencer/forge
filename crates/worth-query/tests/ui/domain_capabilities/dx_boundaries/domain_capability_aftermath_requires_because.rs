use worth_query::facade::foundation::{ProjectMaterializedFacts, ProjectionConsumptionBindingContext, ProjectionConsumptionSource};
use worth_query::facade::runtime::{WorthQueryAdmittedPlanAftermathDraft, WorthQueryAdmittedPlanBoundContributionTarget};

fn main() {
    let _ = WorthQueryAdmittedPlanAftermathDraft {
        domain: "worth.spatial".to_string(),
        target: unsafe { std::mem::zeroed::<WorthQueryAdmittedPlanBoundContributionTarget>() },
        posture: unsafe { std::mem::zeroed() },
        semantic_code: "projection.contract".to_string(),
        source: unsafe { std::mem::zeroed::<ProjectionConsumptionSource>() },
        binding: unsafe { std::mem::zeroed::<ProjectionConsumptionBindingContext>() },
        requested_facts: unsafe { std::mem::zeroed::<ProjectMaterializedFacts>() },
    };
}
