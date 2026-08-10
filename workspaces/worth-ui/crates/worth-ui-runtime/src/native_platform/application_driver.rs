use crate::facade::{WorthUiApp, WorthUiNativeApplicationShell};
use worth_ui_host_native::{
    UiNativeEventLoopClient, UiNativeEventLoopClientCleanup, UiNativeEventLoopClientClose,
    UiNativeEventLoopDirective, UiNativeReadinessGrant, WorthUiNativeEventLoop,
};

pub(crate) struct UiNativeApplicationDriver {
    application: Option<WorthUiApp>,
    shell: Option<WorthUiNativeApplicationShell>,
    last_ready_generation: u64,
    scale_factor_milli: Option<u32>,
    attribution: Option<worth_ui_host_native::UiNativeClientPresentationAttribution>,
    consumed_application_cleanup_complete: bool,
    pending_cleanup: Option<UiNativeApplicationDriverCleanup>,
}

enum UiNativeApplicationDriverCleanup {
    RuntimeLaunch(crate::runtime::WorthUiRuntimeLaunchDenial),
    Application(crate::facade::WorthUiNativeApplicationCleanup),
    HostSession(crate::facade::WorthUiHostSessionReleaseRecovery),
    UnresolvedApplication,
}

impl UiNativeApplicationDriver {
    pub(crate) fn new(application: WorthUiApp) -> Self {
        Self {
            application: Some(application),
            shell: None,
            last_ready_generation: 0,
            scale_factor_milli: None,
            attribution: None,
            consumed_application_cleanup_complete: false,
            pending_cleanup: None,
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
            shell.rebind_native_surface_scale(grant.scale_factor_milli())?;
            self.scale_factor_milli = Some(grant.scale_factor_milli());
        }
        let outcome = shell.present_frame(u64::MAX, 0).map_err(|_| ())?;
        self.attribution = shell.presentation_attribution(&outcome);
        if self.attribution.is_none() {
            return Err(());
        }
        self.last_ready_generation = grant.generation();
        Ok(UiNativeEventLoopDirective::Continue)
    }

    fn presentation_attribution(
        &self,
    ) -> Option<worth_ui_host_native::UiNativeClientPresentationAttribution> {
        self.attribution
    }

    fn close(mut self) -> UiNativeEventLoopClientClose {
        if let Some(cleanup) = self.pending_cleanup.take() {
            match cleanup.retry() {
                Ok(()) => self.consumed_application_cleanup_complete = true,
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
        let shutdown = shell.shutdown();
        if shutdown.host_session_released() && shutdown.released_surface_count() == 1 {
            UiNativeEventLoopClientClose::Complete
        } else if let Some(cleanup) = shutdown.into_host_cleanup() {
            UiNativeEventLoopClientClose::Incomplete(Box::new(
                UiNativeApplicationDriverCleanup::HostSession(cleanup),
            ))
        } else {
            UiNativeEventLoopClientClose::Incomplete(Box::new(
                UiNativeApplicationDriverCleanup::UnresolvedApplication,
            ))
        }
    }
}

impl UiNativeEventLoopClientCleanup for UiNativeApplicationDriverCleanup {
    fn retry(self: Box<Self>) -> UiNativeEventLoopClientClose {
        match (*self).retry() {
            Ok(()) => UiNativeEventLoopClientClose::Complete,
            Err(cleanup) => UiNativeEventLoopClientClose::Incomplete(Box::new(cleanup)),
        }
    }
}

impl UiNativeApplicationDriverCleanup {
    fn retry(self) -> Result<(), Self> {
        match self {
            Self::RuntimeLaunch(cleanup) => cleanup
                .retry_host_session_cleanup()
                .map(|_| ())
                .map_err(Self::RuntimeLaunch),
            Self::Application(cleanup) => cleanup.retry().map_err(Self::Application),
            Self::HostSession(cleanup) => cleanup.retry().map(|_| ()).map_err(Self::HostSession),
            Self::UnresolvedApplication => Err(Self::UnresolvedApplication),
        }
    }
}
