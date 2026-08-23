use std::time::Duration;

use worth_ui::facade::app::{
    UiMountedFrameOutcome, UiMountedFramePublicationReceipt, WorthUiNativeApplicationShell,
    WorthUiNativeSourceRebindDenial, WorthUiPreparedApplicationGenerationIdentity,
};
use worth_ui::facade::rebind::{UiRebindOutcome, UiRebindReceipt, UiSourceRebindRequest};
use worth_ui::facade::source::{
    WorthUiFilesystemSourceProvider, WorthUiFilesystemSourceWatcher, WorthUiSourcePackageRevision,
};
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_host_egui::WorthUiHostEgui;

use super::native_oracle::{assert_background_and_target, raw_input, BLUE, GREEN};
use super::observed_lifecycle::ObservedPulseLifecycle;
use crate::filesystem_contract_workspace::FilesystemContractWorkspace;

const SETTLEMENT_TIMEOUT: Duration = Duration::from_secs(5);

pub(super) struct RealWatcherPulseWorld {
    context: egui::Context,
    scenario: FilesystemApplicationLifecycleScenario,
    workspace: Option<FilesystemContractWorkspace>,
    watcher: Option<WorthUiFilesystemSourceWatcher>,
    host: Option<WorthUiHostEgui>,
}

impl RealWatcherPulseWorld {
    pub(super) fn new() -> Self {
        let workspace = FilesystemContractWorkspace::new("platform-pulse-real-lifecycle");
        workspace.write(
            "app/main.wui",
            &FilesystemApplicationLifecycleScenario::platform_pulse_source_text(),
        );
        let watcher = WorthUiFilesystemSourceWatcher::start(WorthUiFilesystemSourceProvider::new(
            workspace.root(),
        ))
        .expect("production watcher should own the temporary pulse source");
        Self {
            context: egui::Context::default(),
            scenario: FilesystemApplicationLifecycleScenario::new("platform-pulse-real-lifecycle"),
            workspace: Some(workspace),
            watcher: Some(watcher),
            host: None,
        }
    }

    pub(super) fn launch(&mut self) -> InitialPulsePublication {
        let host = WorthUiHostEgui::new(self.context.clone());
        self.host = Some(host.clone());
        let capabilities = self
            .scenario
            .platform_pulse_capability_application(host.clone());
        let initial = self
            .watcher_mut()
            .take_initial_snapshot()
            .expect("ready watcher should publish one initial snapshot");
        let source = initial.source_revision().clone();
        let submission = initial
            .attempt_candidate_for_certification(capabilities.capabilities())
            .expect("canonical blue pulse source should lower");
        let app = self
            .scenario
            .prepare_platform_pulse_application_with_host(submission, host);
        let mut app = Some(app);
        let mut shell = None;
        let mut outcome = None;
        let native = self.context.run_ui(raw_input(), |_| {
            let mut launched = app
                .take()
                .expect("pulse app launches once")
                .launch_native_surface()
                .expect("public native shell should launch");
            outcome = Some(
                launched
                    .present_frame(10, 0)
                    .unwrap_or_else(|_| panic!("initial mounted frame should complete")),
            );
            shell = Some(launched);
        });
        let UiMountedFrameOutcome::Published(mounted) =
            outcome.expect("initial frame returns one outcome")
        else {
            panic!("initial mounted frame should publish");
        };
        assert_background_and_target(&native.shapes, BLUE);
        InitialPulsePublication {
            shell: shell.expect("native shell remains application-owned"),
            source,
            mounted,
        }
    }

    pub(super) fn replace(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
        edit: ValidPulseEdit,
    ) -> PublishedPulseReplacement {
        self.workspace()
            .write_atomic("app/main.wui", &edit.source_text());
        let snapshot = self
            .watcher_mut()
            .settle(SETTLEMENT_TIMEOUT)
            .expect("real watcher should settle an atomic valid edit");
        let source = snapshot.source_revision().clone();
        let mut snapshot = Some(snapshot);
        let mut outcome = None;
        let native = self.context.run_ui(raw_input(), |_| {
            outcome = Some(publish_rebind(
                shell,
                snapshot.take().expect("settled snapshot is consumed once"),
                edit.deadline(),
                edit.tick(),
            ));
        });
        let receipt = outcome
            .expect("rebind returns one terminal result")
            .expect("valid settled source should enter canonical rebind");
        assert_background_and_target(&native.shapes, edit.color());
        PublishedPulseReplacement { source, receipt }
    }

