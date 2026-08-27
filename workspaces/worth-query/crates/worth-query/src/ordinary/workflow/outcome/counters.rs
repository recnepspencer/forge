#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryWorkflowCounters {
    context_validation_count: usize,
    session_open_attempt_count: usize,
    lower_runtime_execution_attempt_count: usize,
    lower_runtime_execution_completed_count: usize,
    settlement_deferred_count: usize,
    inspection_materialization_count: usize,
}

impl WorthQueryWorkflowCounters {
    pub fn context_validation_count(&self) -> usize {
        self.context_validation_count
    }

    pub fn session_open_attempt_count(&self) -> usize {
        self.session_open_attempt_count
    }

    pub fn lower_runtime_execution_attempt_count(&self) -> usize {
        self.lower_runtime_execution_attempt_count
    }

    pub fn lower_runtime_execution_completed_count(&self) -> usize {
        self.lower_runtime_execution_completed_count
    }

    pub fn settlement_deferred_count(&self) -> usize {
        self.settlement_deferred_count
    }

    pub fn inspection_materialization_count(&self) -> usize {
        self.inspection_materialization_count
    }

    pub(crate) fn context_checked() -> Self {
        Self {
            context_validation_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn execution_attempted(mut self) -> Self {
        self.session_open_attempt_count += 1;
        self.lower_runtime_execution_attempt_count += 1;
        self
    }

    pub(crate) fn lower_runtime_attempted(mut self) -> Self {
        self.lower_runtime_execution_attempt_count += 1;
        self
    }

    pub(crate) fn execution_completed(mut self, inspection_materialized: bool) -> Self {
        self.lower_runtime_execution_completed_count += 1;
        self.inspection_materialization_count += usize::from(inspection_materialized);
        self
    }

    pub(crate) fn settlement_deferred(mut self) -> Self {
        self.lower_runtime_execution_completed_count += 1;
        self.settlement_deferred_count += 1;
        self
    }
}
