use std::collections::BTreeMap;

use crate::graph::{
    UiGraphMountedReceiptAuthoritySeedStore, UiGraphMountedReceiptSlot, UiGraphNodeIdentity,
    UiMountedReceiptIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphMountedReceiptIndex {
    graph_node_to_receipt: BTreeMap<UiGraphNodeIdentity, UiMountedReceiptIdentity>,
}

impl UiGraphMountedReceiptIndex {
    pub(crate) fn build(store: &UiGraphMountedReceiptAuthoritySeedStore) -> Self {
        Self {
            graph_node_to_receipt: store
                .slots()
                .iter()
                .map(|slot| (slot.graph_node_identity(), slot.mounted_receipt_identity()))
                .collect(),
        }
    }

    pub fn receipt_identity_for(
        &self,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Option<UiMountedReceiptIdentity> {
        self.graph_node_to_receipt
            .get(&graph_node_identity)
            .copied()
    }

    pub fn slot_for_node<'a>(
        &self,
        store: &'a UiGraphMountedReceiptAuthoritySeedStore,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Option<&'a UiGraphMountedReceiptSlot> {
        self.receipt_identity_for(graph_node_identity)
            .and_then(|mounted_receipt_identity| store.slot(mounted_receipt_identity))
    }
}
