use crate::runtime::{
    ForgeQueryGraphObligationBudgetExceededPolicy, ForgeQueryGraphObligationExecutionContext,
    ForgeQueryGraphObligationExecutionInput, ForgeQueryGraphObligationExecutionResultEnvelope,
    ForgeQueryGraphObligationExecutionResultRow, ForgeQueryGraphObligationExecutionStatus,
    ForgeQueryGraphObligationSelection, ForgeQueryGraphObligationStateLoadCounters,
    ForgeQueryGraphObligationStateLoadPlan, ForgeQueryGraphObligationSupportStatus,
    ForgeQueryGraphObligationVerdict,
};

use super::advisory_executor::execute_advisory_obligation;
use super::capability_gap_executor::{
    budget_exceeded_capability_gap_screen, execute_capability_gap_screen,
};
use super::executor_family::ForgeQueryGraphObligationExecutorFamily;
use super::preflight_sequencing_executor::execute_preflight_sequencing_obligation;

pub fn execute_selected_graph_obligations_with_context(
    selection: &ForgeQueryGraphObligationSelection,
    execution_context: ForgeQueryGraphObligationExecutionContext,
) -> Option<ForgeQueryGraphObligationExecutionResultEnvelope> {
    if selection.matched_registrations().is_empty() {
        return None;
    }
    let rows = selection
        .matched_registrations()
        .iter()
        .cloned()
        .map(|registration| {
            let input =
                ForgeQueryGraphObligationExecutionInput::from_selected_registration_with_context(
                    selection.selection_digest(),
                    registration,
                    execution_context.clone(),
                );
            execute_selected_graph_obligation(input)
        })
        .collect();
    Some(ForgeQueryGraphObligationExecutionResultEnvelope::new(rows))
}

pub fn execute_selected_graph_obligation(
    input: ForgeQueryGraphObligationExecutionInput,
) -> ForgeQueryGraphObligationExecutionResultRow {
    execute_selected_graph_obligation_input(input)
}

fn execute_selected_graph_obligation_input(
    input: ForgeQueryGraphObligationExecutionInput,
) -> ForgeQueryGraphObligationExecutionResultRow {
    if let Some(budget_row) = budget_denial_before_state_load(&input) {
        return budget_row;
    }
    match input.selected_registration().support_posture().status() {
        ForgeQueryGraphObligationSupportStatus::Supported => {
            execute_supported_graph_obligation(input)
        }
        ForgeQueryGraphObligationSupportStatus::Unsupported => status_row_with_blocking_verdict(
            input,
            ForgeQueryGraphObligationExecutionStatus::Unsupported,
            "selected-obligation-unsupported",
        ),
        ForgeQueryGraphObligationSupportStatus::NotApplicable => {
            ForgeQueryGraphObligationExecutionResultRow::status_only(
                input,
                ForgeQueryGraphObligationExecutionStatus::NotApplicableAfterStateLoad,
            )
        }
        ForgeQueryGraphObligationSupportStatus::DiagnosticOnly => {
            let diagnostic_materialization = input
                .execution_context()
                .artifact_policy()
                .diagnostic_materialization();
            ForgeQueryGraphObligationExecutionResultRow::new_with_diagnostic_materialization(
                input,
                ForgeQueryGraphObligationExecutionStatus::DiagnosticOnly,
                Some(
                    ForgeQueryGraphObligationVerdict::advise("selected-diagnostic-only")
                        .expect("static diagnostic-only context is non-empty"),
                ),
                ForgeQueryGraphObligationStateLoadCounters::none(),
                diagnostic_materialization,
            )
        }
        ForgeQueryGraphObligationSupportStatus::DeferredToBackstop => {
            status_row_with_allowing_verdict(
                input,
                ForgeQueryGraphObligationExecutionStatus::DeferredToBackstop,
                "selected-deferred-to-backstop",
            )
        }
    }
}

