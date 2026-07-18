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

pub(crate) struct UiAllocationNeighborhoodInput {
    pub(crate) root_graph_node_identity: UiGraphNodeIdentity,
    pub(crate) graph_generation: UiGraphGeneration,
    pub(crate) world_identity_digest: u64,
    pub(crate) graph_snapshot_authority_digest: u64,
    pub(crate) measurement_basis_identity_digest: u64,
    pub(crate) layout_operator_planning_contract: UiLayoutOperatorPlanningContract,
    pub(crate) dependency_map: UiMeasurementDependencyMap,
    pub(crate) neighborhood_class: UiAllocationNeighborhoodClass,
    pub(crate) membership_rule: UiAllocationNeighborhoodMembershipRule,
    pub(crate) members: Vec<UiAllocationNeighborhoodMember>,
}

#[cfg(test)]
pub(crate) struct UiAllocationNeighborhoodTestInput {
    pub(crate) root_graph_node_identity: UiGraphNodeIdentity,
    pub(crate) graph_generation: UiGraphGeneration,
    pub(crate) world_identity_digest: u64,
    pub(crate) measurement_basis_identity_digest: u64,
    pub(crate) dependency_map: UiMeasurementDependencyMap,
    pub(crate) neighborhood_class: UiAllocationNeighborhoodClass,
    pub(crate) members: Vec<UiAllocationNeighborhoodMember>,
}

impl UiAllocationNeighborhood {
    #[cfg(test)]
    pub(crate) fn new(
        input: UiAllocationNeighborhoodTestInput,
        _: &crate::graph::UiAllocationNeighborhoodMintAuthority,
    ) -> Self {
        let UiAllocationNeighborhoodTestInput {
            root_graph_node_identity,
            graph_generation,
            world_identity_digest,
            measurement_basis_identity_digest,
            dependency_map,
            neighborhood_class,
            members,
        } = input;
        let membership_rule =
            UiAllocationNeighborhoodMembershipRule::default_for_class(neighborhood_class);
        Self::construct(UiAllocationNeighborhoodInput {
            root_graph_node_identity,
            graph_generation,
            world_identity_digest,
            graph_snapshot_authority_digest: world_identity_digest
                ^ graph_generation.as_u64().rotate_left(11),
            measurement_basis_identity_digest,
            layout_operator_planning_contract: UiLayoutOperatorPlanningContract::new(
                crate::evidence::UiLayoutOperatorPlanningContractInput {
                    operator_kind: crate::declaration::UiDeclarationPlanningOperatorKind::Control,
                    operator_family: crate::evidence::UiLayoutOperatorFamily::Control,
                    containment_kind: crate::evidence::UiLayoutOperatorContainmentKind::Control,
                    mosaic_sizing_contract_id: None,
                    slot_participation_kind:
                        crate::evidence::UiLayoutOperatorSlotParticipationKind::DeclaredParticipant,
                    ordering_guarantee:
                        crate::declaration::UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
                    repetition_posture:
                        crate::declaration::UiDeclarationRepetitionPosture::NotAdmitted,
                    neighborhood_class,
                    membership_rule,
                    measurement_mode: None,
                    constraint_modifier: None,
                    basis_source: None,
                    ownership_posture: None,
                    evidence_requirements: vec![],
                },
            ),
            dependency_map,
            neighborhood_class,
            membership_rule,
            members,
        })
    }

    pub(crate) fn new_with_graph_authority(
        input: UiAllocationNeighborhoodInput,
        _: &crate::graph::UiAllocationNeighborhoodMintAuthority,
    ) -> Self {
        Self::construct(input)
    }

    fn construct(input: UiAllocationNeighborhoodInput) -> Self {
        let UiAllocationNeighborhoodInput {
            root_graph_node_identity,
            graph_generation,
            world_identity_digest,
            graph_snapshot_authority_digest,
            measurement_basis_identity_digest,
            layout_operator_planning_contract,
            dependency_map,
            neighborhood_class,
            membership_rule,
            mut members,
        } = input;
        members.sort_unstable_by_key(UiAllocationNeighborhoodMember::canonical_sort_key);
        let identity =
            UiAllocationNeighborhoodIdentity::new(super::UiAllocationNeighborhoodIdentityInput {
                root_graph_node_identity,
                graph_generation,
                world_identity_digest,
                graph_snapshot_authority_digest,
                measurement_basis_identity_digest,
                layout_operator_contract_identity: layout_operator_planning_contract.identity(),
                dependency_map_identity_digest: dependency_map.identity_digest(),
                neighborhood_class,
                member_identity_digests: members
                    .iter()
                    .map(UiAllocationNeighborhoodMember::identity_digest)
                    .collect(),
            });

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
        input: UiAllocationNeighborhoodInput,
        _: &super::UiAllocationNeighborhoodEvidenceTestAuthority,
    ) -> Self {
        Self::construct(input)
    }

    #[cfg(test)]
    pub(crate) fn new_for_graph_test(
        input: UiAllocationNeighborhoodInput,
        _: &crate::graph::UiAllocationNeighborhoodMintAuthority,
    ) -> Self {
        Self::construct(input)
    }

    #[cfg(test)]
    pub(crate) fn with_members_for_graph_test(
        &self,
        members: Vec<UiAllocationNeighborhoodMember>,
    ) -> Self {
        Self::construct(UiAllocationNeighborhoodInput {
            root_graph_node_identity: self.root_graph_node_identity(),
            graph_generation: self.graph_generation(),
            world_identity_digest: self.world_identity_digest(),
            graph_snapshot_authority_digest: self.graph_snapshot_authority_digest(),
            measurement_basis_identity_digest: self.measurement_basis_identity_digest(),
            layout_operator_planning_contract: self.layout_operator_planning_contract.clone(),
            dependency_map: self.dependency_map.clone(),
            neighborhood_class: self.neighborhood_class(),
            membership_rule: self.membership_rule,
            members,
        })
    }
}
