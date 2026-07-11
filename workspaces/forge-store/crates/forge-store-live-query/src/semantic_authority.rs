use crate::{
    AdmittedNarrowBatchReceipt, BroadenedBatchReceipt, ContinuationRetentionStatus,
    CursorContinuationPlan, StableBasisId, StableBasisReadPlan,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiveQuerySemanticAuthority;

pub const fn live_query_semantic_authority() -> LiveQuerySemanticAuthority {
    LiveQuerySemanticAuthority
}

impl LiveQuerySemanticAuthority {
    pub fn declare_stable_basis_support(
        self,
        id: StableBasisId,
        rows: u32,
        retention: ContinuationRetentionStatus,
    ) -> StableBasisReadPlan {
        StableBasisReadPlan::new(id, rows, retention)
    }

    pub fn declare_continuation_window(
        self,
        id: StableBasisId,
        rows: u32,
        retention: ContinuationRetentionStatus,
    ) -> CursorContinuationPlan {
        CursorContinuationPlan::new(id, rows, retention)
    }

    pub fn admit_narrow_batch(
        self,
        plan: &CursorContinuationPlan,
        rows: u32,
    ) -> AdmittedNarrowBatchReceipt {
        plan.admit_narrow_batch(rows)
    }

    pub fn record_broadened_batch(
        self,
        plan: &CursorContinuationPlan,
        rows: u32,
    ) -> BroadenedBatchReceipt {
        plan.record_broadened_batch(rows)
    }
}
