use super::draft_execution::PreparedCommitExecution;
use super::invariant_phase::enforce_commit_boundary_phase;

pub(super) struct BoundaryValidatedCommitExecution {
    prepared: PreparedCommitExecution,
    invariant_executions: Vec<crate::validation::engine::InvariantExecutionResult>,
}

impl BoundaryValidatedCommitExecution {
    pub(super) fn prepared_mut(&mut self) -> &mut PreparedCommitExecution {
        &mut self.prepared
    }

    pub(super) fn push_invariant(
        &mut self,
        invariant: crate::validation::engine::InvariantExecutionResult,
    ) {
        self.invariant_executions.push(invariant);
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        PreparedCommitExecution,
        Vec<crate::validation::engine::InvariantExecutionResult>,
    ) {
        (self.prepared, self.invariant_executions)
    }
}

pub(super) fn validate_commit_boundary(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    mut prepared: PreparedCommitExecution,
) -> Result<BoundaryValidatedCommitExecution, crate::transactions::data::TransactionCommitError> {
    let prevalidated = prepared.admitted_mut().take_prevalidated_boundary();
    let (_, _, merged_plan, _, commit_log, phase_timing) =
        prepared.admitted_mut().phase_view().into_parts();
    let invariant = enforce_commit_boundary_phase(
        runtime,
        commit_log,
        phase_timing,
        merged_plan,
        prevalidated,
    )?;
    Ok(BoundaryValidatedCommitExecution {
        prepared,
        invariant_executions: vec![invariant],
    })
}
