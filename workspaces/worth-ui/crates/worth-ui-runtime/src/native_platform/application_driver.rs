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
    program: crate::facade::entry::UiNativeApplicationProgram,
    next_frame: usize,
    pending_frame: Option<crate::mounting::UiMountedPresentationInFlight>,
    next_completion_tick: u64,
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
    ) -> Self {
        Self {
            application: Some(application),
            shell: None,
            last_ready_generation: 0,
            scale_factor_milli: None,
            attribution: None,
            consumed_application_cleanup_complete: false,
            pending_cleanup: None,
            program,
            next_frame: 0,
            pending_frame: None,
            next_completion_tick: 1,
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
        if self.next_frame >= self.program.frames().len() && self.pending_frame.is_none() {
            UiNativeEventLoopDirective::Close
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
        if self.pending_frame.is_none() {
            advance_program(
                shell,
                &self.program,
                &mut self.next_frame,
                &mut self.pending_frame,
                &mut self.attribution,
            )?;
        }
        self.last_ready_generation = grant.generation();
        Ok(self.next_directive())
    }

    fn physical_work_progressed(
        &mut self,
        _grant: worth_ui_host_native::UiNativePhysicalProgressGrant,
    ) -> Result<UiNativeEventLoopDirective, ()> {
        let Some(in_flight) = self.pending_frame.take() else {
            return Ok(UiNativeEventLoopDirective::Continue);
        };
        let shell = self.shell.as_mut().ok_or(())?;
        self.next_completion_tick = self.next_completion_tick.saturating_add(1);
        let outcome = shell.complete_frame_presentation(in_flight, self.next_completion_tick);
        if retain_or_attribute(
            shell,
            outcome,
            &mut self.pending_frame,
            &mut self.attribution,
        )? {
            self.next_frame = self.next_frame.saturating_add(1);
            advance_program(
                shell,
                &self.program,
                &mut self.next_frame,
                &mut self.pending_frame,
                &mut self.attribution,
            )?;
        }
        Ok(self.next_directive())
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

fn advance_program(
    shell: &mut WorthUiNativeApplicationShell,
    program: &crate::facade::entry::UiNativeApplicationProgram,
    next_frame: &mut usize,
    pending: &mut Option<crate::mounting::UiMountedPresentationInFlight>,
    attribution: &mut Option<worth_ui_host_native::UiNativeClientPresentationAttribution>,
) -> Result<(), ()> {
    while let Some(frame) = program.frames().get(*next_frame) {
        shell
            .apply_component_presence(frame.component_presence())
            .map_err(|_| ())?;
        shell
            .apply_component_semantic_text(frame.semantic_text())
            .map_err(|_| ())?;
        let tick = u64::try_from(*next_frame + 1).map_err(|_| ())?;
        let outcome = shell.present_frame(u64::MAX, tick).map_err(|_| ())?;
        if !retain_or_attribute(shell, outcome, pending, attribution)? {
            return Ok(());
        }
        *next_frame = next_frame.saturating_add(1);
    }
    Ok(())
}

fn retain_or_attribute(
    shell: &WorthUiNativeApplicationShell,
    outcome: crate::mounting::UiMountedFrameOutcome,
    pending: &mut Option<crate::mounting::UiMountedPresentationInFlight>,
    attribution: &mut Option<worth_ui_host_native::UiNativeClientPresentationAttribution>,
) -> Result<bool, ()> {
    match outcome {
        crate::mounting::UiMountedFrameOutcome::InFlight(in_flight) => {
            *pending = Some(in_flight);
            return Ok(false);
        }
        crate::mounting::UiMountedFrameOutcome::RejectedBeforeEffects(rejected)
            if rejected.rejections().iter().all(|rejection| {
                rejection.denial()
                    == worth_ui_host_contract::UiHostSurfacePresentationDenial::TextAtlasPresentationDeferred
        }) =>
        {
            return Ok(true);
        }
        outcome => {
            retain_presentation_attribution(
                attribution,
                shell.presentation_attribution(&outcome, *attribution),
            )?;
        }
    }
    Ok(true)
}

fn retain_presentation_attribution(
    current: &mut Option<worth_ui_host_native::UiNativeClientPresentationAttribution>,
    observed: Option<worth_ui_host_native::UiNativeClientPresentationAttribution>,
) -> Result<(), ()> {
    if let Some(observed) = observed {
        *current = Some(observed);
        return Ok(());
    }
    current.is_some().then_some(()).ok_or(())
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
        assert_eq!(retain_presentation_attribution(&mut current, None), Err(()));
        retain_presentation_attribution(&mut current, Some(first)).unwrap();
        retain_presentation_attribution(&mut current, None).unwrap();
        assert_eq!(current, Some(first));
        retain_presentation_attribution(&mut current, Some(second)).unwrap();
        assert_eq!(current, Some(second));
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
