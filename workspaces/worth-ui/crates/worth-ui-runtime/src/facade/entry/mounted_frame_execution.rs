use crate::mounting::{
    UiMountedFrameOutcome, UiMountedFramePreparationDenial, UiMountedFrameRequest,
    UiMountedFrameReuse, UiMountedPublicationLeaseDenial,
};
use crate::runtime::WorthUiFrameworkTurn;
use worth_ui_host_contract::UiPresentationDeadline;

use super::{
    mounted_publication::finish_mounted_transition, WorthUiActiveApplicationSession,
    WorthUiActiveFrameworkTurnCompletion,
};

/// A typed stop before one ordinary mounted-frame request can publish.
pub enum WorthUiMountedFrameExecutionStop<'session> {
    PublicationLease(UiMountedPublicationLeaseDenial),
    HostMeasurement(crate::facade::host::UiHostMeasurementEvidenceDenial),
    FrameworkTransition(WorthUiMountedFrameFrameworkTransitionStop<'session>),
    Preparation(UiMountedFramePreparationDenial),
}

/// Opaque framework-transition state retained by an ordinary mounted-frame stop.
///
/// The raw completion remains available to runtime internals, but ordinary callers
/// cannot recover lane-execution authority from a mounted-frame failure.
pub struct WorthUiMountedFrameFrameworkTransitionStop<'session> {
    completion: Box<WorthUiActiveFrameworkTurnCompletion<'session>>,
}

impl WorthUiMountedFrameFrameworkTransitionStop<'_> {
    pub fn generation_identity(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        self.completion.generation_identity()
    }
}

impl WorthUiActiveApplicationSession {
    /// Executes, assembles, presents, and publishes one ordinary mounted frame.
    pub fn execute_mounted_frame(
        &mut self,
        request: UiMountedFrameRequest,
        deadline: UiPresentationDeadline,
        now: u64,
        collect_sources: impl FnOnce(&mut WorthUiFrameworkTurn<'_>),
    ) -> Result<UiMountedFrameOutcome, WorthUiMountedFrameExecutionStop<'_>> {
        let completion = self
            .execute_framework_turn(collect_sources)
            .map_err(WorthUiMountedFrameExecutionStop::PublicationLease)?;
        completion.execute_mounted_frame(request, deadline, now)
    }

    pub(crate) fn execute_mounted_frame_with_content(
        &mut self,
        request: UiMountedFrameRequest,
        deadline: UiPresentationDeadline,
        now: u64,
        semantic_content: crate::mounting::UiMountedSemanticContentInput,
        collect_sources: impl FnOnce(&mut WorthUiFrameworkTurn<'_>),
    ) -> Result<UiMountedFrameOutcome, WorthUiMountedFrameExecutionStop<'_>> {
        let completion = self
            .execute_framework_turn(collect_sources)
            .map_err(WorthUiMountedFrameExecutionStop::PublicationLease)?;
        let execution = completion.into_execution().map_err(|completion| {
            WorthUiMountedFrameExecutionStop::FrameworkTransition(
                WorthUiMountedFrameFrameworkTransitionStop { completion },
            )
        })?;
        let frame = execution
            .prepare_mounted_frame_with_content_internal(request, semantic_content)
            .map_err(WorthUiMountedFrameExecutionStop::Preparation)?;
        let transition =
            execution
                .mounted
                .present_prepared_frame(execution.host_session, frame, deadline, now);
        Ok(finish_mounted_transition(
            execution.host_exchange,
            transition,
        ))
    }

