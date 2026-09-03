use super::ProductUnpublishedNextAction;

use crate::identity::ProductUnpublishedOwnerEffectsIdentity;

/// Explicit recovery cleanup permission. It cannot be inferred from a
/// product publication or from dropping a caller's inspection handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RecoveryCleanupContract {
    action: ProductUnpublishedNextAction,
}

impl RecoveryCleanupContract {
    pub(crate) const fn release_obligations() -> Self {
        Self {
            action: ProductUnpublishedNextAction::ReleaseObligations,
        }
    }

    pub(crate) const fn action(self) -> ProductUnpublishedNextAction {
        self.action
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RecoveryCleanupDenial {
    Missing,
    ForeignCatalog,
    CallerCapabilityLive,
    SettlementRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RecoveryCleanupOutcome {
    identity: ProductUnpublishedOwnerEffectsIdentity,
}

impl RecoveryCleanupOutcome {
    pub(crate) fn identity(&self) -> &ProductUnpublishedOwnerEffectsIdentity {
        &self.identity
    }
}

impl super::catalog::ProductUnpublishedRecoveryCatalog {
    pub(crate) fn cleanup_record(
        &self,
        handle: &super::ProductUnpublishedRecoveryHandle,
    ) -> Result<RecoveryCleanupOutcome, RecoveryCleanupDenial> {
        if handle.catalog_affinity() != self.affinity() {
            return Err(RecoveryCleanupDenial::ForeignCatalog);
        }
        let removed =
            self.remove_record_if_exclusive(handle, |record| !record.settlement_required());
        match removed {
            Ok(Some(record)) => {
                let identity = record.identity().clone();
                drop(record);
                Ok(RecoveryCleanupOutcome { identity })
            }
            Ok(None) => Err(RecoveryCleanupDenial::Missing),
            Err(super::catalog::RecoveryRecordRemovalDenial::CallerCapabilityLive) => {
                Err(RecoveryCleanupDenial::CallerCapabilityLive)
            }
            Err(super::catalog::RecoveryRecordRemovalDenial::NotEligible) => {
                Err(RecoveryCleanupDenial::SettlementRequired)
            }
        }
    }
}
