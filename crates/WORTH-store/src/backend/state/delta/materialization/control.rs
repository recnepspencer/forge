use crate::{
    authority::AuthoritativeExportBundle,
    delta::{
        stable_shared_base_authority_digest, BranchDeltaFallbackClass, BranchDeltaLocality,
        BranchDeltaPerformanceEnvelope, BranchDeltaReadPlan, BranchDeltaReadStrategy,
        ComplexityStatus, SameBranchDescendantWitness, BRANCH_DELTA_FAMILY_VERSION,
    },
    failure::{StoreError, StoreErrorKind},
};

use crate::backend::{
    integrity::branch_key,
    records::{BranchSharedBaseRecord, CommitParentRecord, StoreState},
};

use super::{digests::empty_authoritative_export, direct::hydrate_export_support_records};
use crate::backend::state::delta::planning::regime_for_commit_span;

impl StoreState {
    pub(crate) fn plan_branch_delta_control_from_witness(
        &self,
        witness: &SameBranchDescendantWitness,
    ) -> BranchDeltaReadPlan {
        let commit_ids = witness.commit_ids().to_vec();
        let locality = BranchDeltaLocality {
            branch_id: witness.branch_id().clone(),
            base_frontier_commit_id: witness.base_frontier_commit_id(),
            target_commit_id: witness.target_commit_id(),
            commit_span: commit_ids.len(),
        };
        BranchDeltaReadPlan {
            strategy: BranchDeltaReadStrategy::AuthorityReplayControl,
            regime: regime_for_commit_span(commit_ids.len()),
            locality,
            used_layer_ids: Vec::new(),
            commit_ids: commit_ids.clone(),
            performance: BranchDeltaPerformanceEnvelope {
                layers_traversed: 0,
                records_decoded: commit_ids.len(),
                replay_commit_count: commit_ids.len(),
                fallback_class: BranchDeltaFallbackClass::None,
                complexity_status: ComplexityStatus::Verified,
            },
        }
    }

    pub(crate) fn materialize_authority_replay_control_export(
        &self,
        witness: &SameBranchDescendantWitness,
    ) -> Result<AuthoritativeExportBundle, StoreError> {
        let basis = self
            .branch_shared_base_records
            .get(&branch_key(witness.branch_id()))
            .cloned()
            .unwrap_or_else(|| BranchSharedBaseRecord {
                branch_id: witness.branch_id().clone(),
                source_branch_id: witness.branch_id().clone(),
                source_frontier_commit_id: None,
                delta_family_version: BRANCH_DELTA_FAMILY_VERSION,
                authority_basis_digest: stable_shared_base_authority_digest(
                    witness.branch_id(),
                    None,
                    self.canonicalization_version,
                ),
            });
        let mut export = if let Some(source_frontier_commit_id) = basis.source_frontier_commit_id {
            self.build_snapshot_image(&basis.source_branch_id, source_frontier_commit_id)?
                .authoritative_export()
                .clone()
        } else {
            empty_authoritative_export(self.canonicalization_version)
        };

        let branch_record = self
            .branch_records
            .get(&branch_key(witness.branch_id()))
            .cloned()
            .ok_or_else(|| StoreError::unknown_branch(witness.branch_id()))?;
        export.branch_records = vec![branch_record];

        for commit_id in witness.commit_ids() {
            let commit_record = self.commit_record(*commit_id).cloned().ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "authority replay control read missing commit {} for branch `{}`",
                        commit_id.0,
                        witness.branch_id().0
                    ),
                )
            })?;
            export.commit_envelopes.push(commit_record.clone());
            export.commit_parent_records.extend(
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
        }

        hydrate_export_support_records(self, &mut export, witness.branch_id().clone())?;
        Ok(export)
    }
}
