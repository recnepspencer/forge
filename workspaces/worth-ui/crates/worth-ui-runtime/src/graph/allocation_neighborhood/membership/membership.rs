use crate::evidence::{
    UiAllocationNeighborhoodMember, UiAllocationNeighborhoodMemberRole,
    UiAllocationNeighborhoodMembershipRule,
};
use crate::graph::{
    UiGraphAxisParticipation, UiGraphContainmentClaim, UiGraphNodeIdentity,
    UiGraphParticipationAxis, UiGraphParticipationStatus, UiGraphSnapshot, UiRepeatedInstanceBasis,
};

pub(super) fn derive_members(
    snapshot: &UiGraphSnapshot,
    root_graph_node_identity: UiGraphNodeIdentity,
    membership_rule: UiAllocationNeighborhoodMembershipRule,
) -> Vec<UiAllocationNeighborhoodMember> {
    let candidate_ids = candidate_ids(snapshot, root_graph_node_identity, membership_rule);
    let mut members = Vec::new();

    for candidate_id in candidate_ids {
        let Some(node_record) = snapshot.lookup().graph_node(candidate_id) else {
            continue;
        };
        let node_record = node_record.value();
        let layout_participation = node_record
            .participation_posture()
            .axis(UiGraphParticipationAxis::Layout);
        if !layout_participates_in_planning(snapshot, candidate_id, layout_participation) {
            continue;
        }

        members.push(UiAllocationNeighborhoodMember::new_with_graph_authority(
            candidate_id,
            node_record.authored_provenance_digest(),
            repeated_instance_basis_for(node_record.repeated_instance_basis()),
            layout_participation,
            role_for(root_graph_node_identity, candidate_id, membership_rule),
            node_record.measurement_constraint_modifier(),
            &super::UiAllocationNeighborhoodMintAuthority::mint(),
        ));
    }

    members
}

pub(crate) fn layout_participates_in_planning(
    snapshot: &UiGraphSnapshot,
    graph_node_identity: UiGraphNodeIdentity,
    layout_participation: UiGraphAxisParticipation,
) -> bool {
    if !matches!(
        layout_participation.status(),
        UiGraphParticipationStatus::Admitted
    ) {
        return false;
    }

    !matches!(
        snapshot
            .topology()
            .node_topology(graph_node_identity)
            .map(|topology| topology.containment_claim()),
        Some(UiGraphContainmentClaim::DiagnosticSurface { .. })
    )
}

fn candidate_ids(
    snapshot: &UiGraphSnapshot,
    root_graph_node_identity: UiGraphNodeIdentity,
    membership_rule: UiAllocationNeighborhoodMembershipRule,
) -> Vec<UiGraphNodeIdentity> {
    match membership_rule {
        UiAllocationNeighborhoodMembershipRule::RootOnly => vec![root_graph_node_identity],
        UiAllocationNeighborhoodMembershipRule::ParentSlotPeerGroup => {
            container_peer_group_ids(snapshot, root_graph_node_identity)
        }
    }
}

fn container_peer_group_ids(
    snapshot: &UiGraphSnapshot,
    root_graph_node_identity: UiGraphNodeIdentity,
) -> Vec<UiGraphNodeIdentity> {
    let Some(root_topology) = snapshot.topology().node_topology(root_graph_node_identity) else {
        return vec![root_graph_node_identity];
    };
    let Some(parent_node_identity) = root_topology.parent_node_identity() else {
        return vec![root_graph_node_identity];
    };

    if let Some(slot_topology) = root_topology.slot_topology() {
        return snapshot
            .lookup()
            .slot_occupants(parent_node_identity, slot_topology.slot_name())
            .value()
            .to_vec();
    }

    snapshot
        .lookup()
        .child_nodes(parent_node_identity)
        .value()
        .to_vec()
}

fn repeated_instance_basis_for(
    repeated_instance_basis: &UiRepeatedInstanceBasis,
) -> UiRepeatedInstanceBasis {
    repeated_instance_basis.clone()
}

fn role_for(
    root_graph_node_identity: UiGraphNodeIdentity,
    candidate_id: UiGraphNodeIdentity,
    membership_rule: UiAllocationNeighborhoodMembershipRule,
) -> UiAllocationNeighborhoodMemberRole {
    if candidate_id == root_graph_node_identity {
        UiAllocationNeighborhoodMemberRole::Root
    } else if matches!(
        membership_rule,
        UiAllocationNeighborhoodMembershipRule::ParentSlotPeerGroup
    ) {
        UiAllocationNeighborhoodMemberRole::Peer
    } else {
        UiAllocationNeighborhoodMemberRole::ScopedParticipant
    }
}
