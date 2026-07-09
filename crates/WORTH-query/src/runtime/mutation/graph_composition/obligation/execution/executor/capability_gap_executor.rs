use crate::runtime::{
    WorthQueryGraphObligationDiagnosticMaterialization, WorthQueryGraphObligationExecutionInput,
    WorthQueryGraphObligationExecutionResultRow, WorthQueryGraphObligationExecutionStatus,
    WorthQueryGraphObligationStateLoadCounters, WorthQueryGraphObligationVerdict,
};

pub fn execute_capability_gap_screen(
    input: WorthQueryGraphObligationExecutionInput,
    diagnostic_materialization: WorthQueryGraphObligationDiagnosticMaterialization,
) -> WorthQueryGraphObligationExecutionResultRow {
    WorthQueryGraphObligationExecutionResultRow::new_with_diagnostic_materialization(
        input,
        WorthQueryGraphObligationExecutionStatus::Executed,
        Some(
            WorthQueryGraphObligationVerdict::block("capability-gap-screen-selected")
                .expect("static capability-gap context is non-empty"),
        ),
        WorthQueryGraphObligationStateLoadCounters::none(),
        diagnostic_materialization,
    )
}

pub fn budget_exceeded_capability_gap_screen(
    input: WorthQueryGraphObligationExecutionInput,
    counters: WorthQueryGraphObligationStateLoadCounters,
    verdict: Option<WorthQueryGraphObligationVerdict>,
) -> WorthQueryGraphObligationExecutionResultRow {
    WorthQueryGraphObligationExecutionResultRow::new(
        input,
        WorthQueryGraphObligationExecutionStatus::BudgetExceeded,
        verdict,
        counters,
    )
}
