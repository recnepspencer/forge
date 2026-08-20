use crate::facade::{WorthUiApp, WorthUiNativeApplicationShell};
use worth_ui_host_native::{
    UiNativeEventLoopClient, UiNativeEventLoopClientCleanup, UiNativeEventLoopClientClose,
    UiNativeEventLoopDirective, UiNativeReadinessGrant, WorthUiNativeEventLoop,
};

#[path = "application_driver/physical_recovery_tracker.rs"]
mod physical_recovery_tracker;
#[path = "application_driver/program_progress.rs"]
mod program_progress;
#[path = "application_driver/program_reconstruction.rs"]
mod program_reconstruction;
#[path = "application_driver/runtime_qualification.rs"]
mod runtime_qualification;
#[path = "application_driver/shutdown_observation.rs"]
mod shutdown_observation;
use program_progress::UiNativeApplicationProgramProgress;
use shutdown_observation::{
    client_resource_observation, host_shutdown_observation, map_semantic_frontier_observations,
    map_text_presentation_work, map_transition_observations,
};

pub(crate) struct UiNativeApplicationDriver {
    application: Option<WorthUiApp>,
    shell: Option<WorthUiNativeApplicationShell>,
    last_ready_generation: u64,
    scale_factor_milli: Option<u32>,
    consumed_application_cleanup_complete: bool,
    pending_cleanup: Option<UiNativeApplicationDriverCleanup>,
    progress: UiNativeApplicationProgramProgress,
}

enum UiNativeApplicationDriverCleanup {
    RuntimeLaunch(crate::runtime::WorthUiRuntimeLaunchDenial),
    Application(crate::facade::WorthUiNativeApplicationCleanup),
    HostSession(crate::facade::WorthUiHostSessionReleaseRecovery),
    UnresolvedApplication,
}

impl UiNativeApplicationDriver {
    pub(crate) fn new(
        application: WorthUiApp,
        program: crate::facade::entry::UiNativeApplicationProgram,
        runtime_qualification: Option<
            super::runtime_qualification::UiNativeRuntimeQualificationPlan,
        >,
    ) -> Self {
        Self {
            application: Some(application),
            shell: None,
            last_ready_generation: 0,
            scale_factor_milli: None,
            consumed_application_cleanup_complete: false,
            pending_cleanup: None,
            progress: UiNativeApplicationProgramProgress::new(program, runtime_qualification),
        }
    }

    pub(crate) fn run(
        self,
        event_loop: WorthUiNativeEventLoop,
    ) -> Result<
        worth_ui_host_native::UiNativeEventLoopRunReport,
        worth_ui_host_native::UiNativeEventLoopStopReport,
    > {
        event_loop.run(self)
    }

    fn next_directive(&self) -> UiNativeEventLoopDirective {
        if self.progress.should_close() {
            UiNativeEventLoopDirective::Close
        } else if self.progress.external_observation_ready() {
            UiNativeEventLoopDirective::ExternalObservationReady
        } else {
            UiNativeEventLoopDirective::Continue
        }
    }
}

impl UiNativeEventLoopClient for UiNativeApplicationDriver {
    fn native_surface_ready(
        &mut self,
        grant: UiNativeReadinessGrant,
    ) -> Result<UiNativeEventLoopDirective, ()> {
        if grant.generation() != 0 || self.shell.is_some() {
            return Err(());
        }
        let application = self.application.take().ok_or(())?;
        self.shell = match application.launch_native_surface_at_scale(grant.scale_factor_milli()) {
            Ok(shell) => Some(shell),
            Err(
                crate::facade::WorthUiNativeApplicationShellLaunchDenial::RuntimeLaunchCleanup(
                    cleanup,
                ),
            ) => {
                self.pending_cleanup =
                    Some(UiNativeApplicationDriverCleanup::RuntimeLaunch(cleanup));
                return Err(());
            }
            Err(crate::facade::WorthUiNativeApplicationShellLaunchDenial::ApplicationCleanup(
                cleanup,
            )) => {
                self.pending_cleanup = Some(UiNativeApplicationDriverCleanup::Application(cleanup));
                return Err(());
            }
            Err(denial) => {
                let _ = denial;
                self.consumed_application_cleanup_complete = true;
                return Err(());
            }
        };
        self.shell
            .as_mut()
            .ok_or(())?
            .observe_native_viewport_readiness(grant.client_physical_size(), false);
        self.scale_factor_milli = Some(grant.scale_factor_milli());
        Ok(UiNativeEventLoopDirective::Continue)
    }

    fn redraw_ready(
        &mut self,
        grant: UiNativeReadinessGrant,
    ) -> Result<UiNativeEventLoopDirective, ()> {
        if grant.generation() <= self.last_ready_generation {
            return Err(());
        }
        let shell = self.shell.as_mut().ok_or(())?;
        if self.scale_factor_milli != Some(grant.scale_factor_milli()) {
            if shell
                .rebind_native_surface_scale(grant.scale_factor_milli())
                .is_err()
            {
                return Err(());
            }
            self.scale_factor_milli = Some(grant.scale_factor_milli());
        }
        shell.observe_native_viewport_readiness(grant.client_physical_size(), true);
        shell.commit_pending_native_viewport_measurement()?;
        self.progress
            .observe_readiness_generation(grant.generation());
        if self.progress.advance(shell).is_err() {
            return Err(());
        }
        self.last_ready_generation = grant.generation();
        Ok(self.next_directive())
    }

