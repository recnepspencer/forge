fn continue_from_diagnostic_admission(
    evidence: worth_ui::facade::inspection::UiIntentCausalTraceAdmissionEvidence,
) -> worth_ui::facade::intent::UiResolvedConfirmationIntentRoute {
    evidence
}

fn complete_from_diagnostic_outcome(
    evidence: worth_ui::facade::inspection::UiIntentCausalTraceCompletionEvidence,
) -> worth_ui::facade::intent::UiIntentConsequenceHandle {
    evidence
}
