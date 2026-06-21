use topology::facade::{
    topology_primitive_construction_birth_graph_authority_proof,
    TopologyPrimitiveConstructionQueryReceipt,
};

fn main() {
    let receipt: Option<TopologyPrimitiveConstructionQueryReceipt> = None;
    if let Some(receipt) = receipt {
        let _ = topology_primitive_construction_birth_graph_authority_proof(&receipt);
    }
}
