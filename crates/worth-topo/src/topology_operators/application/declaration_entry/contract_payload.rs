use std::collections::BTreeMap;

use schema::facade::platform::entities::TopologyEntityKind;

use crate::topology_operators::NamingEditContinuityMatrix;
use crate::topology_operators::{
    naming_edit_continuity_matrix_for_contracts, topology_edit_digest_for_contracts,
    topology_edit_families_for_contracts, topology_edit_naming_report_for_contracts,
    TopologyAttachBoundaryMembershipDeclaration, TopologyAttachShellOrWireMembershipDeclaration,
    TopologyCreateInnerLoopOnExistingFaceDeclaration, TopologyCreateTopologyEntityDeclaration,
    TopologyDetachBoundaryMembershipDeclaration, TopologyDetachRadialAdjacencyDeclaration,
    TopologyDetachShellOrWireMembershipDeclaration, TopologyEditContract, TopologyEditDigest,
    TopologyEditFamily, TopologyEditNamingReport, TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration, TopologyRetireTopologyEntityDeclaration,
    TopologyRewireLoopEndpointDeclaration, TopologyRewireLoopSuccessorProgramDeclaration,
    TopologySpliceRadialAdjacencyDeclaration, TopologySpliceRadialAdjacencyProgramDeclaration,
    TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
};

pub(crate) trait TopologyDeclarationContractPayload: Clone {
    const SEMANTIC_FAMILY_KEY: &'static str;

    fn into_contracts(self) -> Vec<TopologyEditContract>;

    fn semantic_families(&self) -> Vec<TopologyEditFamily> {
        topology_edit_families_for_contracts(&self.clone().into_contracts())
    }

    fn created_entity_kinds(&self) -> BTreeMap<String, TopologyEntityKind> {
        self.clone()
            .into_contracts()
            .into_iter()
            .filter_map(|contract| match contract.action {
                crate::topology_operators::TopologyEditAction::CreateTopologyEntity {
                    create_key,
                    kind,
                    ..
                } => Some((create_key.as_str().to_string(), kind)),
                _ => None,
            })
            .collect()
    }

    fn topology_edit_digest(&self) -> TopologyEditDigest {
        topology_edit_digest_for_contracts(&self.clone().into_contracts())
    }

    fn naming_continuity_matrix(&self) -> NamingEditContinuityMatrix {
        naming_edit_continuity_matrix_for_contracts(&self.clone().into_contracts())
    }

    fn naming_report(&self) -> TopologyEditNamingReport {
        topology_edit_naming_report_for_contracts(&self.clone().into_contracts())
    }
}

macro_rules! impl_contract_backed_payload {
    ($ty:ty, $semantic:expr) => {
        impl TopologyDeclarationContractPayload for $ty {
            const SEMANTIC_FAMILY_KEY: &'static str = $semantic;

            fn into_contracts(self) -> Vec<TopologyEditContract> {
                self.into_contracts()
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
