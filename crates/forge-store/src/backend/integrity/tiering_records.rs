use crate::{
    backend::records::StoreState,
    failure::{StoreError, StoreErrorKind},
};

impl StoreState {
    pub fn verify_tiering_record_family(&self) -> Result<(), StoreError> {
        for (artifact_key, record) in &self.tier_residency_records {
            if record.artifact_key.is_empty()
                || record.canonical_replica_locator.is_empty()
                || record.verification_label.is_empty()
            {
                return Err(StoreError::new(
                    StoreErrorKind::PlacementWitnessConstructionViolation,
                    "tier residency records must declare non-empty artifact identity, locator, and verification label",
                ));
            }
            if artifact_key != &record.artifact_key {
                return Err(StoreError::backend_integrity(format!(
                    "tier residency map key `{artifact_key}` drifted from record artifact key `{}`",
                    record.artifact_key
                )));
            }
            let expected_family =
                crate::backend::tiering::placement_family_for_artifact_key(&record.artifact_key)?;
            if expected_family != record.artifact_family {
                return Err(StoreError::backend_integrity(format!(
                    "tier residency `{}` declared family `{}` but artifact key implies `{}`",
                    record.artifact_key,
                    record.artifact_family.label(),
                    expected_family.label()
                )));
            }
            let expected_verification =
                crate::backend::tiering::expected_verification_label(self, &record.artifact_key)?;
            if expected_verification != record.verification_label {
                return Err(StoreError::backend_integrity(format!(
                    "tier residency `{}` verification label drifted from reconstructed truth",
                    record.artifact_key
                )));
            }
        }

        for (artifact_key, record) in &self.tier_transfer_records {
            if record.artifact_key.is_empty() || record.source_replica_locator.is_empty() {
                return Err(StoreError::new(
                    StoreErrorKind::PlacementWitnessConstructionViolation,
                    "tier transfer records must declare non-empty artifact identity and source locator",
                ));
            }
            if record.cutover_completed && record.transferred_replica_locator.is_none() {
                return Err(StoreError::new(
                    StoreErrorKind::PlacementWitnessConstructionViolation,
                    format!(
                        "tier transfer `{}` cannot complete cutover without a transferred replica locator",
                        record.artifact_key
                    ),
                ));
            }
            if record.cutover_completed && record.verification_label.is_none() {
                return Err(StoreError::new(
                    StoreErrorKind::PlacementWitnessConstructionViolation,
                    format!(
                        "tier transfer `{}` cannot complete cutover without a verification label",
                        record.artifact_key
                    ),
                ));
            }
            if artifact_key != &record.artifact_key {
                return Err(StoreError::backend_integrity(format!(
                    "tier transfer map key `{artifact_key}` drifted from record artifact key `{}`",
                    record.artifact_key
                )));
            }
            let expected_family =
                crate::backend::tiering::placement_family_for_artifact_key(&record.artifact_key)?;
            if expected_family != record.artifact_family {
                return Err(StoreError::backend_integrity(format!(
                    "tier transfer `{}` declared family `{}` but artifact key implies `{}`",
                    record.artifact_key,
                    record.artifact_family.label(),
                    expected_family.label()
                )));
            }
            if record.cutover_completed {
                let residency = self
                    .tier_residency_records
                    .get(&record.artifact_key)
                    .ok_or_else(|| {
                        StoreError::backend_integrity(format!(
                            "completed tier transfer `{}` is missing a canonical residency row",
                            record.artifact_key
                        ))
                    })?;
                if residency.artifact_family != record.artifact_family
                    || residency.canonical_residence != record.target_residence
                    || residency.verification_label
                        != record.verification_label.clone().unwrap_or_default()
                {
                    return Err(StoreError::backend_integrity(format!(
                        "completed tier transfer `{}` is inconsistent with canonical residency truth",
                        record.artifact_key
                    )));
                }
            }
        }

        for (coalescing_key, record) in &self.tier_recall_records {
            if coalescing_key != &record.coalescing_key {
                return Err(StoreError::backend_integrity(format!(
                    "tier recall map key `{coalescing_key}` drifted from recall record key `{}`",
                    record.coalescing_key
                )));
            }
            if record.artifact_key.is_empty() || record.scope_key.is_empty() {
                return Err(StoreError::new(
                    StoreErrorKind::PlacementWitnessConstructionViolation,
                    "tier recall records must declare non-empty artifact identity and scope key",
                ));
            }
            let expected_family =
                crate::backend::tiering::placement_family_for_artifact_key(&record.artifact_key)?;
            if expected_family != record.artifact_family {
                return Err(StoreError::backend_integrity(format!(
                    "tier recall `{}` declared family `{}` but artifact key implies `{}`",
                    record.coalescing_key,
                    record.artifact_family.label(),
                    expected_family.label()
                )));
            }
            let expected_key = crate::backend::tiering::recall_record_key(
                &crate::backend::tiering::recall_coalescing_key_for_artifact(
                    record.artifact_family,
                    &record.scope_key,
                ),
            );
            if expected_key != record.coalescing_key {
                return Err(StoreError::backend_integrity(format!(
                    "tier recall `{}` does not match its family-local coalescing identity",
                    record.coalescing_key
                )));
            }
            if record.completion_state
                == crate::backend::records::TierRecallCompletionState::Completed
            {
                return Err(StoreError::backend_integrity(format!(
                    "completed tier recall `{}` must not persist beyond its coalescing window",
                    record.coalescing_key
                )));
            }
        }

        Ok(())
    }
}
