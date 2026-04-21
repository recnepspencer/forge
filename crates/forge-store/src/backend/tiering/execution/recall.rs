use crate::{
    backend::engine::{StateBackedStoreBackend, StatePersistence},
    failure::{StoreError, StoreErrorKind},
    tiering::{
        CoalescedRecallReport, PlacementArtifactFamily, RecallCompletionWitness,
        RecallExecutionDisposition, RetainedReadPlacementPath,
    },
};

use super::{
    recall_coalescing::build_recall_report,
    recall_registry::{admit_or_join_recall, complete_recall, RecallRegistryAdmission},
    shared::{
        artifact_key_for_family, expected_verification_label, recall_coalescing_key_for_artifact,
    },
};

pub(crate) fn execute_cold_recall<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    lease: crate::ColdRecallLease,
    witness: crate::RecallEligibilityWitness,
) -> Result<CoalescedRecallReport, StoreError> {
    let (artifact_family, artifact_key) = match lease.artifact_ref().artifact_family() {
        PlacementArtifactFamily::SnapshotFamily => (
            PlacementArtifactFamily::SnapshotFamily,
            artifact_key_for_family(
                crate::ColdDerivedFamilyPolicy::SnapshotFamily,
                lease.artifact_ref().artifact_id(),
            ),
        ),
        PlacementArtifactFamily::BranchDeltaFamily => (
            PlacementArtifactFamily::BranchDeltaFamily,
            artifact_key_for_family(
                crate::ColdDerivedFamilyPolicy::BranchDeltaFamily,
                lease.artifact_ref().artifact_id(),
            ),
        ),
        PlacementArtifactFamily::Milestone6LayoutFamily => (
            PlacementArtifactFamily::Milestone6LayoutFamily,
            artifact_key_for_family(
                crate::ColdDerivedFamilyPolicy::Milestone6LayoutFamily,
                lease.artifact_ref().artifact_id(),
            ),
        ),
        other => {
            backend.counters().record_tier_recall_failures(1);
            return Err(StoreError::new(
                StoreErrorKind::TierRecallExecutionViolation,
                format!(
                    "cold recall cannot execute over non-derived placement family `{}`",
                    other.label()
                ),
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
            format!("cold recall witness drifted from its lease for artifact `{artifact_key}`"),
        ));
    }

    let coalescing_key =
        recall_coalescing_key_for_artifact(artifact_family, lease.artifact_ref().artifact_id());
    let admission = admit_or_join_recall(
        backend,
        &coalescing_key,
        artifact_family,
        lease.execution_origin(),
        &artifact_key,
        lease.recall_cost_class(),
        lease.amplification_budget(),
    )?;

    let verification_label = expected_verification_label(backend.state(), &artifact_key)?;
    let placement_path = RetainedReadPlacementPath::ColdRecalled;
    let completion = RecallCompletionWitness::new(
        artifact_key.clone(),
        placement_path,
        verification_label.clone(),
    );

    match admission {
        RecallRegistryAdmission::ExecuteFresh { key_string } => {
            complete_recall(backend, &key_string)?;
            backend.counters().record_cold_tier_recalls(1);
            if lease.execution_origin() == crate::PlacementExecutionOrigin::Foreground {
                backend.counters().record_foreground_cold_recalls(1);
            }
            if lease.execution_origin() == crate::PlacementExecutionOrigin::RestartRecovery {
                backend.counters().record_restart_recalls(1);
            }
            backend.counters().record_tier_misses(1);
            Ok(build_recall_report(
                coalescing_key,
                RecallExecutionDisposition::Executed,
                &artifact_key,
                placement_path,
                &verification_label,
                Some(completion),
            ))
        }
        RecallRegistryAdmission::ResumeInFlight { key_string } => {
            complete_recall(backend, &key_string)?;
            backend.counters().record_cold_tier_recalls(1);
            if lease.execution_origin() == crate::PlacementExecutionOrigin::Foreground {
                backend.counters().record_foreground_cold_recalls(1);
            }
            if lease.execution_origin() == crate::PlacementExecutionOrigin::RestartRecovery {
                backend.counters().record_restart_recalls(1);
            }
            backend.counters().record_tier_misses(1);
            Ok(build_recall_report(
                coalescing_key,
                RecallExecutionDisposition::Executed,
                &artifact_key,
                placement_path,
                &verification_label,
                Some(completion),
            ))
        }
        RecallRegistryAdmission::JoinInFlight => Ok(build_recall_report(
            coalescing_key,
            RecallExecutionDisposition::CoalescedJoin,
            &artifact_key,
            placement_path,
            &verification_label,
            None,
        )),
    }
}
