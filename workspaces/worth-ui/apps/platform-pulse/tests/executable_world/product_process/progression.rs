use crate::adjudication::{
    ExecutableFirstFrameEvidence, ExecutableLifecycleCleanupEvidence,
    ExecutablePredecessorPreservationEvidence, ExecutableReplacementEvidence,
    ExecutableVisualClearEvidence, ExecutableVisualOverlayEvidence,
    ExecutableVisualRetirementEvidence, ExecutableVisualSnapshotEvidence,
    ExecutableVisualTraceEvidence,
};
use crate::external_observation::PlatformPulseLifecycleStream;
use crate::failure_teardown::{
    report_without_owned_resources, teardown_installed_world, NativeBoundFailureWorldResources,
    PulseExecutableWorldFailure, PulseExecutableWorldFailureReport, UnboundFailureWorldResources,
};
use crate::installation::{CanonicalPlatformPulse, IsolatedPulseInstallation};
use crate::native_platform::{WindowsNativePlatform, WindowsProcessBoundNativeClientArea};
use crate::source_delta::{
    AppliedPulseSourceDelta, CanonicalBlueRecoverySourceDelta, GreenPulseSourceDelta,
    MalformedPulseSourceDelta,
};
use std::time::{Duration, Instant};

use super::{CargoBuiltPlatformPulse, LivePlatformPulseProcess};

pub(crate) struct PulseExecutableWorld<State> {
    pub(super) state: State,
}

pub(crate) struct Installed {
    pub(super) installation: IsolatedPulseInstallation,
}

pub(crate) struct AwaitingFirstFrame {
    pub(super) installation: IsolatedPulseInstallation,
    pub(super) process: LivePlatformPulseProcess,
    pub(super) lifecycle: PlatformPulseLifecycleStream,
    pub(super) launch_started: Instant,
}

pub(crate) struct NativeBoundExecutableWorld {
    pub(super) installation: IsolatedPulseInstallation,
    pub(super) process: LivePlatformPulseProcess,
    pub(super) lifecycle: PlatformPulseLifecycleStream,
    pub(super) platform: WindowsNativePlatform,
    pub(super) native_client: WindowsProcessBoundNativeClientArea,
}

pub(crate) struct Published<Stage> {
    pub(super) world: NativeBoundExecutableWorld,
    pub(super) stage: Stage,
}

pub(crate) struct InitialBlue {
    pub(super) evidence: ExecutableFirstFrameEvidence,
    pub(super) launch_to_first_publication: Duration,
}

pub(crate) struct SnapshotCaptured<Stage> {
    pub(super) prior: Stage,
    pub(super) evidence: ExecutableVisualSnapshotEvidence,
}

pub(crate) struct IdentityTraced<Stage> {
    pub(super) snapshot: SnapshotCaptured<Stage>,
    pub(super) evidence: ExecutableVisualTraceEvidence,
}

pub(crate) struct OverlayPublished<Stage> {
    pub(super) trace: IdentityTraced<Stage>,
    pub(super) evidence: ExecutableVisualOverlayEvidence,
}

pub(crate) struct OverlayCleared<Stage> {
    pub(super) overlay: OverlayPublished<Stage>,
    pub(super) evidence: ExecutableVisualClearEvidence,
}

impl OverlayCleared<InitialBlue> {
    pub(super) fn initial(&self) -> &InitialBlue {
        &self.overlay.trace.snapshot.prior
    }

    pub(super) fn snapshot_evidence(&self) -> &ExecutableVisualSnapshotEvidence {
        &self.overlay.trace.snapshot.evidence
    }
}

pub(crate) struct AwaitingReplacement {
    pub(super) world: NativeBoundExecutableWorld,
    pub(super) initial: OverlayCleared<InitialBlue>,
    pub(super) action: AppliedPulseSourceDelta<GreenPulseSourceDelta>,
}

pub(crate) struct GreenSuccessor {
    pub(super) initial: OverlayCleared<InitialBlue>,
    pub(super) evidence: ExecutableReplacementEvidence<GreenPulseSourceDelta>,
    pub(super) retirement: ExecutableVisualRetirementEvidence,
}

pub(crate) struct AwaitingPreservation {
    pub(super) world: NativeBoundExecutableWorld,
    pub(super) green: GreenSuccessor,
    pub(super) action: AppliedPulseSourceDelta<MalformedPulseSourceDelta>,
}

pub(crate) struct PreservedPredecessor {
    pub(super) world: NativeBoundExecutableWorld,
    pub(super) green: GreenSuccessor,
    pub(super) evidence: ExecutablePredecessorPreservationEvidence,
}

pub(crate) struct AwaitingRecovery {
    pub(super) world: NativeBoundExecutableWorld,
    pub(super) preserved: PreservedPredecessorEvidence,
    pub(super) action: AppliedPulseSourceDelta<CanonicalBlueRecoverySourceDelta>,
}

