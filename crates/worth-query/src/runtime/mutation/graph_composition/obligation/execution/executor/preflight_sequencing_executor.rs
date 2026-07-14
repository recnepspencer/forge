use crate::runtime::{
    WorthQueryGraphObligationExecutionInput, WorthQueryGraphObligationExecutionResultRow,
    WorthQueryGraphObligationExecutionStatus, WorthQueryGraphObligationStateLoadCounters,
    WorthQueryGraphObligationVerdict,
};

pub fn execute_preflight_sequencing_obligation(
    input: WorthQueryGraphObligationExecutionInput,
) -> WorthQueryGraphObligationExecutionResultRow {
    if input.execution_context().preflight_witness().is_satisfied() {
        return WorthQueryGraphObligationExecutionResultRow::executed(
            input,
            WorthQueryGraphObligationVerdict::allow_with_context(
                "preflight-prerequisite-satisfied",
            )
            .expect("static preflight context is non-empty"),
            WorthQueryGraphObligationStateLoadCounters::none(),
        );
    }
    WorthQueryGraphObligationExecutionResultRow::new(
        input,
        WorthQueryGraphObligationExecutionStatus::BlockedByPrerequisite,
        Some(
            WorthQueryGraphObligationVerdict::block("preflight-prerequisite-not-satisfied")
                .expect("static preflight context is non-empty"),
        ),
        WorthQueryGraphObligationStateLoadCounters::none(),
    )
}
