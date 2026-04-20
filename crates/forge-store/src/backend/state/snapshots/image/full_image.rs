use std::collections::{BTreeMap, BTreeSet};

use forge_relational::facade::history::{BranchId, CommitId};

use super::full_image_digests::{
    insert_branch_artifact_digest, insert_branch_head_artifact_digest,
    insert_commit_artifact_digests, insert_commit_parent_artifact_digests,
    insert_support_artifact_digests,
};
use crate::{
    authority::AuthoritativeExportBundle,
    backend::{
        integrity::{branch_key, lineage_support_artifact_id, schema_support_artifact_id},
        records::{
            BranchHeadRecord, CommitParentRecord, CommitSupportSummaryRecord,
            DurableCursorIdentityRecord, LineageSupportRecord, SchemaSupportRecord,
            StoreState, SubscriberCheckpointRecord,
        },
    },
    failure::{StoreError, StoreErrorKind},
    snapshot::SnapshotImageBundle,
};

impl StoreState {
    pub fn build_snapshot_image(
        &self,
        branch_id: &BranchId,
        frontier_commit_id: CommitId,
    ) -> Result<SnapshotImageBundle, StoreError> {
        let frontier_record = self.commit_record(frontier_commit_id).ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::SnapshotBasisUnsupported,
                format!("frontier commit {} not found", frontier_commit_id.0),
            )
        })?;
        if &frontier_record.envelope.branch_context != branch_id {
            return Err(StoreError::new(
                StoreErrorKind::SnapshotBasisAmbiguous,
                format!(
                    "frontier commit {} belongs to branch `{}` not requested branch `{}`",
                    frontier_commit_id.0, frontier_record.envelope.branch_context.0, branch_id.0
                ),
            ));
        }
        let branch_record = self
            .branch_records
            .get(&branch_key(branch_id))
            .cloned()
            .ok_or_else(|| StoreError::unknown_branch(branch_id))?;
        let history_range = self.snapshot_history_range(frontier_commit_id)?;
        let commit_set: BTreeSet<_> = history_range.iter().copied().collect();

        let commit_envelopes = history_range
            .iter()
            .map(|commit_id| {
                self.commit_record(*commit_id).cloned().ok_or_else(|| {
                    StoreError::backend_integrity("snapshot history range commit missing")
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let commit_parent_records = history_range
            .iter()
            .flat_map(|commit_id| {
                self.commit_record(*commit_id)
                    .into_iter()
                    .flat_map(|record| record.envelope.commit.parents.iter().copied().enumerate())
                    .map(
                        move |(parent_position, parent_commit_id)| CommitParentRecord {
                            commit_id: *commit_id,
                            parent_position,
                            parent_commit_id,
                        },
                    )
            })
            .filter(|record| commit_set.contains(&record.parent_commit_id))
            .collect::<Vec<_>>();
        let commit_support_summaries = history_range
            .iter()
            .filter_map(|commit_id| self.commit_support_summaries.get(&commit_id.0).cloned())
            .collect::<Vec<CommitSupportSummaryRecord>>();
        let schema_support_records = history_range
            .iter()
            .filter_map(|commit_id| {
                self.schema_support_records
                    .get(&schema_support_artifact_id(*commit_id))
                    .cloned()
            })
            .collect::<Vec<SchemaSupportRecord>>();
        let lineage_support_records = history_range
            .iter()
            .filter_map(|commit_id| {
                self.lineage_support_records
                    .get(&lineage_support_artifact_id(*commit_id))
                    .cloned()
            })
            .collect::<Vec<LineageSupportRecord>>();
        let durable_cursor_identity_records = self
            .durable_cursor_identity_records
            .values()
            .filter(|record| {
                record.branch_id == *branch_id
                    && commit_set.contains(&record.latest_basis_commit_id)
            })
            .cloned()
            .collect::<Vec<DurableCursorIdentityRecord>>();
        let subscriber_checkpoint_records = self
            .subscriber_checkpoint_records
            .values()
            .filter(|record| {
                record.branch_id == *branch_id && commit_set.contains(&record.basis_commit_id)
            })
            .cloned()
            .collect::<Vec<SubscriberCheckpointRecord>>();
        let stable_basis_records = self
            .stable_basis_records
            .values()
            .filter(|record| {
                record.request.branch_id() == branch_id
                    && commit_set.contains(&record.request.frontier_commit_id())
            })
            .cloned()
            .collect::<Vec<_>>();

        let branch_head_record = BranchHeadRecord {
            branch_id: branch_id.clone(),
            head_commit_id: Some(frontier_commit_id),
            head_commit_digest: Some(frontier_record.envelope_digest.clone()),
            head_update_sequence: frontier_record.commit_sequence,
        };
        let mut authoritative_artifact_digests = BTreeMap::new();
        insert_branch_artifact_digest(
            &mut authoritative_artifact_digests,
            self.canonicalization_version,
            &branch_record,
        )?;
        insert_branch_head_artifact_digest(
            &mut authoritative_artifact_digests,
            self.canonicalization_version,
            &branch_head_record,
        )?;
        insert_commit_artifact_digests(
            &mut authoritative_artifact_digests,
            self.canonicalization_version,
            &commit_envelopes,
        );
        insert_commit_parent_artifact_digests(
            &mut authoritative_artifact_digests,
            self.canonicalization_version,
            &commit_parent_records,
        )?;
        insert_support_artifact_digests(
            &mut authoritative_artifact_digests,
            self.canonicalization_version,
            &commit_support_summaries,
            &schema_support_records,
            &lineage_support_records,
            &durable_cursor_identity_records,
            &subscriber_checkpoint_records,
            &stable_basis_records,
        )?;

        let mut bundle = AuthoritativeExportBundle {
            canonicalization_version: self.canonicalization_version,
            branch_records: vec![branch_record],
            branch_head_records: vec![branch_head_record],
            commit_envelopes,
            commit_parent_records,
            commit_support_summaries,
            schema_support_records,
            lineage_support_records,
            durable_cursor_identity_records,
            subscriber_checkpoint_records,
            stable_basis_records,
            authoritative_artifact_digests: authoritative_artifact_digests.into_values().collect(),
        };
        bundle.canonicalize_order();
        Ok(SnapshotImageBundle::new(bundle))
    }
}