    pub(super) fn preserve_malformed(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
    ) -> PreservedPulseReplacement {
        self.workspace()
            .write("app/main.wui", "component platform.pulse.component.seed {");
        let snapshot = self
            .watcher_mut()
            .settle(SETTLEMENT_TIMEOUT)
            .expect("stable malformed bytes still form a filesystem snapshot");
        let source = snapshot.source_revision().clone();
        let request = UiSourceRebindRequest::new(snapshot)
            .with_deadline(shell.rebind_deadline_at(30))
            .observed_at_tick(21);
        let denial = match shell.begin_source_rebind(request) {
            Err(denial) => denial,
            Ok(outcome) => {
                drop(outcome);
                panic!("malformed authored source must deny before rebind");
            }
        };
        let Some(worth_ui::facade::source::UiSourceRebindAttemptFailure::CompilationDenied(
            receipt,
        )) = denial.source_failure()
        else {
            panic!("malformed pulse source should retain the DSL owner's typed denial");
        };
        assert!(!receipt.report().diagnostics().is_empty());
        assert_eq!(receipt.basis().source_revision(), &source);
        let generation = shell.generation_identity().clone();
        let host = self
            .host
            .as_ref()
            .expect("launched egui host remains retained")
            .clone();
        let native = self.context.run_ui(raw_input(), |_| {
            assert!(matches!(
                shell.present_frame(30, 21),
                Ok(UiMountedFrameOutcome::Unchanged(_))
                    | Ok(UiMountedFrameOutcome::Published(_))
                    | Ok(UiMountedFrameOutcome::Reconciled(_))
            ));
            host.repaint_retained_surfaces();
        });
        assert_background_and_target(&native.shapes, GREEN);
        assert_eq!(shell.generation_identity(), &generation);
        PreservedPulseReplacement {
            source,
            denial,
            generation,
        }
    }

    pub(super) fn shutdown(
        mut self,
        shell: WorthUiNativeApplicationShell,
        observations: &mut ObservedPulseLifecycle,
    ) {
        let watcher = self
            .watcher
            .take()
            .expect("open production watcher")
            .shutdown()
            .expect("production watcher should release its OS registration");
        assert!(watcher.observed_notification_count() > 0);
        let application = shell.shutdown();
        assert!(application.host_session_released());
        assert_eq!(application.released_surface_count(), 1);
        assert_eq!(application.mounted_shutdown_attempt_count(), 0);
        observations.shutdown(&watcher, application);
        self.workspace
            .take()
            .expect("open filesystem world")
            .close();
    }

    fn workspace(&self) -> &FilesystemContractWorkspace {
        self.workspace.as_ref().expect("open filesystem world")
    }

    fn watcher_mut(&mut self) -> &mut WorthUiFilesystemSourceWatcher {
        self.watcher.as_mut().expect("open production watcher")
    }
}

pub(super) struct InitialPulsePublication {
    pub(super) shell: WorthUiNativeApplicationShell,
    pub(super) source: WorthUiSourcePackageRevision,
    pub(super) mounted: UiMountedFramePublicationReceipt,
}

pub(super) struct PublishedPulseReplacement {
    pub(super) source: WorthUiSourcePackageRevision,
    pub(super) receipt: UiRebindReceipt,
}

fn publish_rebind(
    shell: &mut WorthUiNativeApplicationShell,
    snapshot: worth_ui::facade::source::WorthUiSettledSourceSnapshot,
    deadline_tick: u64,
    now_tick: u64,
) -> Result<UiRebindReceipt, WorthUiNativeSourceRebindDenial> {
    let request = UiSourceRebindRequest::new(snapshot)
        .with_deadline(shell.rebind_deadline_at(deadline_tick))
        .observed_at_tick(now_tick);
    match shell.begin_source_rebind(request)? {
        UiRebindOutcome::Published(receipt) => Ok(receipt),
        UiRebindOutcome::Duplicate(_) => {
            panic!("changing the pulse color was classified as duplicate");
        }
        UiRebindOutcome::ObservedNoChange(receipt) => {
            drop(receipt);
            panic!("changing the pulse color was classified as no-change");
        }
        UiRebindOutcome::RejectedBeforeEffects(denial) => {
            panic!(
                "changing the pulse color was rejected: {:?}",
                denial.cause()
            );
        }
        UiRebindOutcome::CancelledBeforeEffects(_) => {
            panic!("changing the pulse color was cancelled");
        }
        UiRebindOutcome::TimedOutBeforeEffects(_) => {
            panic!("changing the pulse color timed out");
        }
        UiRebindOutcome::SupersededBeforeEffects(_) => {
            panic!("changing the pulse color was superseded");
        }
        UiRebindOutcome::InFlight(handle) => {
            drop(handle.dispose());
            panic!("changing the pulse color remained in flight");
        }
        UiRebindOutcome::Indeterminate(recovery) => {
            drop(recovery);
            panic!("changing the pulse color became indeterminate");
        }
        UiRebindOutcome::InternalDefect(defect) => {
            panic!("changing the pulse color hit defect {:?}", defect.kind());
        }
    }
}

pub(super) struct PreservedPulseReplacement {
    pub(super) source: WorthUiSourcePackageRevision,
    pub(super) denial: WorthUiNativeSourceRebindDenial,
    pub(super) generation: WorthUiPreparedApplicationGenerationIdentity,
}

#[derive(Clone, Copy)]
pub(super) enum ValidPulseEdit {
    Green,
    BlueRecovery,
}

impl ValidPulseEdit {
    fn source_text(self) -> String {
        match self {
            Self::Green => {
                FilesystemApplicationLifecycleScenario::platform_pulse_green_source_text()
            }
            Self::BlueRecovery => {
                FilesystemApplicationLifecycleScenario::platform_pulse_source_text()
            }
        }
    }

    fn color(self) -> egui::Color32 {
        match self {
            Self::Green => GREEN,
            Self::BlueRecovery => BLUE,
        }
    }

    fn deadline(self) -> u64 {
        match self {
            Self::Green => 20,
            Self::BlueRecovery => 40,
        }
    }

    fn tick(self) -> u64 {
        match self {
            Self::Green => 11,
            Self::BlueRecovery => 31,
        }
    }
}
