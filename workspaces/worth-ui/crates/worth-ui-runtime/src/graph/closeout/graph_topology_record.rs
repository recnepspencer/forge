use crate::declaration::UiDeclarationOrderingGuarantee;
use crate::graph::{
    UiGraphContainmentClaim, UiGraphMosaicMembership, UiGraphNodeIdentity, UiGraphNodeTopology,
    UiGraphPageMembership, UiGraphParentResolutionClaim, UiGraphRegionMembership,
    UiGraphSlotTopology,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphTopologyRecord {
    owner_node_identity: UiGraphNodeIdentity,
    containment_claim: UiGraphContainmentClaim,
    parent_resolution_claim: UiGraphParentResolutionClaim,
    parent_node_identity: Option<UiGraphNodeIdentity>,
    slot_topology: Option<UiGraphSlotTopology>,
    ordering_guarantee: UiDeclarationOrderingGuarantee,
    page_membership: Option<UiGraphPageMembership>,
    region_membership: Option<UiGraphRegionMembership>,
    mosaic_membership: Option<UiGraphMosaicMembership>,
}

impl UiGraphTopologyRecord {
    pub fn owner_node_identity(&self) -> UiGraphNodeIdentity {
        self.owner_node_identity
    }

    pub fn containment_claim(&self) -> &UiGraphContainmentClaim {
        &self.containment_claim
    }

    pub fn parent_resolution_claim(&self) -> &UiGraphParentResolutionClaim {
        &self.parent_resolution_claim
    }

    pub fn parent_node_identity(&self) -> Option<UiGraphNodeIdentity> {
        self.parent_node_identity
    }

    pub fn slot_topology(&self) -> Option<&UiGraphSlotTopology> {
        self.slot_topology.as_ref()
    }

    pub fn ordering_guarantee(&self) -> UiDeclarationOrderingGuarantee {
        self.ordering_guarantee
    }

    pub fn page_membership(&self) -> Option<UiGraphPageMembership> {
        self.page_membership
    }

    pub fn region_membership(&self) -> Option<&UiGraphRegionMembership> {
        self.region_membership.as_ref()
    }

    pub fn mosaic_membership(&self) -> Option<&UiGraphMosaicMembership> {
        self.mosaic_membership.as_ref()
    }
}

impl From<&UiGraphNodeTopology> for UiGraphTopologyRecord {
    fn from(topology: &UiGraphNodeTopology) -> Self {
        Self {
            owner_node_identity: topology.owner_node_identity(),
            containment_claim: topology.containment_claim().clone(),
            parent_resolution_claim: topology.parent_resolution_claim().clone(),
            parent_node_identity: topology.parent_node_identity(),
            slot_topology: topology.slot_topology().cloned(),
            ordering_guarantee: topology.ordering_guarantee(),
            page_membership: topology.membership_facts().page_membership(),
            region_membership: topology.membership_facts().region_membership().cloned(),
            mosaic_membership: topology.membership_facts().mosaic_membership().cloned(),
        }
    }
}
