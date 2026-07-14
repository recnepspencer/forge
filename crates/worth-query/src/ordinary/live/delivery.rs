use crate::evidence_identity::WorthQueryEvidenceIdentity;
use crate::runtime::{WorthQueryPatchBatch, WorthQueryRuntimeDeliveryBatch};
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
    cause_kind: WorthQueryManagedLiveDeliveryCauseKind,
    cause_identity: WorthQueryEvidenceIdentity,
    has_relational_patch: bool,
    ordered_cause_count: usize,
    suppressed_cause_count: usize,
    denied_cause_count: usize,
}

impl WorthQueryManagedLiveDeliveryBatch {
    fn from_runtime(batch: WorthQueryRuntimeDeliveryBatch) -> Self {
        let mixed = batch.mixed_cause_delivery();
        Self {
            sequence: batch.sequence(),
            cause_kind: WorthQueryManagedLiveDeliveryCauseKind::from_runtime(
                batch.delivery_cause_kind(),
            ),
            cause_identity: batch.delivery_cause_identity().clone(),
            has_relational_patch: batch.has_relational_patch(),
            ordered_cause_count: mixed.ordered_cause_identities().len(),
            suppressed_cause_count: mixed.suppressed_cause_identities().len(),
            denied_cause_count: mixed.denied_cause_identities().len(),
        }
    }

    pub fn sequence(&self) -> u64 {
        self.sequence
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
pub struct WorthQueryManagedLiveDelivery {
    resource_name: String,
    batches: Vec<WorthQueryManagedLiveDeliveryBatch>,
}

impl WorthQueryManagedLiveDelivery {
    pub(crate) fn from_runtime(batch: WorthQueryPatchBatch) -> Self {
        debug_assert!(batch.live_patches.is_empty());
        debug_assert!(batch.derived_patches.is_empty());
        debug_assert!(batch.derived_patch_notes.is_empty());
        Self {
            resource_name: batch.view_name,
            batches: batch
                .query_delivery_batches
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
