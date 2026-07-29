use super::{
    UiGraphConsumedAspectIndex, UiGraphMountEligibilityIndex, UiGraphNodeIdentityIndex,
    UiGraphParticipationIndexes, UiGraphPublishedAspectIndex, UiGraphTopologyIndexes,
};
use crate::graph::{
    UiGraphDeclarationCorrespondence, UiGraphMountEligibilityStore, UiGraphNode, UiGraphTopology,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphCoreIndexes {
    node_identity_index: UiGraphNodeIdentityIndex,
    declaration_correspondence: UiGraphDeclarationCorrespondence,
    topology_indexes: UiGraphTopologyIndexes,
    participation_indexes: UiGraphParticipationIndexes,
    mount_eligibility_index: UiGraphMountEligibilityIndex,
    published_aspect_index: UiGraphPublishedAspectIndex,
    consumed_aspect_index: UiGraphConsumedAspectIndex,
}

impl UiGraphCoreIndexes {
    pub(crate) fn build(
        nodes: &[UiGraphNode],
        topology: &UiGraphTopology,
        mount_eligibilities: &UiGraphMountEligibilityStore,
    ) -> Self {
        let mount_eligibility_index = UiGraphMountEligibilityIndex::build(mount_eligibilities);
        let node_aspects = nodes
            .iter()
            .map(|node| (node.aspect_contract(), node.graph_node_identity()))
            .collect::<Vec<_>>();

        Self {
            node_identity_index: UiGraphNodeIdentityIndex::build(nodes),
            declaration_correspondence: UiGraphDeclarationCorrespondence::rebuild(nodes),
            topology_indexes: UiGraphTopologyIndexes::build(topology),
            participation_indexes: UiGraphParticipationIndexes::build(nodes, topology),
            mount_eligibility_index: mount_eligibility_index.clone(),
            published_aspect_index: UiGraphPublishedAspectIndex::build(
                &node_aspects,
                mount_eligibilities,
                &mount_eligibility_index,
            ),
            consumed_aspect_index: UiGraphConsumedAspectIndex::build(
                &node_aspects,
                mount_eligibilities,
                &mount_eligibility_index,
            ),
        }
    }

    pub(crate) fn rebuild(
        nodes: &[UiGraphNode],
        topology: &UiGraphTopology,
        mount_eligibilities: &UiGraphMountEligibilityStore,
    ) -> Self {
        Self::build(nodes, topology, mount_eligibilities)
    }

    pub fn node_identity(&self) -> &UiGraphNodeIdentityIndex {
        &self.node_identity_index
    }

    pub fn declaration_correspondence(&self) -> &UiGraphDeclarationCorrespondence {
        &self.declaration_correspondence
    }

    pub fn topology(&self) -> &UiGraphTopologyIndexes {
        &self.topology_indexes
    }

    pub fn participation(&self) -> &UiGraphParticipationIndexes {
        &self.participation_indexes
    }

    pub fn mount_eligibilities(&self) -> &UiGraphMountEligibilityIndex {
        &self.mount_eligibility_index
    }

    pub fn published_aspects(&self) -> &UiGraphPublishedAspectIndex {
        &self.published_aspect_index
    }

    pub fn consumed_aspects(&self) -> &UiGraphConsumedAspectIndex {
        &self.consumed_aspect_index
    }
}
