use schema::facade::platform::authority::planner_owned_routing_semantic_graph::PlannerSelectedRouteIdentity;
use schema::facade::topology_authoring::DerivedTruthBasisIdentity;

fn main() {
    let truth_basis = DerivedTruthBasisIdentity {
        mutation_digest_hex: "copied-truth-basis-digest".to_string(),
        touched_aspect_count: 1,
    };
    let _ = PlannerSelectedRouteIdentity {
        selected_family_identity_digest: truth_basis.mutation_digest_hex.clone(),
        selected_route_name: "copied-from-truth-basis".to_string(),
        identity_digest: truth_basis.mutation_digest_hex,
    };
}
