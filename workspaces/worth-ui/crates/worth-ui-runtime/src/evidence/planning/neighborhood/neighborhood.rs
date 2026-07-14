use crate::evidence::{
    UiAllocationNeighborhoodClass, UiAllocationNeighborhoodIdentity,
    UiAllocationNeighborhoodMember, UiAllocationNeighborhoodMembershipRule,
    UiLayoutOperatorPlanningContract, UiMeasurementDependencyMap,
};
use crate::graph::{UiGraphGeneration, UiGraphNodeIdentity};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationNeighborhood {
    identity: UiAllocationNeighborhoodIdentity,
    layout_operator_planning_contract: UiLayoutOperatorPlanningContract,
    dependency_map: UiMeasurementDependencyMap,
    membership_rule: UiAllocationNeighborhoodMembershipRule,
    members: Box<[UiAllocationNeighborhoodMember]>,
}

impl UiAllocationNeighborhood {
    #[cfg(test)]
    pub(crate) fn new(
        root_graph_node_identity: UiGraphNodeIdentity,
        graph_generation: UiGraphGeneration,
        world_identity_digest: u64,
        measurement_basis_identity_digest: u64,
        dependency_map: UiMeasurementDependencyMap,
        neighborhood_class: UiAllocationNeighborhoodClass,
        members: Vec<UiAllocationNeighborhoodMember>,
        _: &crate::graph::UiAllocationNeighborhoodMintAuthority,
    ) -> Self {
        let membership_rule =
            UiAllocationNeighborhoodMembershipRule::default_for_class(neighborhood_class);
        Self::construct(
            root_graph_node_identity,
            graph_generation,
            world_identity_digest,
            world_identity_digest ^ graph_generation.as_u64().rotate_left(11),
            measurement_basis_identity_digest,
            UiLayoutOperatorPlanningContract::new(
                crate::declaration::UiDeclarationPlanningOperatorKind::Control,
                crate::evidence::UiLayoutOperatorFamily::Control,
                crate::evidence::UiLayoutOperatorContainmentKind::Control,
                None,
                crate::evidence::UiLayoutOperatorSlotParticipationKind::DeclaredParticipant,
                crate::declaration::UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
                crate::declaration::UiDeclarationRepetitionPosture::NotAdmitted,
                neighborhood_class,
                membership_rule,
                None,
                None,
                None,
                None,
                vec![],
            ),
            dependency_map,
            neighborhood_class,
            membership_rule,
            members,
        )
    }

    pub(crate) fn new_with_graph_authority(
        root_graph_node_identity: UiGraphNodeIdentity,
        graph_generation: UiGraphGeneration,
        world_identity_digest: u64,
        graph_snapshot_authority_digest: u64,
        measurement_basis_identity_digest: u64,
        layout_operator_planning_contract: UiLayoutOperatorPlanningContract,
        dependency_map: UiMeasurementDependencyMap,
        neighborhood_class: UiAllocationNeighborhoodClass,
        membership_rule: UiAllocationNeighborhoodMembershipRule,
        members: Vec<UiAllocationNeighborhoodMember>,
        _: &crate::graph::UiAllocationNeighborhoodMintAuthority,
    ) -> Self {
        Self::construct(
            root_graph_node_identity,
            graph_generation,
            world_identity_digest,
            graph_snapshot_authority_digest,
            measurement_basis_identity_digest,
            layout_operator_planning_contract,
            dependency_map,
            neighborhood_class,
            membership_rule,
            members,
        )
    }

    fn construct(
        root_graph_node_identity: UiGraphNodeIdentity,
        graph_generation: UiGraphGeneration,
        world_identity_digest: u64,
        graph_snapshot_authority_digest: u64,
        measurement_basis_identity_digest: u64,
        layout_operator_planning_contract: UiLayoutOperatorPlanningContract,
        dependency_map: UiMeasurementDependencyMap,
        neighborhood_class: UiAllocationNeighborhoodClass,
        membership_rule: UiAllocationNeighborhoodMembershipRule,
        mut members: Vec<UiAllocationNeighborhoodMember>,
    ) -> Self {
        members.sort_unstable_by_key(UiAllocationNeighborhoodMember::canonical_sort_key);
        let identity = UiAllocationNeighborhoodIdentity::new(
            root_graph_node_identity,
            graph_generation,
            world_identity_digest,
            graph_snapshot_authority_digest,
            measurement_basis_identity_digest,
            layout_operator_planning_contract.identity(),
            dependency_map.identity_digest(),
            neighborhood_class,
            members
                .iter()
                .map(UiAllocationNeighborhoodMember::identity_digest)
                .collect(),
        );

        Self {
            identity,
            layout_operator_planning_contract,
            dependency_map,
            membership_rule,
            members: members.into_boxed_slice(),
        }
    }

