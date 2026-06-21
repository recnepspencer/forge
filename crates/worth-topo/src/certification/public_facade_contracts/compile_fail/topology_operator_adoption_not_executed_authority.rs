use forge_query::facade::consumer_kit::ForgeQueryGraphObligationAdoptionProof;
use topology::facade::topology_primitive_construction_birth_graph_authority_proof;

fn main() {
    let proof: Option<ForgeQueryGraphObligationAdoptionProof> = None;
    if let Some(proof) = proof {
        let _ = topology_primitive_construction_birth_graph_authority_proof(&proof);
    }
}
