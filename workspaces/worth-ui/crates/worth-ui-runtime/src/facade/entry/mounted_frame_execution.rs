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
