use crate::runtime::{
    WorthQueryGraphObligationBudgetExceededPolicy, WorthQueryGraphObligationExecutionContext,
    WorthQueryGraphObligationExecutionInput, WorthQueryGraphObligationExecutionResultEnvelope,
    WorthQueryGraphObligationExecutionResultRow, WorthQueryGraphObligationExecutionStatus,
    WorthQueryGraphObligationSelection, WorthQueryGraphObligationStateLoadCounters,
    WorthQueryGraphObligationStateLoadPlan, WorthQueryGraphObligationSupportStatus,
    WorthQueryGraphObligationVerdict,
};

use super::advisory_executor::execute_advisory_obligation;
use super::capability_gap_executor::{
    budget_exceeded_capability_gap_screen, execute_capability_gap_screen,
};
use super::executor_family::WorthQueryGraphObligationExecutorFamily;
use super::preflight_sequencing_executor::execute_preflight_sequencing_obligation;

pub fn execute_selected_graph_obligations_with_context(
    selection: &WorthQueryGraphObligationSelection,
    execution_context: WorthQueryGraphObligationExecutionContext,
) -> Option<WorthQueryGraphObligationExecutionResultEnvelope> {
    if selection.matched_registrations().is_empty() {
        return None;
    }
    let rows = selection
        .matched_registrations()
        .iter()
        .cloned()
        .map(|registration| {
            let input =
                WorthQueryGraphObligationExecutionInput::from_selected_registration_with_context(
                    selection.selection_digest(),
                    registration,
                    execution_context.clone(),
                );
            execute_selected_graph_obligation(input)
        })
        .collect();
    Some(WorthQueryGraphObligationExecutionResultEnvelope::new(rows))
}

pub fn execute_selected_graph_obligation(
    input: WorthQueryGraphObligationExecutionInput,
) -> WorthQueryGraphObligationExecutionResultRow {
    execute_selected_graph_obligation_input(input)
}

fn execute_selected_graph_obligation_input(
    input: WorthQueryGraphObligationExecutionInput,
) -> WorthQueryGraphObligationExecutionResultRow {
    if let Some(budget_row) = budget_denial_before_state_load(&input) {
        return budget_row;
    }
    match input.selected_registration().support_posture().status() {
        WorthQueryGraphObligationSupportStatus::Supported => {
            execute_supported_graph_obligation(input)
        }
        WorthQueryGraphObligationSupportStatus::Unsupported => status_row_with_blocking_verdict(
            input,
            WorthQueryGraphObligationExecutionStatus::Unsupported,
            "selected-obligation-unsupported",
        ),
        WorthQueryGraphObligationSupportStatus::NotApplicable => {
            WorthQueryGraphObligationExecutionResultRow::status_only(
                input,
                WorthQueryGraphObligationExecutionStatus::NotApplicableAfterStateLoad,
            )
        }
        WorthQueryGraphObligationSupportStatus::DiagnosticOnly => {
            let diagnostic_materialization = input
                .execution_context()
                .artifact_policy()
                .diagnostic_materialization();
            WorthQueryGraphObligationExecutionResultRow::new_with_diagnostic_materialization(
                input,
                WorthQueryGraphObligationExecutionStatus::DiagnosticOnly,
                Some(
                    WorthQueryGraphObligationVerdict::advise("selected-diagnostic-only")
                        .expect("static diagnostic-only context is non-empty"),
                ),
                WorthQueryGraphObligationStateLoadCounters::none(),
                diagnostic_materialization,
            )
        }
        WorthQueryGraphObligationSupportStatus::DeferredToBackstop => {
            status_row_with_allowing_verdict(
                input,
                WorthQueryGraphObligationExecutionStatus::DeferredToBackstop,
                "selected-deferred-to-backstop",
            )
        }
    }
}

