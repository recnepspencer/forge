use crate::runtime::{
    WorthQueryGraphObligationExecutionInput, WorthQueryGraphObligationExecutionResultRow,
    WorthQueryGraphObligationStateLoadCounters, WorthQueryGraphObligationVerdict,
};

pub fn execute_advisory_obligation(
    input: WorthQueryGraphObligationExecutionInput,
) -> WorthQueryGraphObligationExecutionResultRow {
    WorthQueryGraphObligationExecutionResultRow::executed(
        input,
        WorthQueryGraphObligationVerdict::advise("advisory-obligation-selected")
            .expect("static advisory context is non-empty"),
        WorthQueryGraphObligationStateLoadCounters::none(),
    )
}
