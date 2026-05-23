use forge_query::facade::runtime::{
    admit_runtime_intent_request, forge_query_domain, ForgeQueryIntentAdmissionDecision,
    ForgeQueryRawIntentAdmissionRequest,
};
use forge_query::facade::RawBasisIntent;

fn main() {
    let request = ForgeQueryRawIntentAdmissionRequest::basis_observation_lane(RawBasisIntent::CurrentHead)
        .expect("basis observation request should build");
    let ForgeQueryIntentAdmissionDecision::Admitted(plan) = admit_runtime_intent_request(request)
    else {
        panic!("basis observation lane should admit");
    };

    let _ = forge_query_domain("worth.spatial")
        .for_admitted_intent_plan(&plan)
        .advises("arbitration.requires_clarification")
        .materialize();
}
