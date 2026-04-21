use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        records::{TierRecallCompletionState, TierRecallRecord},
    },
    failure::StoreError,
    tiering::RecallCoalescingKey,
};

use super::shared::{recall_record, recall_record_key};

pub(crate) enum RecallRegistryAdmission {
    ExecuteFresh { key_string: String },
    ResumeInFlight { key_string: String },
    JoinInFlight,
}

pub(crate) fn admit_or_join_recall<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    coalescing_key: &RecallCoalescingKey,
    artifact_family: crate::PlacementArtifactFamily,
    execution_origin: crate::PlacementExecutionOrigin,
    artifact_key: &str,
    recall_cost_class: crate::RecallCostClass,
    amplification_budget: crate::RecallAmplificationBudget,
) -> Result<RecallRegistryAdmission, StoreError> {
    let key_string = recall_record_key(coalescing_key);
    let maybe_existing = backend
        .state()
        .tier_recall_records
        .get(&key_string)
        .cloned();

    match maybe_existing {
        Some(record) if record.completion_state == TierRecallCompletionState::Completed => {
            Err(crate::StoreError::new(
                crate::StoreErrorKind::TierRecallExecutionViolation,
                format!(
                    "completed tier recall `{}` must not remain resident as coalescing state",
                    record.coalescing_key
                ),
            ))
        }
        Some(_) if execution_origin == crate::PlacementExecutionOrigin::RestartRecovery => {
            Ok(RecallRegistryAdmission::ResumeInFlight { key_string })
        }
        Some(_) => {
            backend.counters().record_recall_duplicate_suppression(1);
            Ok(RecallRegistryAdmission::JoinInFlight)
        }
        None => {
            let mut next = backend.state().clone();
            next.tier_recall_records.insert(
                key_string.clone(),
                TierRecallRecord {
                    coalescing_key: key_string.clone(),
                    artifact_family,
                    scope_class: coalescing_key.scope_class(),
                    scope_key: coalescing_key.scope_key().to_string(),
                    execution_origin,
                    artifact_key: artifact_key.to_string(),
                    recall_cost_class,
                    amplification_budget,
                    completion_state: TierRecallCompletionState::InFlight,
                },
            );
            backend.commit_replacement_state(next)?;
            backend.counters().record_recall_coalesced_requests(1);
            Ok(RecallRegistryAdmission::ExecuteFresh { key_string })
        }
    }
}

pub(crate) fn complete_recall<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    coalescing_key: &str,
) -> Result<(), StoreError> {
    let mut next = backend.state().clone();
    next.tier_recall_records
        .remove(coalescing_key)
        .ok_or_else(|| recall_record(backend.state(), coalescing_key).unwrap_err())?;
    backend.commit_replacement_state(next)?;
    Ok(())
}

#[cfg(test)]
pub(crate) fn admit_inflight_cold_recall<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    artifact_ref: crate::PlacementBoundArtifactRef,
    execution_origin: crate::PlacementExecutionOrigin,
) -> Result<(), StoreError> {
    let artifact_family = artifact_ref.artifact_family();
    let artifact_key = match artifact_family {
        crate::PlacementArtifactFamily::SnapshotFamily => {
            format!("snapshot:{}", artifact_ref.artifact_id())
        }
        crate::PlacementArtifactFamily::BranchDeltaFamily => {
            format!("branch_delta:{}", artifact_ref.artifact_id())
        }
        crate::PlacementArtifactFamily::Milestone6LayoutFamily => {
            format!("milestone6_layout:{}", artifact_ref.artifact_id())
        }
        other => {
            return Err(crate::StoreError::new(
                crate::StoreErrorKind::TierRecallExecutionViolation,
                format!(
                    "cannot seed in-flight cold recall for non-derived placement family `{}`",
                    other.label()
                ),
            ))
        }
    };
    let coalescing_key = crate::backend::tiering::recall_coalescing_key_for_artifact(
        artifact_family,
        artifact_ref.artifact_id(),
    );
    let recall_cost_class = match execution_origin {
        crate::PlacementExecutionOrigin::Foreground => crate::RecallCostClass::Bounded,
        crate::PlacementExecutionOrigin::Background
        | crate::PlacementExecutionOrigin::RestartRecovery => crate::RecallCostClass::Deferred,
    };
    match admit_or_join_recall(
        backend,
        &coalescing_key,
        artifact_family,
        execution_origin,
        &artifact_key,
        recall_cost_class,
        crate::RecallAmplificationBudget::SingleFamilyLocalUnit,
    )? {
        RecallRegistryAdmission::ExecuteFresh { .. } => Ok(()),
        RecallRegistryAdmission::ResumeInFlight { .. } | RecallRegistryAdmission::JoinInFlight => {
            Err(crate::StoreError::new(
                crate::StoreErrorKind::TierRecallExecutionViolation,
                format!(
                    "test-only in-flight recall admission expected a fresh recall unit for `{artifact_key}`"
                ),
            ))
        }
    }
}
