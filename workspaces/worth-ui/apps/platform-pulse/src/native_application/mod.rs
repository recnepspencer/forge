use worth_ui::facade::app::{UiMountedFrameOutcome, WorthUiNativeApplicationShell};
use worth_ui::facade::source::WorthUiSourcePackageRevision;

use crate::application::PlatformPulsePreparationDenial;
use crate::lifecycle_observation_publication::{
    PlatformPulseObservationPublicationDenial, PlatformPulseObservationPublisher,
};
use crate::source_watch::PlatformPulseSourceWatch;
use crate::visual_identity_execution::{
    PlatformPulseContentMutationReadiness, PlatformPulseVisualExecutionDenial,
    PlatformPulseVisualIdentityExecution,
};

mod composition;
mod first_frame;
mod frame_execution_diagnostic;
mod input;
mod intent;
mod lifecycle;
mod projection;
mod query;
mod readiness;
mod rebind;
mod source_rebind;
mod terminal_error;

pub(crate) use composition::PlatformPulseApplication;

use projection::PlatformPulseProjectionRebindDenial;
use terminal_error::PlatformPulseTerminalError;

enum PlatformPulsePendingManagedRebind {
    Projection(query::PlatformPulsePendingProjection),
    Source(WorthUiSourcePackageRevision),
    IntentPosture(intent::PlatformPulsePendingIntentPosture),
    IntentConsequence(intent::PlatformPulsePendingIntentConsequence),
}

pub(crate) struct PlatformPulseApplicationRuntime {
    initial_source: Option<WorthUiSourcePackageRevision>,
    shell: Option<WorthUiNativeApplicationShell>,
    source_watch: Option<PlatformPulseSourceWatch>,
    query_watch: Option<crate::query_source::PlatformPulseExternalValueWatch>,
    query_lifecycle: Option<crate::query_source::PlatformPulseQueryLifecycle>,
    intent_watch: Option<worth_ui_platform_pulse::intent::PlatformPulseIntentInputWatch>,
    intent_gate: Option<worth_ui_platform_pulse::intent::PlatformPulseExecutorGate>,
    intent_action_owner: Option<worth_ui_platform_pulse::intent::PlatformPulseActionPortOwner>,
    pending_query_actions: Vec<PlatformPulsePendingQueryAction>,
    pending_managed_rebind: Option<PlatformPulsePendingManagedRebind>,
    pending_intent_postures: std::collections::VecDeque<intent::PlatformPulsePreparedIntentPosture>,
    pending_intent_execution_transitions:
        std::collections::VecDeque<worth_ui::facade::intent::UiIntentExecutionTransition>,
    intent_evidence_index: intent::PlatformPulseIntentEvidenceIndex,
    native_input: input::PlatformPulseNativeInputIngress,
    publisher: PlatformPulseObservationPublisher,
    terminal_error: Option<PlatformPulseTerminalError>,
    observation_error: Option<PlatformPulseObservationPublicationDenial>,
    terminal_reported: bool,
    visual_identity: PlatformPulseVisualIdentityExecution,
    intent_clock: intent::PlatformPulseIntentClock,
    presentation_tick: u64,
}

struct PlatformPulsePendingQueryAction {
    reference: worth_ui_platform_pulse::intent::PlatformPulseActionAttemptReference,
    evidence_reference: worth_ui::facade::inspection::UiIntentEvidenceReference,
    projection: worth_ui_platform_pulse::observation_contract::PlatformPulseQueryProjectionEvidence,
}

