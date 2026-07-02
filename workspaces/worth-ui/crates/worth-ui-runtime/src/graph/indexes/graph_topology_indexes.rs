use std::collections::BTreeMap;

use crate::graph::{
    UiGraphMosaicMembershipIndex, UiGraphNodeIdentity, UiGraphPageMembershipIndex,
    UiGraphParentChildIndex, UiGraphRegionMembershipIndex, UiGraphSlotOccupancyIndex,
    UiGraphTopology,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphTopologyIndexes {
    parent_child_index: UiGraphParentChildIndex,
    slot_occupancy_index: UiGraphSlotOccupancyIndex,
    page_membership_index: UiGraphPageMembershipIndex,
    region_membership_index: UiGraphRegionMembershipIndex,
    mosaic_membership_index: UiGraphMosaicMembershipIndex,
}

impl UiGraphTopologyIndexes {
    pub(crate) fn build(topology: &UiGraphTopology) -> Self {
        let mut children_by_parent = BTreeMap::<UiGraphNodeIdentity, Vec<UiGraphNodeIdentity>>::new();
        let mut occupants_by_parent_and_slot =
            BTreeMap::<UiGraphNodeIdentity, BTreeMap<Box<str>, Vec<UiGraphNodeIdentity>>>::new();
        let mut members_by_page = BTreeMap::<UiGraphNodeIdentity, Vec<UiGraphNodeIdentity>>::new();
        let mut members_by_region = BTreeMap::<Box<str>, Vec<UiGraphNodeIdentity>>::new();
        let mut members_by_mosaic = BTreeMap::<Box<str>, Vec<UiGraphNodeIdentity>>::new();

        for node_topology in topology.node_topologies() {
            if let Some(parent_node_identity) = node_topology.parent_node_identity() {
                children_by_parent
                    .entry(parent_node_identity)
                    .or_default()
                    .push(node_topology.owner_node_identity());

                if let Some(slot_topology) = node_topology.slot_topology() {
                    occupants_by_parent_and_slot
                        .entry(parent_node_identity)
                        .or_default()
                        .entry(slot_topology.slot_name().into())
                        .or_default()
                        .push(node_topology.owner_node_identity());
                }
            }

            if let Some(page_membership) = node_topology.membership_facts().page_membership() {
                members_by_page
                    .entry(page_membership.page_node_identity())
                    .or_default()
                    .push(node_topology.owner_node_identity());
            }

            if let Some(region_membership) = node_topology.membership_facts().region_membership() {
                members_by_region
                    .entry(region_membership.region_name().into())
                    .or_default()
                    .push(node_topology.owner_node_identity());
            }

            if let Some(mosaic_membership) = node_topology.membership_facts().mosaic_membership() {
                members_by_mosaic
                    .entry(mosaic_membership.mosaic_name().into())
                    .or_default()
                    .push(node_topology.owner_node_identity());
            }
        }

        Self {
            parent_child_index: UiGraphParentChildIndex::new(children_by_parent),
            slot_occupancy_index: UiGraphSlotOccupancyIndex::new(occupants_by_parent_and_slot),
            page_membership_index: UiGraphPageMembershipIndex::new(members_by_page),
            region_membership_index: UiGraphRegionMembershipIndex::new(members_by_region),
            mosaic_membership_index: UiGraphMosaicMembershipIndex::new(members_by_mosaic),
        }
    }

    pub fn parent_child(&self) -> &UiGraphParentChildIndex {
        &self.parent_child_index
    }

    pub fn slot_occupancy(&self) -> &UiGraphSlotOccupancyIndex {
        &self.slot_occupancy_index
    }

    pub fn page_membership(&self) -> &UiGraphPageMembershipIndex {
        &self.page_membership_index
    }

    pub fn region_membership(&self) -> &UiGraphRegionMembershipIndex {
        &self.region_membership_index
    }

    pub fn mosaic_membership(&self) -> &UiGraphMosaicMembershipIndex {
        &self.mosaic_membership_index
    }
}