    pub fn identity(&self) -> &UiAllocationNeighborhoodIdentity {
        &self.identity
    }

    pub fn root_graph_node_identity(&self) -> UiGraphNodeIdentity {
        self.identity.root_graph_node_identity()
    }

    pub fn graph_generation(&self) -> UiGraphGeneration {
        self.identity.graph_generation()
    }

    pub fn world_identity_digest(&self) -> u64 {
        self.identity.world_identity_digest()
    }

    pub fn graph_snapshot_authority_digest(&self) -> u64 {
        self.identity.graph_snapshot_authority_digest()
    }

    pub fn measurement_basis_identity_digest(&self) -> u64 {
        self.identity.measurement_basis_identity_digest()
    }

    pub fn layout_operator_planning_contract(&self) -> &UiLayoutOperatorPlanningContract {
        &self.layout_operator_planning_contract
    }

    pub fn layout_operator_contract_identity(
        &self,
    ) -> crate::evidence::UiLayoutOperatorContractIdentity {
        self.layout_operator_planning_contract.identity()
    }

    pub fn layout_operator_contract_identity_digest(&self) -> u64 {
        self.layout_operator_planning_contract
            .identity()
            .identity_digest()
    }

    pub fn dependency_map(&self) -> &UiMeasurementDependencyMap {
        &self.dependency_map
    }

    pub fn neighborhood_class(&self) -> UiAllocationNeighborhoodClass {
        self.identity.neighborhood_class()
    }

    pub fn membership_rule(&self) -> UiAllocationNeighborhoodMembershipRule {
        self.membership_rule
    }

    pub fn members(&self) -> &[UiAllocationNeighborhoodMember] {
        &self.members
    }

    #[cfg(test)]
    pub(crate) fn new_for_evidence_test(
        root_graph_node_identity: UiGraphNodeIdentity,
        graph_generation: UiGraphGeneration,
        world_identity_digest: u64,
        measurement_basis_identity_digest: u64,
        layout_operator_planning_contract: UiLayoutOperatorPlanningContract,
        dependency_map: UiMeasurementDependencyMap,
        neighborhood_class: UiAllocationNeighborhoodClass,
        membership_rule: UiAllocationNeighborhoodMembershipRule,
        members: Vec<UiAllocationNeighborhoodMember>,
        _: &super::UiAllocationNeighborhoodEvidenceTestAuthority,
    ) -> Self {
        Self::construct(
            root_graph_node_identity,
            graph_generation,
            world_identity_digest,
            world_identity_digest ^ graph_generation.as_u64().rotate_left(11),
            measurement_basis_identity_digest,
            layout_operator_planning_contract,
            dependency_map,
            neighborhood_class,
            membership_rule,
            members,
        )
    }

    #[cfg(test)]
    pub(crate) fn new_for_graph_test(
        root_graph_node_identity: UiGraphNodeIdentity,
        graph_generation: UiGraphGeneration,
        world_identity_digest: u64,
        measurement_basis_identity_digest: u64,
        layout_operator_planning_contract: UiLayoutOperatorPlanningContract,
        dependency_map: UiMeasurementDependencyMap,
        neighborhood_class: UiAllocationNeighborhoodClass,
        membership_rule: UiAllocationNeighborhoodMembershipRule,
        members: Vec<UiAllocationNeighborhoodMember>,
        _: &crate::graph::UiAllocationNeighborhoodMintAuthority,
    ) -> Self {
        Self::construct(
            root_graph_node_identity,
            graph_generation,
            world_identity_digest,
            world_identity_digest ^ graph_generation.as_u64().rotate_left(11),
            measurement_basis_identity_digest,
            layout_operator_planning_contract,
            dependency_map,
            neighborhood_class,
            membership_rule,
            members,
        )
    }

    #[cfg(test)]
    pub(crate) fn with_members_for_graph_test(
        &self,
        members: Vec<UiAllocationNeighborhoodMember>,
    ) -> Self {
        Self::construct(
            self.root_graph_node_identity(),
            self.graph_generation(),
            self.world_identity_digest(),
            self.graph_snapshot_authority_digest(),
            self.measurement_basis_identity_digest(),
            self.layout_operator_planning_contract.clone(),
            self.dependency_map.clone(),
            self.neighborhood_class(),
            self.membership_rule,
            members,
        )
    }
}
