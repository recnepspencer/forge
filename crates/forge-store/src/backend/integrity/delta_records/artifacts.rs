use std::collections::BTreeSet;

use crate::{
    backend::{
        integrity::parent_artifact_id,
        records::{BranchDeltaLayerArtifacts, BranchDeltaLayerRecord, CommitParentRecord, StoreState},
    },
    failure::{StoreError, StoreErrorKind},
};

impl StoreState {
    pub(super) fn verify_branch_delta_layer_artifacts(
        &self,
        record: &BranchDeltaLayerRecord,
    ) -> Result<(), StoreError> {
        let artifacts = &record.artifacts;
        verify_commit_envelopes(self, record, artifacts)?;
        verify_parent_records(self, record, artifacts)?;
        verify_support_summaries(self, record, artifacts)?;
        verify_schema_support(self, record, artifacts)?;
        verify_lineage_support(self, record, artifacts)?;
        Ok(())
    }
}

fn verify_commit_envelopes(
    state: &StoreState,
    record: &BranchDeltaLayerRecord,
    artifacts: &BranchDeltaLayerArtifacts,
) -> Result<(), StoreError> {
    let artifact_commit_ids = artifacts
        .commit_envelopes
        .iter()
        .map(|entry| entry.envelope.commit.commit_id)
        .collect::<Vec<_>>();
    if artifact_commit_ids != record.commit_ids {
        return Err(StoreError::new(
            StoreErrorKind::BranchDeltaPublicationGap,
            format!(
                "branch delta layer {} artifact commit envelopes drifted from the declared segment",
                record.branch_delta_layer_id.0
            ),
        ));
    }
    for commit_record in &artifacts.commit_envelopes {
        if commit_record.envelope.branch_context != record.branch_id {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaPublicationGap,
                format!(
                    "branch delta layer {} artifact commit {} drifted onto branch `{}`",
                    record.branch_delta_layer_id.0,
                    commit_record.envelope.commit.commit_id.0,
                    commit_record.envelope.branch_context.0
                ),
            ));
        }
        let authoritative = state
            .commit_record(commit_record.envelope.commit.commit_id)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "branch delta layer {} artifact commit {} is missing from authority",
                        record.branch_delta_layer_id.0,
                        commit_record.envelope.commit.commit_id.0
                    ),
                )
            })?;
        if authoritative != commit_record {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaDigestMismatch,
                format!(
                    "branch delta layer {} artifact commit {} drifted from authoritative commit storage",
                    record.branch_delta_layer_id.0,
                    commit_record.envelope.commit.commit_id.0
                ),
            ));
        }
    }
    Ok(())
}

fn verify_parent_records(
    state: &StoreState,
    record: &BranchDeltaLayerRecord,
    artifacts: &BranchDeltaLayerArtifacts,
) -> Result<(), StoreError> {
    let expected_parent_records = artifacts
        .commit_envelopes
        .iter()
        .flat_map(|commit_record| {
            commit_record
                .envelope
                .commit
                .parents
                .iter()
                .copied()
                .enumerate()
                .map(|(parent_position, parent_commit_id)| CommitParentRecord {
                    commit_id: commit_record.envelope.commit.commit_id,
                    parent_position,
                    parent_commit_id,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut expected_parent_artifacts = BranchDeltaLayerArtifacts {
        commit_envelopes: Vec::new(),
        commit_parent_records: expected_parent_records,
        commit_support_summaries: Vec::new(),
        schema_support_records: Vec::new(),
        lineage_support_records: Vec::new(),
    };
    expected_parent_artifacts.canonicalize_order();
    if artifacts.commit_parent_records != expected_parent_artifacts.commit_parent_records {
        return Err(StoreError::new(
            StoreErrorKind::BranchDeltaPublicationGap,
            format!(
                "branch delta layer {} artifact parent records drifted from the admitted commit ancestry",
                record.branch_delta_layer_id.0
            ),
        ));
    }
    for parent_record in &artifacts.commit_parent_records {
        let key = parent_artifact_id(parent_record.commit_id, parent_record.parent_position);
        let authoritative = state.commit_parent_records.get(&key).ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::BranchDeltaPublicationGap,
                format!(
                    "branch delta layer {} artifact parent {}:{} is missing from authority",
                    record.branch_delta_layer_id.0,
                    parent_record.commit_id.0,
                    parent_record.parent_position
                ),
            )
        })?;
        if authoritative != parent_record {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaDigestMismatch,
                format!(
                    "branch delta layer {} artifact parent {}:{} drifted from authoritative parent storage",
                    record.branch_delta_layer_id.0,
                    parent_record.commit_id.0,
                    parent_record.parent_position
                ),
            ));
        }
    }
    Ok(())
}

