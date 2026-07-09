use crate::authority::commit::pipeline::{
    execute_authoritative_commit, AuthoritativeCommitContext,
};
use crate::commit_strategies::data::{
    CanonicalStrategyCommitRequest, LoweredStrategyCommitPlan, NativeStrategyCommitRequest,
    StrategyCommitRequestError, StrategyExecutionDraft, StrategyLoweringError,
    ValidatedStrategyCommitPlan,
};
use crate::commit_strategies::logic::{
    bind_execution, canonicalize_request, execute_bound_strategy, lower_execution,
    validate_lowered_plan as validate_lowered_strategy_plan,
};
use crate::logic::runtime::RelationalRuntime;
use crate::snapshots::data::SnapshotHandle;
use crate::transactions::data::{CommitResult, TransactionCommitError, TransactionOptions};

#[derive(Debug, Clone, Copy)]
pub struct CommitStrategiesFacade<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl<'runtime> CommitStrategiesFacade<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn canonicalize_request(
        &self,
        request: &NativeStrategyCommitRequest,
    ) -> Result<CanonicalStrategyCommitRequest, StrategyCommitRequestError> {
        canonicalize_request(self.runtime.commit_strategy_registry(), request)
    }

    pub fn execute(
        &self,
        request: &CanonicalStrategyCommitRequest,
        snapshot: &SnapshotHandle,
    ) -> Result<StrategyExecutionDraft, crate::commit_strategies::StrategyExecutionError> {
        let bound = bind_execution(self.runtime, request, snapshot)?;
        execute_bound_strategy(bound)
    }
}

#[derive(Debug)]
pub struct CommitStrategiesAuthorityFacade<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl<'runtime> CommitStrategiesAuthorityFacade<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn lower_execution(
        &mut self,
        request: &CanonicalStrategyCommitRequest,
        execution: &StrategyExecutionDraft,
        options: TransactionOptions,
    ) -> Result<LoweredStrategyCommitPlan, StrategyLoweringError> {
        lower_execution(self.runtime, request, execution, options)
    }

    pub fn execute_lowered_commit(
        &mut self,
        lowered: LoweredStrategyCommitPlan,
    ) -> Result<CommitResult, TransactionCommitError> {
        execute_authoritative_commit(
            self.runtime,
            AuthoritativeCommitContext::from_strategy(self.runtime, lowered),
        )
    }

    pub fn validate_lowered_plan(
        &mut self,
        lowered: LoweredStrategyCommitPlan,
    ) -> Result<ValidatedStrategyCommitPlan, TransactionCommitError> {
        validate_lowered_strategy_plan(self.runtime, lowered)
    }

    pub fn execute_validated_commit(
        &mut self,
        validated: ValidatedStrategyCommitPlan,
    ) -> Result<CommitResult, TransactionCommitError> {
        execute_authoritative_commit(
            self.runtime,
            AuthoritativeCommitContext::from_validated_strategy(self.runtime, validated),
        )
    }
}

#[cfg(test)]
#[path = "facade_tests/mod.rs"]
mod tests;
