use super::admission::{PreExecutionBudgetEnvelope, PreExecutionBudgetScope};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreExecutionBudgetAdmissionReceipt {
    request: super::request::PreExecutionBudgetRequest,
    scope: PreExecutionBudgetScope,
    admitted_envelope: PreExecutionBudgetEnvelope,
}

impl PreExecutionBudgetAdmissionReceipt {
    pub(crate) const fn new(
        request: super::request::PreExecutionBudgetRequest,
        scope: PreExecutionBudgetScope,
        admitted_envelope: PreExecutionBudgetEnvelope,
    ) -> Self {
        Self {
            request,
            scope,
            admitted_envelope,
        }
    }

    pub const fn request(self) -> super::request::PreExecutionBudgetRequest {
        self.request
    }

    pub const fn scope(self) -> PreExecutionBudgetScope {
        self.scope
    }

    pub const fn admitted_envelope(self) -> PreExecutionBudgetEnvelope {
        self.admitted_envelope
    }
}
