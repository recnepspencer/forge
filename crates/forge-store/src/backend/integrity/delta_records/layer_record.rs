use crate::{
    backend::records::{BranchDeltaLayerRecord, StoreState},
    delta::{BRANCH_DELTA_FAMILY_VERSION, stable_branch_delta_layer_authority_digest},
    failure::{StoreError, StoreErrorKind},
};

impl StoreState {
    pub(super) fn verify_branch_delta_layer_record(
        &self,
        record: &BranchDeltaLayerRecord,
    ) -> Result<(), StoreError> {
        verify_layer_shape(self, record)?;
        verify_commit_interval(self, record)?;
        self.verify_branch_delta_layer_artifacts(record)?;
        self.verify_replacement_lineage(record)?;
        verify_authority_basis_digest(self, record)?;
        Ok(())
    }
}

fn verify_layer_shape(state: &StoreState, record: &BranchDeltaLayerRecord) -> Result<(), StoreError> {
    if record.delta_family_version != BRANCH_DELTA_FAMILY_VERSION {
        return Err(StoreError::new(
            StoreErrorKind::BranchDeltaFamilyVersionUnsupported,
            format!(
                "branch delta layer {} used unsupported family version {}",
                record.branch_delta_layer_id.0, record.delta_family_version
            ),
        ));
    }
    if !state.branch_records.contains_key(&record.branch_id.0) {
        return Err(StoreError::unknown_branch(&record.branch_id));
    }
    if record.commit_ids.is_empty() {
        return Err(StoreError::new(
            StoreErrorKind::BranchDeltaPublicationGap,
            format!(
                "branch delta layer {} published an empty commit segment",
                record.branch_delta_layer_id.0
            ),
        ));
    }
    let target_record = state
        .commit_record(record.target_frontier_commit_id)
        .ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::BranchDeltaPublicationGap,
                format!(
                    "branch delta layer {} target commit {} missing",
                    record.branch_delta_layer_id.0, record.target_frontier_commit_id.0
                ),
            )
        })?;
    if target_record.envelope.branch_context != record.branch_id {
        return Err(StoreError::new(
            StoreErrorKind::BranchDeltaPublicationGap,
            format!(
                "branch delta layer {} target commit {} drifted onto branch `{}`",
                record.branch_delta_layer_id.0,
                record.target_frontier_commit_id.0,
                target_record.envelope.branch_context.0
            ),
        ));
    }
    if record.commit_ids.last().copied() != Some(record.target_frontier_commit_id) {
        return Err(StoreError::new(
            StoreErrorKind::BranchDeltaPublicationGap,
            format!(
                "branch delta layer {} does not end at its declared target frontier",
                record.branch_delta_layer_id.0
            ),
        ));
    }
    Ok(())
}

fn verify_commit_interval(
    state: &StoreState,
    record: &BranchDeltaLayerRecord,
) -> Result<(), StoreError> {
    for commit_id in &record.commit_ids {
        let commit_record = state.commit_record(*commit_id).ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::BranchDeltaPublicationGap,
                format!(
                    "branch delta layer {} references missing commit {}",
                    record.branch_delta_layer_id.0, commit_id.0
                ),
            )
        })?;
        if commit_record.envelope.branch_context != record.branch_id {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaPublicationGap,
                format!(
                    "branch delta layer {} commit {} drifted onto branch `{}`",
                    record.branch_delta_layer_id.0,
                    commit_id.0,
                    commit_record.envelope.branch_context.0
                ),
            ));
        }
    }
    let mut expected_parent = record.base_frontier_commit_id;
    for commit_id in &record.commit_ids {
        let commit_record = state.commit_record(*commit_id).ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::BranchDeltaPublicationGap,
                format!(
                    "branch delta layer {} references missing commit {}",
                    record.branch_delta_layer_id.0, commit_id.0
                ),
            )
        })?;
        match commit_record.envelope.commit.parents.as_slice() {
            [] if expected_parent.is_none() => {}
            [parent] if Some(*parent) == expected_parent => {}
            [parent] => {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "branch delta layer {} commit {} expected parent {:?} but found {}",
                        record.branch_delta_layer_id.0,
                        commit_id.0,
                        expected_parent.map(|id| id.0),
                        parent.0
                    ),
                ));
            }
            _ => {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "branch delta layer {} commit {} requires merge-aware widening, which persisted delta segments do not admit",
                        record.branch_delta_layer_id.0, commit_id.0
                    ),
                ));
            }
        }
        expected_parent = Some(*commit_id);
    }
    Ok(())
}

fn verify_authority_basis_digest(
    state: &StoreState,
    record: &BranchDeltaLayerRecord,
) -> Result<(), StoreError> {
    let expected_digest = stable_branch_delta_layer_authority_digest(
        &record.branch_id,
        record.base_frontier_commit_id,
        record.target_frontier_commit_id,
        &record.commit_ids,
        state.canonicalization_version,
    );
    if record.authority_basis_digest != expected_digest {
        return Err(StoreError::new(
            StoreErrorKind::BranchDeltaDigestMismatch,
            format!(
                "branch delta layer {} digest drifted from authoritative basis",
                record.branch_delta_layer_id.0
            ),
        ));
    }
    Ok(())
}
