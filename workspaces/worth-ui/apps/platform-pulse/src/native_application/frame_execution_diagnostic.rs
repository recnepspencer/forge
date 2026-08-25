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
        UiMountedFrameOutcome::Superseded(_) => "superseded".to_owned(),
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
        worth_ui::facade::app::WorthUiMountedFrameExecutionStop::HostMeasurement(denial) => {
            format!("host-measurement:{denial:?}")
        }
        worth_ui::facade::app::WorthUiMountedFrameExecutionStop::HostMeasurementTransition(
            denial,
        ) => host_measurement_transition_label(denial).to_owned(),
        worth_ui::facade::app::WorthUiMountedFrameExecutionStop::FrameworkTransition(_) => {
            "framework-transition".to_owned()
        }
        worth_ui::facade::app::WorthUiMountedFrameExecutionStop::Preparation(denial) => {
            format!("preparation:{denial:?}")
        }
    }
}

fn host_measurement_transition_label(
    denial: &worth_ui::facade::app::UiMountedHostMeasurementTransitionDenial,
) -> &'static str {
    use worth_ui::facade::app::UiMountedHostMeasurementTransitionDenial as Denial;
    use worth_ui::facade::app::UiMountedHostMeasurementUnexpectedTransition as Unexpected;

    match denial {
        Denial::AllocationReplanDenied(_) => "host-measurement:allocation-replan-denied",
        Denial::ViewportResizeDenied(_) => "host-measurement:viewport-resize-denied",
        Denial::AllocationReplanSelectionDenied(_) => {
            "host-measurement:allocation-replan-selection-denied"
        }
        Denial::AllocationFrameResolutionDenied(_) => {
            "host-measurement:allocation-frame-resolution-denied"
        }
        Denial::AllocationInvalidationNarrowingDenied(_) => {
            "host-measurement:allocation-invalidation-narrowing-denied"
        }
        Denial::FrameworkTransitionPlanningDenied(_) => {
            "host-measurement:framework-transition-planning-denied"
        }
        Denial::FrameworkTransitionExecutionDenied(_) => {
            "host-measurement:framework-transition-execution-denied"
        }
        Denial::DispatcherDenied { .. } => "host-measurement:dispatcher-denied",
        Denial::UnexpectedSuccessfulTransition(transition) => match transition {
            Unexpected::ReadyToExecute => "host-measurement:unexpected-ready-to-execute",
            Unexpected::ResizePreviewPublished => {
                "host-measurement:unexpected-resize-preview-published"
            }
            Unexpected::DurableResizeCommitted => {
                "host-measurement:unexpected-durable-resize-committed"
            }
            Unexpected::DragResizePreviewPending => {
                "host-measurement:unexpected-drag-resize-preview-pending"
            }
        },
    }
}
