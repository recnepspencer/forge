use crate::runtime::{
    ForgeQueryGraphObligationDiagnosticMaterialization, ForgeQueryGraphObligationExecutionInput,
    ForgeQueryGraphObligationExecutionResultRow, ForgeQueryGraphObligationExecutionStatus,
    ForgeQueryGraphObligationStateLoadCounters, ForgeQueryGraphObligationVerdict,
};

pub fn execute_capability_gap_screen(
    input: ForgeQueryGraphObligationExecutionInput,
    diagnostic_materialization: ForgeQueryGraphObligationDiagnosticMaterialization,
) -> ForgeQueryGraphObligationExecutionResultRow {
    ForgeQueryGraphObligationExecutionResultRow::new_with_diagnostic_materialization(
        input,
        ForgeQueryGraphObligationExecutionStatus::Executed,
        Some(
            ForgeQueryGraphObligationVerdict::block("capability-gap-screen-selected")
                .expect("static capability-gap context is non-empty"),
        ),
        ForgeQueryGraphObligationStateLoadCounters::none(),
        diagnostic_materialization,
    )
}

pub fn budget_exceeded_capability_gap_screen(
    input: ForgeQueryGraphObligationExecutionInput,
    counters: ForgeQueryGraphObligationStateLoadCounters,
    verdict: Option<ForgeQueryGraphObligationVerdict>,
) -> ForgeQueryGraphObligationExecutionResultRow {
    ForgeQueryGraphObligationExecutionResultRow::new(
        input,
        ForgeQueryGraphObligationExecutionStatus::BudgetExceeded,
        verdict,
        counters,
    )
}
