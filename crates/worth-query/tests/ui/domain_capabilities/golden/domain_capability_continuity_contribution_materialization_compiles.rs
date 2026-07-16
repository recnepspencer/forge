#[path = "../support/installed_domain.rs"]
mod installed_domain;

use worth_query::facade::runtime::WorthQueryAdmittedIntentPlan;

fn continuity_common_lane(plan: &WorthQueryAdmittedIntentPlan) {
    let installation = installed_domain::install("continuity-golden");
    let _continuity = installation
        .contributions()
        .for_admitted_intent_plan(plan).expect("installed contribution authority must remain current")
        .preserves_continuity("identity.edge_split", "edge:before", "edge:after")
        .because("edge split replaces one edge with one canonical successor")
        .materialize();
}

fn main() {}
