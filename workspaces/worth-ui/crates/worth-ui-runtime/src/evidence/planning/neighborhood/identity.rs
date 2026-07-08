use crate::declaration::stable_text_digest;
use crate::evidence::{UiAllocationNeighborhoodClass, UiLayoutOperatorContractIdentity};
use crate::graph::{UiGraphGeneration, UiGraphNodeIdentity};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationNeighborhoodIdentity {
    root_graph_node_identity: UiGraphNodeIdentity,
    graph_generation: UiGraphGeneration,
    world_identity_digest: u64,
    measurement_basis_identity_digest: u64,
    layout_operator_contract_identity: UiLayoutOperatorContractIdentity,
    dependency_map_identity_digest: u64,
    neighborhood_class: UiAllocationNeighborhoodClass,
    member_identity_digests: Box<[u64]>,
    identity_digest: u64,
}

impl UiAllocationNeighborhoodIdentity {
    pub(crate) fn new(
        root_graph_node_identity: UiGraphNodeIdentity,
        graph_generation: UiGraphGeneration,
        world_identity_digest: u64,
        measurement_basis_identity_digest: u64,
        layout_operator_contract_identity: UiLayoutOperatorContractIdentity,
        dependency_map_identity_digest: u64,
        neighborhood_class: UiAllocationNeighborhoodClass,
        mut member_identity_digests: Vec<u64>,
    ) -> Self {
        member_identity_digests.sort_unstable();
        let identity_digest = member_identity_digests.iter().fold(
            stable_text_digest("allocation-neighborhood-identity")
                ^ root_graph_node_identity.digest().rotate_left(7)
                ^ graph_generation.as_u64().rotate_left(13)
                ^ world_identity_digest.rotate_left(17)
                ^ measurement_basis_identity_digest.rotate_left(23)
                ^ layout_operator_contract_identity
                    .identity_digest()
                    .rotate_left(29)
                ^ dependency_map_identity_digest.rotate_left(31)
                ^ (neighborhood_class as u64).rotate_left(37),
            |digest, member_identity_digest| digest.rotate_left(11) ^ member_identity_digest,
        );

        Self {
            root_graph_node_identity,
            graph_generation,
            world_identity_digest,
            measurement_basis_identity_digest,
            layout_operator_contract_identity,
            dependency_map_identity_digest,
            neighborhood_class,
            member_identity_digests: member_identity_digests.into_boxed_slice(),
            identity_digest,
        }
    }

    pub fn root_graph_node_identity(&self) -> UiGraphNodeIdentity {
        self.root_graph_node_identity
    }

    pub fn graph_generation(&self) -> UiGraphGeneration {
        self.graph_generation
    }

    pub fn world_identity_digest(&self) -> u64 {
        self.world_identity_digest
    }

    pub fn measurement_basis_identity_digest(&self) -> u64 {
        self.measurement_basis_identity_digest
    }

    pub fn layout_operator_contract_identity(&self) -> UiLayoutOperatorContractIdentity {
        self.layout_operator_contract_identity
    }

    pub fn layout_operator_contract_identity_digest(&self) -> u64 {
        self.layout_operator_contract_identity.identity_digest()
    }

    pub fn dependency_map_identity_digest(&self) -> u64 {
        self.dependency_map_identity_digest
    }

    pub fn neighborhood_class(&self) -> UiAllocationNeighborhoodClass {
        self.neighborhood_class
    }

    pub fn member_identity_digests(&self) -> &[u64] {
        &self.member_identity_digests
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
}
