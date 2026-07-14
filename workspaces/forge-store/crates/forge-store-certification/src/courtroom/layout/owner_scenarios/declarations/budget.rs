use forge_store_budgets::{
    pre_execution_budget_admission, PreExecutionBudgetEnvelope, PreExecutionBudgetRequest,
    PreExecutionBudgetScope,
};

use super::super::LayoutOwnerObservationLedger;

pub(super) fn execute(ledger: &mut LayoutOwnerObservationLedger) {
    use PreExecutionBudgetScope as Scope;

    let envelope = PreExecutionBudgetEnvelope::new(Scope::Foreground, 8, 2, 2, 2, 8);
    let requests = [
        PreExecutionBudgetRequest::new(Scope::Foreground, 8, 2, 2, 2, 8),
        PreExecutionBudgetRequest::new(Scope::Maintenance, 0, 0, 0, 0, 0),
        PreExecutionBudgetRequest::new(Scope::Foreground, 9, 0, 0, 0, 0),
        PreExecutionBudgetRequest::new(Scope::Foreground, 0, 3, 0, 0, 0),
        PreExecutionBudgetRequest::new(Scope::Foreground, 0, 0, 3, 0, 0),
        PreExecutionBudgetRequest::new(Scope::Foreground, 0, 0, 0, 3, 0),
        PreExecutionBudgetRequest::new(Scope::Foreground, 0, 0, 0, 0, 9),
    ];

    for request in requests {
        let outcome = pre_execution_budget_admission().admit(request, envelope);
        ledger.record_pre_execution_budget_admission(outcome.owner_case_observation());
    }
}
