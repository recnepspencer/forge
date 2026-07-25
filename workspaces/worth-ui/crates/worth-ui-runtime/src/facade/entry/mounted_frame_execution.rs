use crate::mounting::{
    UiMountedFrameOutcome, UiMountedFramePreparationDenial, UiMountedFrameRequest,
    UiMountedFrameReuse, UiMountedPublicationLeaseDenial,
};
use crate::runtime::WorthUiFrameworkTurn;
use worth_ui_host_contract::UiPresentationDeadline;

use super::{
    mounted_publication::WorthUiMountedPublicationAuthority, WorthUiActiveApplicationSession,
    WorthUiActiveFrameworkTurnCompletion, WorthUiActiveFrameworkTurnExecution,
};

/// A typed stop before one ordinary mounted-frame request can publish.
pub enum WorthUiMountedFrameExecutionStop<'session> {
    PublicationLease(UiMountedPublicationLeaseDenial),
    FrameworkTransition(Box<WorthUiActiveFrameworkTurnCompletion<'session>>),
    Preparation(UiMountedFramePreparationDenial),
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
}

impl<'session> WorthUiActiveFrameworkTurnCompletion<'session> {
    fn execute_mounted_frame(
        self,
        request: UiMountedFrameRequest,
        deadline: UiPresentationDeadline,
        now: u64,
    ) -> Result<UiMountedFrameOutcome, WorthUiMountedFrameExecutionStop<'session>> {
        let Self {
            generation_identity,
            graph,
            active_plan_digest,
            host_session_identity,
            completion,
            mounted_identity,
            mounted_retention,
            host_session,
            mounted_presentation,
            mounted_publication_reservations,
            host_observations,
        } = self;
        let runtime_execution = match completion.into_execution() {
            Ok(execution) => execution,
            Err(completion) => {
                return Err(WorthUiMountedFrameExecutionStop::FrameworkTransition(
                    Box::new(WorthUiActiveFrameworkTurnCompletion {
                        generation_identity,
                        graph,
                        active_plan_digest,
                        host_session_identity,
                        completion: *completion,
                        mounted_identity,
                        mounted_retention,
                        host_session,
                        mounted_presentation,
                        mounted_publication_reservations,
                        host_observations,
                    }),
                ));
            }
        };
        let capability_report = host_session.capability_report();
        let execution = WorthUiActiveFrameworkTurnExecution {
            generation_identity,
            graph,
            host_session_identity,
            execution: runtime_execution,
            mounted_identity: &mut *mounted_identity,
            host_protocol: host_session.protocol(),
            host_capability_generation: capability_report.observation_generation(),
            host_capability_profile_digest: capability_report.profile_identity_digest(),
        };

        match execution.classify_mounted_frame_reuse_internal(&request) {
            UiMountedFrameReuse::Exact(witness) => {
                let receipt = witness.publication().clone();
                drop(execution);
                Ok(UiMountedFrameOutcome::Unchanged(receipt))
            }
            UiMountedFrameReuse::ComparisonRequired(_) => {
                let frame = execution
                    .prepare_mounted_frame_internal(request)
                    .map_err(WorthUiMountedFrameExecutionStop::Preparation)?;
                Ok(WorthUiMountedPublicationAuthority {
                    mounted_identity,
                    mounted_retention,
                    host_session,
                    mounted_presentation,
                    mounted_publication_reservations,
                    host_observations,
                }
                .present(frame, deadline, now))
            }
        }
    }
}