fn verify_support_summaries(
    state: &StoreState,
    record: &BranchDeltaLayerRecord,
    artifacts: &BranchDeltaLayerArtifacts,
) -> Result<(), StoreError> {
    let mut seen_summary_commits = BTreeSet::new();
    for summary in &artifacts.commit_support_summaries {
        if summary.branch_id != record.branch_id {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaPublicationGap,
                format!(
                    "branch delta layer {} support summary for commit {} drifted onto branch `{}`",
                    record.branch_delta_layer_id.0, summary.commit_id.0, summary.branch_id.0
                ),
            ));
        }
        if !record.commit_ids.contains(&summary.commit_id)
            || !seen_summary_commits.insert(summary.commit_id)
        {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaPublicationGap,
                format!(
                    "branch delta layer {} support summaries are not a one-per-commit subset of the declared segment",
                    record.branch_delta_layer_id.0
                ),
            ));
        }
        if let Some(authoritative) = state.commit_support_summaries.get(&summary.commit_id.0) {
            if authoritative != summary {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaDigestMismatch,
                    format!(
                        "branch delta layer {} support summary for commit {} drifted from authoritative support storage",
                        record.branch_delta_layer_id.0, summary.commit_id.0
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn verify_schema_support(
    state: &StoreState,
    record: &BranchDeltaLayerRecord,
    artifacts: &BranchDeltaLayerArtifacts,
) -> Result<(), StoreError> {
    let mut seen_schema_commits = BTreeSet::new();
    for schema_record in &artifacts.schema_support_records {
        if schema_record.branch_id != record.branch_id {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaPublicationGap,
                format!(
                    "branch delta layer {} schema support for commit {} drifted onto branch `{}`",
                    record.branch_delta_layer_id.0,
                    schema_record.commit_id.0,
                    schema_record.branch_id.0
                ),
            ));
        }
        if !record.commit_ids.contains(&schema_record.commit_id)
            || !seen_schema_commits.insert(schema_record.commit_id)
        {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaPublicationGap,
                format!(
                    "branch delta layer {} schema support rows are not a one-per-commit subset of the declared segment",
                    record.branch_delta_layer_id.0
                ),
            ));
        }
        if let Some(authoritative) = state.schema_support_records.get(&schema_record.artifact_id) {
            if authoritative != schema_record {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaDigestMismatch,
                    format!(
                        "branch delta layer {} schema support for commit {} drifted from authoritative support storage",
                        record.branch_delta_layer_id.0, schema_record.commit_id.0
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn verify_lineage_support(
    state: &StoreState,
    record: &BranchDeltaLayerRecord,
    artifacts: &BranchDeltaLayerArtifacts,
) -> Result<(), StoreError> {
    let mut seen_lineage_commits = BTreeSet::new();
    for lineage_record in &artifacts.lineage_support_records {
        if lineage_record.branch_id != record.branch_id {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaPublicationGap,
                format!(
                    "branch delta layer {} lineage support for commit {} drifted onto branch `{}`",
                    record.branch_delta_layer_id.0,
                    lineage_record.commit_id.0,
                    lineage_record.branch_id.0
                ),
            ));
        }
        if !record.commit_ids.contains(&lineage_record.commit_id)
            || !seen_lineage_commits.insert(lineage_record.commit_id)
        {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaPublicationGap,
                format!(
                    "branch delta layer {} lineage support rows are not a one-per-commit subset of the declared segment",
                    record.branch_delta_layer_id.0
                ),
            ));
        }
        if let Some(authoritative) = state.lineage_support_records.get(&lineage_record.artifact_id)
        {
            if authoritative != lineage_record {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaDigestMismatch,
                    format!(
                        "branch delta layer {} lineage support for commit {} drifted from authoritative support storage",
                        record.branch_delta_layer_id.0, lineage_record.commit_id.0
                    ),
                ));
            }
        }
    }
    Ok(())
}
