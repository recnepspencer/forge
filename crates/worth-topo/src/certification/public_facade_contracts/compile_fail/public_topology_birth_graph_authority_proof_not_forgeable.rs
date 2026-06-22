use topology::facade::{
    TopologyConstructionQueryMutationSurface,
    TopologyPrimitiveConstructionBirthGraphAuthorityProof,
};

fn main() {
    let _ = TopologyPrimitiveConstructionBirthGraphAuthorityProof {
        mutation_surface: TopologyConstructionQueryMutationSurface::ComposeGraph,
        compose_program_digest: String::new(),
        execution_evidence_digest: String::new(),
        graph_obligation_envelope_digest: String::new(),
        graph_obligation_selected_count: 1,
        proof_digest: String::new(),
    };
}
