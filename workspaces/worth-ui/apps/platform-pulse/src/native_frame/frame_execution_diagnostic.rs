use worth_ui::facade::app::UiMountedFrameOutcome;

pub(super) fn outcome_label(outcome: &UiMountedFrameOutcome) -> String {
    match outcome {
        UiMountedFrameOutcome::Published(_) => "published".to_owned(),
        UiMountedFrameOutcome::Unchanged(_) => "unchanged".to_owned(),
        UiMountedFrameOutcome::Reconciled(_) => "reconciled".to_owned(),
        UiMountedFrameOutcome::RejectedBeforeEffects(_) => "rejected-before-effects".to_owned(),
        UiMountedFrameOutcome::InFlight(_) => "in-flight".to_owned(),
        UiMountedFrameOutcome::PresentationIndeterminate(_) => {
            "presentation-indeterminate".to_owned()
        }
        UiMountedFrameOutcome::RetentionDenied(_) => "retention-denied".to_owned(),
        UiMountedFrameOutcome::AdmissionDenied(_) => "admission-denied".to_owned(),
        UiMountedFrameOutcome::CompletionDenied(_) => "completion-denied".to_owned(),
    }
}

pub(super) fn stop_label(
    stop: &worth_ui::facade::app::WorthUiMountedFrameExecutionStop<'_>,
) -> String {
    match stop {
        worth_ui::facade::app::WorthUiMountedFrameExecutionStop::PublicationLease(_) => {
            "publication-lease".to_owned()
        }
        worth_ui::facade::app::WorthUiMountedFrameExecutionStop::FrameworkTransition(_) => {
            "framework-transition".to_owned()
        }
        worth_ui::facade::app::WorthUiMountedFrameExecutionStop::Preparation(denial) => {
            format!("preparation:{denial:?}")
        }
    }
}
