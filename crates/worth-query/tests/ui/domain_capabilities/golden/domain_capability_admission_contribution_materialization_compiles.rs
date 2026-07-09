use worth_query::facade::runtime::{
    admit_runtime_intent_request, worth_query_domain, WorthQueryIntentAdmissionDecision,
    WorthQueryRawIntentAdmissionRequest,
};
use worth_query::facade::RawBasisIntent;

fn admission_common_lane() {
    let request = WorthQueryRawIntentAdmissionRequest::basis_observation_lane(RawBasisIntent::CurrentHead)
        .expect("basis observation request should build");
    let WorthQueryIntentAdmissionDecision::Admitted(plan) = admit_runtime_intent_request(request)
    else {
        panic!("basis observation lane should admit");
    };

    let _decision = worth_query_domain("worth.spatial")
        .for_admitted_intent_plan(&plan)
        .advises("arbitration.requires_clarification")
        .because("multiple spatial candidates remain admissible")
        .materialize();
}

fn main() {}
