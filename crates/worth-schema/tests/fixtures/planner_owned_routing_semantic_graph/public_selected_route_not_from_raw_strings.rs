use schema::facade::platform::authority::planner_owned_routing_semantic_graph::PlannerSelectedRouteIdentity;

fn main() {
    let _ = PlannerSelectedRouteIdentity {
        selected_family_identity_digest: "copied-family".to_string(),
        selected_route_name: "rendered-route".to_string(),
        identity_digest: "copied-digest".to_string(),
    };
}
