use crate::recovery::{
    ProductUnpublishedOwnerEffects, ProductUnpublishedRecoveryHandle, RecoveryContinuationContract,
};

use super::RuntimeWorldOwnerRoot;

#[cfg(test)]
#[path = "recovery_service/tests.rs"]
mod settlement_catalog_tests;

impl<D, I, E, Ctx, T> RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    /// Enumerate owner-issued recovery handles. A handle is only an
    /// attempt-affine inspection key; it cannot publish a product branch or
    /// mint a component-owner capability.
    pub fn recovery_handles(&self) -> Vec<ProductUnpublishedRecoveryHandle> {
        let affinity = self.state.recovery.affinity();
        self.state
            .recovery
            .identities()
            .into_iter()
            .map(|identity| ProductUnpublishedRecoveryHandle::new(identity, affinity))
            .collect()
    }

    pub fn recovery_record_count(&self) -> usize {
        self.state.recovery.installed_slots()
    }

    pub fn inspect_recovery(
        &self,
        handle: &ProductUnpublishedRecoveryHandle,
    ) -> Option<ProductUnpublishedOwnerEffects> {
        self.state
            .recovery
            .lookup_record(handle)
            .map(ProductUnpublishedOwnerEffects::from_catalog_record)
    }

    /// Consume a caller capability and explicitly release a record only when
    /// no Relational settlement route remains. The catalog retains custody if
    /// a separate caller still inspects the same record.
    pub(crate) fn cleanup_recovery(&self, effects: ProductUnpublishedOwnerEffects) -> bool {
        let handle = effects.recovery_handle();
        drop(effects);
        self.state.recovery.cleanup_record(&handle).is_ok()
    }

    pub(crate) fn cleanup_recovery_handle(
        &self,
        handle: &ProductUnpublishedRecoveryHandle,
    ) -> bool {
        self.state.recovery.cleanup_record(handle).is_ok()
    }
}

impl<D, I, E, Ctx, T> super::super::ports::RuntimeWorldRecoveryService
    for RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    fn continue_effects(
        &self,
        effects: ProductUnpublishedOwnerEffects,
    ) -> Result<RecoveryContinuationContract, super::super::ports::RuntimeWorldOwnerUnavailable>
    {
        if self.lifecycle_observation() != super::super::RuntimeWorldOwnerLifecycleObservation::Open
        {
            return Err(super::super::ports::RuntimeWorldOwnerUnavailable::new());
        }
        let handle = effects.recovery_handle();
        if self.state.recovery.lookup_record(&handle).is_none() {
            return Err(super::super::ports::RuntimeWorldOwnerUnavailable::new());
        }
        Ok(RecoveryContinuationContract::new(
            effects.next_actions().to_vec(),
        ))
    }
}
