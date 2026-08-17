#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativePlatformStopReason {
    EventLoopCreation,
    WindowCreation,
    GraphicsPreparation,
    ApplicationDriver,
    EventLoopRun,
    IncompleteCleanup,
}

#[derive(Debug)]
pub struct UiNativePlatformStopReport {
    report: worth_ui_host_native::UiNativeEventLoopStopReport,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiNativePlatformCloseReceipt {
    report: worth_ui_host_native::UiNativeEventLoopRunReport,
}

#[must_use]
#[derive(Debug)]
pub enum UiNativePlatformOutcome {
    ApplicationPreparationDenied(super::UiNativeApplicationPreparationDenial),
    Closed(UiNativePlatformCloseReceipt),
    Stopped(UiNativePlatformStopReport),
}

impl UiNativePlatformCloseReceipt {
    pub(crate) fn from_native_report(
        report: worth_ui_host_native::UiNativeEventLoopRunReport,
    ) -> Self {
        Self { report }
    }

    pub fn presentation(&self) -> &worth_ui_host_native::UiNativePresentationObservation {
        self.report.presentation()
    }

    pub const fn terminal_census(&self) -> worth_ui_host_native::UiNativeResourceCensus {
        self.report.terminal_census()
    }

    pub fn graphics(&self) -> &worth_ui_host_native::UiNativeGraphicsObservation {
        self.report.graphics()
    }

    pub fn event_loop_thread(&self) -> &str {
        self.report.event_loop_thread()
    }

    pub const fn event_loop_thread_matches_launch(&self) -> bool {
        self.report.event_loop_thread_matches_launch()
    }

    pub const fn client_attribution(
        &self,
    ) -> worth_ui_host_native::UiNativeClientPresentationAttribution {
        self.report.client_attribution()
    }

    pub const fn readiness_signals(&self) -> u64 {
        self.report.readiness_signals()
    }

    pub const fn redraw_turns(&self) -> u64 {
        self.report.redraw_turns()
    }

    pub const fn idle_wait_turns(&self) -> u64 {
        self.report.idle_wait_turns()
    }

    pub const fn coalesced_wakes(&self) -> u64 {
        self.report.coalesced_wakes()
    }

    pub const fn port_crossings(&self) -> u8 {
        self.report.port_crossings()
    }

    pub const fn peak_census(&self) -> worth_ui_host_native::UiNativeResourceCensus {
        self.report.peak_census()
    }

    pub fn retained_frames(&self) -> &[worth_ui_host_native::UiNativeRetainedFrameObservation] {
        self.report.retained_frames()
    }

    #[doc(hidden)]
    pub fn peak_text_layout_count(&self) -> usize {
        self.report.peak_text_layout_count()
    }
}

impl UiNativePlatformStopReport {
    pub(crate) fn from_native_report(
        report: worth_ui_host_native::UiNativeEventLoopStopReport,
    ) -> Self {
        Self { report }
    }

    pub const fn reason(&self) -> UiNativePlatformStopReason {
        match self.report.cause() {
            worth_ui_host_native::UiNativeEventLoopRunDenial::EventLoopCreation => {
                UiNativePlatformStopReason::EventLoopCreation
            }
            worth_ui_host_native::UiNativeEventLoopRunDenial::WindowCreation => {
                UiNativePlatformStopReason::WindowCreation
            }
            worth_ui_host_native::UiNativeEventLoopRunDenial::GraphicsPreparation => {
                UiNativePlatformStopReason::GraphicsPreparation
            }
            worth_ui_host_native::UiNativeEventLoopRunDenial::ApplicationDriver => {
                UiNativePlatformStopReason::ApplicationDriver
            }
            worth_ui_host_native::UiNativeEventLoopRunDenial::EventLoopRun => {
                UiNativePlatformStopReason::EventLoopRun
            }
            worth_ui_host_native::UiNativeEventLoopRunDenial::IncompleteCleanup => {
                UiNativePlatformStopReason::IncompleteCleanup
            }
        }
    }

    pub const fn effect_posture(&self) -> worth_ui_host_native::UiNativeEffectPosture {
        self.report.effect_posture()
    }

    pub const fn peak_census(&self) -> worth_ui_host_native::UiNativeResourceCensus {
        self.report.peak_census()
    }

    pub const fn terminal_census(&self) -> worth_ui_host_native::UiNativeResourceCensus {
        self.report.terminal_census()
    }

    pub const fn client_cleanup_complete(&self) -> bool {
        self.report.client_cleanup_complete()
    }

    pub fn into_cleanup(self) -> Option<worth_ui_host_native::UiNativeEventLoopCleanup> {
        self.report.into_cleanup()
    }
}
