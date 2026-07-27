use std::fmt;

use worth_ui_platform_pulse::observation_contract::{
    PlatformPulseLifecycleObservation, PlatformPulseLifecycleObservationEnvelope,
    PlatformPulseShutdownCompleted,
};

use crate::external_observation::{
    LifecycleStreamMeasurement, NormalNativeCloseRequestObservation,
};
use crate::installation::PulseInstallationCleanupEvidence;
use crate::product_process::SuccessfulPlatformPulseExit;

#[derive(Debug)]
pub(crate) struct ExecutableLifecycleCleanupEvidence {
    close_request: NormalNativeCloseRequestObservation,
    shutdown_envelope: PlatformPulseLifecycleObservationEnvelope,
    shutdown: PlatformPulseShutdownCompleted,
    lifecycle_measurement: LifecycleStreamMeasurement,
    successful_exit: SuccessfulPlatformPulseExit,
    installation_cleanup: PulseInstallationCleanupEvidence,
}

pub(crate) struct CausalLifecycleCleanupObservationSet {
    process_id: u32,
    close_request: NormalNativeCloseRequestObservation,
    shutdown_envelope: PlatformPulseLifecycleObservationEnvelope,
    lifecycle_measurement: LifecycleStreamMeasurement,
}

pub(crate) struct ExecutableLifecycleCleanupObservationSet {
    causal: CausalLifecycleCleanupObservationSet,
    successful_exit: SuccessfulPlatformPulseExit,
    installation_cleanup: PulseInstallationCleanupEvidence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExecutableLifecycleCleanupFailure {
    MissingShutdownCompletion,
    ProcessIdentityMismatch,
    HostSessionNotReleased,
    MountedPresentationNotQuiescent(u64),
    NoReleasedSurface,
    InstallationResidue,
}

impl fmt::Display for ExecutableLifecycleCleanupFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingShutdownCompletion => {
                formatter.write_str("terminal lifecycle event was not shutdown completion")
            }
            Self::ProcessIdentityMismatch => {
                formatter.write_str("normal-close request belongs to a different process")
            }
            Self::HostSessionNotReleased => formatter.write_str("host session was not released"),
            Self::MountedPresentationNotQuiescent(count) => write!(
                formatter,
                "{count} exceptional mounted presentation shutdown attempt(s) remained"
            ),
            Self::NoReleasedSurface => formatter.write_str("shutdown released no host surface"),
            Self::InstallationResidue => {
                formatter.write_str("isolated source installation remained after cleanup")
            }
        }
    }
}

pub(crate) fn adjudicate_lifecycle_cleanup(
    observations: ExecutableLifecycleCleanupObservationSet,
) -> Result<ExecutableLifecycleCleanupEvidence, ExecutableLifecycleCleanupFailure> {
    let ExecutableLifecycleCleanupObservationSet {
        causal,
        successful_exit,
        installation_cleanup,
    } = observations;
    let shutdown = match causal.shutdown_envelope.outcome() {
        PlatformPulseLifecycleObservation::ShutdownCompleted(shutdown) => *shutdown,
        _ => {
            return Err(ExecutableLifecycleCleanupFailure::MissingShutdownCompletion);
        }
    };
    if causal.close_request.process_id() != causal.process_id {
        return Err(ExecutableLifecycleCleanupFailure::ProcessIdentityMismatch);
    }
    if !shutdown.host_session_released() {
        return Err(ExecutableLifecycleCleanupFailure::HostSessionNotReleased);
    }
    if shutdown.mounted_shutdown_attempt_count() != 0 {
        return Err(
            ExecutableLifecycleCleanupFailure::MountedPresentationNotQuiescent(
                shutdown.mounted_shutdown_attempt_count(),
            ),
        );
    }
    if shutdown.released_surface_count() == 0 {
        return Err(ExecutableLifecycleCleanupFailure::NoReleasedSurface);
    }
    if !installation_cleanup.removed_owned_root() {
        return Err(ExecutableLifecycleCleanupFailure::InstallationResidue);
    }
    Ok(ExecutableLifecycleCleanupEvidence {
        close_request: causal.close_request,
        shutdown_envelope: causal.shutdown_envelope,
        shutdown,
        lifecycle_measurement: causal.lifecycle_measurement,
        successful_exit,
        installation_cleanup,
    })
}

impl CausalLifecycleCleanupObservationSet {
    pub(crate) fn new(
        process_id: u32,
        close_request: NormalNativeCloseRequestObservation,
        shutdown_envelope: PlatformPulseLifecycleObservationEnvelope,
        lifecycle_measurement: LifecycleStreamMeasurement,
    ) -> Self {
        Self {
            process_id,
            close_request,
            shutdown_envelope,
            lifecycle_measurement,
        }
    }

    pub(crate) fn join_resource_disposition(
        self,
        successful_exit: SuccessfulPlatformPulseExit,
        installation_cleanup: PulseInstallationCleanupEvidence,
    ) -> ExecutableLifecycleCleanupObservationSet {
        ExecutableLifecycleCleanupObservationSet {
            causal: self,
            successful_exit,
            installation_cleanup,
        }
    }
}

impl ExecutableLifecycleCleanupEvidence {
    pub(crate) fn close_request_count(&self) -> u32 {
        self.close_request.request_count()
    }

    pub(crate) fn shutdown_sequence(&self) -> u64 {
        self.shutdown_envelope.sequence().value()
    }

    pub(crate) fn shutdown(&self) -> PlatformPulseShutdownCompleted {
        self.shutdown
    }

    pub(crate) fn lifecycle_measurement(&self) -> LifecycleStreamMeasurement {
        self.lifecycle_measurement
    }

    pub(crate) fn successful_exit(&self) -> SuccessfulPlatformPulseExit {
        self.successful_exit
    }

    pub(crate) fn installation_removed(&self) -> bool {
        self.installation_cleanup.removed_owned_root()
    }
}
