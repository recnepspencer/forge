use crate::{
    backend::engine::{StateBackedStoreBackend, StatePersistence},
    failure::{StoreError, StoreErrorKind},
    tiering::RetiredTierReplica,
};

use super::shared::transfer_record;

pub(crate) fn retire_tier_replica<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    cutover: crate::TierCutoverWitness,
) -> Result<RetiredTierReplica, StoreError> {
    let transfer = transfer_record(backend.state(), cutover.artifact_key())?;
    if !transfer.cutover_completed {
        return Err(StoreError::new(
            StoreErrorKind::TierCutoverViolation,
            format!(
                "artifact `{}` cannot retire its prior replica before cutover is complete",
                cutover.artifact_key()
            ),
        ));
    }
    let retired_locator = transfer.source_replica_locator.clone();
    let mut next = backend.state().clone();
    next.tier_transfer_records.remove(cutover.artifact_key());
    backend.commit_replacement_state(next)?;
    Ok(RetiredTierReplica::new(cutover, retired_locator))
}
