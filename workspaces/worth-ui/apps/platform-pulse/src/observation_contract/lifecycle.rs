use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", content = "payload")]
pub enum PlatformPulseLifecycleObservation {
    ProcessStarted(PlatformPulseProcessStarted),
    FirstFramePublished(PlatformPulseFirstFramePublished),
    ReplacementPublished(PlatformPulseReplacementPublished),
    ReplacementDeniedPreserving(PlatformPulseReplacementPreserved),
    ShutdownCompleted(PlatformPulseShutdownCompleted),
    TerminalFailure(PlatformPulseTerminalFailure),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseProcessStarted {}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseSourceSnapshotObservation {
    final_package_digest: u64,
    event_burst_digest: u64,
    source_sequence: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseApplicationGenerationObservation {
    semantic_package_fingerprint: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseMountedFrameObservation {
    pub(super) diagnostic_value: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseFirstFramePublished {
    pub(super) source: PlatformPulseSourceSnapshotObservation,
    pub(super) generation: PlatformPulseApplicationGenerationObservation,
    pub(super) frame: PlatformPulseMountedFrameObservation,
    pub(super) actual_native_effect_count: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseReplacementPublished {
    pub(super) source: PlatformPulseSourceSnapshotObservation,
    pub(super) predecessor_generation: PlatformPulseApplicationGenerationObservation,
    pub(super) active_generation: PlatformPulseApplicationGenerationObservation,
    pub(super) successor_frame: PlatformPulseMountedFrameObservation,
    pub(super) actual_native_effect_count: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseReplacementPreserved {
    pub(super) source: PlatformPulseSourceSnapshotObservation,
    pub(super) active_generation: PlatformPulseApplicationGenerationObservation,
    pub(super) active_frame: PlatformPulseMountedFrameObservation,
    pub(super) denial_family: PlatformPulseReplacementDenialFamily,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseReplacementDenialFamily {
    DslCompilation,
    SourceIngress,
    RuntimePreparation,
    Candidate,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseShutdownCompleted {
    pub(super) watcher_backend: PlatformPulseWatcherBackendObservation,
    pub(super) observed_notification_count: u64,
    pub(super) mounted_shutdown_attempt_count: u64,
    pub(super) host_session_released: bool,
    pub(super) released_surface_count: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseWatcherBackendObservation {
    Fsevent,
    Inotify,
    Kqueue,
    ReadDirectoryChanges,
    OtherNative,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseTerminalFailure {
    family: PlatformPulseTerminalFailureFamily,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseTerminalFailureFamily {
    LaunchConfiguration(PlatformPulseLaunchConfigurationDenialKind),
    FilesystemWatcher,
    ApplicationPreparation,
    CandidateSubmission,
    NativeSurfaceLaunch,
    MountedFrameExecution,
    NativeApplicationReplacement,
    SourceWorkerPanicked,
    NativeEventLoop,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlatformPulseLaunchConfigurationDenial {
    UnexpectedArgument,
    MissingSourceRootValue,
    SurplusArgument,
    RelativeSourceRoot(PathBuf),
    MissingSourceRoot(PathBuf),
    SourceRootMetadataUnavailable(PathBuf),
    SourceRootNotDirectory(PathBuf),
    MissingEntrySource(PathBuf),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseLaunchConfigurationDenialKind {
    UnexpectedArgument,
    MissingSourceRootValue,
    SurplusArgument,
    RelativeSourceRoot,
    MissingSourceRoot,
    SourceRootMetadataUnavailable,
    SourceRootNotDirectory,
    MissingEntrySource,
}

impl PlatformPulseLaunchConfigurationDenial {
    pub fn kind(&self) -> PlatformPulseLaunchConfigurationDenialKind {
        match self {
            Self::UnexpectedArgument => {
                PlatformPulseLaunchConfigurationDenialKind::UnexpectedArgument
            }
            Self::MissingSourceRootValue => {
                PlatformPulseLaunchConfigurationDenialKind::MissingSourceRootValue
            }
            Self::SurplusArgument => PlatformPulseLaunchConfigurationDenialKind::SurplusArgument,
            Self::RelativeSourceRoot(_) => {
                PlatformPulseLaunchConfigurationDenialKind::RelativeSourceRoot
            }
            Self::MissingSourceRoot(_) => {
                PlatformPulseLaunchConfigurationDenialKind::MissingSourceRoot
            }
            Self::SourceRootMetadataUnavailable(_) => {
                PlatformPulseLaunchConfigurationDenialKind::SourceRootMetadataUnavailable
            }
            Self::SourceRootNotDirectory(_) => {
                PlatformPulseLaunchConfigurationDenialKind::SourceRootNotDirectory
            }
            Self::MissingEntrySource(_) => {
                PlatformPulseLaunchConfigurationDenialKind::MissingEntrySource
            }
        }
    }
}

macro_rules! accessors {
    ($type:ty, $($name:ident : $return:ty),+ $(,)?) => {
        impl $type {
            $(pub fn $name(&self) -> $return { self.$name })+
        }
    };
}

accessors!(
    PlatformPulseSourceSnapshotObservation,
    final_package_digest: u64,
    event_burst_digest: u64,
    source_sequence: u64,
);
accessors!(
    PlatformPulseApplicationGenerationObservation,
    semantic_package_fingerprint: u64,
);
accessors!(PlatformPulseMountedFrameObservation, diagnostic_value: u64);
accessors!(
    PlatformPulseFirstFramePublished,
    source: PlatformPulseSourceSnapshotObservation,
    generation: PlatformPulseApplicationGenerationObservation,
    frame: PlatformPulseMountedFrameObservation,
    actual_native_effect_count: u64,
);
accessors!(
    PlatformPulseReplacementPublished,
    source: PlatformPulseSourceSnapshotObservation,
    predecessor_generation: PlatformPulseApplicationGenerationObservation,
    active_generation: PlatformPulseApplicationGenerationObservation,
    successor_frame: PlatformPulseMountedFrameObservation,
    actual_native_effect_count: u64,
);
accessors!(
    PlatformPulseReplacementPreserved,
    source: PlatformPulseSourceSnapshotObservation,
    active_generation: PlatformPulseApplicationGenerationObservation,
    active_frame: PlatformPulseMountedFrameObservation,
    denial_family: PlatformPulseReplacementDenialFamily,
);
accessors!(
    PlatformPulseShutdownCompleted,
    watcher_backend: PlatformPulseWatcherBackendObservation,
    observed_notification_count: u64,
    mounted_shutdown_attempt_count: u64,
    host_session_released: bool,
    released_surface_count: u64,
);
accessors!(
    PlatformPulseTerminalFailure,
    family: PlatformPulseTerminalFailureFamily,
);

impl PlatformPulseProcessStarted {
    pub(super) fn new() -> Self {
        Self {}
    }
}

impl PlatformPulseSourceSnapshotObservation {
    pub(super) fn from_revision(
        revision: &worth_ui::facade::source::WorthUiSourcePackageRevision,
    ) -> Self {
        Self {
            final_package_digest: revision.final_package_digest(),
            event_burst_digest: revision.event_burst_digest(),
            source_sequence: revision.sequence(),
        }
    }
}

impl PlatformPulseApplicationGenerationObservation {
    pub(super) fn from_generation(
        generation: &worth_ui::facade::app::WorthUiPreparedApplicationGenerationIdentity,
    ) -> Self {
        Self {
            semantic_package_fingerprint: generation
                .semantic_package_identity()
                .narrowing_fingerprint(),
        }
    }
}

impl PlatformPulseTerminalFailure {
    pub(super) fn new(family: PlatformPulseTerminalFailureFamily) -> Self {
        Self { family }
    }
}
