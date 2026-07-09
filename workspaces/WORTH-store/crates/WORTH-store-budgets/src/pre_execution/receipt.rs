use super::admission::{S8PreExecutionBudgetEnvelope, S8PreExecutionBudgetScope};
use super::request::S8PreExecutionPlanBinding;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8PreExecutionBudgetAdmissionReceipt {
    plan_binding: S8PreExecutionPlanBinding,
    scope: S8PreExecutionBudgetScope,
    admitted_envelope: S8PreExecutionBudgetEnvelope,
}

impl S8PreExecutionBudgetAdmissionReceipt {
    pub(crate) const fn new(
        plan_binding: S8PreExecutionPlanBinding,
        scope: S8PreExecutionBudgetScope,
        admitted_envelope: S8PreExecutionBudgetEnvelope,
    ) -> Self {
        Self {
            plan_binding,
            scope,
            admitted_envelope,
        }
    }

    pub const fn plan_binding(self) -> S8PreExecutionPlanBinding {
        self.plan_binding
    }

    pub const fn scope(self) -> S8PreExecutionBudgetScope {
        self.scope
    }

    pub const fn admitted_envelope(self) -> S8PreExecutionBudgetEnvelope {
        self.admitted_envelope
    }
}