    pub(crate) fn execute_mounted_frame_with_application_presentation(
        &mut self,
        request: UiMountedFrameRequest,
        deadline: UiPresentationDeadline,
        now: u64,
        host_measurement: Option<(
            crate::facade::WorthUiHostMeasurementCapability,
            crate::facade::WorthUiHostMeasurementSessionInput,
        )>,
        collect_sources: impl FnOnce(&mut WorthUiFrameworkTurn<'_>),
    ) -> Result<UiMountedFrameOutcome, WorthUiMountedFrameExecutionStop<'_>> {
        let projection = self
            .presentation
            .project()
            .map_err(WorthUiMountedFrameExecutionStop::Preparation)?;
        let mut host_measurement_denial = None;
        let completion = self
            .execute_framework_turn(|turn| {
                if let Some((capability, input)) = host_measurement {
                    turn.host_measurement(|source| {
                        if let Err(denial) =
                            source.collect_and_submit_capability(&capability, input)
                        {
                            host_measurement_denial = Some(denial);
                        }
                    });
                }
                collect_sources(turn);
            })
            .map_err(WorthUiMountedFrameExecutionStop::PublicationLease)?;
        if let Some(denial) = host_measurement_denial {
            return Err(WorthUiMountedFrameExecutionStop::HostMeasurement(denial));
        }
        let execution = completion.into_execution().map_err(|completion| {
            WorthUiMountedFrameExecutionStop::FrameworkTransition(
                WorthUiMountedFrameFrameworkTransitionStop { completion },
            )
        })?;
        let frame = execution
            .prepare_mounted_frame_with_content_internal(request, projection.content())
            .map_err(WorthUiMountedFrameExecutionStop::Preparation)?;
        execution.presentation.commit(&projection);
        let transition =
            execution
                .mounted
                .present_prepared_frame(execution.host_session, frame, deadline, now);
        Ok(finish_mounted_transition(
            execution.host_exchange,
            transition,
        ))
    }

