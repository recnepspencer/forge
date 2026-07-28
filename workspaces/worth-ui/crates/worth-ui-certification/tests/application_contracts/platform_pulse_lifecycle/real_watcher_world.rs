use std::time::Duration;

use worth_ui::facade::app::{
    UiMountedFrameOutcome, UiMountedFramePublicationReceipt, WorthUiApplicationCutoverReceipt,
    WorthUiNativeApplicationReplacementOutcome, WorthUiNativeApplicationShell,
    WorthUiPreparedApplicationGenerationIdentity,
};
use worth_ui::facade::source::{
    WorthUiFilesystemSourceProvider, WorthUiFilesystemSourceWatcher, WorthUiSourcePackageRevision,
    WorthUiWatchedCandidateSubmissionDenial,
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
        }
    }

    pub(super) fn launch(&mut self) -> InitialPulsePublication {
        let host = WorthUiHostEgui::new(self.context.clone());
        let capabilities = self
            .scenario
            .platform_pulse_capability_application(host.clone());
        let initial = self
            .watcher_mut()
            .take_initial_snapshot()
            .expect("ready watcher should publish one initial snapshot");
        let source = initial.source_revision().clone();
        let submission = initial
            .lower_to_candidate_submission(capabilities.capabilities())
            .expect("canonical blue pulse source should lower");
        let app = self
            .scenario
            .prepare_platform_pulse_application_with_host(submission, host);
        let mut app = Some(app);
        let mut shell = None;
        let mut outcome = None;
        let native = self.context.run(raw_input(), |_| {
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
        let submission = snapshot
            .lower_to_candidate_submission(shell.capabilities())
            .expect("valid pulse edit should lower");
        let mut submission = Some(submission);
        let mut outcome = None;
        let native = self.context.run(raw_input(), |_| {
            outcome = Some(
                shell
                    .replace_application(
                        submission.take().expect("submission is consumed once"),
                        edit.deadline(),
                        edit.tick(),
                    )
                    .expect("valid whole-application replacement should publish"),
            );
        });
        let WorthUiNativeApplicationReplacementOutcome::Published {
            application,
            mounted,
        } = outcome.expect("replacement returns one outcome")
        else {
            panic!("changing the pulse color is not a semantic no-op");
        };
        assert_background_and_target(&native.shapes, edit.color());
        PublishedPulseReplacement {
            source,
            application,
            mounted,
        }
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
        let denial = snapshot
            .lower_to_candidate_submission(shell.capabilities())
            .expect_err("malformed authored source must deny before replacement");
        let WorthUiWatchedCandidateSubmissionDenial::DslCompilation(report) = &denial else {
            panic!("malformed pulse source should retain the DSL owner's typed denial");
        };
        assert!(!report.diagnostics().is_empty());
        let generation = shell.generation_identity().clone();
        let native = self.context.run(raw_input(), |_| {
            assert!(matches!(
                shell.present_frame(30, 21),
                Ok(UiMountedFrameOutcome::Unchanged(_))
                    | Ok(UiMountedFrameOutcome::Published(_))
                    | Ok(UiMountedFrameOutcome::Reconciled(_))
            ));
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
    pub(super) application: WorthUiApplicationCutoverReceipt,
    pub(super) mounted: UiMountedFramePublicationReceipt,
}

pub(super) struct PreservedPulseReplacement {
    pub(super) source: WorthUiSourcePackageRevision,
    pub(super) denial: WorthUiWatchedCandidateSubmissionDenial,
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
