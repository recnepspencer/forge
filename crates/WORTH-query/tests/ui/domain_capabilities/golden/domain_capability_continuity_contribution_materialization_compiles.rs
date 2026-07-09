use worth_query::facade::runtime::{
    admit_runtime_intent_request, worth_query_domain, WorthQueryIntentAdmissionDecision,
    WorthQueryRawIntentAdmissionRequest,
};
use worth_query::facade::RawBasisIntent;

fn continuity_common_lane() {
    let request = WorthQueryRawIntentAdmissionRequest::basis_observation_lane(RawBasisIntent::CurrentHead)
        .expect("basis observation request should build");
    let WorthQueryIntentAdmissionDecision::Admitted(plan) = admit_runtime_intent_request(request)
    else {
        panic!("basis observation lane should admit");
    };

    let _continuity = worth_query_domain("worth.spatial")
        .for_admitted_intent_plan(&plan)
        .preserves_continuity("identity.edge_split", "edge:before", "edge:after")
        .because("edge split replaces one edge with one canonical successor")
        .materialize();
}

fn main() {}