    pub(crate) fn execute_mounted_rebound_frame_with_application_presentation(
        &mut self,
        request: UiMountedFrameRequest,
        replacements: &[crate::mounting::UiMountedSurfaceReconciliationBinding],
        deadline: UiPresentationDeadline,
        now: u64,
        collect_sources: impl FnOnce(&mut WorthUiFrameworkTurn<'_>),
    ) -> Result<UiMountedFrameOutcome, WorthUiMountedFrameExecutionStop<'_>> {
        let projection = self
            .presentation
            .project()
            .map_err(WorthUiMountedFrameExecutionStop::Preparation)?;
        let completion = self
            .execute_framework_turn(collect_sources)
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
                replacements,
            )
            .map_err(WorthUiMountedFrameExecutionStop::Preparation)?;
        execution.presentation.commit(&projection);
        let transition =
            execution
                .mounted
                .present_prepared_frame(execution.host_session, frame, deadline, now);
        Ok(finish_mounted_transition(
            execution.host_exchange,
            transition,
        ))
    }

    pub(crate) fn prepare_mounted_reconciliation_frame_with_content(
        &mut self,
        request: UiMountedFrameRequest,
        semantic_content: crate::mounting::UiMountedSemanticContentInput,
        replacements: &[crate::mounting::UiMountedSurfaceReconciliationBinding],
        collect_sources: impl FnOnce(&mut WorthUiFrameworkTurn<'_>),
    ) -> Result<crate::mounting::UiPreparedMountedFrame, WorthUiMountedFrameExecutionStop<'_>> {
        let completion = self
            .execute_framework_turn(collect_sources)
            .map_err(WorthUiMountedFrameExecutionStop::PublicationLease)?;
        let execution = completion.into_execution().map_err(|completion| {
            WorthUiMountedFrameExecutionStop::FrameworkTransition(
                WorthUiMountedFrameFrameworkTransitionStop { completion },
            )
        })?;
        execution
            .prepare_mounted_reconciliation_frame_with_content_internal(
                request,
                semantic_content,
                replacements,
            )
            .map_err(WorthUiMountedFrameExecutionStop::Preparation)
    }

    pub(crate) fn prepare_mounted_reconciliation_frame_with_application_presentation(
        &mut self,
        request: UiMountedFrameRequest,
        replacements: &[crate::mounting::UiMountedSurfaceReconciliationBinding],
        collect_sources: impl FnOnce(&mut WorthUiFrameworkTurn<'_>),
    ) -> Result<crate::mounting::UiPreparedMountedFrame, WorthUiMountedFrameExecutionStop<'_>> {
        let projection = self
            .presentation
            .project()
            .map_err(WorthUiMountedFrameExecutionStop::Preparation)?;
        let completion = self
            .execute_framework_turn(collect_sources)
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
                replacements,
            )
            .map_err(WorthUiMountedFrameExecutionStop::Preparation)?;
        execution.presentation.commit(&projection);
        Ok(frame)
    }

    pub(crate) fn prepare_mounted_reconstruction_frame_with_application_presentation(
        &mut self,
        request: UiMountedFrameRequest,
        replacements: &[crate::mounting::UiMountedSurfaceReconciliationBinding],
        collect_sources: impl FnOnce(&mut WorthUiFrameworkTurn<'_>),
    ) -> Result<crate::mounting::UiPreparedMountedFrame, WorthUiMountedFrameExecutionStop<'_>> {
        let projection = self
            .presentation
            .project_complete()
            .map_err(WorthUiMountedFrameExecutionStop::Preparation)?;
        let completion = self
            .execute_framework_turn(collect_sources)
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
                replacements,
            )
            .map_err(WorthUiMountedFrameExecutionStop::Preparation)?;
        execution.presentation.commit(&projection);
        Ok(frame)
    }

    pub(crate) fn prepare_mounted_frame_with_application_presentation(
        &mut self,
        request: UiMountedFrameRequest,
        collect_sources: impl FnOnce(&mut WorthUiFrameworkTurn<'_>),
    ) -> Result<crate::mounting::UiPreparedMountedFrame, WorthUiMountedFrameExecutionStop<'_>> {
        let projection = self
            .presentation
            .project()
            .map_err(WorthUiMountedFrameExecutionStop::Preparation)?;
        let completion = self
            .execute_framework_turn(collect_sources)
            .map_err(WorthUiMountedFrameExecutionStop::PublicationLease)?;
        let execution = completion.into_execution().map_err(|completion| {
            WorthUiMountedFrameExecutionStop::FrameworkTransition(
                WorthUiMountedFrameFrameworkTransitionStop { completion },
            )
        })?;
        let frame = execution
            .prepare_mounted_frame_with_content_internal(request, projection.content())
            .map_err(WorthUiMountedFrameExecutionStop::Preparation)?;
        execution.presentation.commit(&projection);
        Ok(frame)
    }

    pub(crate) fn prepare_mounted_superseding_frame_with_application_presentation(
        &mut self,
        request: UiMountedFrameRequest,
        predecessor: &crate::mounting::UiPreparedMountedFrame,
        collect_sources: impl FnOnce(&mut WorthUiFrameworkTurn<'_>),
    ) -> Result<crate::mounting::UiPreparedMountedFrame, WorthUiMountedFrameExecutionStop<'_>> {
        let projection = self
            .presentation
            .project()
            .map_err(WorthUiMountedFrameExecutionStop::Preparation)?;
        let completion = self
            .execute_framework_turn(collect_sources)
            .map_err(WorthUiMountedFrameExecutionStop::PublicationLease)?;
        let execution = completion.into_execution().map_err(|completion| {
            WorthUiMountedFrameExecutionStop::FrameworkTransition(
                WorthUiMountedFrameFrameworkTransitionStop { completion },
            )
        })?;
        let frame = execution
            .prepare_mounted_superseding_frame_with_content_internal(
                request,
                projection.content(),
                predecessor,
            )
            .map_err(WorthUiMountedFrameExecutionStop::Preparation)?;
        execution.presentation.commit(&projection);
        Ok(frame)
    }
}

impl<'session> WorthUiActiveFrameworkTurnCompletion<'session> {
    fn execute_mounted_frame(
        self,
        request: UiMountedFrameRequest,
        deadline: UiPresentationDeadline,
        now: u64,
    ) -> Result<UiMountedFrameOutcome, WorthUiMountedFrameExecutionStop<'session>> {
        let execution = self.into_execution().map_err(|completion| {
            WorthUiMountedFrameExecutionStop::FrameworkTransition(
                WorthUiMountedFrameFrameworkTransitionStop { completion },
            )
        })?;

        match execution.classify_mounted_frame_reuse_internal(&request) {
            UiMountedFrameReuse::Exact(witness) => Ok(UiMountedFrameOutcome::Unchanged(
                witness.publication().clone(),
            )),
            UiMountedFrameReuse::ComparisonRequired(_) => {
                let frame = execution
                    .prepare_mounted_frame_internal(request)
                    .map_err(WorthUiMountedFrameExecutionStop::Preparation)?;
                let transition = execution.mounted.present_prepared_frame(
                    execution.host_session,
                    frame,
                    deadline,
                    now,
                );
                Ok(finish_mounted_transition(
                    execution.host_exchange,
                    transition,
                ))
            }
        }
    }
}
