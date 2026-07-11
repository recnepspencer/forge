#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiveQuerySemanticAuthority;

pub const fn live_query_semantic_authority() -> LiveQuerySemanticAuthority {
    LiveQuerySemanticAuthority
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StableBasisId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContinuationRetentionStatus {
    Retained,
    RetentionRebindRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableBasisReadPlan {
    stable_basis_id: StableBasisId,
    declared_support_rows: u32,
    retention_status: ContinuationRetentionStatus,
}

impl StableBasisReadPlan {
    pub(crate) fn new(
        stable_basis_id: StableBasisId,
        declared_support_rows: u32,
        retention_status: ContinuationRetentionStatus,
    ) -> Self {
        Self {
            stable_basis_id,
            declared_support_rows,
            retention_status,
        }
    }

    pub const fn stable_basis_id(&self) -> StableBasisId {
        self.stable_basis_id
    }

    pub const fn declared_support_rows(&self) -> u32 {
        self.declared_support_rows
    }

    pub const fn retention_status(&self) -> ContinuationRetentionStatus {
        self.retention_status
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorContinuationPlan {
    stable_basis_id: StableBasisId,
    declared_window_rows: u32,
    retention_status: ContinuationRetentionStatus,
}

impl CursorContinuationPlan {
    pub(crate) fn new(
        stable_basis_id: StableBasisId,
        declared_window_rows: u32,
        retention_status: ContinuationRetentionStatus,
    ) -> Self {
        Self {
            stable_basis_id,
            declared_window_rows,
            retention_status,
        }
    }

    pub(crate) fn admit_narrow_batch(&self, admitted_window_rows: u32) -> AdmittedNarrowBatchReceipt {
        AdmittedNarrowBatchReceipt {
            stable_basis_id: self.stable_basis_id,
            admitted_window_rows,
        }
    }

    pub(crate) fn record_broadened_batch(
        &self,
        broadened_window_rows: u32,
    ) -> BroadenedBatchReceipt {
        BroadenedBatchReceipt {
            stable_basis_id: self.stable_basis_id,
            broadened_window_rows,
        }
    }

    pub const fn stable_basis_id(&self) -> StableBasisId {
        self.stable_basis_id
    }

    pub const fn declared_window_rows(&self) -> u32 {
        self.declared_window_rows
    }

    pub const fn retention_status(&self) -> ContinuationRetentionStatus {
        self.retention_status
    }
}

impl LiveQuerySemanticAuthority {
    pub fn declare_stable_basis_support(
        self,
        stable_basis_id: StableBasisId,
        declared_support_rows: u32,
        retention_status: ContinuationRetentionStatus,
    ) -> StableBasisReadPlan {
        StableBasisReadPlan::new(
            stable_basis_id,
            declared_support_rows,
            retention_status,
        )
    }

    pub fn declare_continuation_window(
        self,
        stable_basis_id: StableBasisId,
        declared_window_rows: u32,
        retention_status: ContinuationRetentionStatus,
    ) -> CursorContinuationPlan {
        CursorContinuationPlan::new(
            stable_basis_id,
            declared_window_rows,
            retention_status,
        )
    }

    pub fn admit_narrow_batch(
        self,
        plan: &CursorContinuationPlan,
        admitted_window_rows: u32,
    ) -> AdmittedNarrowBatchReceipt {
        plan.admit_narrow_batch(admitted_window_rows)
    }

    pub fn record_broadened_batch(
        self,
        plan: &CursorContinuationPlan,
        broadened_window_rows: u32,
    ) -> BroadenedBatchReceipt {
        plan.record_broadened_batch(broadened_window_rows)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedNarrowBatchReceipt {
    stable_basis_id: StableBasisId,
    admitted_window_rows: u32,
}

impl AdmittedNarrowBatchReceipt {
    pub const fn stable_basis_id(&self) -> StableBasisId {
        self.stable_basis_id
    }

    pub const fn admitted_window_rows(&self) -> u32 {
        self.admitted_window_rows
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BroadenedBatchReceipt {
    stable_basis_id: StableBasisId,
    broadened_window_rows: u32,
}

impl BroadenedBatchReceipt {
    pub const fn stable_basis_id(&self) -> StableBasisId {
        self.stable_basis_id
    }

    pub const fn broadened_window_rows(&self) -> u32 {
        self.broadened_window_rows
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuationBatchResult {
    stable_basis_id: StableBasisId,
    returned_rows: u32,
}

impl ContinuationBatchResult {
    pub const fn new(stable_basis_id: StableBasisId, returned_rows: u32) -> Self {
        Self {
            stable_basis_id,
            returned_rows,
        }
    }

    pub const fn stable_basis_id(&self) -> StableBasisId {
        self.stable_basis_id
    }

    pub const fn returned_rows(&self) -> u32 {
        self.returned_rows
    }
}
