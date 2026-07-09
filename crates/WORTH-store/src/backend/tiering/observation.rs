use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        records::StoreState,
    },
    failure::{StoreError, StoreErrorKind},
    ColdDerivedFamilyPolicy, PlacementObservationScopeClass, WorkingSetObservationWindow,
};

pub(crate) fn observe_working_set<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    scope_class: PlacementObservationScopeClass,
    scope_key: &str,
) -> Result<WorkingSetObservationWindow, StoreError> {
    let observed_artifact_keys = observed_artifact_keys(backend.state(), scope_class, scope_key)?;
    backend.counters().record_working_set_observation_windows(1);
    Ok(WorkingSetObservationWindow::new(
        scope_class,
        scope_key.to_string(),
        observed_artifact_keys,
    ))
}

pub(crate) fn observed_artifact_keys(
    state: &StoreState,
    scope_class: PlacementObservationScopeClass,
    scope_key: &str,
) -> Result<Vec<String>, StoreError> {
    match scope_class {
        PlacementObservationScopeClass::Branch => branch_observed_artifacts(state, scope_key),
        PlacementObservationScopeClass::RetainedBasis => {
            retained_basis_observed_artifacts(state, scope_key)
        }
        PlacementObservationScopeClass::ArtifactFamily => {
            artifact_family_observed_artifacts(state, scope_key)
        }
    }
}

fn branch_observed_artifacts(
    state: &StoreState,
    branch_key: &str,
) -> Result<Vec<String>, StoreError> {
    let branch_record = state.branch_head_records.get(branch_key).ok_or_else(|| {
        StoreError::unknown_branch(&worth_relational::facade::history::BranchId(
            branch_key.to_string(),
        ))
    })?;
    let Some(head_commit_id) = branch_record.head_commit_id else {
        return Err(StoreError::new(
            StoreErrorKind::PlacementWitnessConstructionViolation,
            format!("branch `{branch_key}` has no authoritative head to observe"),
        ));
    };
    let mut observed = vec![format!(
        "authoritative_branch_head:{branch_key}@{}",
        head_commit_id.0
    )];
    observed.extend(
        state
            .stable_basis_records
            .values()
            .filter(|record| record.request.branch_id().0 == branch_key)
            .map(|record| {
                format!(
                    "stable_basis:{}",
                    record.requested_stable_basis_id().as_str()
                )
            }),
    );
    Ok(observed)
}

fn retained_basis_observed_artifacts(
    state: &StoreState,
    basis_key: &str,
) -> Result<Vec<String>, StoreError> {
    let mut observed = state
        .retention_basis_records
        .values()
        .filter(|record| record.basis_label == basis_key)
        .map(|record| format!("retention_basis:{}", record.artifact_id))
        .collect::<Vec<_>>();
    observed.extend(
        state
            .stable_basis_records
            .values()
            .filter(|record| record.requested_stable_basis_id().as_str() == basis_key)
            .map(|record| format!("stable_basis:{}", record.artifact_id)),
    );
    if let Some(snapshot_suffix) = basis_key.strip_prefix("snapshot:") {
        if let Ok(snapshot_id) = snapshot_suffix.parse::<u64>() {
            if state.snapshot_basis_records.contains_key(&snapshot_id) {
                observed.push(format!("snapshot_basis:{snapshot_id}"));
            }
        }
    }
    if observed.is_empty() {
        return Err(StoreError::new(
            StoreErrorKind::PlacementWitnessConstructionViolation,
            format!("retained basis `{basis_key}` is not present in durable state"),
        ));
    }
    Ok(observed)
}

fn artifact_family_observed_artifacts(
    state: &StoreState,
    family_key: &str,
) -> Result<Vec<String>, StoreError> {
    let observed = match family_key {
        key if key == ColdDerivedFamilyPolicy::SnapshotFamily.label() => state
            .snapshot_basis_records
            .values()
            .map(|record| format!("snapshot:{}", record.snapshot_id.0))
            .collect(),
        key if key == ColdDerivedFamilyPolicy::BranchDeltaFamily.label() => state
            .branch_delta_layer_records
            .values()
            .map(|record| format!("branch_delta:{}", record.branch_delta_layer_id.0))
            .collect(),
        key if key == ColdDerivedFamilyPolicy::Milestone6LayoutFamily.label() => state
            .milestone_6_layout_materialization_records
            .values()
            .map(|record| format!("milestone6_layout:{}", record.artifact_id))
            .collect(),
        _ => {
            return Err(StoreError::new(
                StoreErrorKind::PlacementWitnessConstructionViolation,
                format!(
                    "artifact family `{family_key}` is not admitted for milestone 13 observation"
                ),
            ));
        }
    };
    Ok(observed)
}