impl PlatformPulseApplicationRuntime {
    fn present(&mut self) {
        // The permanent pulse has no animation-driven ordinary frame loop.
        // Once the first publication exists, explicit overlay and replacement
        // transitions own every later presentation attempt. Re-presenting here
        // could supersede an exact in-flight visual capture between polls.
        if self.initial_source.is_none() {
            return;
        }
        let Some(shell) = self.shell.as_mut() else {
            return;
        };
        self.presentation_tick = self.presentation_tick.saturating_add(1);
        let deadline = self.presentation_tick.saturating_add(1);
        match shell.present_frame(deadline, self.presentation_tick) {
            Ok(
                UiMountedFrameOutcome::Published(publication)
                | UiMountedFrameOutcome::Reconciled(publication),
            ) => {
                let Some(source) = self.initial_source.take() else {
                    return;
                };
                if let Err(error) = self.publish_first_frame(&source, &publication) {
                    self.fail(
                        PlatformPulseTerminalError::ObservationPublication,
                        Err(error),
                    );
                    return;
                }
                if let Err(denial) = self
                    .visual_identity
                    .arm_after_first_frame(std::time::Instant::now())
                {
                    self.fail_visual_identity(denial);
                }
            }
            Ok(UiMountedFrameOutcome::Unchanged(_)) if self.initial_source.is_none() => {}
            Ok(UiMountedFrameOutcome::Unchanged(_)) => {
                self.fail(PlatformPulseTerminalError::UnexpectedInitialFrame, Ok(()));
            }
            Ok(outcome) => {
                let observation = self.publisher.frame_outcome_failure(&outcome);
                self.fail(
                    PlatformPulseTerminalError::FrameExecution(
                        frame_execution_diagnostic::outcome_label(&outcome),
                    ),
                    observation,
                );
            }
            Err(denial) => {
                let detail = frame_execution_diagnostic::stop_label(&denial);
                let observation = self.publisher.frame_execution_failure(&denial);
                drop(denial);
                self.fail(
                    PlatformPulseTerminalError::FrameExecution(detail),
                    observation,
                );
            }
        }
    }

    fn fail(
        &mut self,
        error: PlatformPulseTerminalError,
        observation: Result<(), PlatformPulseObservationPublicationDenial>,
    ) {
        self.terminal_error = Some(error);
        self.observation_error = observation.err();
    }

    fn advance_visual_identity(&mut self) {
        let Some(shell) = self.shell.as_mut() else {
            return;
        };
        let result = self.visual_identity.advance(
            shell,
            &self.publisher,
            &mut self.presentation_tick,
            std::time::Instant::now(),
        );
        if let Err(denial) = result {
            self.fail_visual_identity(denial);
        }
    }

    fn prepare_content_mutations(&mut self) -> bool {
        if self.visual_identity.content_mutation_readiness()
            == PlatformPulseContentMutationReadiness::DeferredForVisualComparison
        {
            self.advance_visual_identity();
        }
        self.terminal_error.is_none()
            && self.visual_identity.content_mutation_readiness()
                == PlatformPulseContentMutationReadiness::Ready
    }

    fn fail_visual_identity(&mut self, denial: PlatformPulseVisualExecutionDenial) {
        let observation = self.publisher.visual_identity_failure();
        self.fail(
            PlatformPulseTerminalError::VisualIdentity(denial),
            observation,
        );
    }

    pub(crate) fn report_terminal_error(&mut self) {
        if self.terminal_reported {
            return;
        }
        if let Some(error) = &self.terminal_error {
            eprintln!("WORTH UI platform pulse stopped: {error}");
        }
        if let Some(error) = self.observation_error {
            eprintln!("WORTH UI platform pulse could not publish terminal evidence: {error:?}");
        }
        self.terminal_reported = true;
    }
}

impl Drop for PlatformPulseApplicationRuntime {
    fn drop(&mut self) {
        let _ = self.shutdown_product();
    }
}

pub(crate) fn publish_preparation_failure(
    publisher: &PlatformPulseObservationPublisher,
    denial: &PlatformPulsePreparationDenial,
) -> Result<(), PlatformPulseObservationPublicationDenial> {
    match denial {
        PlatformPulsePreparationDenial::WatcherStart(denial)
        | PlatformPulsePreparationDenial::InitialSourceSettlement(denial) => {
            publisher.filesystem_watcher_failure(denial)
        }
        PlatformPulsePreparationDenial::CapabilityApplication(denial) => {
            publisher.application_preparation_failure(denial)
        }
        PlatformPulsePreparationDenial::InitialSourceLowering(denial) => {
            publisher.candidate_submission_failure(denial)
        }
        PlatformPulsePreparationDenial::QueryInstallation(_)
        | PlatformPulsePreparationDenial::QueryRegistration(_)
        | PlatformPulsePreparationDenial::QueryViewRegistration(_) => {
            publisher.query_preparation_failure()
        }
        PlatformPulsePreparationDenial::IntentInput(_)
        | PlatformPulsePreparationDenial::IntentFact(_)
        | PlatformPulsePreparationDenial::IntentDefinition(_)
        | PlatformPulsePreparationDenial::IntentProvider(_) => {
            publisher.intent_preparation_failure()
        }
    }
}