fn execute_supported_graph_obligation(
    input: ForgeQueryGraphObligationExecutionInput,
) -> ForgeQueryGraphObligationExecutionResultRow {
    let diagnostic_materialization = input
        .execution_context()
        .artifact_policy()
        .diagnostic_materialization();
    match ForgeQueryGraphObligationExecutorFamily::from_obligation_kind(
        input.selected_registration().kind(),
    ) {
        ForgeQueryGraphObligationExecutorFamily::SelectionBackedDispatch
        | ForgeQueryGraphObligationExecutorFamily::OperatingContextGate => {
            ForgeQueryGraphObligationExecutionResultRow::executed(
                input,
                ForgeQueryGraphObligationVerdict::allow_with_context("selected-for-execution")
                    .expect("static allow context is non-empty"),
                ForgeQueryGraphObligationStateLoadCounters::none(),
            )
        }
        ForgeQueryGraphObligationExecutorFamily::AdvisoryObligation => {
            execute_advisory_obligation(input)
        }
        ForgeQueryGraphObligationExecutorFamily::CapabilityGapScreen => {
            execute_capability_gap_screen(input, diagnostic_materialization)
        }
        ForgeQueryGraphObligationExecutorFamily::PreflightSequencing => {
            execute_preflight_sequencing_obligation(input)
        }
    }
}

fn budget_denial_before_state_load(
    input: &ForgeQueryGraphObligationExecutionInput,
) -> Option<ForgeQueryGraphObligationExecutionResultRow> {
    let budget = input.executor_contract().execution_budget();
    let state_load_plan = ForgeQueryGraphObligationStateLoadPlan::from_execution_input(input);
    let required_state_scope = state_load_plan.required_state_scope_count();
    let max_state_scope = budget.max_state_scope()?;
    if required_state_scope <= max_state_scope {
        return None;
    }
    let counters = state_load_plan.counters_before_state_load();
    let verdict = budget_exceeded_verdict(budget.budget_exceeded_policy());
    Some(
        match ForgeQueryGraphObligationExecutorFamily::from_obligation_kind(
            input.selected_registration().kind(),
        ) {
            ForgeQueryGraphObligationExecutorFamily::CapabilityGapScreen => {
                budget_exceeded_capability_gap_screen(input.clone(), counters, verdict)
            }
            _ => ForgeQueryGraphObligationExecutionResultRow::new(
                input.clone(),
                ForgeQueryGraphObligationExecutionStatus::BudgetExceeded,
                verdict,
                counters,
            ),
        },
    )
}

fn budget_exceeded_verdict(
    policy: ForgeQueryGraphObligationBudgetExceededPolicy,
) -> Option<ForgeQueryGraphObligationVerdict> {
    match policy {
        ForgeQueryGraphObligationBudgetExceededPolicy::FailClosed => Some(
            ForgeQueryGraphObligationVerdict::block("obligation-execution-budget-exceeded")
                .expect("static budget context is non-empty"),
        ),
        ForgeQueryGraphObligationBudgetExceededPolicy::Advisory => Some(
            ForgeQueryGraphObligationVerdict::advise("obligation-execution-budget-exceeded")
                .expect("static budget context is non-empty"),
        ),
        ForgeQueryGraphObligationBudgetExceededPolicy::DiagnosticOnly
        | ForgeQueryGraphObligationBudgetExceededPolicy::DeferredToBackstop => None,
    }
}

fn status_row_with_blocking_verdict(
    input: ForgeQueryGraphObligationExecutionInput,
    status: ForgeQueryGraphObligationExecutionStatus,
    context: &'static str,
) -> ForgeQueryGraphObligationExecutionResultRow {
    ForgeQueryGraphObligationExecutionResultRow::new(
        input,
        status,
        Some(
            ForgeQueryGraphObligationVerdict::block(context)
                .expect("static blocking context is non-empty"),
        ),
        ForgeQueryGraphObligationStateLoadCounters::none(),
    )
}

fn status_row_with_allowing_verdict(
    input: ForgeQueryGraphObligationExecutionInput,
    status: ForgeQueryGraphObligationExecutionStatus,
    context: &'static str,
) -> ForgeQueryGraphObligationExecutionResultRow {
    ForgeQueryGraphObligationExecutionResultRow::new(
        input,
        status,
        Some(
            ForgeQueryGraphObligationVerdict::allow_with_context(context)
                .expect("static allowing context is non-empty"),
        ),
        ForgeQueryGraphObligationStateLoadCounters::none(),
    )
}
