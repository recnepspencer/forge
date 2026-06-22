use topology::facade::{
    topology_primitive_construction_birth_graph_authority_proof,
    TopologyPrimitiveConstructionQueryAdmittedHandoff,
};

fn main() {
    let handoff: Option<TopologyPrimitiveConstructionQueryAdmittedHandoff> = None;
    if let Some(handoff) = handoff {
        let _ = topology_primitive_construction_birth_graph_authority_proof(&handoff);
    }
}
