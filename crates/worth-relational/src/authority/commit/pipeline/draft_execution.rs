use super::authority_context::PreparedAuthorityScope;
use super::draft_preparation_phase::record_draft_preparation_phase;
use super::execution_admission::AdmittedCommitExecution;
use crate::authority::commit::phases::prepare::prepare_authoritative_working_state_scope;

pub(super) struct PreparedCommitExecution {
    admitted: AdmittedCommitExecution,
    public_structural_summary: crate::transactions::data::CommitStructuralSummary,
    working_state: crate::storage::overlay::WorkingState,
}

impl PreparedCommitExecution {
    pub(super) fn admitted_mut(&mut self) -> &mut AdmittedCommitExecution {
        &mut self.admitted
    }

    pub(super) fn mutation_parts(
        &mut self,
    ) -> (
        &mut AdmittedCommitExecution,
        &mut crate::storage::overlay::WorkingState,
    ) {
        (&mut self.admitted, &mut self.working_state)
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        AdmittedCommitExecution,
        crate::transactions::data::CommitStructuralSummary,
        crate::storage::overlay::WorkingState,
    ) {
        (
            self.admitted,
            self.public_structural_summary,
            self.working_state,
        )
    }
}

pub(super) fn prepare_commit_execution(
    runtime: &mut crate::runtime::RelationalRuntime,
    mut admitted: AdmittedCommitExecution,
) -> PreparedCommitExecution {
    let merge_parent_count = admitted
        .merge_history_plan()
        .map(|plan| plan.requested_merge_parent_count)
        .unwrap_or(admitted.options().merge_parent_branches.len());
    let PreparedAuthorityScope {
        structural_summary,
        working_state,
        phase_timing: prepared_timing,
    } = admitted.take_prepared_scope().unwrap_or_else(|| {
        let (structural_summary, working_state, phase_timing) =
            prepare_authoritative_working_state_scope(
                runtime,
                admitted.merged_plan(),
                merge_parent_count,
            );
        PreparedAuthorityScope {
            structural_summary,
            working_state,
            phase_timing,
        }
    });
    admitted.phase_timing_mut().draft_structural_summary_micros =
        prepared_timing.draft_structural_summary_micros;
    admitted.phase_timing_mut().draft_working_state_clone_micros =
        prepared_timing.draft_working_state_clone_micros;
    let public_structural_summary = structural_summary.public_summary(
        runtime
            .config
            .schema
            .descriptor_semantics_policy
            .current_write_version(),
    );
    let (commit_log, phase_timing) = admitted.commit_phase_state();
    record_draft_preparation_phase(
        runtime,
        commit_log,
        phase_timing,
        &working_state,
        &structural_summary,
        &public_structural_summary,
    );
    PreparedCommitExecution {
        admitted,
        public_structural_summary,
        working_state,
    }
}
