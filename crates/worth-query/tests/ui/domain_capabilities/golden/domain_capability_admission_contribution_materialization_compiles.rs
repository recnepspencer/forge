use worth_query::facade::runtime::{worth_query_domain, WorthQueryAdmittedIntentPlan};

fn admission_common_lane(plan: &WorthQueryAdmittedIntentPlan) {
    let _decision = worth_query_domain("worth.spatial")
        .for_admitted_intent_plan(plan)
        .advises("arbitration.requires_clarification")
        .because("multiple spatial candidates remain admissible")
        .materialize();
}

fn main() {}
