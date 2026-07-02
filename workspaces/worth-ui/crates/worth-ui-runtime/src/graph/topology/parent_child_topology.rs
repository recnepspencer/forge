use std::collections::BTreeMap;

use crate::declaration::{stable_text_digest, UiDeclarationOrderingGuarantee};
use crate::graph::{
    UiGraphContainmentClaim, UiGraphMosaicMembership, UiGraphNodeIdentity, UiGraphPageMembership,
    UiGraphParentResolutionClaim, UiGraphRegionMembership, UiGraphSlotTopology,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphMembershipFacts {
    page_membership: Option<UiGraphPageMembership>,
    region_membership: Option<UiGraphRegionMembership>,
    mosaic_membership: Option<UiGraphMosaicMembership>,
}

impl UiGraphMembershipFacts {
    pub(in crate::graph::topology) const fn new(
        page_membership: Option<UiGraphPageMembership>,
        region_membership: Option<UiGraphRegionMembership>,
        mosaic_membership: Option<UiGraphMosaicMembership>,
    ) -> Self {
        Self {
            page_membership,
            region_membership,
            mosaic_membership,
        }
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphNodeTopology {
    owner_node_identity: UiGraphNodeIdentity,
    containment_claim: UiGraphContainmentClaim,
    parent_resolution_claim: UiGraphParentResolutionClaim,
    parent_node_identity: Option<UiGraphNodeIdentity>,
    slot_topology: Option<UiGraphSlotTopology>,
    ordering_guarantee: UiDeclarationOrderingGuarantee,
    membership_facts: UiGraphMembershipFacts,
}

impl UiGraphNodeTopology {
    pub(in crate::graph::topology) const fn new(
        owner_node_identity: UiGraphNodeIdentity,
        containment_claim: UiGraphContainmentClaim,
        parent_resolution_claim: UiGraphParentResolutionClaim,
        parent_node_identity: Option<UiGraphNodeIdentity>,
        slot_topology: Option<UiGraphSlotTopology>,
        ordering_guarantee: UiDeclarationOrderingGuarantee,
        membership_facts: UiGraphMembershipFacts,
    ) -> Self {
        Self {
            owner_node_identity,
            containment_claim,
            parent_resolution_claim,
            parent_node_identity,
            slot_topology,
            ordering_guarantee,
            membership_facts,
        }
    }

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

    pub fn membership_facts(&self) -> &UiGraphMembershipFacts {
        &self.membership_facts
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphTopology {
    node_topologies: BTreeMap<UiGraphNodeIdentity, UiGraphNodeTopology>,
}

impl UiGraphTopology {
    pub(in crate::graph::topology) fn new(
        node_topologies: BTreeMap<UiGraphNodeIdentity, UiGraphNodeTopology>,
    ) -> Self {
        Self { node_topologies }
    }

    pub fn node_topology(&self, node_identity: UiGraphNodeIdentity) -> Option<&UiGraphNodeTopology> {
        self.node_topologies.get(&node_identity)
    }

    pub(crate) fn node_topologies(&self) -> impl Iterator<Item = &UiGraphNodeTopology> {
        self.node_topologies.values()
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        self.node_topologies.values().fold(
            stable_text_digest("graph-topology"),
            |digest, topology| {
                let parent_digest = topology
                    .parent_node_identity()
                    .map(UiGraphNodeIdentity::digest)
                    .unwrap_or_else(|| stable_text_digest("graph-topology:root"));
                let slot_digest = topology
                    .slot_topology()
                    .map(|slot| stable_text_digest(slot.slot_name()))
                    .unwrap_or_else(|| stable_text_digest("graph-topology:no-slot"));
                let page_digest = topology
                    .membership_facts()
                    .page_membership()
                    .map(|page| page.page_node_identity().digest())
                    .unwrap_or_else(|| stable_text_digest("graph-topology:no-page"));
                let parent_resolution_digest = topology.parent_resolution_claim().identity_digest();
                let claim_digest = topology
                    .containment_claim()
                    .identity_digest();
                let region_digest = topology
                    .membership_facts()
                    .region_membership()
                    .map(|region| stable_text_digest(region.region_name()))
                    .unwrap_or_else(|| stable_text_digest("graph-topology:no-region"));
                let mosaic_digest = topology
                    .membership_facts()
                    .mosaic_membership()
                    .map(|mosaic| stable_text_digest(mosaic.mosaic_name()))
                    .unwrap_or_else(|| stable_text_digest("graph-topology:no-mosaic"));

                digest.rotate_left(7)
                    ^ topology.owner_node_identity().digest()
                    ^ parent_digest.rotate_left(11)
                    ^ slot_digest.rotate_left(17)
                    ^ page_digest.rotate_left(23)
                    ^ parent_resolution_digest.rotate_left(25)
                    ^ claim_digest.rotate_left(27)
                    ^ region_digest.rotate_left(29)
                    ^ mosaic_digest.rotate_left(31)
            },
        )
    }
}

impl UiGraphContainmentClaim {
    fn identity_digest(&self) -> u64 {
        match self {
            Self::RootPage => stable_text_digest("graph-topology:root-page"),
            Self::PageSet { page_set_name } => {
                stable_text_digest("graph-topology:page-set")
                    ^ stable_text_digest(page_set_name).rotate_left(7)
            }
            Self::Region { region_name } => {
                stable_text_digest("graph-topology:region")
                    ^ stable_text_digest(region_name).rotate_left(7)
            }
            Self::Mosaic { mosaic_name } => {
                stable_text_digest("graph-topology:mosaic")
                    ^ stable_text_digest(mosaic_name).rotate_left(7)
            }
            Self::LocalComposition {
                local_composition_name,
            } => {
                stable_text_digest("graph-topology:local-composition")
                    ^ stable_text_digest(local_composition_name).rotate_left(7)
            }
            Self::Control { control_name } => {
                stable_text_digest("graph-topology:control")
                    ^ stable_text_digest(control_name).rotate_left(7)
            }
            Self::DiagnosticSurface {
                diagnostic_surface_name,
            } => {
                stable_text_digest("graph-topology:diagnostic-surface")
                    ^ stable_text_digest(diagnostic_surface_name).rotate_left(7)
            }
        }
    }
}
