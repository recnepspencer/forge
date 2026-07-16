#[path = "../support/installed_domain.rs"]
mod installed_domain;

use worth_query::facade::runtime::WorthQueryAdmittedIntentPlan;

fn missing_because(plan: &WorthQueryAdmittedIntentPlan) {
    let installation = installed_domain::install("continuity-requires-because");
    let _ = installation
        .contributions()
        .for_admitted_intent_plan(plan).expect("installed contribution authority must remain current")
        .preserves_continuity("identity.edge_split", "edge:before", "edge:after")
        .materialize();
}

fn main() {}
