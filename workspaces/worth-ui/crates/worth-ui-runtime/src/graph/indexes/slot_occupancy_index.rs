use std::collections::BTreeMap;

use crate::graph::UiGraphNodeIdentity;

const EMPTY_NODE_SET: [UiGraphNodeIdentity; 0] = [];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphSlotOccupancyIndex {
    occupants_by_parent_and_slot:
        BTreeMap<UiGraphNodeIdentity, BTreeMap<Box<str>, Vec<UiGraphNodeIdentity>>>,
}

impl UiGraphSlotOccupancyIndex {
    pub(crate) fn new(
        occupants_by_parent_and_slot: BTreeMap<
            UiGraphNodeIdentity,
            BTreeMap<Box<str>, Vec<UiGraphNodeIdentity>>,
        >,
    ) -> Self {
        Self {
            occupants_by_parent_and_slot,
        }
    }

    pub fn slot_occupants(
        &self,
        parent_node_identity: UiGraphNodeIdentity,
        slot_name: &str,
    ) -> &[UiGraphNodeIdentity] {
        self.occupants_by_parent_and_slot
            .get(&parent_node_identity)
            .and_then(|slots| slots.get(slot_name))
            .map(Vec::as_slice)
            .unwrap_or(&EMPTY_NODE_SET)
    }
}
