use crate::{
    backend::engine::{StateBackedStoreBackend, StatePersistence},
    failure::{StoreError, StoreErrorKind},
    tiering::{AuthoritativeTierMovePlan, DerivedTierMovePlan, TierTransferIntent},
};

use super::shared::{
    current_residency_record, placement_family_for_artifact_key, record_background_move_counters,
};

pub(crate) fn prepare_authoritative_tier_move<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    plan: AuthoritativeTierMovePlan,
) -> Result<TierTransferIntent, StoreError> {
    prepare_tier_move(
        backend,
        plan.artifact_key(),
        plan.target_residence(),
        plan.execution_origin(),
    )
}

pub(crate) fn prepare_derived_tier_move<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    plan: DerivedTierMovePlan,
) -> Result<TierTransferIntent, StoreError> {
    let artifact_key = match plan.artifact_family() {
        crate::PlacementArtifactFamily::SnapshotFamily => {
            format!("snapshot:{}", plan.artifact_id())
        }
        crate::PlacementArtifactFamily::BranchDeltaFamily => {
            format!("branch_delta:{}", plan.artifact_id())
        }
        crate::PlacementArtifactFamily::Milestone6LayoutFamily => {
            format!("milestone6_layout:{}", plan.artifact_id())
        }
        other => {
            return Err(StoreError::new(
                StoreErrorKind::PlacementWitnessConstructionViolation,
                format!(
                    "derived move plan cannot target non-derived placement family `{}`",
                    other.label()
                ),
            ))
        }
    };
    prepare_tier_move(
        backend,
        &artifact_key,
        plan.target_residence(),
        plan.execution_origin(),
    )
}

fn prepare_tier_move<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    artifact_key: &str,
    target_residence: crate::TierResidenceClass,
    execution_origin: crate::PlacementExecutionOrigin,
) -> Result<TierTransferIntent, StoreError> {
    let current = current_residency_record(backend.state(), artifact_key)?;
    if backend
        .state()
        .tier_transfer_records
        .contains_key(artifact_key)
    {
        return Err(StoreError::new(
            StoreErrorKind::TierResidencyManifestViolation,
            format!("artifact `{artifact_key}` already has an in-flight tier transfer"),
        ));
    }
    let intent = TierTransferIntent::new(
        artifact_key.to_string(),
        current.canonical_residence,
        target_residence,
        execution_origin,
    );
    let artifact_family = placement_family_for_artifact_key(artifact_key)?;
    let mut next = backend.state().clone();
    next.tier_transfer_records.insert(
        artifact_key.to_string(),
        crate::backend::records::TierTransferRecord {
            artifact_key: artifact_key.to_string(),
            artifact_family,
            source_residence: current.canonical_residence,
            target_residence,
            execution_origin,
            source_replica_locator: current.canonical_replica_locator,
            transferred_replica_locator: None,
            verification_label: None,
            cutover_completed: false,
        },
    );
    backend.commit_replacement_state(next)?;
    record_background_move_counters(backend, artifact_family, execution_origin);
    Ok(intent)
}
