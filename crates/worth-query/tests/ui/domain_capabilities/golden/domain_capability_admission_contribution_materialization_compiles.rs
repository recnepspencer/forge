#[path = "../support/installed_domain.rs"]
mod installed_domain;

use worth_query::facade::runtime::WorthQueryAdmittedIntentPlan;

fn admission_common_lane(plan: &WorthQueryAdmittedIntentPlan) {
    let installation = installed_domain::install("admission-golden");
    let _decision = installation
        .contributions()
        .for_admitted_intent_plan(plan).expect("installed contribution authority must remain current")
        .advises("arbitration.requires_clarification")
        .because("multiple spatial candidates remain admissible")
        .materialize();
}

fn main() {}
