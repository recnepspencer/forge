use crate::runtime::{
    ForgeQueryGraphObligationExecutionInput, ForgeQueryGraphObligationExecutionResultRow,
    ForgeQueryGraphObligationStateLoadCounters, ForgeQueryGraphObligationVerdict,
};

pub fn execute_advisory_obligation(
    input: ForgeQueryGraphObligationExecutionInput,
) -> ForgeQueryGraphObligationExecutionResultRow {
    ForgeQueryGraphObligationExecutionResultRow::executed(
        input,
        ForgeQueryGraphObligationVerdict::advise("advisory-obligation-selected")
            .expect("static advisory context is non-empty"),
        ForgeQueryGraphObligationStateLoadCounters::none(),
    )
}
