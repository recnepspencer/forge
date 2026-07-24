use std::collections::BTreeMap;

use crate::graph::{
    UiGraphMountEligibilityIdentity, UiGraphMountEligibilitySlot, UiGraphMountEligibilityStore,
    UiGraphNodeIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphMountEligibilityIndex {
    graph_node_to_receipt: BTreeMap<UiGraphNodeIdentity, UiGraphMountEligibilityIdentity>,
}

impl UiGraphMountEligibilityIndex {
    pub(crate) fn build(store: &UiGraphMountEligibilityStore) -> Self {
        Self {
            graph_node_to_receipt: store
                .slots()
                .iter()
                .map(|slot| {
                    (
                        slot.graph_node_identity(),
                        slot.mount_eligibility_identity(),
                    )
                })
                .collect(),
        }
    }

    pub fn receipt_identity_for(
        &self,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Option<UiGraphMountEligibilityIdentity> {
        self.graph_node_to_receipt
            .get(&graph_node_identity)
            .copied()
    }

    pub fn slot_for_node<'a>(
        &self,
        store: &'a UiGraphMountEligibilityStore,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Option<&'a UiGraphMountEligibilitySlot> {
        self.receipt_identity_for(graph_node_identity)
            .and_then(|mount_eligibility_identity| store.slot(mount_eligibility_identity))
    }
}
