use eframe::egui;
use std::fmt;
use worth_ui::facade::app::{
    UiMountedFrameOutcome, WorthUiApp, WorthUiNativeApplicationShell,
    WorthUiNativeApplicationShellLaunchDenial, WorthUiNativeSourceRebindDenial,
};
use worth_ui::facade::source::{WorthUiFilesystemWatcherDenial, WorthUiSourcePackageRevision};
use worth_ui_host_egui::WorthUiHostEgui;

use crate::application::{PlatformPulsePreparationDenial, PreparedPlatformPulse};
use crate::launch_configuration::AdmittedPlatformPulseLaunchConfiguration;
use crate::lifecycle_observation_publication::{
    PlatformPulseObservationPublicationDenial, PlatformPulseObservationPublisher,
};
use crate::source_watch::{PlatformPulseSourceWatch, PlatformPulseSourceWatchShutdownDenial};
use crate::visual_identity_execution::{
    PlatformPulseVisualExecutionDenial, PlatformPulseVisualIdentityExecution,
};

mod first_frame;
mod input;
#[cfg(test)]
mod input_reachability_tests;
mod projection;
mod query;
mod rebind;
mod source_rebind;

use projection::PlatformPulseProjectionRebindDenial;

pub(crate) struct PlatformPulseNativeFrame {
    prepared: Option<WorthUiApp>,
    initial_source: Option<WorthUiSourcePackageRevision>,
    shell: Option<WorthUiNativeApplicationShell>,
    source_watch: Option<PlatformPulseSourceWatch>,
    query_watch: Option<crate::query_source::PlatformPulseExternalValueWatch>,
    query_lifecycle: Option<crate::query_source::PlatformPulseQueryLifecycle>,
    host: Option<WorthUiHostEgui>,
    native_input: input::PlatformPulseNativeInputIngress,
    publisher: PlatformPulseObservationPublisher,
    terminal_error: Option<PlatformPulseTerminalError>,
    observation_error: Option<PlatformPulseObservationPublicationDenial>,
    terminal_reported: bool,
    visual_identity: PlatformPulseVisualIdentityExecution,
    tick: u64,
}

enum PlatformPulseTerminalError {
    Preparation(PlatformPulsePreparationDenial),
    NativeSurfaceLaunch(WorthUiNativeApplicationShellLaunchDenial),
    SourceWatcher(WorthUiFilesystemWatcherDenial),
    FrameExecution(String),
    UnexpectedInitialFrame,
    NativeRebind(WorthUiNativeSourceRebindDenial),
    NativeProjection(PlatformPulseProjectionRebindDenial),
    QueryLifecycle(crate::query_source::PlatformPulseQueryLifecycleDenial),
    QueryWatch(crate::query_source::PlatformPulseExternalValueWatchDenial),
    VisualIdentity(PlatformPulseVisualExecutionDenial),
    ObservationPublication,
}

impl fmt::Display for PlatformPulseTerminalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Preparation(denial) => {
                write!(formatter, "application preparation: {denial}")
            }
            Self::NativeSurfaceLaunch(denial) => {
                write!(formatter, "native surface launch: {denial:?}")
            }
            Self::SourceWatcher(denial) => write!(formatter, "source watcher: {denial:?}"),
            Self::FrameExecution(detail) => {
                write!(formatter, "mounted frame execution: {detail}")
            }
            Self::UnexpectedInitialFrame => formatter.write_str("initial frame did not publish"),
            Self::NativeRebind(denial) => {
                write!(formatter, "native source rebind: {denial:?}")
            }
            Self::NativeProjection(denial) => {
                write!(formatter, "native projection rebind: {denial}")
            }
            Self::QueryLifecycle(denial) => write!(formatter, "Query lifecycle: {denial}"),
            Self::QueryWatch(denial) => write!(formatter, "Query source watch: {denial}"),
            Self::VisualIdentity(denial) => {
                write!(formatter, "visual identity pulse: {denial}")
            }
            Self::ObservationPublication => {
                formatter.write_str("lifecycle observation publication")
            }
        }
    }
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
                    host: None,
                    native_input: input::PlatformPulseNativeInputIngress::default(),
                    publisher,
                    terminal_error: Some(PlatformPulseTerminalError::Preparation(denial)),
                    observation_error,
                    terminal_reported: false,
                    visual_identity: PlatformPulseVisualIdentityExecution::new(),
                    tick: 0,
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
            host: Some(prepared.host),
            native_input: input::PlatformPulseNativeInputIngress::default(),
            publisher,
            terminal_error: None,
            observation_error: None,
            terminal_reported: false,
            visual_identity: PlatformPulseVisualIdentityExecution::new(),
            tick: 0,
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
        self.tick = self.tick.saturating_add(1);
        let deadline = self.tick.saturating_add(1);
        match shell.present_frame(deadline, self.tick) {
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
                    PlatformPulseTerminalError::FrameExecution(frame_outcome_label(&outcome)),
                    observation,
                );
            }
            Err(denial) => {
                let detail = frame_stop_label(&denial);
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
            &mut self.tick,
            std::time::Instant::now(),
        );
        if let Err(denial) = result {
            self.fail_visual_identity(denial);
        }
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
        if let Err(error) = result {
            self.fail(
                PlatformPulseTerminalError::ObservationPublication,
                Err(error),
            );
        }
    }

    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(host) = &self.host {
            host.repaint_retained_surfaces();
        }
        if self.terminal_error.is_none() {
            self.ensure_launched();
            self.poll_query();
            self.poll_source();
            self.present();
            self.advance_visual_identity();
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
        let application = self
            .shell
            .take()
            .map(WorthUiNativeApplicationShell::shutdown);
        if self.terminal_error.is_some() {
            return;
        }
        let publication = match (watcher, application, query, query_watcher) {
            (Some(Ok(watcher)), Some(application), Some(Ok(query)), Some(Ok(query_watcher))) => {
                self.publisher
                    .shutdown(&watcher, query, query_watcher, application)
            }
            (Some(Err(PlatformPulseSourceWatchShutdownDenial::Watcher(denial))), _, _, _) => {
                self.publisher.filesystem_watcher_failure(&denial)
            }
            (Some(Err(PlatformPulseSourceWatchShutdownDenial::WorkerPanicked)), _, _, _) => {
                self.publisher.source_worker_panicked()
            }
            (_, _, Some(Err(_)), _) => self.publisher.query_shutdown_failure(),
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
        | PlatformPulsePreparationDenial::QueryRegistration(_) => {
            publisher.query_preparation_failure()
        }
    }
}

fn frame_outcome_label(outcome: &UiMountedFrameOutcome) -> String {
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

fn frame_stop_label(stop: &worth_ui::facade::app::WorthUiMountedFrameExecutionStop<'_>) -> String {
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
