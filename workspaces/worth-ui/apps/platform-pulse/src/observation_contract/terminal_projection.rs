use worth_ui::facade::app::{
    UiMountedFrameOutcome, WorthUiApplicationPreparationDenial, WorthUiMountedFrameExecutionStop,
    WorthUiNativeApplicationShellLaunchDenial, WorthUiNativeApplicationShutdownReceipt,
    WorthUiNativeSourceRebindDenial,
};
use worth_ui::facade::source::{
    UiSourceRebindAttemptFailure, WorthUiFilesystemWatcherBackend, WorthUiFilesystemWatcherDenial,
    WorthUiFilesystemWatcherShutdownReceipt,
};

use super::envelope::PlatformPulseLifecycleObservationEnvelope;
use super::lifecycle::{
    PlatformPulseLaunchConfigurationDenial, PlatformPulseLifecycleObservation,
    PlatformPulseShutdownCompleted, PlatformPulseTerminalFailure,
    PlatformPulseTerminalFailureFamily, PlatformPulseWatcherBackendObservation,
};
use super::projection::{
    PlatformPulseLifecycleObservationProjectionDenial, PlatformPulseLifecycleObservationStream,
    PlatformPulseObservationState,
};

impl PlatformPulseLifecycleObservationStream {
    pub fn project_shutdown(
        &mut self,
        watcher: &WorthUiFilesystemWatcherShutdownReceipt,
        application: WorthUiNativeApplicationShutdownReceipt,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        self.published_predecessor()?;
        let visual_capture = application.visual_capture();
        let visual_overlay = application.visual_overlay();
        let outcome =
            PlatformPulseLifecycleObservation::ShutdownCompleted(PlatformPulseShutdownCompleted {
                watcher_backend: watcher_backend(watcher.backend()),
                observed_notification_count: watcher.observed_notification_count(),
                mounted_shutdown_attempt_count: application.mounted_shutdown_attempt_count() as u64,
                host_session_released: application.host_session_released(),
                released_surface_count: application.released_surface_count() as u64,
                cancelled_visual_capture_count: visual_capture.cancelled_capture_count() as u64,
                disposed_visual_snapshot_count: visual_capture.disposed_snapshot_count() as u64,
                disposed_visual_pixel_bytes: visual_capture.disposed_pixel_bytes(),
                disposed_visual_structural_bytes: visual_capture.disposed_structural_bytes(),
                cancelled_pending_overlay_count: visual_overlay.cancelled_pending_count() as u64,
                disposed_published_overlay_count: visual_overlay.disposed_published_count() as u64,
                disposed_clearing_overlay_count: visual_overlay.disposed_clearing_count() as u64,
            });
        let envelope = self.next_envelope(outcome)?;
        self.state = PlatformPulseObservationState::Terminal;
        Ok(envelope)
    }

    pub fn project_launch_configuration_failure(
        &mut self,
        denial: &PlatformPulseLaunchConfigurationDenial,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        self.project_terminal(PlatformPulseTerminalFailureFamily::LaunchConfiguration(
            denial.kind(),
        ))
    }

    pub fn project_filesystem_watcher_failure(
        &mut self,
        _denial: &WorthUiFilesystemWatcherDenial,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        self.project_terminal(PlatformPulseTerminalFailureFamily::FilesystemWatcher)
    }

    pub fn project_application_preparation_failure(
        &mut self,
        _denial: &WorthUiApplicationPreparationDenial,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        self.project_terminal(PlatformPulseTerminalFailureFamily::ApplicationPreparation)
    }

    pub fn project_candidate_submission_failure(
        &mut self,
        _denial: &UiSourceRebindAttemptFailure,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        self.project_terminal(PlatformPulseTerminalFailureFamily::CandidateSubmission)
    }

    pub fn project_native_surface_launch_failure(
        &mut self,
        _denial: &WorthUiNativeApplicationShellLaunchDenial,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        self.project_terminal(PlatformPulseTerminalFailureFamily::NativeSurfaceLaunch)
    }

    pub fn project_frame_execution_failure(
        &mut self,
        _denial: &WorthUiMountedFrameExecutionStop<'_>,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        self.project_terminal(PlatformPulseTerminalFailureFamily::MountedFrameExecution)
    }

    pub fn project_frame_outcome_failure(
        &mut self,
        outcome: &UiMountedFrameOutcome,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        match outcome {
            UiMountedFrameOutcome::RejectedBeforeEffects(_)
            | UiMountedFrameOutcome::InFlight(_)
            | UiMountedFrameOutcome::PresentationIndeterminate(_)
            | UiMountedFrameOutcome::RetentionDenied(_)
            | UiMountedFrameOutcome::AdmissionDenied(_)
            | UiMountedFrameOutcome::CompletionDenied(_) => {
                self.project_terminal(PlatformPulseTerminalFailureFamily::MountedFrameExecution)
            }
            UiMountedFrameOutcome::Published(_)
            | UiMountedFrameOutcome::Unchanged(_)
            | UiMountedFrameOutcome::Reconciled(_) => {
                Err(PlatformPulseLifecycleObservationProjectionDenial::OutcomeIsNotFailure)
            }
        }
    }

    pub fn project_native_rebind_failure(
        &mut self,
        _denial: &WorthUiNativeSourceRebindDenial,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        self.project_terminal(PlatformPulseTerminalFailureFamily::NativeApplicationReplacement)
    }

    pub fn project_native_rebind_outcome_failure(
        &mut self,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        self.project_terminal(PlatformPulseTerminalFailureFamily::NativeApplicationReplacement)
    }

    pub fn project_source_worker_panic(
        &mut self,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        self.project_terminal(PlatformPulseTerminalFailureFamily::SourceWorkerPanicked)
    }

    pub fn project_visual_identity_failure(
        &mut self,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        self.project_terminal(PlatformPulseTerminalFailureFamily::VisualIdentity)
    }

    pub fn project_native_event_loop_failure(
        &mut self,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        self.project_terminal(PlatformPulseTerminalFailureFamily::NativeEventLoop)
    }

    fn project_terminal(
        &mut self,
        family: PlatformPulseTerminalFailureFamily,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        if matches!(self.state, PlatformPulseObservationState::Terminal) {
            return Err(PlatformPulseLifecycleObservationProjectionDenial::StreamTerminated);
        }
        let envelope = self.next_envelope(PlatformPulseLifecycleObservation::TerminalFailure(
            PlatformPulseTerminalFailure::new(family),
        ))?;
        self.state = PlatformPulseObservationState::Terminal;
        Ok(envelope)
    }
}

fn watcher_backend(
    backend: WorthUiFilesystemWatcherBackend,
) -> PlatformPulseWatcherBackendObservation {
    match backend {
        WorthUiFilesystemWatcherBackend::Fsevent => PlatformPulseWatcherBackendObservation::Fsevent,
        WorthUiFilesystemWatcherBackend::Inotify => PlatformPulseWatcherBackendObservation::Inotify,
        WorthUiFilesystemWatcherBackend::Kqueue => PlatformPulseWatcherBackendObservation::Kqueue,
        WorthUiFilesystemWatcherBackend::ReadDirectoryChanges => {
            PlatformPulseWatcherBackendObservation::ReadDirectoryChanges
        }
        WorthUiFilesystemWatcherBackend::OtherNative => {
            PlatformPulseWatcherBackendObservation::OtherNative
        }
    }
}