fn execute_supported_graph_obligation(
    input: WorthQueryGraphObligationExecutionInput,
) -> WorthQueryGraphObligationExecutionResultRow {
    let diagnostic_materialization = input
        .execution_context()
        .artifact_policy()
        .diagnostic_materialization();
    match WorthQueryGraphObligationExecutorFamily::from_obligation_kind(
        input.selected_registration().kind(),
    ) {
        WorthQueryGraphObligationExecutorFamily::SelectionBackedDispatch
        | WorthQueryGraphObligationExecutorFamily::OperatingContextGate => {
            WorthQueryGraphObligationExecutionResultRow::executed(
                input,
                WorthQueryGraphObligationVerdict::allow_with_context("selected-for-execution")
                    .expect("static allow context is non-empty"),
                WorthQueryGraphObligationStateLoadCounters::none(),
            )
        }
        WorthQueryGraphObligationExecutorFamily::AdvisoryObligation => {
            execute_advisory_obligation(input)
        }
        WorthQueryGraphObligationExecutorFamily::CapabilityGapScreen => {
            execute_capability_gap_screen(input, diagnostic_materialization)
        }
        WorthQueryGraphObligationExecutorFamily::PreflightSequencing => {
            execute_preflight_sequencing_obligation(input)
        }
    }
}

fn budget_denial_before_state_load(
    input: &WorthQueryGraphObligationExecutionInput,
) -> Option<WorthQueryGraphObligationExecutionResultRow> {
    let budget = input.executor_contract().execution_budget();
    let state_load_plan = WorthQueryGraphObligationStateLoadPlan::from_execution_input(input);
    let required_state_scope = state_load_plan.required_state_scope_count();
    let max_state_scope = budget.max_state_scope()?;
    if required_state_scope <= max_state_scope {
        return None;
    }
    let counters = state_load_plan.counters_before_state_load();
    let verdict = budget_exceeded_verdict(budget.budget_exceeded_policy());
    Some(
        match WorthQueryGraphObligationExecutorFamily::from_obligation_kind(
            input.selected_registration().kind(),
        ) {
            WorthQueryGraphObligationExecutorFamily::CapabilityGapScreen => {
                budget_exceeded_capability_gap_screen(input.clone(), counters, verdict)
            }
            _ => WorthQueryGraphObligationExecutionResultRow::new(
                input.clone(),
                WorthQueryGraphObligationExecutionStatus::BudgetExceeded,
                verdict,
                counters,
            ),
        },
    )
}

fn budget_exceeded_verdict(
    policy: WorthQueryGraphObligationBudgetExceededPolicy,
) -> Option<WorthQueryGraphObligationVerdict> {
    match policy {
        WorthQueryGraphObligationBudgetExceededPolicy::FailClosed => Some(
            WorthQueryGraphObligationVerdict::block("obligation-execution-budget-exceeded")
                .expect("static budget context is non-empty"),
        ),
        WorthQueryGraphObligationBudgetExceededPolicy::Advisory => Some(
            WorthQueryGraphObligationVerdict::advise("obligation-execution-budget-exceeded")
                .expect("static budget context is non-empty"),
        ),
        WorthQueryGraphObligationBudgetExceededPolicy::DiagnosticOnly
        | WorthQueryGraphObligationBudgetExceededPolicy::DeferredToBackstop => None,
    }
}

fn status_row_with_blocking_verdict(
    input: WorthQueryGraphObligationExecutionInput,
    status: WorthQueryGraphObligationExecutionStatus,
    context: &'static str,
) -> WorthQueryGraphObligationExecutionResultRow {
    WorthQueryGraphObligationExecutionResultRow::new(
        input,
        status,
        Some(
            WorthQueryGraphObligationVerdict::block(context)
                .expect("static blocking context is non-empty"),
        ),
        WorthQueryGraphObligationStateLoadCounters::none(),
    )
}

fn status_row_with_allowing_verdict(
    input: WorthQueryGraphObligationExecutionInput,
    status: WorthQueryGraphObligationExecutionStatus,
    context: &'static str,
) -> WorthQueryGraphObligationExecutionResultRow {
    WorthQueryGraphObligationExecutionResultRow::new(
        input,
        status,
        Some(
            WorthQueryGraphObligationVerdict::allow_with_context(context)
                .expect("static allowing context is non-empty"),
        ),
        WorthQueryGraphObligationStateLoadCounters::none(),
    )
}
