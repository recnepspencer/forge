use crate::ContinuationRetentionStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StableBasisId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableBasisReadPlan {
    stable_basis_id: StableBasisId,
    declared_support_rows: u32,
    retention_status: ContinuationRetentionStatus,
}

impl StableBasisReadPlan {
    pub(super) const fn new(
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
