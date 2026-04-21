use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        records::{StoreState, TierRecallRecord, TierResidencyRecord, TierTransferRecord},
    },
    failure::{StoreError, StoreErrorKind},
    tiering::{PlacementArtifactFamily, RecallCoalescingKey, TierResidenceClass},
};

pub(crate) fn placement_family_for_artifact_key(
    artifact_key: &str,
) -> Result<PlacementArtifactFamily, StoreError> {
    if artifact_key.starts_with("authoritative_branch_head:") {
        Ok(PlacementArtifactFamily::AuthoritativeBranchHead)
    } else if artifact_key.starts_with("retained_authority:") {
        Ok(PlacementArtifactFamily::RetainedAuthority)
    } else if artifact_key.starts_with("snapshot:") {
        Ok(PlacementArtifactFamily::SnapshotFamily)
    } else if artifact_key.starts_with("branch_delta:") {
        Ok(PlacementArtifactFamily::BranchDeltaFamily)
    } else if artifact_key.starts_with("milestone6_layout:") {
        Ok(PlacementArtifactFamily::Milestone6LayoutFamily)
    } else if artifact_key.starts_with("stable_basis:") {
        Ok(PlacementArtifactFamily::StableBasis)
    } else {
        Err(StoreError::new(
            StoreErrorKind::PlacementWitnessConstructionViolation,
            format!("artifact key `{artifact_key}` is not admitted for tier execution"),
        ))
    }
}

pub(super) fn default_residence_for_family(family: PlacementArtifactFamily) -> TierResidenceClass {
    match family {
        PlacementArtifactFamily::AuthoritativeBranchHead => TierResidenceClass::Hot,
        PlacementArtifactFamily::RetainedAuthority | PlacementArtifactFamily::StableBasis => {
            TierResidenceClass::Hot
        }
        PlacementArtifactFamily::SnapshotFamily
        | PlacementArtifactFamily::BranchDeltaFamily
        | PlacementArtifactFamily::Milestone6LayoutFamily => TierResidenceClass::Warm,
    }
}

pub(super) fn default_locator(artifact_key: &str, residence: TierResidenceClass) -> String {
    let tier = match residence {
        TierResidenceClass::Hot => "hot",
        TierResidenceClass::Warm => "warm",
        TierResidenceClass::Cold => "cold",
    };
    format!("{tier}://{artifact_key}")
}

pub(super) fn artifact_key_for_family(
    family: crate::ColdDerivedFamilyPolicy,
    artifact_id: &str,
) -> String {
    match family {
        crate::ColdDerivedFamilyPolicy::SnapshotFamily => format!("snapshot:{artifact_id}"),
        crate::ColdDerivedFamilyPolicy::BranchDeltaFamily => format!("branch_delta:{artifact_id}"),
        crate::ColdDerivedFamilyPolicy::Milestone6LayoutFamily => {
            format!("milestone6_layout:{artifact_id}")
        }
    }
}

pub(crate) fn recall_coalescing_key_for_artifact(
    artifact_family: PlacementArtifactFamily,
    artifact_id: &str,
) -> RecallCoalescingKey {
    RecallCoalescingKey::new(
        artifact_family,
        crate::PlacementObservationScopeClass::ArtifactFamily,
        artifact_id,
    )
}

pub(crate) fn recall_record_key(key: &RecallCoalescingKey) -> String {
    format!(
        "{}|{}|{}",
        key.artifact_family().label(),
        key.scope_class().label(),
        key.scope_key()
    )
}

