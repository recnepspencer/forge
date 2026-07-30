use std::fmt;

pub(super) enum PlatformPulseTerminalError {
    Preparation(crate::application::PlatformPulsePreparationDenial),
    NativeSurfaceLaunch(worth_ui::facade::app::WorthUiNativeApplicationShellLaunchDenial),
    SourceWatcher(worth_ui::facade::source::WorthUiFilesystemWatcherDenial),
    FrameExecution(String),
    UnexpectedInitialFrame,
    NativeRebind(worth_ui::facade::app::WorthUiNativeSourceRebindDenial),
    NativeProjection(super::projection::PlatformPulseProjectionRebindDenial),
    QueryLifecycle(crate::query_source::PlatformPulseQueryLifecycleDenial),
    QueryWatch(crate::query_source::PlatformPulseExternalValueWatchDenial),
    VisualIdentity(crate::visual_identity_execution::PlatformPulseVisualExecutionDenial),
    NativeInput(worth_ui_host_egui::UiEguiRawInputIngressStopReason),
    ObservationPublication,
}

impl fmt::Display for PlatformPulseTerminalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Preparation(denial) => write!(formatter, "application preparation: {denial}"),
            Self::NativeSurfaceLaunch(denial) => {
                write!(formatter, "native surface launch: {denial:?}")
            }
            Self::SourceWatcher(denial) => write!(formatter, "source watcher: {denial:?}"),
            Self::FrameExecution(detail) => {
                write!(formatter, "mounted frame execution: {detail}")
            }
            Self::UnexpectedInitialFrame => formatter.write_str("initial frame did not publish"),
            Self::NativeRebind(denial) => write!(formatter, "native source rebind: {denial:?}"),
            Self::NativeProjection(denial) => {
                write!(formatter, "native projection rebind: {denial}")
            }
            Self::QueryLifecycle(denial) => write!(formatter, "Query lifecycle: {denial}"),
            Self::QueryWatch(denial) => write!(formatter, "Query source watch: {denial}"),
            Self::VisualIdentity(denial) => write!(formatter, "visual identity pulse: {denial}"),
            Self::NativeInput(denial) => {
                write!(formatter, "native input observation: {denial:?}")
            }
            Self::ObservationPublication => {
                formatter.write_str("lifecycle observation publication")
            }
        }
    }
}
