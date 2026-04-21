use crate::{
    backend::engine::{StateBackedStoreBackend, StatePersistence},
    failure::{StoreError, StoreErrorKind},
    tiering::TierCutoverWitness,
};

use super::shared::{placement_family_for_artifact_key, transfer_record, transfer_record_mut};

pub(crate) fn cutover_tier_replica<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    verified: crate::VerifiedTierReplica,
) -> Result<TierCutoverWitness, StoreError> {
    let artifact_key = verified.transferred_replica().intent().artifact_key();
    let transfer = transfer_record(backend.state(), artifact_key)?;
    let Some(locator) = &transfer.transferred_replica_locator else {
        backend.counters().record_tier_move_cutover_rejections(1);
        return Err(StoreError::new(
            StoreErrorKind::TierCutoverViolation,
            format!("artifact `{artifact_key}` cannot cut over without a transferred replica"),
        ));
    };
    let Some(verification_label) = &transfer.verification_label else {
        backend.counters().record_tier_move_cutover_rejections(1);
        return Err(StoreError::new(
            StoreErrorKind::TierCutoverViolation,
            format!(
                "artifact `{artifact_key}` cannot cut over without a persisted verification label"
            ),
        ));
    };
    if verification_label != verified.verification_label() {
        backend.counters().record_tier_move_cutover_rejections(1);
        return Err(StoreError::new(
            StoreErrorKind::TierCutoverViolation,
            format!(
                "artifact `{artifact_key}` verification drifted before cutover: persisted `{verification_label}` but witness carried `{}`",
                verified.verification_label()
            ),
        ));
    }
    let mut next = backend.state().clone();
    let transfer = transfer_record_mut(&mut next, artifact_key)?;
    transfer.cutover_completed = true;
    next.tier_residency_records.insert(
        artifact_key.to_string(),
        crate::backend::records::TierResidencyRecord {
            artifact_key: artifact_key.to_string(),
            artifact_family: placement_family_for_artifact_key(artifact_key)?,
            canonical_residence: verified.transferred_replica().intent().target_residence(),
            canonical_replica_locator: locator.clone(),
            verification_label: verification_label.clone(),
        },
    );
    backend.commit_replacement_state(next)?;
    backend.counters().record_tier_move_cutovers(1);
    Ok(TierCutoverWitness::new(
        artifact_key.to_string(),
        verified.transferred_replica().intent().target_residence(),
    ))
}
