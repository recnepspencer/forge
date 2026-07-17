use crate::evidence::{
    UiAllocationNeighborhood, UiAllocationNeighborhoodClass, UiAllocationNeighborhoodMember,
    UiAllocationNeighborhoodMemberRole, UiMeasurementDependencyMap,
};
use crate::graph::allocation_neighborhood::equivalent_identity;
use crate::graph::{
    UiGraphAxisParticipation, UiGraphNodeIdentity, UiGraphParticipationStatus,
    UiRepeatedInstanceBasis,
};

#[test]
fn allocation_neighborhood_identity_ignores_member_order() {
    let left = synthetic_neighborhood([
        synthetic_member(801, 1, UiAllocationNeighborhoodMemberRole::Root, admitted()),
        synthetic_member(802, 2, UiAllocationNeighborhoodMemberRole::Peer, admitted()),
    ]);
    let right = synthetic_neighborhood([
        synthetic_member(802, 2, UiAllocationNeighborhoodMemberRole::Peer, admitted()),
        synthetic_member(801, 1, UiAllocationNeighborhoodMemberRole::Root, admitted()),
    ]);

    assert!(equivalent_identity(&left, &right));
    assert_eq!(
        left.identity().member_identity_digests(),
        right.identity().member_identity_digests()
    );
}

#[test]
fn allocation_neighborhood_identity_ignores_participation_provenance_for_admitted_members() {
    let left = synthetic_neighborhood([synthetic_member(
        901,
        11,
        UiAllocationNeighborhoodMemberRole::Root,
        UiGraphAxisParticipation::new(
            UiGraphParticipationStatus::Admitted,
            crate::graph::UiGraphParticipationReasonSource::GraphInstantiation,
            crate::graph::UiGraphParticipationReasonCode::InstantiatedNodeExists,
            crate::graph::UiGraphParticipationEvidenceHandle::InstantiationPlan,
        ),
    )]);
    let right = synthetic_neighborhood([synthetic_member(
        901,
        11,
        UiAllocationNeighborhoodMemberRole::Root,
        admitted(),
    )]);

    assert!(equivalent_identity(&left, &right));
    assert_eq!(left.identity(), right.identity());
}

#[test]
fn deferred_layout_posture_does_not_count_as_planning_membership() {
    let deferred = synthetic_member(
        999,
        99,
        UiAllocationNeighborhoodMemberRole::Root,
        UiGraphAxisParticipation::runtime_mutation(UiGraphParticipationStatus::Deferred),
    );

    assert!(!deferred.layout_participates());
}

fn admitted() -> UiGraphAxisParticipation {
    UiGraphAxisParticipation::runtime_mutation(UiGraphParticipationStatus::Admitted)
}

fn synthetic_neighborhood<const N: usize>(
    members: [UiAllocationNeighborhoodMember; N],
) -> UiAllocationNeighborhood {
    let authority = super::super::UiAllocationNeighborhoodMintAuthority::mint();
    UiAllocationNeighborhood::new(
        crate::evidence::UiAllocationNeighborhoodTestInput {
            root_graph_node_identity: UiGraphNodeIdentity::new(801),
            graph_generation: crate::graph::UiGraphGeneration::initial(),
            world_identity_digest: 77,
            measurement_basis_identity_digest: 88,
            dependency_map: UiMeasurementDependencyMap::new(vec![]),
            neighborhood_class: UiAllocationNeighborhoodClass::ContainerPeerGroup,
            members: members.into_iter().collect(),
        },
        &authority,
    )
}

fn synthetic_member(
    node_digest: u64,
    declaration_digest: u64,
    role: UiAllocationNeighborhoodMemberRole,
    layout_participation: UiGraphAxisParticipation,
) -> UiAllocationNeighborhoodMember {
    let authority = super::super::UiAllocationNeighborhoodMintAuthority::mint();
    UiAllocationNeighborhoodMember::new_for_graph_test(
        UiGraphNodeIdentity::new(node_digest),
        declaration_digest,
        UiRepeatedInstanceBasis::declaration_keyed(
            crate::declaration::UiDeclarationIdentityDigest::new(declaration_digest),
        ),
        layout_participation,
        role,
        None,
        &authority,
    )
}
