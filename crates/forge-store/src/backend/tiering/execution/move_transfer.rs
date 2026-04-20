use crate::{
    backend::engine::{StateBackedStoreBackend, StatePersistence},
    failure::StoreError,
    tiering::TransferredTierReplica,
};

use super::shared::{default_locator, transfer_record_mut};

pub(crate) fn transfer_tier_replica<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    intent: crate::TierTransferIntent,
) -> Result<TransferredTierReplica, StoreError> {
    let mut next = backend.state().clone();
    let transfer = transfer_record_mut(&mut next, intent.artifact_key())?;
    let locator = default_locator(intent.artifact_key(), intent.target_residence());
    transfer.transferred_replica_locator = Some(locator.clone());
    backend.commit_replacement_state(next)?;
    Ok(TransferredTierReplica::new(intent, locator))
}
