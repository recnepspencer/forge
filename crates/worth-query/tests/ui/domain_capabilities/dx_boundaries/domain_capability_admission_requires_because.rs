use worth_query::facade::runtime::{worth_query_domain, WorthQueryAdmittedIntentPlan};

fn missing_because(plan: &WorthQueryAdmittedIntentPlan) {
    let _ = worth_query_domain("worth.spatial")
        .for_admitted_intent_plan(plan)
        .advises("arbitration.requires_clarification")
        .materialize();
}

fn main() {}
