use crate::merge::MergeAccess;
use crate::runtime::RelationalRuntime;
use crate::transactions::data::MergeExecutionOutcome;

impl RelationalRuntime {
    pub(crate) fn merge_access(&self) -> MergeAccess<'_> {
        MergeAccess::new(self)
    }

    #[cfg(test)]
    pub fn prepare_merge_execution(
        &self,
        request: crate::merge::data::MergeExecutionRequest,
    ) -> Result<
        crate::merge::data::PreparedMergeExecution,
        crate::merge::data::MergeExecutionPreparationError,
    > {
        self.merge().prepare_merge_execution(request)
    }

    #[cfg(not(test))]
    pub fn prepare_merge_execution(
        &self,
        request: crate::merge::data::OwnerBoundMergeExecutionRequest,
    ) -> Result<
        crate::merge::data::PreparedMergeExecution,
        crate::merge::data::MergeExecutionPreparationError,
    > {
        self.merge().prepare_merge_execution(request)
    }

    pub fn execute_prepared_merge(
        &self,
        prepared: crate::merge::data::PreparedMergeExecution,
    ) -> Result<MergeExecutionOutcome, crate::merge::data::MergeExecutionError> {
        crate::merge::execution::execute_prepared_merge(self, prepared)
    }
}
