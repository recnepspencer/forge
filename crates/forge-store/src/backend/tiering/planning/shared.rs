use forge_relational::facade::history::BranchId;

use crate::{
    backend::records::StoreState,
    failure::{StoreError, StoreErrorKind},
    tiering::{
        ColdDerivedFamilyPolicy, PlacementArtifactFamily, PlacementBoundArtifactRef,
        PlacementBudgetClass, PlacementExecutionOrigin,
    },
};

pub(super) fn move_budget_for_origin(origin: PlacementExecutionOrigin) -> PlacementBudgetClass {
    match origin {
        PlacementExecutionOrigin::Foreground => PlacementBudgetClass::ForegroundResidentOnly,
        PlacementExecutionOrigin::Background | PlacementExecutionOrigin::RestartRecovery => {
            PlacementBudgetClass::BackgroundOnly
        }
    }
}

pub(super) fn branch_id_for_basis(
    state: &StoreState,
    basis_key: &str,
) -> Result<BranchId, StoreError> {
    if let Some(snapshot_suffix) = basis_key.strip_prefix("snapshot:") {
        if let Ok(snapshot_id) = snapshot_suffix.parse::<u64>() {
            if let Some(record) = state.snapshot_basis_records.get(&snapshot_id) {
                return Ok(record.snapshot_branch_id.clone());
            }
        }
    }
    if let Some(record) = state
        .retention_basis_records
        .values()
        .find(|record| record.basis_label == basis_key)
    {
        return record.branch_id.clone().ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::PlacementWitnessConstructionViolation,
                format!("retained basis `{basis_key}` is missing branch-local authority context"),
            )
        });
    }
    if let Some(record) = state
        .stable_basis_records
        .values()
        .find(|record| record.requested_stable_basis_id().as_str() == basis_key)
    {
        return Ok(record.request.branch_id().clone());
    }
    Err(StoreError::new(
        StoreErrorKind::PlacementWitnessConstructionViolation,
        format!(
            "retained basis `{basis_key}` is not available for authoritative placement planning"
        ),
    ))
}

pub(super) fn ensure_family_artifact_present(
    state: &StoreState,
    family: ColdDerivedFamilyPolicy,
    artifact_id: &str,
) -> Result<(), StoreError> {
    let present = match family {
        ColdDerivedFamilyPolicy::SnapshotFamily => artifact_id
            .parse::<u64>()
            .ok()
            .map(|snapshot_id| state.snapshot_basis_records.contains_key(&snapshot_id))
            .unwrap_or(false),
        ColdDerivedFamilyPolicy::BranchDeltaFamily => artifact_id
            .parse::<u64>()
            .ok()
            .map(|layer_id| state.branch_delta_layer_records.contains_key(&layer_id))
            .unwrap_or(false),
        ColdDerivedFamilyPolicy::Milestone6LayoutFamily => state
            .milestone_6_layout_materialization_records
            .contains_key(artifact_id),
    };
    if present {
        Ok(())
    } else {
        Err(StoreError::new(
            StoreErrorKind::PlacementWitnessConstructionViolation,
            format!(
                "derived artifact `{artifact_id}` is not present in admitted family `{}`",
                family.label()
            ),
        ))
    }
}

pub(super) fn ensure_branch_head_present(
    state: &StoreState,
    branch_id: &str,
) -> Result<(), StoreError> {
    let Some(record) = state.branch_head_records.get(branch_id) else {
        return Err(StoreError::unknown_branch(&BranchId(branch_id.to_string())));
    };
    if record.head_commit_id.is_some() {
        Ok(())
    } else {
        Err(StoreError::new(
            StoreErrorKind::PlacementWitnessConstructionViolation,
            format!("branch `{branch_id}` has no resident authoritative head"),
        ))
    }
}

pub(super) fn ensure_stable_basis_present(
    state: &StoreState,
    artifact_id: &str,
) -> Result<(), StoreError> {
    if state.stable_basis_records.contains_key(artifact_id) {
        Ok(())
    } else {
        Err(StoreError::new(
            StoreErrorKind::PlacementWitnessConstructionViolation,
            format!("stable basis `{artifact_id}` is not present in durable state"),
        ))
    }
}

pub(super) fn artifact_key_for_family(
    family: ColdDerivedFamilyPolicy,
    artifact_id: &str,
) -> String {
    match family {
        ColdDerivedFamilyPolicy::SnapshotFamily => format!("snapshot:{artifact_id}"),
        ColdDerivedFamilyPolicy::BranchDeltaFamily => format!("branch_delta:{artifact_id}"),
        ColdDerivedFamilyPolicy::Milestone6LayoutFamily => {
            format!("milestone6_layout:{artifact_id}")
        }
    }
}

pub(super) fn family_from_read_ref(
    artifact_ref: &PlacementBoundArtifactRef,
) -> Result<ColdDerivedFamilyPolicy, StoreError> {
    match artifact_ref.artifact_family() {
        PlacementArtifactFamily::SnapshotFamily => Ok(ColdDerivedFamilyPolicy::SnapshotFamily),
        PlacementArtifactFamily::BranchDeltaFamily => {
            Ok(ColdDerivedFamilyPolicy::BranchDeltaFamily)
        }
        PlacementArtifactFamily::Milestone6LayoutFamily => {
            Ok(ColdDerivedFamilyPolicy::Milestone6LayoutFamily)
        }
        other => Err(StoreError::new(
            StoreErrorKind::PlacementRawLocatorBoundaryViolation,
            format!(
                "artifact family `{}` is not admitted for cold recall lease planning",
                other.label()
            ),
        )),
    }
}
