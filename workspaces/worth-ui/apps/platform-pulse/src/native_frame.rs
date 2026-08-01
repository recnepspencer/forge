use eframe::egui;
use worth_ui::facade::app::{UiMountedFrameOutcome, WorthUiApp, WorthUiNativeApplicationShell};
use worth_ui::facade::source::WorthUiSourcePackageRevision;
use worth_ui_host_egui::WorthUiHostEgui;

use crate::application::{PlatformPulsePreparationDenial, PreparedPlatformPulse};
use crate::launch_configuration::AdmittedPlatformPulseLaunchConfiguration;
use crate::lifecycle_observation_publication::{
    PlatformPulseObservationPublicationDenial, PlatformPulseObservationPublisher,
};
use crate::source_watch::{PlatformPulseSourceWatch, PlatformPulseSourceWatchShutdownDenial};
use crate::visual_identity_execution::{
    PlatformPulseContentMutationReadiness, PlatformPulseVisualExecutionDenial,
    PlatformPulseVisualIdentityExecution,
};

mod first_frame;
mod frame_execution_diagnostic;
mod input;
#[cfg(test)]
mod input_reachability_tests;
mod intent;
mod projection;
mod query;
mod rebind;
mod source_rebind;
mod terminal_error;

use projection::PlatformPulseProjectionRebindDenial;
use terminal_error::PlatformPulseTerminalError;

pub(crate) struct PlatformPulseNativeFrame {
    prepared: Option<WorthUiApp>,
    initial_source: Option<WorthUiSourcePackageRevision>,
    shell: Option<WorthUiNativeApplicationShell>,
    source_watch: Option<PlatformPulseSourceWatch>,
    query_watch: Option<crate::query_source::PlatformPulseExternalValueWatch>,
    query_lifecycle: Option<crate::query_source::PlatformPulseQueryLifecycle>,
    intent_watch: Option<worth_ui_platform_pulse::intent::PlatformPulseIntentInputWatch>,
    intent_gate: Option<worth_ui_platform_pulse::intent::PlatformPulseExecutorGate>,
    intent_action_owner: Option<worth_ui_platform_pulse::intent::PlatformPulseActionPortOwner>,
    pending_query_actions: Vec<PlatformPulsePendingQueryAction>,
    intent_evidence_index: intent::PlatformPulseIntentEvidenceIndex,
    host: Option<WorthUiHostEgui>,
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

impl PlatformPulseNativeFrame {
    pub(crate) fn new(
        creation: &eframe::CreationContext<'_>,
        launch: AdmittedPlatformPulseLaunchConfiguration,
        publisher: PlatformPulseObservationPublisher,
    ) -> Self {
        match crate::application::prepare(creation.egui_ctx.clone(), &launch) {
            Ok(prepared) => Self::from_prepared(prepared, publisher),
            Err(denial) => {
                let observation_error = publish_preparation_failure(&publisher, &denial).err();
                Self {
                    prepared: None,
                    initial_source: None,
                    shell: None,
                    source_watch: None,
                    query_watch: None,
                    query_lifecycle: None,
                    intent_watch: None,
                    intent_gate: None,
                    intent_action_owner: None,
                    pending_query_actions: Vec::new(),
                    intent_evidence_index: intent::PlatformPulseIntentEvidenceIndex::new(),
                    host: None,
                    native_input: input::PlatformPulseNativeInputIngress::default(),
                    publisher,
                    terminal_error: Some(PlatformPulseTerminalError::Preparation(Box::new(denial))),
                    observation_error,
                    terminal_reported: false,
                    visual_identity: PlatformPulseVisualIdentityExecution::new(),
                    intent_clock: intent::PlatformPulseIntentClock::new(),
                    presentation_tick: 0,
                }
            }
        }
    }

    fn from_prepared(
        prepared: PreparedPlatformPulse,
        publisher: PlatformPulseObservationPublisher,
    ) -> Self {
        Self {
            prepared: Some(prepared.app),
            initial_source: Some(prepared.initial_source),
            shell: None,
            source_watch: Some(PlatformPulseSourceWatch::spawn(prepared.watcher)),
            query_watch: Some(prepared.query_watcher),
            query_lifecycle: Some(prepared.query_lifecycle),
            intent_watch: Some(prepared.intent_watcher),
            intent_gate: Some(prepared.intent_gate),
            intent_action_owner: Some(prepared.intent_action_owner),
            pending_query_actions: Vec::new(),
            intent_evidence_index: intent::PlatformPulseIntentEvidenceIndex::new(),
            host: Some(prepared.host),
            native_input: input::PlatformPulseNativeInputIngress::default(),
            publisher,
            terminal_error: None,
            observation_error: None,
            terminal_reported: false,
            visual_identity: PlatformPulseVisualIdentityExecution::new(),
            intent_clock: intent::PlatformPulseIntentClock::new(),
            presentation_tick: 0,
        }
    }

