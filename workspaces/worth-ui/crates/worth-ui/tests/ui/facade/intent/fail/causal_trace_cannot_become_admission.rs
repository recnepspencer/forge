use worth_ui::facade::inspection::UiIntentCausalTraceEvidence;

fn substitute_reporting_for_authority<I: worth_ui::facade::intent::UiIntent>(
    trace: UiIntentCausalTraceEvidence,
) -> worth_ui::facade::intent::UiAdmittedIntent<I> {
    trace
}
