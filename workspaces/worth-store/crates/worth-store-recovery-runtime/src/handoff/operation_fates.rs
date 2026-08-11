use worth_store_recovery_physics::{ReconciledOperationFate, ReconciledOperationFates};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryOperationFateSet {
    reconciled: ReconciledOperationFates,
}

impl RecoveryOperationFateSet {
    pub(crate) const fn new(reconciled: ReconciledOperationFates) -> Self {
        Self { reconciled }
    }

    pub fn operations(&self) -> &[ReconciledOperationFate] {
        self.reconciled.operations()
    }

    pub const fn acknowledged_durable(&self) -> u64 {
        self.reconciled.acknowledged_durable()
    }

    pub const fn durable_unacknowledged(&self) -> u64 {
        self.reconciled.durable_unacknowledged()
    }

    pub const fn proven_no_effect(&self) -> u64 {
        self.reconciled.proven_no_effect()
    }

    pub const fn indeterminate(&self) -> u64 {
        self.reconciled.indeterminate()
    }
}
