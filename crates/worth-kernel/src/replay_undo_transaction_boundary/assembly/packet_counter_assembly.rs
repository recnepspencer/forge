use super::super::ReplayUndoTransactionBoundaryPacketCounters;
use topology::facade::TopologyUndoScopeProduct;
use worth_spatial::facade::replay_undo_semantic_graph::{
    SpatialReplayScopeProduct, SpatialUndoScopeProduct,
};

pub fn assemble_replay_undo_transaction_boundary_packet_counters(
    topology_undo_scope_product: &TopologyUndoScopeProduct<'_>,
    spatial_replay_scope_product: &SpatialReplayScopeProduct<'_>,
    spatial_undo_scope_product: &SpatialUndoScopeProduct<'_>,
    mutation_claim_count: usize,
) -> ReplayUndoTransactionBoundaryPacketCounters {
    ReplayUndoTransactionBoundaryPacketCounters::new(
        topology_undo_scope_product
            .counters()
            .touched_subject_count(),
        spatial_replay_scope_product
            .counters()
            .touched_subject_count(),
        spatial_undo_scope_product
            .counters()
            .touched_subject_count(),
        mutation_claim_count,
        spatial_replay_scope_product.counters().raw_row_scan_count(),
        spatial_replay_scope_product
            .counters()
            .broad_receipt_scan_count(),
        spatial_replay_scope_product
            .counters()
            .caller_owned_scan_count(),
        spatial_replay_scope_product
            .counters()
            .retained_replay_binding_count(),
        spatial_undo_scope_product
            .counters()
            .lookup_consumed_workload_handoff_count(),
        spatial_undo_scope_product.counters().raw_row_scan_count(),
        spatial_undo_scope_product
            .counters()
            .broad_receipt_scan_count(),
        spatial_undo_scope_product
            .counters()
            .caller_owned_scan_count(),
    )
}
