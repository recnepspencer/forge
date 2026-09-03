use crate::publication::RelationalAttemptProgressPosture;
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
        let settlement_required = effects.progress().relational_requires_settlement();
        if !settlement_required {
            let actions = effects.next_actions().to_vec();
            drop(effects);
            return Ok(RecoveryContinuationContract::new(actions));
        }

        drop(effects);
        let mut update = self
            .state
            .recovery
            .take_record_for_update(&handle)
            .ok_or_else(super::super::ports::RuntimeWorldOwnerUnavailable::new)?;
        let record = update
            .record_mut()
            .ok_or_else(super::super::ports::RuntimeWorldOwnerUnavailable::new)?;
        let mut recovery = record
            .take_relational_recovery()
            .map_err(|()| super::super::ports::RuntimeWorldOwnerUnavailable::new())?;

        if let Some(performed) = recovery.take_performed() {
            match self
                .state
                .relational
                .settlement_port()
                .settle_performed_publication(performed)
            {
                Ok(result)
                    if result.outcome().commit.commit_id
                        == recovery.commit_identity().commit_id() =>
                {
                    record.settle_relational_recovery(recovery, result);
                }
                Ok(_) => record.retain_identity_repair(recovery),
                Err(error) => match error.deferred_settlement() {
                    Some(settlement)
                        if settlement.commit().commit_id
                            == recovery.commit_identity().commit_id() =>
                    {
                        record.retain_pending_relational_settlement(recovery, settlement.clone());
                    }
                    Some(_) | None => record.retain_identity_repair(recovery),
                },
            }
        } else if let Some(settlement) = recovery.settlement().cloned() {
            let performed_result = settlement.performed_result().clone();
            match self
                .state
                .relational
                .settlement_port()
                .repair_deferred_publication_settlement(&settlement)
            {
                Ok(receipt)
                    if receipt == *settlement.commit()
                        && performed_result.outcome().commit == receipt =>
                {
                    record.settle_relational_recovery(recovery, performed_result);
                }
                Ok(_) | Err(_) => record.restore_relational_recovery(recovery),
            }
        } else if let Some(commit_identity) = recovery.take_identity_repair() {
            match self
                .state
                .relational
                .settlement_port()
                .repair_pending_publication_settlement(commit_identity.commit_id())
            {
                Ok(receipt) if receipt.commit_id == commit_identity.commit_id() => {
                    record.settle_relational_recovery_with_receipt(recovery, receipt);
                }
                Ok(_) | Err(_) => {
                    recovery.restore_identity_repair();
                    record.restore_relational_recovery(recovery);
                }
            }
        } else {
            record.restore_relational_recovery(recovery);
            update.finish();
            return Err(super::super::ports::RuntimeWorldOwnerUnavailable::new());
        }

        let actions = record.next_actions().to_vec();
        update.finish();
        Ok(RecoveryContinuationContract::new(actions))
    }
}
