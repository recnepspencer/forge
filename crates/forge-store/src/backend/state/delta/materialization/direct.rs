use std::collections::BTreeSet;

use crate::{authority::AuthoritativeExportBundle, delta::BranchDeltaReadPlan, failure::{StoreError, StoreErrorKind}};
use forge_relational::facade::history::BranchId;

use crate::backend::{integrity::branch_key, records::{BranchHeadRecord, StoreState}};

use super::digests::empty_authoritative_export;
use crate::backend::state::delta::artifacts::branch_delta_layer_artifacts_empty;

impl StoreState {
    pub(crate) fn materialize_branch_delta_export(
        &self,
        plan: &BranchDeltaReadPlan,
    ) -> Result<AuthoritativeExportBundle, StoreError> {
        let basis = self
            .branch_shared_base_records
            .get(&branch_key(&plan.locality.branch_id))
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaBasisUnsupported,
                    format!(
                        "branch `{}` does not publish a shared-base branch delta basis yet",
                        plan.locality.branch_id.0
                    ),
                )
            })?;
        let mut export = if let Some(source_frontier_commit_id) = basis.source_frontier_commit_id {
            self.build_snapshot_image(&basis.source_branch_id, source_frontier_commit_id)?
                .authoritative_export()
                .clone()
        } else {
            empty_authoritative_export(self.canonicalization_version)
        };

        let branch_record = self
            .branch_records
            .get(&branch_key(&plan.locality.branch_id))
            .cloned()
            .ok_or_else(|| StoreError::unknown_branch(&plan.locality.branch_id))?;
        export.branch_records = vec![branch_record];

        for layer_id in &plan.used_layer_ids {
            let layer = self.branch_delta_layer_records.get(&layer_id.0).ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "branch delta direct-layer read missing published layer {} during materialization",
                        layer_id.0
                    ),
                )
            })?;
            let layer_artifacts = if branch_delta_layer_artifacts_empty(&layer.artifacts) {
                self.build_branch_delta_layer_artifacts(&layer.branch_id, &layer.commit_ids)?
            } else {
                layer.artifacts.clone()
            };
            export
                .commit_envelopes
                .extend(layer_artifacts.commit_envelopes.iter().cloned());
            export
                .commit_parent_records
                .extend(layer_artifacts.commit_parent_records.iter().cloned());
        }

        hydrate_export_support_records(self, &mut export, plan.locality.branch_id.clone())?;
        Ok(export)
    }
}

pub(super) fn hydrate_export_support_records(
    state: &StoreState,
    export: &mut AuthoritativeExportBundle,
    branch_id: BranchId,
) -> Result<(), StoreError> {
    let final_commit_set = export
        .commit_envelopes
        .iter()
        .map(|record| record.envelope.commit.commit_id)
        .collect::<BTreeSet<_>>();
    export.commit_support_summaries = state
        .commit_support_summaries
        .values()
        .filter(|record| final_commit_set.contains(&record.commit_id))
        .cloned()
        .collect();
    export.schema_support_records = state
        .schema_support_records
        .values()
        .filter(|record| final_commit_set.contains(&record.commit_id))
        .cloned()
        .collect();
    export.lineage_support_records = state
        .lineage_support_records
        .values()
        .filter(|record| final_commit_set.contains(&record.commit_id))
        .cloned()
        .collect();
    export.durable_cursor_identity_records = state
        .durable_cursor_identity_records
        .values()
        .filter(|record| {
            record.branch_id == branch_id && final_commit_set.contains(&record.latest_basis_commit_id)
        })
        .cloned()
        .collect();
    export.subscriber_checkpoint_records = state
        .subscriber_checkpoint_records
        .values()
        .filter(|record| record.branch_id == branch_id && final_commit_set.contains(&record.basis_commit_id))
        .cloned()
        .collect();

    let target_commit_id = export
        .commit_envelopes
        .last()
        .map(|record| record.envelope.commit.commit_id)
        .ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::BranchDeltaReadTargetIllegal,
                format!("target commit for branch `{}` not found during branch delta materialization", branch_id.0),
            )
        })?;
    let target_record = state.commit_record(target_commit_id).ok_or_else(|| {
        StoreError::new(
            StoreErrorKind::BranchDeltaReadTargetIllegal,
            format!(
                "target commit {} not found during branch delta materialization",
                target_commit_id.0
            ),
        )
    })?;
    export.branch_head_records = vec![BranchHeadRecord {
        branch_id: branch_id.clone(),
        head_commit_id: Some(target_commit_id),
        head_commit_digest: Some(target_record.envelope_digest.clone()),
        head_update_sequence: target_record.commit_sequence,
    }];
    export.authoritative_artifact_digests = state
        .rebuild_authoritative_export_digests(export)?
        .into_values()
        .collect();
    export.canonicalize_order();
    Ok(())
}
