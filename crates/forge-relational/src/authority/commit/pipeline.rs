use crate::authority::commit::phases::artifacts::prepare_publication_artifacts;
use crate::authority::commit::phases::finalize::{
    finalize_commit_publication, FinalizeCommitInput,
};
use crate::authority::commit::phases::history::resolve_commit_history;
use crate::authority::commit::phases::invariants::{
    run_commit_boundary_invariants, run_snapshot_publication_invariants,
};
use crate::authority::commit::phases::mutation::run_authoritative_mutation;
use crate::authority::commit::phases::prepare::{
    prepare_working_state_scope, record_preparation_counters,
};
use crate::authority::commit::phases::publication::{
    append_durable_commit,
};
use crate::publication::data::PublicationStatus;
use crate::transactions::logic::RelationalTransaction;
use crate::transactions::data::{CommitOutcome, TransactionCommitError};

impl<'a> RelationalTransaction<'a> {
    /// Executes the serialized truth-commit pipeline.
    ///
    /// The phases are intentionally explicit:
    /// 1. build a deterministic merged plan over the current immutable committed state
    /// 2. run commit-boundary invariants before any authoritative mutation
    /// 3. apply the authoritative plan into detached working state
    /// 4. run mutation-sensitive and publication invariants
    /// 5. assemble the canonical publication bundle and durable envelope
    /// 6. publish history/version visibility atomically into the runtime on success
    ///
    /// Any failure before publication discards the touched-partition overlay without making the
    /// commit visible.
    pub fn commit(mut self) -> Result<CommitOutcome, TransactionCommitError> {
        let prepared = prepare_working_state_scope(&mut self)?;
        let planning_state = prepared.planning_state;
        let merged_plan = prepared.merged_plan;
        let mut working_state = prepared.working_state;
        record_preparation_counters(self.runtime, &working_state, &planning_state, &merged_plan);
        run_commit_boundary_invariants(self.runtime, &merged_plan)?;

        let mutation = run_authoritative_mutation(&mut self, &mut working_state, &merged_plan)?;
        let version_id = mutation.version_id;
        let effect = mutation.effect;

        let history = resolve_commit_history(&mut self, version_id)?;
        let commit_id = history.commit_id;
        let branch_id = history.branch_id.clone();
        let commit_reference = history.commit_reference.clone();
        let merge_base_commits = history.merge_base_commits.clone();

        {
            if let Err(error) =
                run_snapshot_publication_invariants(self.runtime, &working_state, version_id, &merged_plan)
            {
                return Err(TransactionCommitError::Publication(error));
            }
        }

        let publication = prepare_publication_artifacts(
            self.runtime,
            &mut working_state,
            &commit_reference,
            &branch_id,
            version_id,
            self.options.merge_parent_branches.clone(),
            merge_base_commits.clone(),
            &merged_plan,
            effect,
        )?;

        append_durable_commit(
            self.runtime,
            &publication.canonical_commit_envelope,
            commit_id,
            &branch_id,
        )?;

        finalize_commit_publication(
            self.runtime,
            working_state,
            FinalizeCommitInput {
                changed_records: publication.changed_records.clone(),
                version_id,
                previous_branch_head_version: history.previous_branch_head_version,
                commit_id,
                commit_reference: commit_reference.clone(),
                canonical_commit_envelope: publication.canonical_commit_envelope,
                patch_position: publication.patch.position,
                branch_id,
                merge_base_commits,
                artifacts: publication.artifacts,
                merge_parent_branches: self.options.merge_parent_branches.clone(),
                adjacency_deltas: publication.adjacency_deltas,
            },
        );

        Ok(CommitOutcome {
            transaction_id: self.transaction_id,
            commit: commit_reference,
            version_id,
            snapshot: publication.published_snapshot,
            changed_records: publication.changed_records,
            publication_status: PublicationStatus::Published,
        })
    }
}
