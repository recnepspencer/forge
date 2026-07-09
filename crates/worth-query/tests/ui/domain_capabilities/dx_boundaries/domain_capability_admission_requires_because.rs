use worth_query::facade::runtime::{
    admit_runtime_intent_request, worth_query_domain, WorthQueryIntentAdmissionDecision,
    WorthQueryRawIntentAdmissionRequest,
};
use worth_query::facade::RawBasisIntent;

fn main() {
    let request = WorthQueryRawIntentAdmissionRequest::basis_observation_lane(RawBasisIntent::CurrentHead)
        .expect("basis observation request should build");
    let WorthQueryIntentAdmissionDecision::Admitted(plan) = admit_runtime_intent_request(request)
    else {
        panic!("basis observation lane should admit");
    };

    let _ = worth_query_domain("worth.spatial")
        .for_admitted_intent_plan(&plan)
        .advises("arbitration.requires_clarification")
        .materialize();
}
