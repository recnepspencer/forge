use schema::facade::platform::authority::planner_owned_routing_semantic_graph::{
    PlannerMismatchLocus, PlannerWitnessIdentity, PlannerWitnessRole,
};

fn main() {
    let _ = PlannerWitnessIdentity {
        selected_route_identity_digest: "copied-route".to_string(),
        role: PlannerWitnessRole::DenialOrAdvisory,
        mismatch_locus: PlannerMismatchLocus::SelectedRoute,
        witness_reason: "rendered reason".to_string(),
        identity_digest: "copied-witness".to_string(),
    };
}
