use super::UiNativeApplicationDriver;
use crate::native_platform::{
    UiNativeApplicationReadinessPort, UiNativeApplicationRuntimeDirective,
};

impl UiNativeApplicationDriver {
    pub(super) fn application_readiness_owner_count(
        &self,
    ) -> worth_ui_host_native::UiNativeApplicationReadinessOwnerCount {
        self.application_runtime.as_ref().map_or_else(
            worth_ui_host_native::UiNativeApplicationReadinessOwnerCount::none,
            |runtime| runtime.readiness_owner_count().into_host(),
        )
    }

    pub(super) fn install_application_readiness(
        &mut self,
        ports: Box<[worth_ui_host_native::UiNativeApplicationReadinessPort]>,
    ) -> Result<(), ()> {
        let expected = usize::from(self.application_readiness_owner_count().get());
        if self.application_runtime_ports.is_some() || ports.len() != expected {
            return Err(());
        }
        self.application_runtime_ports = Some(
            ports
                .into_vec()
                .into_iter()
                .map(UiNativeApplicationReadinessPort::from_host)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        );
        Ok(())
    }

    pub(super) fn activate_application_runtime(&mut self) -> Result<(), ()> {
        let Some(runtime) = self.application_runtime.as_mut() else {
            return self
                .application_runtime_ports
                .take()
                .filter(|ports| ports.is_empty())
                .map(|_| ())
                .ok_or(());
        };
        let ports = self.application_runtime_ports.take().ok_or(())?;
        let shell = self.shell.take().ok_or(())?;
        self.application_runtime_active = true;
        match runtime.activate(shell, ports) {
            Ok(shell) => {
                self.shell = Some(shell);
                Ok(())
            }
            Err(stopped) => {
                self.shell = Some(stopped.into_application());
                Err(())
            }
        }
    }

    pub(super) fn progress_application_runtime(
        &mut self,
        grant: worth_ui_host_native::UiNativeApplicationReadinessGrant,
    ) -> Result<worth_ui_host_native::UiNativeEventLoopDirective, ()> {
        if !self.application_runtime_active {
            return Err(());
        }
        let runtime = self.application_runtime.as_mut().ok_or(())?;
        let shell = self.shell.take().ok_or(())?;
        match runtime.readiness_ready(shell, grant.owner_ordinal(), grant.generation()) {
            Ok((shell, directive)) => {
                self.shell = Some(shell);
                Ok(map_directive(directive))
            }
            Err(stopped) => {
                self.shell = Some(stopped.into_application());
                Err(())
            }
        }
    }

    pub(super) fn progress_application_runtime_observations(
        &mut self,
        settlement: crate::facade::entry::UiNativeObservationIngressSettlement,
    ) -> Result<worth_ui_host_native::UiNativeEventLoopDirective, ()> {
        if !self.application_runtime_active {
            return Ok(worth_ui_host_native::UiNativeEventLoopDirective::Continue);
        }
        let runtime = self.application_runtime.as_mut().ok_or(())?;
        let shell = self.shell.take().ok_or(())?;
        let progress =
            crate::native_platform::UiNativeApplicationObservationProgress::from_settlement(
                settlement,
            );
        match runtime.native_observations_ready(shell, progress) {
            Ok((shell, directive)) => {
                self.shell = Some(shell);
                Ok(map_directive(directive))
            }
            Err(stopped) => {
                self.shell = Some(stopped.into_application());
                Err(())
            }
        }
    }

    pub(super) fn progress_application_runtime_physical(
        &mut self,
        grant: worth_ui_host_native::UiNativePhysicalProgressGrant,
    ) -> Result<worth_ui_host_native::UiNativeEventLoopDirective, ()> {
        if !self.application_runtime_active {
            return Err(());
        }
        let runtime = self.application_runtime.as_mut().ok_or(())?;
        let shell = self.shell.take().ok_or(())?;
        let progress =
            crate::native_platform::UiNativeApplicationPhysicalProgress::from_host(grant);
        match runtime.physical_work_progressed(shell, progress) {
            Ok((shell, directive)) => {
                self.shell = Some(shell);
                Ok(map_directive(directive))
            }
            Err(stopped) => {
                self.shell = Some(stopped.into_application());
                Err(())
            }
        }
    }

    pub(super) fn application_runtime_shell(
        &self,
    ) -> Option<&crate::facade::WorthUiNativeApplicationShell> {
        self.shell.as_ref().or_else(|| {
            self.pending_application_runtime_close
                .as_ref()
                .map(crate::native_platform::UiNativeApplicationRuntimeCloseIncomplete::application)
        })
    }

    pub(super) fn close_application_runtime(
        &mut self,
    ) -> Result<Option<crate::facade::WorthUiNativeApplicationShutdownReceipt>, ()> {
        if let Some(incomplete) = self.pending_application_runtime_close.take() {
            return match incomplete.retry() {
                Ok(closed) => {
                    self.application_runtime_active = false;
                    Ok(Some(closed.into_application_shutdown()))
                }
                Err(incomplete) => {
                    self.pending_application_runtime_close = Some(incomplete);
                    Err(())
                }
            };
        }
        if !self.application_runtime_active {
            self.application_runtime.take();
            self.application_runtime_ports.take();
            return Ok(None);
        }
        let runtime = self.application_runtime.take().ok_or(())?;
        let shell = self.shell.take().ok_or(())?;
        self.application_runtime_ports.take();
        match runtime.close(shell) {
            Ok(closed) => {
                self.application_runtime_active = false;
                Ok(Some(closed.into_application_shutdown()))
            }
            Err(incomplete) => {
                self.pending_application_runtime_close = Some(incomplete);
                Err(())
            }
        }
    }
}

fn map_directive(
    directive: UiNativeApplicationRuntimeDirective,
) -> worth_ui_host_native::UiNativeEventLoopDirective {
    match directive {
        UiNativeApplicationRuntimeDirective::Continue => {
            worth_ui_host_native::UiNativeEventLoopDirective::Continue
        }
        UiNativeApplicationRuntimeDirective::Close => {
            worth_ui_host_native::UiNativeEventLoopDirective::Close
        }
    }
}
