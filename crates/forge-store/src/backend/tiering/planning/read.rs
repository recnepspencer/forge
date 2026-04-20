use crate::{
    backend::engine::{StateBackedStoreBackend, StatePersistence},
    failure::StoreError,
    tiering::{
        ColdDerivedFamilyPolicy, ColdRecallLease, PlacementArtifactFamily,
        PlacementBoundArtifactRef, PlacementBudgetClass, PlacementExecutionOrigin,
        PlacementObservationScopeClass, ReadPlacementPlanningReport, RecallAmplificationBudget,
        RecallBreadthSummary, RecallCoalescingKey, RecallCostClass, RecallEligibilityWitness,
        ResidentReadLease, TierMoveRejection, TierResidenceClass,
    },
};

use super::shared::{
    artifact_key_for_family, ensure_branch_head_present, ensure_family_artifact_present,
    ensure_stable_basis_present, family_from_read_ref,
};

pub(crate) fn plan_resident_read_lease<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    artifact_ref: PlacementBoundArtifactRef,
    execution_origin: PlacementExecutionOrigin,
) -> Result<ReadPlacementPlanningReport, StoreError> {
    let report = match artifact_ref.artifact_family() {
        PlacementArtifactFamily::AuthoritativeBranchHead => {
            ensure_branch_head_present(backend.state(), artifact_ref.artifact_id())?;
            let lease = ResidentReadLease::new(
                artifact_ref,
                TierResidenceClass::Hot,
                PlacementBudgetClass::ForegroundResidentOnly,
                execution_origin,
            );
            backend.counters().record_hot_tier_resident_reads(1);
            ReadPlacementPlanningReport::new(
                Some(lease),
                None,
                None,
                RecallBreadthSummary::new(0, 0),
                None,
            )
        }
        PlacementArtifactFamily::StableBasis => {
            ensure_stable_basis_present(backend.state(), artifact_ref.artifact_id())?;
            let lease = ResidentReadLease::new(
                artifact_ref,
                TierResidenceClass::Warm,
                PlacementBudgetClass::ForegroundResidentOnly,
                execution_origin,
            );
            backend.counters().record_warm_tier_resident_reads(1);
            ReadPlacementPlanningReport::new(
                Some(lease),
                None,
                None,
                RecallBreadthSummary::new(0, 0),
                None,
            )
        }
        other => ReadPlacementPlanningReport::new(
            None,
            None,
            None,
            RecallBreadthSummary::new(0, 0),
            Some(TierMoveRejection::RawLocatorBoundaryViolation {
                locator: other.label().to_string(),
            }),
        ),
    };

    Ok(report)
}

pub(crate) fn plan_cold_recall_lease<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    artifact_ref: PlacementBoundArtifactRef,
    execution_origin: PlacementExecutionOrigin,
) -> Result<ReadPlacementPlanningReport, StoreError> {
    let family = family_from_read_ref(&artifact_ref)?;
    ensure_family_artifact_present(backend.state(), family, artifact_ref.artifact_id())?;
    let recall_cost_class = match execution_origin {
        PlacementExecutionOrigin::Foreground => RecallCostClass::Bounded,
        PlacementExecutionOrigin::Background | PlacementExecutionOrigin::RestartRecovery => {
            RecallCostClass::Deferred
        }
    };
    let recall_witness = RecallEligibilityWitness::new(
        artifact_key_for_family(family, artifact_ref.artifact_id()),
        recall_cost_class,
        RecallAmplificationBudget::SingleFamilyLocalUnit,
    );
    let lease = ColdRecallLease::new(
        artifact_ref,
        recall_cost_class,
        RecallAmplificationBudget::SingleFamilyLocalUnit,
        execution_origin,
    );
    Ok(ReadPlacementPlanningReport::new(
        None,
        Some(lease),
        Some(recall_witness),
        RecallBreadthSummary::new(1, 0),
        None,
    ))
}

pub(crate) fn plan_broadened_recall<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    family: ColdDerivedFamilyPolicy,
    scope_class: PlacementObservationScopeClass,
    scope_key: &str,
    widened_artifact_keys: Vec<String>,
    execution_origin: PlacementExecutionOrigin,
) -> Result<crate::BroadenedRecallPlan, StoreError> {
    let _ = RecallCoalescingKey::new(placement_artifact_family(family), scope_class, scope_key);
    backend.counters().record_broadened_recall_plans(1);
    Ok(crate::BroadenedRecallPlan::new(
        scope_class,
        scope_key.to_string(),
        widened_artifact_keys,
        execution_origin,
    ))
}

fn placement_artifact_family(family: ColdDerivedFamilyPolicy) -> PlacementArtifactFamily {
    match family {
        ColdDerivedFamilyPolicy::SnapshotFamily => PlacementArtifactFamily::SnapshotFamily,
        ColdDerivedFamilyPolicy::BranchDeltaFamily => PlacementArtifactFamily::BranchDeltaFamily,
        ColdDerivedFamilyPolicy::Milestone6LayoutFamily => {
            PlacementArtifactFamily::Milestone6LayoutFamily
        }
    }
}
