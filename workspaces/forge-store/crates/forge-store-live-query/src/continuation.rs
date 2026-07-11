use crate::StableBasisId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContinuationRetentionStatus {
    Retained,
    RetentionRebindRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorContinuationPlan {
    stable_basis_id: StableBasisId,
    declared_window_rows: u32,
    retention_status: ContinuationRetentionStatus,
}

impl CursorContinuationPlan {
    pub(super) const fn new(
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

    pub(super) const fn admit_narrow_batch(
        &self,
        admitted_window_rows: u32,
    ) -> AdmittedNarrowBatchReceipt {
        AdmittedNarrowBatchReceipt {
            stable_basis_id: self.stable_basis_id,
            admitted_window_rows,
        }
    }

    pub(super) const fn record_broadened_batch(
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