    fn physical_work_progressed(
        &mut self,
        grant: worth_ui_host_native::UiNativePhysicalProgressGrant,
    ) -> Result<UiNativeEventLoopDirective, ()> {
        let shell = self.shell.as_mut().ok_or(())?;
        self.progress.physical_work_progressed(shell, grant)?;
        Ok(self.next_directive())
    }

    fn external_close_requested(&mut self) -> Result<UiNativeEventLoopDirective, ()> {
        self.progress.request_external_close();
        Ok(self.next_directive())
    }

    fn presentation_attribution(
        &self,
    ) -> Option<worth_ui_host_native::UiNativeClientPresentationAttribution> {
        self.progress.attribution(self.shell.as_ref())
    }

    fn close(mut self) -> UiNativeEventLoopClientClose {
        if let Some(cleanup) = self.pending_cleanup.take() {
            match cleanup.retry() {
                Ok(query_close) => {
                    return UiNativeEventLoopClientClose::CompleteWithObservation(
                        host_shutdown_observation(&query_close),
                    );
                }
                Err(cleanup) => return UiNativeEventLoopClientClose::Incomplete(Box::new(cleanup)),
            }
        }
        let Some(shell) = self.shell.take() else {
            return if self.application.take().is_some()
                || self.consumed_application_cleanup_complete
            {
                UiNativeEventLoopClientClose::Complete
            } else {
                UiNativeEventLoopClientClose::Incomplete(Box::new(
                    UiNativeApplicationDriverCleanup::UnresolvedApplication,
                ))
            };
        };
        let runtime_derived_state_reconstruction = shell.runtime_derived_state_reconstruction();
        let shutdown = shell.shutdown();
        let close_observation = worth_ui_host_native::UiNativeClientShutdownObservation::from_client_with_presentation_evidence(
            shutdown.closed_query_resources(),
            shutdown.query_close_complete(),
            map_transition_observations(shutdown.query_transitions()),
            shutdown.query_transition_trace_complete(),
            map_semantic_frontier_observations(shutdown.query_semantic_frontiers()),
            shutdown.query_semantic_frontier_trace_complete(),
        ).with_text_presentation_work(
            map_text_presentation_work(shutdown.text_presentation_work()),
            shutdown.text_presentation_work_trace_complete(),
        ).with_authored_mounted_instances(
            shutdown.authored_mounted_instances().to_vec().into_boxed_slice(),
        ).with_derived_state_reconstruction(runtime_derived_state_reconstruction)
        .with_resources(client_resource_observation(shutdown.client_resource_peaks()));
        if shutdown.host_session_released()
            && shutdown.released_surface_count() == 1
            && shutdown.query_close_complete()
        {
            UiNativeEventLoopClientClose::CompleteWithObservation(close_observation)
        } else if let Some(cleanup) = shutdown.into_application_cleanup() {
            UiNativeEventLoopClientClose::Incomplete(Box::new(
                UiNativeApplicationDriverCleanup::Application(cleanup),
            ))
        } else {
            UiNativeEventLoopClientClose::Incomplete(Box::new(
                UiNativeApplicationDriverCleanup::UnresolvedApplication,
            ))
        }
    }
}

#[cfg(test)]
fn retain_presentation_attribution(
    current: &mut Option<worth_ui_host_native::UiNativeClientPresentationAttribution>,
    observed: Option<worth_ui_host_native::UiNativeClientPresentationAttribution>,
) {
    if let Some(observed) = observed {
        *current = Some(observed);
    }
}

#[cfg(test)]
mod tests {
    use super::retain_presentation_attribution;
    use worth_ui_host_native::UiNativeClientPresentationAttribution;

    #[test]
    fn deferred_logical_frames_preserve_the_last_physical_attribution() {
        let first = UiNativeClientPresentationAttribution::reported([1, 2, 3, 4, 5, 6], [7, 8]);
        let second =
            UiNativeClientPresentationAttribution::reported([9, 10, 11, 12, 13, 14], [15, 16]);
        let mut current = None;
        retain_presentation_attribution(&mut current, None);
        assert_eq!(current, None);
        retain_presentation_attribution(&mut current, Some(first));
        retain_presentation_attribution(&mut current, None);
        assert_eq!(current, Some(first));
        retain_presentation_attribution(&mut current, Some(second));
        assert_eq!(current, Some(second));
    }
}

impl UiNativeEventLoopClientCleanup for UiNativeApplicationDriverCleanup {
    fn retry(self: Box<Self>) -> UiNativeEventLoopClientClose {
        match (*self).retry() {
            Ok(query_close) => UiNativeEventLoopClientClose::CompleteWithObservation(
                host_shutdown_observation(&query_close),
            ),
            Err(cleanup) => UiNativeEventLoopClientClose::Incomplete(Box::new(cleanup)),
        }
    }
}

impl UiNativeApplicationDriverCleanup {
    fn retry(self) -> Result<crate::facade::entry::UiNativeApplicationQueryCloseObservation, Self> {
        match self {
            Self::RuntimeLaunch(cleanup) => cleanup
                .retry_host_session_cleanup()
                .map(|_| {
                    crate::facade::entry::UiNativeApplicationQueryCloseObservation::empty_complete()
                })
                .map_err(Self::RuntimeLaunch),
            Self::Application(cleanup) => cleanup.retry().map_err(Self::Application),
            Self::HostSession(cleanup) => cleanup
                .retry()
                .map(|_| {
                    crate::facade::entry::UiNativeApplicationQueryCloseObservation::empty_complete()
                })
                .map_err(Self::HostSession),
            Self::UnresolvedApplication => Err(Self::UnresolvedApplication),
        }
    }
}
