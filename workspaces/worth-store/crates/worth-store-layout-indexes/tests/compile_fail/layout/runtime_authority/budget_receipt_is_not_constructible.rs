use worth_store_budgets::{
    PreExecutionBudgetAdmissionReceipt, PreExecutionBudgetEnvelope, PreExecutionBudgetRequest,
    PreExecutionBudgetScope,
};

fn worth() -> PreExecutionBudgetAdmissionReceipt {
    PreExecutionBudgetAdmissionReceipt {
        request: PreExecutionBudgetRequest::new(PreExecutionBudgetScope::Foreground, 0, 0, 0, 0, 0),
        scope: PreExecutionBudgetScope::Foreground,
        admitted_envelope: PreExecutionBudgetEnvelope::foreground_default(),
    }
}

fn main() {}
