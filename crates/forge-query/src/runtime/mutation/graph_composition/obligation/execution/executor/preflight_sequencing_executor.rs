use crate::runtime::{
    ForgeQueryGraphObligationExecutionInput, ForgeQueryGraphObligationExecutionResultRow,
    ForgeQueryGraphObligationExecutionStatus, ForgeQueryGraphObligationStateLoadCounters,
    ForgeQueryGraphObligationVerdict,
};

pub fn execute_preflight_sequencing_obligation(
    input: ForgeQueryGraphObligationExecutionInput,
) -> ForgeQueryGraphObligationExecutionResultRow {
    if input.execution_context().preflight_witness().is_satisfied() {
        return ForgeQueryGraphObligationExecutionResultRow::executed(
            input,
            ForgeQueryGraphObligationVerdict::allow_with_context(
                "preflight-prerequisite-satisfied",
            )
            .expect("static preflight context is non-empty"),
            ForgeQueryGraphObligationStateLoadCounters::none(),
        );
    }
    ForgeQueryGraphObligationExecutionResultRow::new(
        input,
        ForgeQueryGraphObligationExecutionStatus::BlockedByPrerequisite,
        Some(
            ForgeQueryGraphObligationVerdict::block("preflight-prerequisite-not-satisfied")
                .expect("static preflight context is non-empty"),
        ),
        ForgeQueryGraphObligationStateLoadCounters::none(),
    )
}
