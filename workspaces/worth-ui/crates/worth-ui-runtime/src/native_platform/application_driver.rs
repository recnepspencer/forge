use crate::facade::{WorthUiApp, WorthUiNativeApplicationShell};
use worth_ui_host_native::{
    UiNativeEventLoopClient, UiNativeEventLoopClientClose, UiNativeEventLoopDirective,
    UiNativeObservationReadinessGrant, UiNativeReadinessGrant, WorthUiNativeEventLoop,
};

#[path = "application_driver/application_runtime.rs"]
mod application_runtime;
#[path = "application_driver/cleanup.rs"]
mod cleanup;
#[path = "application_driver/physical_recovery_tracker.rs"]
mod physical_recovery_tracker;
#[path = "application_driver/program_progress.rs"]
mod program_progress;
#[path = "application_driver/program_reconstruction.rs"]
mod program_reconstruction;
#[path = "application_driver/runtime_qualification.rs"]
mod runtime_qualification;
#[path = "application_driver/shutdown_observation.rs"]
pub(crate) mod shutdown_observation;
#[cfg(test)]
#[path = "application_driver/tests.rs"]
mod tests;
mod visual_snapshot;
use cleanup::UiNativeApplicationDriverCleanup;
#[cfg(test)]
use cleanup::UiNativeApplicationDriverCleanupCompletion;
use program_progress::UiNativeApplicationProgramProgress;
use shutdown_observation::UiNativeDriverShutdownEvidence;

pub(crate) struct UiNativeApplicationDriver {
    application: Option<WorthUiApp>,
    shell: Option<WorthUiNativeApplicationShell>,
    last_ready_generation: u64,
    last_observation_ready_generation: u64,
    observation_ingress_counts: [u64; 5],
    scale_factor_milli: Option<u32>,
    consumed_application_cleanup_complete: bool,
    pending_cleanup: Option<UiNativeApplicationDriverCleanup>,
    progress: UiNativeApplicationProgramProgress,
    application_runtime: Option<Box<dyn super::UiNativeApplicationRuntime>>,
    application_runtime_ports: Option<Box<[super::UiNativeApplicationReadinessPort]>>,
    application_runtime_active: bool,
    pending_application_runtime_close: Option<super::UiNativeApplicationRuntimeCloseIncomplete>,
}

impl UiNativeApplicationDriver {
    pub(crate) fn new(
        application: WorthUiApp,
        program: crate::facade::entry::UiNativeApplicationProgram,
        runtime_qualification: Option<
            super::runtime_qualification::UiNativeRuntimeQualificationPlan,
        >,
        application_runtime: Option<Box<dyn super::UiNativeApplicationRuntime>>,
    ) -> Self {
        Self {
            application: Some(application),
            shell: None,
            last_ready_generation: 0,
            last_observation_ready_generation: 0,
            observation_ingress_counts: [0; 5],
            scale_factor_milli: None,
            consumed_application_cleanup_complete: false,
            pending_cleanup: None,
            progress: UiNativeApplicationProgramProgress::new(program, runtime_qualification),
            application_runtime,
            application_runtime_ports: None,
            application_runtime_active: false,
            pending_application_runtime_close: None,
        }
    }

    #[cfg(test)]
    fn from_launched_shell_for_test(shell: WorthUiNativeApplicationShell) -> Self {
        Self {
            application: None,
            shell: Some(shell),
            last_ready_generation: 0,
            last_observation_ready_generation: 0,
            observation_ingress_counts: [0; 5],
            scale_factor_milli: Some(1_000),
            consumed_application_cleanup_complete: false,
            pending_cleanup: None,
            progress: UiNativeApplicationProgramProgress::new(
                crate::facade::entry::UiNativeApplicationProgram::single_frame(),
                None,
            ),
            application_runtime: None,
            application_runtime_ports: None,
            application_runtime_active: false,
            pending_application_runtime_close: None,
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
        } else {
            UiNativeEventLoopDirective::Continue
        }
    }
}

impl UiNativeEventLoopClient for UiNativeApplicationDriver {
    fn application_readiness_owner_count(
        &self,
    ) -> worth_ui_host_native::UiNativeApplicationReadinessOwnerCount {
        self.application_readiness_owner_count()
    }

    fn install_application_readiness(
        &mut self,
        ports: Box<[worth_ui_host_native::UiNativeApplicationReadinessPort]>,
    ) -> Result<(), ()> {
        self.install_application_readiness(ports)
    }

