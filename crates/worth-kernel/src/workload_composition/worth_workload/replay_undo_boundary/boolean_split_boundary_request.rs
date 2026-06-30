use crate::replay_undo_transaction_boundary::ReplayUndoTransactionBoundarySupportSource;
use topology::facade::TopologyUndoScopeProduct;
use worth_spatial::facade::replay_undo_semantic_graph::{
    SpatialReplayScopeProduct, SpatialUndoScopeProduct,
};

#[derive(Clone, Copy)]
pub struct BooleanSplitReplayUndoBoundaryRequest<'a> {
    topology_undo_scope_product: &'a TopologyUndoScopeProduct<'a>,
    spatial_replay_scope_product: &'a SpatialReplayScopeProduct<'a>,
    spatial_undo_scope_product: &'a SpatialUndoScopeProduct<'a>,
    support_source: ReplayUndoTransactionBoundarySupportSource,
}

impl<'a> BooleanSplitReplayUndoBoundaryRequest<'a> {
    pub fn new(
        topology_undo_scope_product: &'a TopologyUndoScopeProduct<'a>,
        spatial_replay_scope_product: &'a SpatialReplayScopeProduct<'a>,
        spatial_undo_scope_product: &'a SpatialUndoScopeProduct<'a>,
    ) -> Self {
        Self {
            topology_undo_scope_product,
            spatial_replay_scope_product,
            spatial_undo_scope_product,
            support_source: ReplayUndoTransactionBoundarySupportSource::Ordinary,
        }
    }

    pub fn with_support_source(
        self,
        support_source: ReplayUndoTransactionBoundarySupportSource,
    ) -> Self {
        Self {
            topology_undo_scope_product: self.topology_undo_scope_product,
            spatial_replay_scope_product: self.spatial_replay_scope_product,
            spatial_undo_scope_product: self.spatial_undo_scope_product,
            support_source,
        }
    }

    pub const fn topology_undo_scope_product(&self) -> &'a TopologyUndoScopeProduct<'a> {
        self.topology_undo_scope_product
    }

    pub const fn spatial_replay_scope_product(&self) -> &'a SpatialReplayScopeProduct<'a> {
        self.spatial_replay_scope_product
    }

    pub const fn spatial_undo_scope_product(&self) -> &'a SpatialUndoScopeProduct<'a> {
        self.spatial_undo_scope_product
    }

    pub const fn support_source(&self) -> ReplayUndoTransactionBoundarySupportSource {
        self.support_source
    }
}
