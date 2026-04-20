use crate::{
    backend::engine::{StateBackedStoreBackend, StatePersistence},
    failure::{StoreError, StoreErrorKind},
    tiering::{ColdRecallTierPath, RecallCompletionWitness},
};

use super::shared::{artifact_key_for_family, expected_verification_label};

pub(crate) fn execute_cold_recall<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    lease: crate::ColdRecallLease,
    witness: crate::RecallEligibilityWitness,
) -> Result<RecallCompletionWitness, StoreError> {
    let artifact_key = match lease.artifact_ref().artifact_family() {
        crate::PlacementArtifactFamily::SnapshotFamily => {
            artifact_key_for_family(crate::ColdDerivedFamilyPolicy::SnapshotFamily, lease.artifact_ref().artifact_id())
        }
        crate::PlacementArtifactFamily::BranchDeltaFamily => {
            artifact_key_for_family(crate::ColdDerivedFamilyPolicy::BranchDeltaFamily, lease.artifact_ref().artifact_id())
        }
        crate::PlacementArtifactFamily::Milestone6LayoutFamily => {
            artifact_key_for_family(crate::ColdDerivedFamilyPolicy::Milestone6LayoutFamily, lease.artifact_ref().artifact_id())
        }
        other => {
            backend.counters().record_tier_recall_failures(1);
            return Err(StoreError::new(
                StoreErrorKind::TierRecallExecutionViolation,
                format!("cold recall cannot execute over non-derived placement family `{}`", other.label()),
            ));
        }
    };

    if witness.artifact_key() != artifact_key
        || witness.recall_cost_class() != lease.recall_cost_class()
        || witness.amplification_budget() != lease.amplification_budget()
    {
        backend.counters().record_tier_recall_failures(1);
        return Err(StoreError::new(
            StoreErrorKind::TierRecallExecutionViolation,
            format!(
                "cold recall witness drifted from its lease for artifact `{artifact_key}`"
            ),
        ));
    }

    let verification_label = expected_verification_label(backend.state(), &artifact_key)?;
    backend.counters().record_cold_tier_recalls(1);
    if lease.execution_origin() == crate::PlacementExecutionOrigin::Foreground {
        backend.counters().record_foreground_cold_recalls(1);
    }
    if lease.execution_origin() == crate::PlacementExecutionOrigin::RestartRecovery {
        backend.counters().record_restart_recalls(1);
    }
    backend.counters().record_tier_misses(1);
    Ok(RecallCompletionWitness::new(
        artifact_key,
        ColdRecallTierPath::ColdRecalled,
        verification_label,
    ))
}