    fn application_readiness_ready(
        &mut self,
        grant: worth_ui_host_native::UiNativeApplicationReadinessGrant,
    ) -> Result<UiNativeEventLoopDirective, ()> {
        self.progress_application_runtime(grant)
    }

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
                self.pending_cleanup = Some(UiNativeApplicationDriverCleanup::Application {
                    cleanup,
                    evidence: UiNativeDriverShutdownEvidence::empty(),
                });
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
            .observe_native_viewport_readiness(
                grant.client_physical_size(),
                grant.scale_factor_milli(),
                false,
            );
        self.scale_factor_milli = Some(grant.scale_factor_milli());
        self.activate_application_runtime()?;
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
        shell.observe_native_viewport_readiness(
            grant.client_physical_size(),
            grant.scale_factor_milli(),
            true,
        );
        self.progress
            .observe_readiness(grant.generation(), grant.surface_basis_generation());
        if self.progress.advance(shell).is_err() {
            eprintln!("native-driver-diagnostic: redraw program progress denied");
            return Err(());
        }
        self.last_ready_generation = grant.generation();
        Ok(self.next_directive())
    }

    fn physical_work_progressed(
        &mut self,
        grant: worth_ui_host_native::UiNativePhysicalProgressGrant,
    ) -> Result<UiNativeEventLoopDirective, ()> {
        if self.application_runtime_active {
            return self
                .progress_application_runtime_physical(grant)
                .map_err(|()| {
                    eprintln!("native-driver-diagnostic: application physical progress denied");
                });
        }
        let shell = self.shell.as_mut().ok_or(())?;
        self.progress.physical_work_progressed(shell, grant)?;
        Ok(self.next_directive())
    }

    fn native_observations_ready(
        &mut self,
        grant: UiNativeObservationReadinessGrant,
    ) -> Result<UiNativeEventLoopDirective, ()> {
        if grant.generation() <= self.last_observation_ready_generation {
            return Err(());
        }
        let shell = self.shell.as_mut().ok_or(())?;
        let settlement = shell.admit_native_observation_batches(grant.reachability());
        let (applied, duplicate, quarantined, denied) = settlement.counts();
        for (total, observed) in self.observation_ingress_counts[..4].iter_mut().zip([
            applied,
            duplicate,
            quarantined,
            denied,
        ]) {
            *total = total.saturating_add(observed as u64);
        }
        if settlement.drain_denial().is_some() {
            eprintln!("native-driver-diagnostic: observation drain denied");
            self.observation_ingress_counts[4] =
                self.observation_ingress_counts[4].saturating_add(1);
            return Err(());
        }
        self.last_observation_ready_generation = grant.generation();
        let directive = self
            .progress_application_runtime_observations(settlement)
            .map_err(|()| {
                eprintln!("native-driver-diagnostic: application observation progress denied");
            })?;
        if matches!(directive, UiNativeEventLoopDirective::Close) {
            Ok(directive)
        } else {
            Ok(self.next_directive())
        }
    }

    fn external_close_requested(&mut self) -> Result<UiNativeEventLoopDirective, ()> {
        self.progress.request_external_close();
        Ok(self.next_directive())
    }

    fn presentation_attribution(
        &self,
    ) -> Option<worth_ui_host_native::UiNativeClientPresentationAttribution> {
        self.shell
            .as_ref()
            .and_then(WorthUiNativeApplicationShell::current_presentation_attribution)
    }

    fn close(mut self) -> UiNativeEventLoopClientClose {
        let runtime_derived_state_reconstruction = self
            .application_runtime_shell()
            .and_then(WorthUiNativeApplicationShell::runtime_derived_state_reconstruction);
        let application_runtime_shutdown = match self.close_application_runtime() {
            Ok(shutdown) => shutdown,
            Err(()) => {
                return UiNativeEventLoopClientClose::Incomplete(Box::new(self));
            }
        };
        if self.pending_application_runtime_close.is_some() {
            return UiNativeEventLoopClientClose::Incomplete(Box::new(self));
        }
        if let Some(cleanup) = self.pending_cleanup.take() {
            match cleanup.retry() {
                Ok(completion) => return completion.into_client_close(),
                Err(cleanup) => return UiNativeEventLoopClientClose::Incomplete(Box::new(cleanup)),
            }
        }
        let shutdown = if let Some(shutdown) = application_runtime_shutdown {
            shutdown
        } else {
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
            shell.shutdown()
        };
        let evidence = UiNativeDriverShutdownEvidence::captured(
            runtime_derived_state_reconstruction,
            self.observation_ingress_counts,
            self.progress.take_visual_snapshot(),
        );
        if shutdown.host_session_released()
            && shutdown.released_surface_count() == 1
            && shutdown.query_close_complete()
            && shutdown.intent_resources_empty()
        {
            let query_close = shutdown.into_query_close_observation();
            UiNativeEventLoopClientClose::CompleteWithObservation(evidence.finalize(&query_close))
        } else if let Some(cleanup) = shutdown.into_application_cleanup() {
            UiNativeEventLoopClientClose::Incomplete(Box::new(
                UiNativeApplicationDriverCleanup::Application { cleanup, evidence },
            ))
        } else {
            UiNativeEventLoopClientClose::Incomplete(Box::new(
                UiNativeApplicationDriverCleanup::UnresolvedApplication,
            ))
        }
    }
}
