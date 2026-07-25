use super::{
    UiGraphConsumedAspectIndex, UiGraphMountEligibilityIndex, UiGraphNodeIdentityIndex,
    UiGraphParticipationIndexes, UiGraphPublishedAspectIndex, UiGraphTopologyIndexes,
};
use crate::graph::{
    UiGraphDeclarationCorrespondence, UiGraphMountEligibilityStore, UiGraphNode,
    UiGraphNodeInstantiationEntry, UiGraphTopology,
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
        node_entries: &[UiGraphNodeInstantiationEntry],
        nodes: &[UiGraphNode],
        declaration_correspondence: UiGraphDeclarationCorrespondence,
        topology: &UiGraphTopology,
        mount_eligibilities: &UiGraphMountEligibilityStore,
    ) -> Self {
        let mount_eligibility_index = UiGraphMountEligibilityIndex::build(mount_eligibilities);
        let node_aspects = node_entries
            .iter()
            .zip(nodes.iter())
            .map(|(entry, node)| (entry.aspect_contract(), node.graph_node_identity()))
            .collect::<Vec<_>>();

        Self {
            node_identity_index: UiGraphNodeIdentityIndex::build(nodes),
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
            declaration_correspondence,
        }
    }

    pub(crate) fn rebuild_participation_for_successor(
        nodes: &[UiGraphNode],
        topology: &UiGraphTopology,
        prior: &Self,
    ) -> Self {
        Self {
            node_identity_index: UiGraphNodeIdentityIndex::build(nodes),
            declaration_correspondence: prior.declaration_correspondence.clone(),
            topology_indexes: UiGraphTopologyIndexes::build(topology),
            participation_indexes: UiGraphParticipationIndexes::build(nodes, topology),
            mount_eligibility_index: prior.mount_eligibility_index.clone(),
            published_aspect_index: prior.published_aspect_index.clone(),
            consumed_aspect_index: prior.consumed_aspect_index.clone(),
        }
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
