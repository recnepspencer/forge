#[path = "../support/installed_domain.rs"]
mod installed_domain;

use worth_query::facade::runtime::WorthQueryAdmittedIntentPlan;

fn missing_because(plan: &WorthQueryAdmittedIntentPlan) {
    let installation = installed_domain::install("admission-requires-because");
    let _ = installation
        .contributions()
        .for_admitted_intent_plan(plan).expect("installed contribution authority must remain current")
        .advises("arbitration.requires_clarification")
        .materialize();
}

fn main() {}
