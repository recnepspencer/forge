use crate::{
    backend::engine::{StateBackedStoreBackend, StatePersistence},
    failure::{StoreError, StoreErrorKind},
    tiering::VerifiedTierReplica,
};

use super::shared::{expected_verification_label, transfer_record, transfer_record_mut};

pub(crate) fn verify_tier_replica<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    transferred: crate::TransferredTierReplica,
) -> Result<VerifiedTierReplica, StoreError> {
    let persisted = transfer_record(backend.state(), transferred.intent().artifact_key())?;
    let Some(locator) = &persisted.transferred_replica_locator else {
        return Err(StoreError::new(
            StoreErrorKind::TierTransferVerificationFailed,
            format!(
                "artifact `{}` has not recorded a transferred replica locator",
                transferred.intent().artifact_key()
            ),
        ));
    };
    if locator != transferred.replica_locator() {
        backend.counters().record_tier_move_cutover_rejections(1);
        return Err(StoreError::new(
            StoreErrorKind::TierTransferVerificationFailed,
            format!(
                "transferred replica locator drifted for `{}`: persisted `{locator}` but witness carried `{}`",
                transferred.intent().artifact_key(),
                transferred.replica_locator()
            ),
        ));
    }
    let verification_label =
        expected_verification_label(backend.state(), transferred.intent().artifact_key())?;
    let mut next = backend.state().clone();
    let transfer = transfer_record_mut(&mut next, transferred.intent().artifact_key())?;
    transfer.verification_label = Some(verification_label.clone());
    backend.commit_replacement_state(next)?;
    Ok(VerifiedTierReplica::new(transferred, verification_label))
}
