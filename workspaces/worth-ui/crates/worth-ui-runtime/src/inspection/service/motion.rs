pub(crate) fn why_motion_interrupted(
    owner: Option<&crate::runtime::motion::UiMotionRuntimeState>,
) -> Option<worth_ui_inspection::UiMotionInterruptedInspectionSummary> {
    let fact = owner?.last_interruption()?;
    let crate::runtime::motion::UiMotionProducedFactKind::Retargeted(disposition) = fact.kind()
    else {
        return None;
    };
    let reason = match disposition {
        crate::runtime::motion::UiMotionRetargetDisposition::Install { predecessor: crate::runtime::motion::UiMotionRetargetPredecessor::CurrentPresentationSample } => worth_ui_inspection::UiMotionInterruptedInspectionReason::RetargetedFromCurrentPresentation,
        crate::runtime::motion::UiMotionRetargetDisposition::Install { predecessor: crate::runtime::motion::UiMotionRetargetPredecessor::CommittedSemanticPredecessor } => worth_ui_inspection::UiMotionInterruptedInspectionReason::RestartedFromSemanticPredecessor,
        crate::runtime::motion::UiMotionRetargetDisposition::FinishThenApply => worth_ui_inspection::UiMotionInterruptedInspectionReason::FinishThenApply,
        crate::runtime::motion::UiMotionRetargetDisposition::SnapToTarget => worth_ui_inspection::UiMotionInterruptedInspectionReason::SnappedToTarget,
        crate::runtime::motion::UiMotionRetargetDisposition::CancelDrop => worth_ui_inspection::UiMotionInterruptedInspectionReason::CancelledAndDropped,
    };
    Some(
        worth_ui_inspection::UiMotionInterruptedInspectionSummary::new(
            worth_ui_inspection::UiRuntimeServiceInspectionSource::new(
                worth_ui_inspection::UiRuntimeServiceInspectionFamily::Motion,
                Some(fact.track().diagnostic_value()),
                fact.publication_sequence(),
            ),
            reason,
            fact.successor_revision(),
            worth_ui_inspection::UiRuntimeServiceInspectionCost::latest_record(1, 1),
        ),
    )
}
