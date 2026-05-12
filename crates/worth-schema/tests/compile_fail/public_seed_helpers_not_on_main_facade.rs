use schema::facade::{
    seed_minimal_topology, seed_milestone_one_primitive, seed_milestone_one_primitive_on_branch,
    verify_topology_intent, verify_topology_intent_on_branch,
};

fn main() {
    let _ = seed_minimal_topology;
    let _ = seed_milestone_one_primitive;
    let _ = seed_milestone_one_primitive_on_branch;
    let _ = verify_topology_intent;
    let _ = verify_topology_intent_on_branch;
}