    fn ensure_launched(&mut self) {
        let Some(prepared) = self.prepared.take() else {
            return;
        };
        match prepared.launch_native_surface() {
            Ok(shell) => {
                self.shell = Some(shell);
                self.publish_initial_projection();
            }
            Err(denial) => {
                let observation = self.publisher.native_surface_launch_failure(&denial);
                self.fail(
                    PlatformPulseTerminalError::NativeSurfaceLaunch(denial),
                    observation,
                );
            }
        }
    }

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

    fn report_terminal(&mut self, context: &egui::Context) {
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
        context.send_viewport_cmd(egui::ViewportCommand::Close);
    }
}

impl eframe::App for PlatformPulseNativeFrame {
    fn raw_input_hook(&mut self, _context: &egui::Context, raw_input: &mut egui::RawInput) {
        let result = self
            .native_input
            .observe(self.host.as_ref(), raw_input, &self.publisher);
        if let Err(denial) = result {
            match denial {
                input::PlatformPulseNativeInputIngressDenial::Adapter {
                    reason,
                    publication,
                } => self.fail(PlatformPulseTerminalError::NativeInput(reason), publication),
                input::PlatformPulseNativeInputIngressDenial::Publication(error) => self.fail(
                    PlatformPulseTerminalError::ObservationPublication,
                    Err(error),
                ),
            }
        } else if self.visual_identity.content_mutation_readiness()
            == PlatformPulseContentMutationReadiness::Ready
        {
            self.admit_native_intent_input();
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let context = ui.ctx();
        if let Some(host) = &self.host {
            host.repaint_retained_surfaces();
        }
        if self.terminal_error.is_none() {
            self.ensure_launched();
            if self.prepare_content_mutations() {
                self.poll_query();
                self.poll_intent_input();
                self.advance_intent_execution();
                self.poll_intent_action_port();
                self.advance_intent_execution();
                self.poll_source();
                self.present();
                self.advance_visual_identity();
            }
        }
        if self.terminal_error.is_some() {
            self.report_terminal(context);
        } else {
            context.request_repaint();
        }
    }
}

impl Drop for PlatformPulseNativeFrame {
    fn drop(&mut self) {
        let visual_shutdown = self
            .shell
            .as_mut()
            .map(|shell| self.visual_identity.shutdown_quiescent(shell));
        if let Some(Err(denial)) = visual_shutdown {
            self.fail_visual_identity(denial);
        }
        let watcher = self
            .source_watch
            .take()
            .map(PlatformPulseSourceWatch::shutdown);
        let query_watcher = self
            .query_watch
            .take()
            .map(crate::query_source::PlatformPulseExternalValueWatch::shutdown);
        let query = self
            .query_lifecycle
            .take()
            .map(crate::query_source::PlatformPulseQueryLifecycle::close);
        let intent_watcher = self
            .intent_watch
            .take()
            .map(worth_ui_platform_pulse::intent::PlatformPulseIntentInputWatch::shutdown);
        self.intent_gate.take();
        self.intent_action_owner.take();
        let application = self
            .shell
            .take()
            .map(WorthUiNativeApplicationShell::shutdown);
        if self.terminal_error.is_some() {
            return;
        }
        let publication = match (watcher, application, query, query_watcher, intent_watcher) {
            (
                Some(Ok(watcher)),
                Some(application),
                Some(Ok(query)),
                Some(Ok(query_watcher)),
                Some(Ok(intent_watcher)),
            ) => {
                self.publisher
                    .shutdown(&watcher, query, query_watcher, intent_watcher, application)
            }
            (Some(Err(PlatformPulseSourceWatchShutdownDenial::Watcher(denial))), _, _, _, _) => {
                self.publisher.filesystem_watcher_failure(&denial)
            }
            (Some(Err(PlatformPulseSourceWatchShutdownDenial::WorkerPanicked)), _, _, _, _) => {
                self.publisher.source_worker_panicked()
            }
            (_, _, Some(Err(_)), _, _) => self.publisher.query_shutdown_failure(),
            (_, _, _, _, Some(Err(_))) => self.publisher.intent_preparation_failure(),
            _ => return,
        };
        if let Err(error) = publication {
            eprintln!("WORTH UI platform pulse shutdown evidence failed: {error:?}");
        }
    }
}

fn publish_preparation_failure(
    publisher: &PlatformPulseObservationPublisher,
    denial: &PlatformPulsePreparationDenial,
) -> Result<(), PlatformPulseObservationPublicationDenial> {
    match denial {
        PlatformPulsePreparationDenial::WatcherStart(denial)
        | PlatformPulsePreparationDenial::InitialSourceSettlement(denial) => {
            publisher.filesystem_watcher_failure(denial)
        }
        PlatformPulsePreparationDenial::CapabilityApplication(denial)
        | PlatformPulsePreparationDenial::FileApplication(denial) => {
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
