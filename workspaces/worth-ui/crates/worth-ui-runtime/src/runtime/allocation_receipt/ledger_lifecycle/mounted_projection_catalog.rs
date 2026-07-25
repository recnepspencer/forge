#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct UiMountedAllocationProjectionCatalog {
    by_graph: crate::runtime::persistent_index::UiPersistentOrdMap<
        crate::graph::UiGraphNodeIdentity,
        super::UiAllocationReceipt,
    >,
}

impl UiMountedAllocationProjectionCatalog {
    pub(crate) fn receipt(
        &self,
        graph_node: crate::graph::UiGraphNodeIdentity,
    ) -> Option<&super::UiAllocationReceipt> {
        self.by_graph.get(&graph_node)
    }

    pub(super) fn insert(&mut self, receipt: super::UiAllocationReceipt) {
        self.by_graph
            .insert(receipt.identity().graph_node_identity(), receipt);
    }

    pub(super) fn remove(&mut self, receipt: &super::UiAllocationReceipt) {
        self.by_graph
            .remove(&receipt.identity().graph_node_identity());
    }

    pub(super) fn replace_with<'a>(
        &mut self,
        receipts: impl IntoIterator<Item = &'a super::UiAllocationReceipt>,
    ) {
        self.by_graph = Default::default();
        for receipt in receipts {
            self.insert(receipt.clone());
        }
    }
}
