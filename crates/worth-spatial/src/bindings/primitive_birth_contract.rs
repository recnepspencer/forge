use super::primitive_birth::PrimitiveConstructionBirthScaffoldInput;
use worth_primitives::PrimitiveConstructionBirthSynopsisContract;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionBirthContractCounts {
    vertex_count: usize,
    edge_count: usize,
    loop_count: usize,
    wire_count: usize,
    face_count: usize,
    shell_count: usize,
    body_count: usize,
}

impl PrimitiveConstructionBirthContractCounts {
    pub(crate) fn from_input(input: &PrimitiveConstructionBirthScaffoldInput) -> Self {
        Self {
            vertex_count: input.expected_vertex_count(),
            edge_count: input.expected_edge_count(),
            loop_count: input.expected_loop_count(),
            wire_count: input.expected_wire_count(),
            face_count: input.expected_face_count(),
            shell_count: input.expected_shell_count(),
            body_count: input.expected_body_count(),
        }
    }
}

pub(crate) fn primitive_birth_contract_matches_counts(
    contract: PrimitiveConstructionBirthSynopsisContract,
    counts: PrimitiveConstructionBirthContractCounts,
) -> bool {
    let topology = contract.topology_contract();
    counts.vertex_count == topology.vertex_count()
        && counts.edge_count == topology.edge_count()
        && counts.loop_count == topology.loop_count()
        && counts.wire_count == topology.wire_count()
        && counts.face_count == topology.face_count()
        && counts.shell_count == topology.shell_count()
        && counts.body_count == topology.body_count()
}

pub(crate) fn primitive_birth_contract_matches_support_planes(
    contract: PrimitiveConstructionBirthSynopsisContract,
    support_plane_count: usize,
) -> bool {
    support_plane_count == contract.support_contract().support_plane_count()
}
