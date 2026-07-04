use std::collections::BTreeMap;

use crate::graph::{
    UiGraphAxisParticipation, UiGraphNode, UiGraphNodeIdentity, UiGraphParticipationAxis,
    UiGraphParticipationPosture, UiGraphTopology,
};

const EMPTY_PAGE_PARTICIPATION: [UiGraphPageParticipationMember; 0] = [];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphPageParticipationMember {
    page_node_identity: UiGraphNodeIdentity,
    member_node_identity: UiGraphNodeIdentity,
    axis: UiGraphParticipationAxis,
    axis_participation: UiGraphAxisParticipation,
}

impl UiGraphPageParticipationMember {
    pub(crate) const fn new(
        page_node_identity: UiGraphNodeIdentity,
        member_node_identity: UiGraphNodeIdentity,
        axis: UiGraphParticipationAxis,
        axis_participation: UiGraphAxisParticipation,
    ) -> Self {
        Self {
            page_node_identity,
            member_node_identity,
            axis,
            axis_participation,
        }
    }

    pub fn page_node_identity(self) -> UiGraphNodeIdentity {
        self.page_node_identity
    }

    pub fn member_node_identity(self) -> UiGraphNodeIdentity {
        self.member_node_identity
    }

    pub fn axis(self) -> UiGraphParticipationAxis {
        self.axis
    }

    pub fn axis_participation(self) -> UiGraphAxisParticipation {
        self.axis_participation
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphPageParticipationIndex {
    members_by_page_and_axis: BTreeMap<
        UiGraphNodeIdentity,
        BTreeMap<UiGraphParticipationAxis, Vec<UiGraphPageParticipationMember>>,
    >,
}

impl UiGraphPageParticipationIndex {
    pub(crate) fn build(nodes: &[UiGraphNode], topology: &UiGraphTopology) -> Self {
        let node_lookup = nodes
            .iter()
            .map(|node| (node.graph_node_identity(), node.participation_posture()))
            .collect::<BTreeMap<_, _>>();
        let mut members_by_page_and_axis = BTreeMap::<
            UiGraphNodeIdentity,
            BTreeMap<UiGraphParticipationAxis, Vec<UiGraphPageParticipationMember>>,
        >::new();

        for node_topology in topology.node_topologies() {
            let Some(page_membership) = node_topology.membership_facts().page_membership() else {
                continue;
            };
            let Some(node_participation) = node_lookup.get(&node_topology.owner_node_identity())
            else {
                continue;
            };

            collect_page_axis_members(
                &mut members_by_page_and_axis,
                page_membership.page_node_identity(),
                node_topology.owner_node_identity(),
                *node_participation,
            );
        }

        Self {
            members_by_page_and_axis,
        }
    }

    pub fn page_axis_members(
        &self,
        page_node_identity: UiGraphNodeIdentity,
        axis: UiGraphParticipationAxis,
    ) -> &[UiGraphPageParticipationMember] {
        self.members_by_page_and_axis
            .get(&page_node_identity)
            .and_then(|members_by_axis| members_by_axis.get(&axis))
            .map(Vec::as_slice)
            .unwrap_or(&EMPTY_PAGE_PARTICIPATION)
    }
}

fn collect_page_axis_members(
    members_by_page_and_axis: &mut BTreeMap<
        UiGraphNodeIdentity,
        BTreeMap<UiGraphParticipationAxis, Vec<UiGraphPageParticipationMember>>,
    >,
    page_node_identity: UiGraphNodeIdentity,
    member_node_identity: UiGraphNodeIdentity,
    participation_posture: UiGraphParticipationPosture,
) {
    for axis in UiGraphParticipationAxis::ALL {
        let axis_participation = participation_posture.axis(axis);
        if axis_participation.status().admitted() {
            members_by_page_and_axis
                .entry(page_node_identity)
                .or_default()
                .entry(axis)
                .or_default()
                .push(UiGraphPageParticipationMember::new(
                    page_node_identity,
                    member_node_identity,
                    axis,
                    axis_participation,
                ));
        }
    }
}
