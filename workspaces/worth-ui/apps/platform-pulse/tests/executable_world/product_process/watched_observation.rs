use std::fmt;
use std::process::ExitStatus;
use std::time::{Duration, Instant};

use worth_ui_platform_pulse::observation_contract::PlatformPulseLifecycleObservationEnvelope;

use crate::external_observation::{
    PlatformPulseLifecycleStream, PlatformPulseLifecycleStreamFailure,
};

use super::{LivePlatformPulseProcess, PlatformPulseProcessLaunchFailure};

const PROCESS_POLL_SLICE: Duration = Duration::from_millis(25);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WatchedPulseTransition {
    GreenReplacement,
    MalformedPreservation,
    CanonicalBlueRecovery,
}

#[derive(Debug)]
pub(crate) enum WatchedPulseObservationFailure {
    Deadline(WatchedPulseTransition),
    ChildExited {
        expected: WatchedPulseTransition,
        status: ExitStatus,
    },
    ProcessPoll(PlatformPulseProcessLaunchFailure),
    Lifecycle(PlatformPulseLifecycleStreamFailure),
}

pub(crate) fn await_watched_observation(
    process: &mut LivePlatformPulseProcess,
    lifecycle: &mut PlatformPulseLifecycleStream,
    expected: WatchedPulseTransition,
    deadline: Instant,
) -> Result<PlatformPulseLifecycleObservationEnvelope, WatchedPulseObservationFailure> {
    loop {
        if let Some(status) = process
            .observed_exit()
            .map_err(WatchedPulseObservationFailure::ProcessPoll)?
        {
            return Err(WatchedPulseObservationFailure::ChildExited { expected, status });
        }
        let now = Instant::now();
        if now >= deadline {
            return Err(WatchedPulseObservationFailure::Deadline(expected));
        }
        let slice_deadline = now
            .checked_add(PROCESS_POLL_SLICE)
            .unwrap_or(deadline)
            .min(deadline);
        match lifecycle.next(slice_deadline) {
            Ok(observation) => return Ok(observation),
            Err(PlatformPulseLifecycleStreamFailure::Deadline) if slice_deadline < deadline => {}
            Err(PlatformPulseLifecycleStreamFailure::Deadline) => {
                return Err(WatchedPulseObservationFailure::Deadline(expected))
            }
            Err(failure) => return Err(WatchedPulseObservationFailure::Lifecycle(failure)),
        }
    }
}

impl fmt::Display for WatchedPulseObservationFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Deadline(expected) => {
                write!(
                    formatter,
                    "{expected:?} watcher observation deadline elapsed"
                )
            }
            Self::ChildExited { expected, status } => {
                write!(formatter, "child exited during {expected:?}: {status}")
            }
            Self::ProcessPoll(failure) => {
                write!(formatter, "poll child during watched transition: {failure}")
            }
            Self::Lifecycle(failure) => {
                write!(formatter, "watched lifecycle observation: {failure}")
            }
        }
    }
}
