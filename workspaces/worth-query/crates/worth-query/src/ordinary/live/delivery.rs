use crate::evidence_identity::WorthQueryEvidenceIdentity;
use crate::runtime::{
    WorthQueryLiveGraphReadMaintenanceReceipt, WorthQueryManagedLiveRuntimeDelivery,
    WorthQueryRuntimeDeliveryBatch,
};
use crate::subscription::QuerySubscriptionDeliveryCauseKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryManagedLiveDeliveryCauseKind {
    RelationalChange,
    Temporal,
    AsyncCompletion,
    AsyncDeniedCompletion,
    AsyncRetry,
    AsyncRevalidation,
    Mixed,
}

impl WorthQueryManagedLiveDeliveryCauseKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RelationalChange => "relational_change",
            Self::Temporal => "temporal",
            Self::AsyncCompletion => "async_completion",
            Self::AsyncDeniedCompletion => "async_denied_completion",
            Self::AsyncRetry => "async_retry",
            Self::AsyncRevalidation => "async_revalidation",
            Self::Mixed => "mixed",
        }
    }

    fn from_runtime(kind: QuerySubscriptionDeliveryCauseKind) -> Self {
        match kind {
            QuerySubscriptionDeliveryCauseKind::RelationalPatch => Self::RelationalChange,
            QuerySubscriptionDeliveryCauseKind::FreshnessOnly
            | QuerySubscriptionDeliveryCauseKind::WindowEntry
            | QuerySubscriptionDeliveryCauseKind::WindowExit
            | QuerySubscriptionDeliveryCauseKind::Deadline
            | QuerySubscriptionDeliveryCauseKind::PreviousValueTransition => Self::Temporal,
            QuerySubscriptionDeliveryCauseKind::AsyncCompletion => Self::AsyncCompletion,
            QuerySubscriptionDeliveryCauseKind::AsyncDeniedCompletion => {
                Self::AsyncDeniedCompletion
            }
            QuerySubscriptionDeliveryCauseKind::AsyncRetry => Self::AsyncRetry,
            QuerySubscriptionDeliveryCauseKind::AsyncRevalidation => Self::AsyncRevalidation,
            QuerySubscriptionDeliveryCauseKind::MixedCause => Self::Mixed,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryManagedLiveDeliveryBatch {
    sequence: u64,
    delivery_batch_identity: WorthQueryEvidenceIdentity,
    delivery_window_identity: WorthQueryEvidenceIdentity,
    consumer_attachment_identity: WorthQueryEvidenceIdentity,
    cause_kind: WorthQueryManagedLiveDeliveryCauseKind,
    cause_identity: WorthQueryEvidenceIdentity,
    has_relational_patch: bool,
    patch_group_identity: WorthQueryEvidenceIdentity,
    patch_group_width: u64,
    maintenance_work: Option<WorthQueryManagedLiveMaintenanceWork>,
    ordered_cause_count: usize,
    suppressed_cause_count: usize,
    denied_cause_count: usize,
}

impl WorthQueryManagedLiveDeliveryBatch {
    fn from_runtime(batch: WorthQueryRuntimeDeliveryBatch) -> Self {
        let mixed = batch.mixed_cause_delivery();
        Self {
            sequence: batch.sequence(),
            delivery_batch_identity: batch.delivery_batch_identity().clone(),
            delivery_window_identity: batch.delivery_window_identity().clone(),
            consumer_attachment_identity: batch.consumer_attachment_identity().clone(),
            cause_kind: WorthQueryManagedLiveDeliveryCauseKind::from_runtime(
                batch.delivery_cause_kind(),
            ),
            cause_identity: batch.delivery_cause_identity().clone(),
            has_relational_patch: batch.has_relational_patch(),
            patch_group_identity: batch.patch_group_identity().clone(),
            patch_group_width: batch.patch_group_width(),
            maintenance_work: batch
                .live_graph_read_maintenance()
                .map(WorthQueryManagedLiveMaintenanceWork::from_runtime),
            ordered_cause_count: mixed.ordered_cause_identities().len(),
            suppressed_cause_count: mixed.suppressed_cause_identities().len(),
            denied_cause_count: mixed.denied_cause_identities().len(),
        }
    }

    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn delivery_batch_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.delivery_batch_identity
    }

    pub fn delivery_window_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.delivery_window_identity
    }

    pub fn consumer_attachment_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.consumer_attachment_identity
    }

    pub fn cause_kind(&self) -> WorthQueryManagedLiveDeliveryCauseKind {
        self.cause_kind
    }

    pub fn cause_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.cause_identity
    }

    pub fn has_relational_patch(&self) -> bool {
        self.has_relational_patch
    }

    pub fn patch_group_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.patch_group_identity
    }

    pub fn patch_group_width(&self) -> u64 {
        self.patch_group_width
    }

    pub fn maintenance_work(&self) -> Option<&WorthQueryManagedLiveMaintenanceWork> {
        self.maintenance_work.as_ref()
    }

    pub fn ordered_cause_count(&self) -> usize {
        self.ordered_cause_count
    }

    pub fn suppressed_cause_count(&self) -> usize {
        self.suppressed_cause_count
    }

    pub fn denied_cause_count(&self) -> usize {
        self.denied_cause_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryManagedLiveMaintenanceWork {
    maintenance_delta_identity: WorthQueryEvidenceIdentity,
    mutation_delta_count: usize,
    affected_requirement_row_count: usize,
    touched_edge_count: usize,
    touched_frontier_count: usize,
    index_update_count: usize,
    live_view_update_count: usize,
    skipped_unaffected_requirement_count: usize,
    strategy_recompute_count: usize,
    background_index_build_count: usize,
}

impl WorthQueryManagedLiveMaintenanceWork {
    fn from_runtime(receipt: &WorthQueryLiveGraphReadMaintenanceReceipt) -> Self {
        let counters = receipt.maintenance_counters();
        Self {
            maintenance_delta_identity: receipt.maintenance_delta_identity().clone(),
            mutation_delta_count: counters.mutation_delta_count(),
            affected_requirement_row_count: counters.affected_requirement_row_count(),
            touched_edge_count: counters.touched_edge_count(),
            touched_frontier_count: counters.touched_frontier_count(),
            index_update_count: counters.index_update_count(),
            live_view_update_count: counters.live_view_update_count(),
            skipped_unaffected_requirement_count: counters.skipped_unaffected_requirement_count(),
            strategy_recompute_count: counters.strategy_recompute_count(),
            background_index_build_count: counters.background_index_build_count(),
        }
    }

    pub fn maintenance_delta_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.maintenance_delta_identity
    }

    pub fn mutation_delta_count(&self) -> usize {
        self.mutation_delta_count
    }

    pub fn affected_requirement_row_count(&self) -> usize {
        self.affected_requirement_row_count
    }

    pub fn touched_edge_count(&self) -> usize {
        self.touched_edge_count
    }

    pub fn touched_frontier_count(&self) -> usize {
        self.touched_frontier_count
    }

    pub fn index_update_count(&self) -> usize {
        self.index_update_count
    }

    pub fn live_view_update_count(&self) -> usize {
        self.live_view_update_count
    }

    pub fn skipped_unaffected_requirement_count(&self) -> usize {
        self.skipped_unaffected_requirement_count
    }

    pub fn strategy_recompute_count(&self) -> usize {
        self.strategy_recompute_count
    }

    pub fn background_index_build_count(&self) -> usize {
        self.background_index_build_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryManagedLiveDelivery {
    resource_name: String,
    batches: Vec<WorthQueryManagedLiveDeliveryBatch>,
}

impl WorthQueryManagedLiveDelivery {
    pub(crate) fn from_runtime(delivery: WorthQueryManagedLiveRuntimeDelivery) -> Self {
        let (resource_name, batches) = delivery.into_parts();
        Self {
            resource_name,
            batches: batches
                .into_iter()
                .map(WorthQueryManagedLiveDeliveryBatch::from_runtime)
                .collect(),
        }
    }

    pub fn resource_name(&self) -> &str {
        &self.resource_name
    }

    pub fn batches(&self) -> &[WorthQueryManagedLiveDeliveryBatch] {
        &self.batches
    }

    pub fn is_empty(&self) -> bool {
        self.batches.is_empty()
    }
}
