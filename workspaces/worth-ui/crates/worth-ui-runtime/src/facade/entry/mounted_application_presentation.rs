use super::{
    mounted_publication::finish_mounted_transition, WorthUiActiveApplicationSession,
    WorthUiMountedFrameExecutionStop, WorthUiMountedFrameFrameworkTransitionStop,
};
use crate::mounting::UiMountedFrameOutcome;
use worth_ui_host_contract::UiPresentationDeadline;

type PendingHostMeasurements = (
    crate::facade::WorthUiHostMeasurementCapability,
    Vec<crate::facade::WorthUiHostMeasurementSessionInput>,
);

#[derive(Debug)]
pub enum UiMountedHostMeasurementTransitionDenial {
    AllocationReplanDenied(crate::runtime::UiAllocationReplanTransactionCommitDenial),
    ViewportResizeDenied(crate::runtime::UiViewportResizeDenial),
    AllocationReplanSelectionDenied(crate::graph::UiReplanLocalityDenial),
    AllocationFrameResolutionDenied(crate::runtime::UiAllocationFrameRejection),
    AllocationInvalidationNarrowingDenied(
        crate::runtime::UiAllocationInvalidationNarrowingRejection,
    ),
    FrameworkTransitionPlanningDenied(crate::runtime::UiFrameworkTransitionPlanningDenial),
    FrameworkTransitionExecutionDenied(crate::runtime::UiFrameworkTransitionExecutionDenial),
    DispatcherDenied {
        denial: crate::runtime::UiAllocationFrameDispatchDenial,
        counters: crate::runtime::UiAllocationFrameDispatcherCounters,
    },
    UnexpectedSuccessfulTransition(UiMountedHostMeasurementUnexpectedTransition),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedHostMeasurementUnexpectedTransition {
    ReadyToExecute,
    ResizePreviewPublished,
    DurableResizeCommitted,
    DragResizePreviewPending,
}

pub(crate) enum UiMountedHostMeasurementSettlementStop {
    PublicationLease(crate::mounting::UiMountedPublicationLeaseDenial),
    Evidence(crate::facade::host::UiHostMeasurementEvidenceDenial),
    Transition(UiMountedHostMeasurementTransitionDenial),
}

impl<'session> From<UiMountedHostMeasurementSettlementStop>
    for WorthUiMountedFrameExecutionStop<'session>
{
    fn from(stop: UiMountedHostMeasurementSettlementStop) -> Self {
        match stop {
            UiMountedHostMeasurementSettlementStop::PublicationLease(denial) => {
                Self::PublicationLease(denial)
            }
            UiMountedHostMeasurementSettlementStop::Evidence(denial) => {
                Self::HostMeasurement(denial)
            }
            UiMountedHostMeasurementSettlementStop::Transition(denial) => {
                Self::HostMeasurementTransition(denial)
            }
        }
    }
}

impl WorthUiActiveApplicationSession {
    pub(crate) fn settle_mounted_host_measurements(
        &mut self,
        host_measurements: Option<PendingHostMeasurements>,
    ) -> Result<(), UiMountedHostMeasurementSettlementStop> {
        let Some((capability, inputs)) = host_measurements else {
            return Ok(());
        };
        let mut evidence_denial = None;
        let completion = self
            .execute_framework_turn(|turn| {
                turn.host_measurement(|source| {
                    for input in inputs {
                        if let Err(observed) =
                            source.collect_and_submit_capability(&capability, input)
                        {
                            evidence_denial.get_or_insert(observed);
                        }
                    }
                });
            })
            .map_err(UiMountedHostMeasurementSettlementStop::PublicationLease)?;
        if let Some(denial) = evidence_denial {
            drop(completion);
            return Err(UiMountedHostMeasurementSettlementStop::Evidence(denial));
        }
        require_committed_host_measurement_transition(completion)
            .map_err(UiMountedHostMeasurementSettlementStop::Transition)
    }

    pub(crate) fn execute_mounted_frame_with_application_presentation(
        &mut self,
        deadline: UiPresentationDeadline,
        now: u64,
    ) -> Result<UiMountedFrameOutcome, WorthUiMountedFrameExecutionStop<'_>> {
        let request = self.mounted_frame_request();
        let projection = self
            .presentation
            .project()
            .map_err(WorthUiMountedFrameExecutionStop::Preparation)?;
        let completion = self
            .execute_framework_turn(|_| {})
            .map_err(WorthUiMountedFrameExecutionStop::PublicationLease)?;
        let execution = completion.into_execution().map_err(|completion| {
            WorthUiMountedFrameExecutionStop::FrameworkTransition(
                WorthUiMountedFrameFrameworkTransitionStop { completion },
            )
        })?;
        let frame = execution
            .prepare_mounted_frame_with_content_internal(
                request,
                projection.content(),
                projection.theme_values(),
            )
            .map_err(WorthUiMountedFrameExecutionStop::Preparation)?;
        execution.presentation.commit(&projection);
        let transition =
            execution
                .mounted
                .present_prepared_frame(execution.host_session, frame, deadline, now);
        Ok(finish_mounted_transition(
            execution.mounted,
            execution.focus,
            execution.portal,
            execution.interaction,
            execution.host_session,
            execution.application_session_identity,
            &execution.generation_identity,
            execution.host_exchange,
            transition,
        ))
    }