pub(crate) fn expected_verification_label(
    state: &StoreState,
    artifact_key: &str,
) -> Result<String, StoreError> {
    if let Some(branch_id) = artifact_key.strip_prefix("authoritative_branch_head:") {
        let branch_id = branch_id.split('@').next().unwrap_or(branch_id);
        let record = state.branch_head_records.get(branch_id).ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::TierTransferVerificationFailed,
                format!("branch head `{branch_id}` is not present for tier verification"),
            )
        })?;
        return record.head_commit_digest.clone().ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::TierTransferVerificationFailed,
                format!(
                    "branch head `{branch_id}` is missing a canonical digest for tier verification"
                ),
            )
        });
    }
    if let Some(snapshot_suffix) = artifact_key.strip_prefix("retained_authority:snapshot:") {
        let snapshot_id = snapshot_suffix.parse::<u64>().map_err(|_| {
            StoreError::new(
                StoreErrorKind::TierTransferVerificationFailed,
                format!("retained authority `{artifact_key}` does not name a valid snapshot id"),
            )
        })?;
        let record = state
            .snapshot_basis_records
            .get(&snapshot_id)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::TierTransferVerificationFailed,
                    format!("snapshot basis `{snapshot_id}` is missing for tier verification"),
                )
            })?;
        return Ok(record.snapshot_authority_digest.clone());
    }
    if let Some(snapshot_suffix) = artifact_key.strip_prefix("snapshot:") {
        let snapshot_id = snapshot_suffix.parse::<u64>().map_err(|_| {
            StoreError::new(
                StoreErrorKind::TierTransferVerificationFailed,
                format!("snapshot artifact `{artifact_key}` does not name a valid snapshot id"),
            )
        })?;
        let record = state
            .snapshot_basis_records
            .get(&snapshot_id)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::TierTransferVerificationFailed,
                    format!("snapshot basis `{snapshot_id}` is missing for tier verification"),
                )
            })?;
        return Ok(record.snapshot_image_digest.clone());
    }
    if let Some(layer_suffix) = artifact_key.strip_prefix("branch_delta:") {
        let layer_id = layer_suffix.parse::<u64>().map_err(|_| {
            StoreError::new(
                StoreErrorKind::TierTransferVerificationFailed,
                format!("branch delta artifact `{artifact_key}` does not name a valid layer id"),
            )
        })?;
        let record = state
            .branch_delta_layer_records
            .get(&layer_id)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::TierTransferVerificationFailed,
                    format!("branch delta layer `{layer_id}` is missing for tier verification"),
                )
            })?;
        return Ok(record.authority_basis_digest.clone());
    }
    if let Some(artifact_id) = artifact_key.strip_prefix("milestone6_layout:") {
        let record = state
            .milestone_6_layout_materialization_records
            .get(artifact_id)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::TierTransferVerificationFailed,
                    format!("layout artifact `{artifact_id}` is missing for tier verification"),
                )
            })?;
        return Ok(record.materialization.semantic_truth_digest().to_string());
    }
    if let Some(stable_basis_id) = artifact_key.strip_prefix("stable_basis:") {
        let record = state
            .stable_basis_records
            .get(stable_basis_id)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::TierTransferVerificationFailed,
                    format!("stable basis `{stable_basis_id}` is missing for tier verification"),
                )
            })?;
        return Ok(record.artifact_id.clone());
    }
    Err(StoreError::new(
        StoreErrorKind::TierTransferVerificationFailed,
        format!("artifact key `{artifact_key}` has no verification strategy"),
    ))
}

pub(super) fn current_residency_record(
    state: &StoreState,
    artifact_key: &str,
) -> Result<TierResidencyRecord, StoreError> {
    if let Some(record) = state.tier_residency_records.get(artifact_key) {
        return Ok(record.clone());
    }
    let artifact_family = placement_family_for_artifact_key(artifact_key)?;
    let canonical_residence = default_residence_for_family(artifact_family);
    let verification_label = expected_verification_label(state, artifact_key)?;
    Ok(TierResidencyRecord {
        artifact_key: artifact_key.to_string(),
        artifact_family,
        canonical_residence,
        canonical_replica_locator: default_locator(artifact_key, canonical_residence),
        verification_label,
    })
}

pub(super) fn transfer_record<'a>(
    state: &'a StoreState,
    artifact_key: &str,
) -> Result<&'a TierTransferRecord, StoreError> {
    state
        .tier_transfer_records
        .get(artifact_key)
        .ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::TierResidencyManifestViolation,
                format!("artifact `{artifact_key}` has no persisted in-flight tier transfer"),
            )
        })
}

pub(super) fn transfer_record_mut<'a>(
    state: &'a mut StoreState,
    artifact_key: &str,
) -> Result<&'a mut TierTransferRecord, StoreError> {
    state
        .tier_transfer_records
        .get_mut(artifact_key)
        .ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::TierResidencyManifestViolation,
                format!("artifact `{artifact_key}` has no persisted in-flight tier transfer"),
            )
        })
}

pub(crate) fn manifest_from_state(state: &StoreState) -> crate::CanonicalResidencyManifest {
    crate::CanonicalResidencyManifest::new(
        state.tier_residency_records.keys().cloned().collect(),
        state.tier_transfer_records.keys().cloned().collect(),
    )
}

pub(crate) fn recall_record<'a>(
    state: &'a StoreState,
    coalescing_key: &str,
) -> Result<&'a TierRecallRecord, StoreError> {
    state
        .tier_recall_records
        .get(coalescing_key)
        .ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::TierRecallExecutionViolation,
                format!("coalesced recall `{coalescing_key}` has no persisted recall record"),
            )
        })
}

pub(super) fn record_background_move_counters<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    artifact_family: PlacementArtifactFamily,
    execution_origin: crate::PlacementExecutionOrigin,
) {
    if execution_origin == crate::PlacementExecutionOrigin::Background {
        backend.counters().record_background_tier_moves(1);
    }
    match artifact_family {
        PlacementArtifactFamily::AuthoritativeBranchHead
        | PlacementArtifactFamily::RetainedAuthority
        | PlacementArtifactFamily::StableBasis => {
            backend.counters().record_authoritative_tier_moves(1);
        }
        PlacementArtifactFamily::SnapshotFamily
        | PlacementArtifactFamily::BranchDeltaFamily
        | PlacementArtifactFamily::Milestone6LayoutFamily => {
            backend.counters().record_derived_tier_moves(1);
        }
    }
}
