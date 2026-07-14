use worth_query::facade::runtime::{worth_query_domain, WorthQueryAdmittedIntentPlan};

fn continuity_common_lane(plan: &WorthQueryAdmittedIntentPlan) {
    let _continuity = worth_query_domain("worth.spatial")
        .for_admitted_intent_plan(plan)
        .preserves_continuity("identity.edge_split", "edge:before", "edge:after")
        .because("edge split replaces one edge with one canonical successor")
        .materialize();
}

fn main() {}
