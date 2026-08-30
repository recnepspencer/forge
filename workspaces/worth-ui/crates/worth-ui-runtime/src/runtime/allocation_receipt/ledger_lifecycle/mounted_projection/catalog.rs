//! Current mounted allocation projection truth indexed by graph identity.

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct UiMountedAllocationProjectionCatalog {
    by_graph: crate::runtime::persistent_index::UiPersistentOrdMap<
        crate::graph::UiGraphNodeIdentity,
        super::mounted_projection_row::UiMountedAllocationProjectionRow,
    >,
}

impl UiMountedAllocationProjectionCatalog {
    pub(crate) fn projection(
        &self,
        graph_node: crate::graph::UiGraphNodeIdentity,
    ) -> Result<
        Option<worth_ui_host_contract::UiMountedAllocationProjection>,
        super::UiMountedAllocationProjectionDenial,
    > {
        self.by_graph
            .get(&graph_node)
            .copied()
            .map(super::mounted_projection_row::UiMountedAllocationProjectionRow::projection)
            .transpose()
    }

    pub(crate) fn viewport_bounds(
        &self,
        graph_node: crate::graph::UiGraphNodeIdentity,
    ) -> Result<
        Option<super::UiCommittedViewportGeometry>,
        super::UiMountedAllocationProjectionDenial,
    > {
        self.by_graph
            .get(&graph_node)
            .copied()
            .map(super::mounted_projection_row::UiMountedAllocationProjectionRow::viewport_bounds)
            .transpose()
            .map(Option::flatten)
    }

    pub(super) fn insert(&mut self, receipt: super::UiAllocationReceipt) {
        let graph_node = receipt.identity().graph_node_identity();
        self.by_graph.insert(
            graph_node,
            super::mounted_projection_row::UiMountedAllocationProjectionRow::from_receipt(&receipt),
        );
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

    pub(super) fn projection_changed_since(
        &self,
        predecessor: &Self,
        graph_node: crate::graph::UiGraphNodeIdentity,
    ) -> bool {
        match (
            predecessor.by_graph.get(&graph_node),
            self.by_graph.get(&graph_node),
        ) {
            (Some(before), Some(after)) => before != after,
            (None, None) => false,
            _ => true,
        }
    }
}
