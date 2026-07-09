use crate::failure::{StoreError, StoreErrorKind};
use worth_relational::facade::history::{BranchId, CommitId};

use crate::backend::{
    integrity::{lineage_support_artifact_id, schema_support_artifact_id},
    records::{BranchDeltaLayerArtifacts, BranchDeltaLayerRecord, CommitParentRecord, StoreState},
};

impl StoreState {
    pub(crate) fn build_branch_delta_layer_artifacts(
        &self,
        branch_id: &BranchId,
        commit_ids: &[CommitId],
    ) -> Result<BranchDeltaLayerArtifacts, StoreError> {
        let mut artifacts = empty_branch_delta_layer_artifacts();
        for commit_id in commit_ids {
            let commit_record = self.commit_record(*commit_id).cloned().ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "branch delta layer artifact build missing commit {} for branch `{}`",
                        commit_id.0, branch_id.0
                    ),
                )
            })?;
            artifacts.commit_envelopes.push(commit_record.clone());
            artifacts.commit_parent_records.extend(
                commit_record
                    .envelope
                    .commit
                    .parents
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(parent_position, parent_commit_id)| CommitParentRecord {
                        commit_id: *commit_id,
                        parent_position,
                        parent_commit_id,
                    }),
            );
            if let Some(summary) = self.commit_support_summaries.get(&commit_id.0).cloned() {
                artifacts.commit_support_summaries.push(summary);
            }
            if let Some(schema_record) = self
                .schema_support_records
                .get(&schema_support_artifact_id(*commit_id))
                .cloned()
            {
                artifacts.schema_support_records.push(schema_record);
            }
            if let Some(lineage_record) = self
                .lineage_support_records
                .get(&lineage_support_artifact_id(*commit_id))
                .cloned()
            {
                artifacts.lineage_support_records.push(lineage_record);
            }
        }
        artifacts.canonicalize_order();
        Ok(artifacts)
    }

    pub(crate) fn backfill_missing_branch_delta_layer_artifacts(
        &mut self,
    ) -> Result<(), StoreError> {
        let layer_ids = self
            .branch_delta_layer_records
            .iter()
            .filter_map(|(layer_id, record)| record.artifacts.is_empty().then_some(*layer_id))
            .collect::<Vec<_>>();
        for layer_id in layer_ids {
            let (branch_id, commit_ids) = {
                let record = self
                    .branch_delta_layer_records
                    .get(&layer_id)
                    .expect("selected branch delta layer should exist");
                (record.branch_id.clone(), record.commit_ids.clone())
            };
            let artifacts = self.build_branch_delta_layer_artifacts(&branch_id, &commit_ids)?;
            if let Some(record) = self.branch_delta_layer_records.get_mut(&layer_id) {
                record.artifacts = artifacts;
            }
        }
        Ok(())
    }
}

pub(super) fn combine_branch_delta_layer_artifacts(
    removed_layers: &[BranchDeltaLayerRecord],
) -> BranchDeltaLayerArtifacts {
    let mut artifacts = empty_branch_delta_layer_artifacts();
    for layer in removed_layers {
        artifacts
            .commit_envelopes
            .extend(layer.artifacts.commit_envelopes.iter().cloned());
        artifacts
            .commit_parent_records
            .extend(layer.artifacts.commit_parent_records.iter().cloned());
        artifacts
            .commit_support_summaries
            .extend(layer.artifacts.commit_support_summaries.iter().cloned());
        artifacts
            .schema_support_records
            .extend(layer.artifacts.schema_support_records.iter().cloned());
        artifacts
            .lineage_support_records
            .extend(layer.artifacts.lineage_support_records.iter().cloned());
    }
    artifacts.canonicalize_order();
    artifacts
}

pub(super) fn empty_branch_delta_layer_artifacts() -> BranchDeltaLayerArtifacts {
    BranchDeltaLayerArtifacts {
        commit_envelopes: Vec::new(),
        commit_parent_records: Vec::new(),
        commit_support_summaries: Vec::new(),
        schema_support_records: Vec::new(),
        lineage_support_records: Vec::new(),
    }
}

pub(super) fn branch_delta_layer_artifacts_empty(artifacts: &BranchDeltaLayerArtifacts) -> bool {
    artifacts.commit_envelopes.is_empty()
        && artifacts.commit_parent_records.is_empty()
        && artifacts.commit_support_summaries.is_empty()
        && artifacts.schema_support_records.is_empty()
        && artifacts.lineage_support_records.is_empty()
}
