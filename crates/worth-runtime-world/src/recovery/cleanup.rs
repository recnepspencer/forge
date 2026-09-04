use super::ProductUnpublishedNextAction;

use crate::identity::{
    ProductBranchIdentity, ProductBranchIncarnation, ProductUnpublishedOwnerEffectsIdentity,
};

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

/// What the catalog released: the record's identity and the product-branch
/// occurrence it named. The occurrence travels with the release because the
/// record was the only thing naming it, and the custody charged to it must be
/// drained by whoever released the record, on every release path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RecoveryCleanupOutcome {
    identity: ProductUnpublishedOwnerEffectsIdentity,
    destination: Option<(ProductBranchIdentity, ProductBranchIncarnation)>,
}

impl RecoveryCleanupOutcome {
    pub(crate) fn identity(&self) -> &ProductUnpublishedOwnerEffectsIdentity {
        &self.identity
    }

    /// The occurrence whose custody the released record was answerable for,
    /// when the attempt created one.
    pub(crate) fn destination(&self) -> Option<(&ProductBranchIdentity, ProductBranchIncarnation)> {
        self.destination
            .as_ref()
            .map(|(branch, incarnation)| (branch, *incarnation))
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
                let destination = record
                    .destination()
                    .map(|(branch, incarnation)| (branch.clone(), incarnation));
                drop(record);
                Ok(RecoveryCleanupOutcome {
                    identity,
                    destination,
                })
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
