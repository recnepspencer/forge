use crate::declaration::{UiAspectName, UiDeclarationIdentity};
use crate::graph::indexes::{
    UiGraphAspectConsumer, UiGraphAspectPublisher, UiGraphLookup, UiGraphLookupCostClass,
    UiGraphLookupFamily, UiGraphLookupReceipt,
};
use crate::graph::{
    UiGraphMountEligibilityIdentity, UiGraphMountEligibilityRecord, UiGraphNodeIdentity,
    UiGraphNodeRecord, UiGraphPageParticipationMember, UiGraphParticipationAxis, UiGraphSnapshot,
    UiGraphTopologyRecord,
};

pub struct UiGraphLookupSurface<'a> {
    snapshot: &'a UiGraphSnapshot,
}

impl<'a> UiGraphLookupSurface<'a> {
    pub(crate) const fn new(snapshot: &'a UiGraphSnapshot) -> Self {
        Self { snapshot }
    }

    pub fn graph_node(
        self,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Option<UiGraphLookup<UiGraphNodeRecord>> {
        self.snapshot
            .core_indexes()
            .node_identity()
            .node(self.snapshot.nodes(), graph_node_identity)
            .map(|node| {
                UiGraphLookup::new(
                    scalar(UiGraphLookupFamily::NodeIdentity),
                    UiGraphNodeRecord::from(node),
                )
            })
    }

    pub fn declaration_instances(
        self,
        declaration_identity: &UiDeclarationIdentity,
    ) -> UiGraphLookup<&'a [UiGraphNodeIdentity]> {
        UiGraphLookup::new(
            set(UiGraphLookupFamily::DeclarationCorrespondence),
            self.snapshot
                .core_indexes()
                .declaration_correspondence()
                .graph_node_ids_for(declaration_identity),
        )
    }

    pub fn child_nodes(
        self,
        parent_node_identity: UiGraphNodeIdentity,
    ) -> UiGraphLookup<&'a [UiGraphNodeIdentity]> {
        UiGraphLookup::new(
            set(UiGraphLookupFamily::ParentChild),
            self.snapshot
                .core_indexes()
                .topology()
                .parent_child()
                .children_of(parent_node_identity),
        )
    }

    pub fn slot_occupants(
        self,
        parent_node_identity: UiGraphNodeIdentity,
        slot_name: &str,
    ) -> UiGraphLookup<&'a [UiGraphNodeIdentity]> {
        UiGraphLookup::new(
            set(UiGraphLookupFamily::SlotOccupancy),
            self.snapshot
                .core_indexes()
                .topology()
                .slot_occupancy()
                .slot_occupants(parent_node_identity, slot_name),
        )
    }

    pub fn topology_node(
        self,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Option<UiGraphLookup<UiGraphTopologyRecord>> {
        self.snapshot
            .topology()
            .node_topology(graph_node_identity)
            .map(|topology| {
                UiGraphLookup::new(
                    scalar(UiGraphLookupFamily::TopologyNode),
                    UiGraphTopologyRecord::from(topology),
                )
            })
    }

    pub fn page_members(
        self,
        page_node_identity: UiGraphNodeIdentity,
    ) -> UiGraphLookup<&'a [UiGraphNodeIdentity]> {
        UiGraphLookup::new(
            set(UiGraphLookupFamily::PageMembership),
            self.snapshot
                .core_indexes()
                .topology()
                .page_membership()
                .page_members(page_node_identity),
        )
    }

    pub fn region_members(self, region_name: &str) -> UiGraphLookup<&'a [UiGraphNodeIdentity]> {
        UiGraphLookup::new(
            set(UiGraphLookupFamily::RegionMembership),
            self.snapshot
                .core_indexes()
                .topology()
                .region_membership()
                .region_members(region_name),
        )
    }

    pub fn mosaic_members(self, mosaic_name: &str) -> UiGraphLookup<&'a [UiGraphNodeIdentity]> {
        UiGraphLookup::new(
            set(UiGraphLookupFamily::MosaicMembership),
            self.snapshot
                .core_indexes()
                .topology()
                .mosaic_membership()
                .mosaic_members(mosaic_name),
        )
    }

    pub fn page_participation(
        self,
        page_node_identity: UiGraphNodeIdentity,
        axis: UiGraphParticipationAxis,
    ) -> UiGraphLookup<&'a [UiGraphPageParticipationMember]> {
        UiGraphLookup::new(
            neighborhood(UiGraphLookupFamily::PageParticipation),
            self.snapshot
                .core_indexes()
                .participation()
                .page_participation()
                .page_axis_members(page_node_identity, axis),
        )
    }

    pub fn published_aspect(
        self,
        aspect: &UiAspectName,
    ) -> UiGraphLookup<&'a [UiGraphAspectPublisher]> {
        UiGraphLookup::new(
            neighborhood(UiGraphLookupFamily::PublishedAspect),
            self.snapshot
                .core_indexes()
                .published_aspects()
                .publishers_for(aspect),
        )
    }

    pub fn consumed_aspect(
        self,
        aspect: &UiAspectName,
    ) -> UiGraphLookup<&'a [UiGraphAspectConsumer]> {
        UiGraphLookup::new(
            neighborhood(UiGraphLookupFamily::ConsumedAspect),
            self.snapshot
                .core_indexes()
                .consumed_aspects()
                .consumers_for(aspect),
        )
    }

    pub fn mount_eligibility_slot(
        self,
        mount_eligibility_identity: UiGraphMountEligibilityIdentity,
    ) -> Option<UiGraphLookup<UiGraphMountEligibilityRecord>> {
        self.snapshot
            .mount_eligibilities()
            .slot(mount_eligibility_identity)
            .map(|slot| {
                UiGraphLookup::new(
                    scalar(UiGraphLookupFamily::MountEligibilitySlot),
                    UiGraphMountEligibilityRecord::from(*slot),
                )
            })
    }

    pub fn mount_eligibility_slot_for_node(
        self,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Option<UiGraphLookup<UiGraphMountEligibilityRecord>> {
        self.snapshot
            .core_indexes()
            .mount_eligibilities()
            .slot_for_node(self.snapshot.mount_eligibilities(), graph_node_identity)
            .map(|slot| {
                UiGraphLookup::new(
                    scalar(UiGraphLookupFamily::MountEligibilitySlot),
                    UiGraphMountEligibilityRecord::from(*slot),
                )
            })
    }
}

const fn scalar(family: UiGraphLookupFamily) -> UiGraphLookupReceipt {
    UiGraphLookupReceipt::new(family, UiGraphLookupCostClass::IndexedScalar)
}

const fn set(family: UiGraphLookupFamily) -> UiGraphLookupReceipt {
    UiGraphLookupReceipt::new(family, UiGraphLookupCostClass::IndexedSet)
}

const fn neighborhood(family: UiGraphLookupFamily) -> UiGraphLookupReceipt {
    UiGraphLookupReceipt::new(family, UiGraphLookupCostClass::IndexedNeighborhood)
}
