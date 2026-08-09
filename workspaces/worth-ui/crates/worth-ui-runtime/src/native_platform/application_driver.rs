use crate::facade::{WorthUiApp, WorthUiNativeApplicationShell};
use worth_ui_host_native::{
    UiNativeEventLoopClient, UiNativeEventLoopDirective, UiNativeReadinessGrant,
    WorthUiNativeEventLoop,
};

pub(crate) struct UiNativeApplicationDriver {
    application: Option<WorthUiApp>,
    shell: Option<WorthUiNativeApplicationShell>,
    last_ready_generation: u64,
    scale_factor_milli: Option<u32>,
    attribution: Option<worth_ui_host_native::UiNativeClientPresentationAttribution>,
}

impl UiNativeApplicationDriver {
    pub(crate) fn new(application: WorthUiApp) -> Self {
        Self {
            application: Some(application),
            shell: None,
            last_ready_generation: 0,
            scale_factor_milli: None,
            attribution: None,
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
        self.shell = Some(
            application
                .launch_native_surface_at_scale(grant.scale_factor_milli())
                .map_err(|_| ())?,
        );
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

    fn close(mut self) -> Result<(), ()> {
        let Some(shell) = self.shell.take() else {
            return self.application.take().map(|_| ()).ok_or(());
        };
        let shutdown = shell.shutdown();
        if shutdown.host_session_released() && shutdown.released_surface_count() == 1 {
            Ok(())
        } else {
            Err(())
        }
    }
}