pub(crate) struct PreservedPredecessorEvidence {
    pub(super) green: GreenSuccessor,
    pub(super) evidence: ExecutablePredecessorPreservationEvidence,
}

pub(crate) struct RecoveredBlue {
    pub(super) preserved: PreservedPredecessorEvidence,
    pub(super) evidence: ExecutableReplacementEvidence<CanonicalBlueRecoverySourceDelta>,
}

pub(crate) struct Closed {
    pub(super) evidence: ExecutableLifecycleCleanupEvidence,
}

impl PulseExecutableWorld<Installed> {
    pub(crate) fn install(
        canonical: CanonicalPlatformPulse,
    ) -> Result<Self, PulseExecutableWorldFailureReport> {
        let installation = IsolatedPulseInstallation::install(canonical).map_err(|failure| {
            report_without_owned_resources(PulseExecutableWorldFailure::Installation(failure))
        })?;
        Ok(Self {
            state: Installed { installation },
        })
    }

    pub(crate) fn launch(
        self,
        binary: CargoBuiltPlatformPulse,
    ) -> Result<PulseExecutableWorld<AwaitingFirstFrame>, PulseExecutableWorldFailureReport> {
        let installation = self.state.installation;
        let launch = match binary.launch(installation.source_root()) {
            Ok(launch) => launch,
            Err(failure) => {
                return Err(teardown_installed_world(
                    PulseExecutableWorldFailure::Launch(failure),
                    installation,
                ))
            }
        };
        Ok(PulseExecutableWorld {
            state: AwaitingFirstFrame {
                installation,
                process: launch.process,
                lifecycle: launch.lifecycle,
                launch_started: launch.launch_started,
            },
        })
    }
}

impl PulseExecutableWorld<Published<InitialBlue>> {
    pub(crate) fn evidence(&self) -> &ExecutableFirstFrameEvidence {
        &self.state.stage.evidence
    }

    pub(crate) fn launch_to_first_publication(&self) -> Duration {
        self.state.stage.launch_to_first_publication
    }
}

impl PulseExecutableWorld<Published<SnapshotCaptured<InitialBlue>>> {
    pub(crate) fn evidence(&self) -> &ExecutableVisualSnapshotEvidence {
        &self.state.stage.evidence
    }
}

impl PulseExecutableWorld<Published<IdentityTraced<InitialBlue>>> {
    pub(crate) fn evidence(&self) -> &ExecutableVisualTraceEvidence {
        &self.state.stage.evidence
    }
}

impl PulseExecutableWorld<Published<OverlayPublished<InitialBlue>>> {
    pub(crate) fn evidence(&self) -> &ExecutableVisualOverlayEvidence {
        &self.state.stage.evidence
    }
}

impl PulseExecutableWorld<Published<OverlayCleared<InitialBlue>>> {
    pub(crate) fn evidence(&self) -> &ExecutableVisualClearEvidence {
        &self.state.stage.evidence
    }

    pub(crate) fn initial_evidence(&self) -> &ExecutableFirstFrameEvidence {
        &self.state.stage.initial().evidence
    }
}

impl PulseExecutableWorld<Published<GreenSuccessor>> {
    pub(crate) fn evidence(&self) -> &ExecutableReplacementEvidence<GreenPulseSourceDelta> {
        &self.state.stage.evidence
    }

    pub(crate) fn retirement_evidence(&self) -> ExecutableVisualRetirementEvidence {
        self.state.stage.retirement
    }
}

impl PulseExecutableWorld<PreservedPredecessor> {
    pub(crate) fn evidence(&self) -> &ExecutablePredecessorPreservationEvidence {
        &self.state.evidence
    }
}

impl PulseExecutableWorld<Published<RecoveredBlue>> {
    pub(crate) fn evidence(
        &self,
    ) -> &ExecutableReplacementEvidence<CanonicalBlueRecoverySourceDelta> {
        &self.state.stage.evidence
    }

    pub(crate) fn preservation_evidence(&self) -> &ExecutablePredecessorPreservationEvidence {
        &self.state.stage.preserved.evidence
    }

    pub(crate) fn source_action_count(&self) -> u32 {
        self.state
            .stage
            .preserved
            .green
            .evidence
            .action()
            .action_count()
            + self.state.stage.preserved.evidence.action().action_count()
            + self.state.stage.evidence.action().action_count()
    }
}

impl PulseExecutableWorld<Closed> {
    pub(crate) fn evidence(&self) -> &ExecutableLifecycleCleanupEvidence {
        &self.state.evidence
    }
}

impl NativeBoundExecutableWorld {
    pub(super) fn into_failure_resources(self) -> NativeBoundFailureWorldResources {
        UnboundFailureWorldResources::new(self.installation, self.process, self.lifecycle)
            .bind_native(self.platform, self.native_client)
    }
}
