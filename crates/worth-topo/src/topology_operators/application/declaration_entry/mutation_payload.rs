use crate::topology_operators::NamingMutationContinuityMatrix;
use crate::topology_operators::{
    TopologyAttachBoundaryMembershipDeclaration, TopologyAttachShellOrWireMembershipDeclaration,
    TopologyCreateInnerLoopOnExistingFaceDeclaration, TopologyCreateTopologyEntityDeclaration,
    TopologyDeclaredMutationSequence, TopologyDetachBoundaryMembershipDeclaration,
    TopologyDetachRadialAdjacencyDeclaration, TopologyDetachShellOrWireMembershipDeclaration,
    TopologyMutationDigest, TopologyMutationFamily,
    TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration, TopologyRetireTopologyEntityDeclaration,
    TopologyRewireLoopEndpointDeclaration, TopologyRewireLoopSuccessorProgramDeclaration,
    TopologySpliceRadialAdjacencyDeclaration, TopologySpliceRadialAdjacencyProgramDeclaration,
    TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
};

pub(crate) trait TopologyDeclarationMutationPayload: Clone {
    const SEMANTIC_FAMILY_KEY: &'static str;

    fn into_mutation_sequence(self) -> TopologyDeclaredMutationSequence;

    fn semantic_families(&self) -> Vec<TopologyMutationFamily> {
        self.clone().into_mutation_sequence().families().to_vec()
    }

    fn topology_mutation_digest(&self) -> TopologyMutationDigest {
        self.clone()
            .into_mutation_sequence()
            .topology_mutation_digest()
            .clone()
    }

    fn naming_continuity_matrix(&self) -> NamingMutationContinuityMatrix {
        self.clone()
            .into_mutation_sequence()
            .naming_continuity_matrix()
            .clone()
    }
}

macro_rules! impl_contract_backed_payload {
    ($ty:ty, $semantic:expr) => {
        impl TopologyDeclarationMutationPayload for $ty {
            const SEMANTIC_FAMILY_KEY: &'static str = $semantic;

            fn into_mutation_sequence(self) -> TopologyDeclaredMutationSequence {
                self.declared_mutation_sequence()
            }
        }
    };
}

impl_contract_backed_payload!(
    TopologyCreateTopologyEntityDeclaration,
    "topology.create_topology_entity"
);
impl_contract_backed_payload!(
    TopologyAttachBoundaryMembershipDeclaration,
    "topology.attach_boundary_membership"
);
impl_contract_backed_payload!(
    TopologyAttachShellOrWireMembershipDeclaration,
    "topology.attach_shell_or_wire_membership"
);
impl_contract_backed_payload!(
    TopologyCreateInnerLoopOnExistingFaceDeclaration,
    "topology.create_inner_loop_on_existing_face"
);
impl_contract_backed_payload!(
    TopologyRetireTopologyEntityDeclaration,
    "topology.retire_topology_entity"
);
impl_contract_backed_payload!(
    TopologyDetachBoundaryMembershipDeclaration,
    "topology.detach_boundary_membership"
);
impl_contract_backed_payload!(
    TopologyRewireLoopEndpointDeclaration,
    "topology.rewire_loop_endpoint"
);
impl_contract_backed_payload!(
    TopologyDetachShellOrWireMembershipDeclaration,
    "topology.detach_shell_or_wire_membership"
);
impl_contract_backed_payload!(
    TopologySpliceRadialAdjacencyDeclaration,
    "topology.splice_radial_adjacency"
);
impl_contract_backed_payload!(
    TopologyDetachRadialAdjacencyDeclaration,
    "topology.detach_radial_adjacency"
);
impl_contract_backed_payload!(
    TopologyRewireLoopSuccessorProgramDeclaration,
    "topology.rewire_loop_successor_program"
);
impl_contract_backed_payload!(
    TopologySpliceRadialAdjacencyProgramDeclaration,
    "topology.splice_radial_adjacency_program"
);
impl_contract_backed_payload!(
    TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
    "topology.rehome_all_owned_half_edges_to_new_wire"
);
impl_contract_backed_payload!(
    TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    "topology.rehome_all_owned_faces_to_new_shell"
);
impl_contract_backed_payload!(
    TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    "topology.split_connected_half_edge_set_to_new_wire"
);
impl_contract_backed_payload!(
    TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
    "topology.split_single_face_from_two_face_shell_to_new_shell"
);
