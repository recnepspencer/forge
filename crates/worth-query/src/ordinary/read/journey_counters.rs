#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryReadJourneyCounters {
    context_admission_attempt_count: usize,
    lower_runtime_execution_attempt_count: usize,
    lower_runtime_execution_completed_count: usize,
}

impl WorthQueryReadJourneyCounters {
    pub fn context_admission_attempt_count(&self) -> usize {
        self.context_admission_attempt_count
    }

    pub fn lower_runtime_execution_attempt_count(&self) -> usize {
        self.lower_runtime_execution_attempt_count
    }

    pub fn lower_runtime_execution_completed_count(&self) -> usize {
        self.lower_runtime_execution_completed_count
    }

    pub(crate) fn begin_context_admission() -> Self {
        Self {
            context_admission_attempt_count: 1,
            lower_runtime_execution_attempt_count: 0,
            lower_runtime_execution_completed_count: 0,
        }
    }

    pub(crate) fn record_lower_runtime_execution_attempt(mut self) -> Self {
        self.lower_runtime_execution_attempt_count += 1;
        self
    }

    pub(crate) fn record_lower_runtime_execution_completed(mut self) -> Self {
        self.lower_runtime_execution_completed_count += 1;
        self
    }
}