    pub(crate) fn execute_mounted_rebound_frame_with_application_presentation(
        &mut self,
        replacements: &[crate::mounting::UiMountedSurfaceReconciliationBinding],
        deadline: UiPresentationDeadline,
        now: u64,
    ) -> Result<UiMountedFrameOutcome, WorthUiMountedFrameExecutionStop<'_>> {
        let request = self.mounted_frame_request();
        let projection = self
            .presentation
            .project()
            .map_err(WorthUiMountedFrameExecutionStop::Preparation)?;
        let completion = self
            .execute_framework_turn(|_| {})
            .map_err(WorthUiMountedFrameExecutionStop::PublicationLease)?;
        let execution = completion.into_execution().map_err(|completion| {
            WorthUiMountedFrameExecutionStop::FrameworkTransition(
                WorthUiMountedFrameFrameworkTransitionStop { completion },
            )
        })?;
        let frame = execution
            .prepare_mounted_reconciliation_frame_with_content_internal(
                request,
                projection.content(),
                projection.theme_values(),
                replacements,
            )
            .map_err(WorthUiMountedFrameExecutionStop::Preparation)?;
        execution.presentation.commit(&projection);
        let transition =
            execution
                .mounted
                .present_prepared_frame(execution.host_session, frame, deadline, now);
        Ok(finish_mounted_transition(
            execution.mounted,
            execution.focus,
            execution.portal,
            execution.interaction,
            execution.host_session,
            execution.application_session_identity,
            &execution.generation_identity,
            execution.host_exchange,
            transition,
        ))
    }
}

fn require_committed_host_measurement_transition(
    completion: super::WorthUiActiveFrameworkTurnCompletion<'_>,
) -> Result<(), UiMountedHostMeasurementTransitionDenial> {
    use crate::runtime::WorthUiFrameworkTurnCompletion as Completion;
    use UiMountedHostMeasurementTransitionDenial as Denial;
    use UiMountedHostMeasurementUnexpectedTransition as Unexpected;

    match completion.into_completion() {
        Completion::AllocationInvalidationsNarrowed {
            transaction:
                crate::runtime::UiAllocationReplanTransactionOutcome::Committed(_)
                | crate::runtime::UiAllocationReplanTransactionOutcome::Replayed(_),
            ..
        }
        | Completion::ViewportResizeResolved { .. } => Ok(()),
        Completion::AllocationInvalidationsNarrowed {
            transaction: crate::runtime::UiAllocationReplanTransactionOutcome::Denied(denial),
            ..
        } => Err(Denial::AllocationReplanDenied(denial)),
        Completion::ViewportResizeDenied { denial, .. } => {
            Err(Denial::ViewportResizeDenied(denial))
        }
        Completion::AllocationReplanSelectionDenied { denial } => {
            Err(Denial::AllocationReplanSelectionDenied(denial))
        }
        Completion::AllocationFrameResolutionDenied { rejection } => {
            Err(Denial::AllocationFrameResolutionDenied(rejection))
        }
        Completion::AllocationInvalidationNarrowingDenied { rejection } => {
            Err(Denial::AllocationInvalidationNarrowingDenied(rejection))
        }
        Completion::FrameworkTransitionPlanningDenied { denial } => {
            Err(Denial::FrameworkTransitionPlanningDenied(denial))
        }
        Completion::FrameworkTransitionExecutionDenied { denial } => {
            Err(Denial::FrameworkTransitionExecutionDenied(denial))
        }
        Completion::Denied { denial, counters } => {
            Err(Denial::DispatcherDenied { denial, counters })
        }
        Completion::ReadyToExecute { .. } => Err(Denial::UnexpectedSuccessfulTransition(
            Unexpected::ReadyToExecute,
        )),
        Completion::ResizePreviewPublished { .. } => Err(Denial::UnexpectedSuccessfulTransition(
            Unexpected::ResizePreviewPublished,
        )),
        Completion::DurableResizeCommitted { .. } => Err(Denial::UnexpectedSuccessfulTransition(
            Unexpected::DurableResizeCommitted,
        )),
        Completion::DragResizePreviewPending { .. } => Err(Denial::UnexpectedSuccessfulTransition(
            Unexpected::DragResizePreviewPending,
        )),
    }
}
