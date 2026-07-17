use super::operation_cutover::{
    ReadmittedPointInTimeRecoveryCurrent, ReadmittedRollbackCurrent,
    RecoverySourceLeaseFinalizationDenial,
};

impl ReadmittedPointInTimeRecoveryCurrent {
    pub fn release_source_lease(
        mut self,
    ) -> Result<
        worth_store_physical_isolation::RecoverySourceLeaseReleaseReceipt,
        RecoverySourceLeaseFinalizationDenial,
    > {
        match self.0.source_lease.take() {
            Some(super::post_verification::RecoveryCutoverSourceLease::PointInTime(lease)) => lease
                .release()
                .map_err(RecoverySourceLeaseFinalizationDenial::Isolation),
            _ => Err(RecoverySourceLeaseFinalizationDenial::MissingOrWrongLease),
        }
    }
}

impl ReadmittedRollbackCurrent {
    pub fn release_source_lease(
        mut self,
    ) -> Result<
        worth_store_physical_isolation::RecoverySourceLeaseReleaseReceipt,
        RecoverySourceLeaseFinalizationDenial,
    > {
        match self.0.source_lease.take() {
            Some(super::post_verification::RecoveryCutoverSourceLease::Rollback(lease)) => lease
                .release()
                .map_err(RecoverySourceLeaseFinalizationDenial::Isolation),
            _ => Err(RecoverySourceLeaseFinalizationDenial::MissingOrWrongLease),
        }
    }
}
