use std::fmt;
use std::thread;
use std::time::{Duration, Instant};

use crate::product_process::{LivePlatformPulseProcess, PlatformPulseProcessLaunchFailure};

const REQUIRED_LIVENESS_HOLD: Duration = Duration::from_millis(500);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct StableProcessLivenessObservation {
    process_id: u32,
    held_for: Duration,
    liveness_checks: u32,
}

#[derive(Debug)]
pub(crate) enum StableProcessLivenessFailure {
    Poll(PlatformPulseProcessLaunchFailure),
    ExitedBeforeHold,
    ExitedDuringHold,
}

impl fmt::Display for StableProcessLivenessFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Poll(failure) => write!(formatter, "liveness poll failed: {failure}"),
            Self::ExitedBeforeHold => {
                formatter.write_str("product exited before the liveness hold began")
            }
            Self::ExitedDuringHold => {
                formatter.write_str("product exited during the required liveness hold")
            }
        }
    }
}

pub(crate) fn observe_stable_process_liveness(
    process: &mut LivePlatformPulseProcess,
) -> Result<StableProcessLivenessObservation, StableProcessLivenessFailure> {
    if process
        .observed_exit()
        .map_err(StableProcessLivenessFailure::Poll)?
        .is_some()
    {
        return Err(StableProcessLivenessFailure::ExitedBeforeHold);
    }
    let started = Instant::now();
    thread::sleep(REQUIRED_LIVENESS_HOLD);
    if process
        .observed_exit()
        .map_err(StableProcessLivenessFailure::Poll)?
        .is_some()
    {
        return Err(StableProcessLivenessFailure::ExitedDuringHold);
    }
    Ok(StableProcessLivenessObservation {
        process_id: process.id(),
        held_for: started.elapsed(),
        liveness_checks: 2,
    })
}

impl StableProcessLivenessObservation {
    pub(crate) fn process_id(self) -> u32 {
        self.process_id
    }

    pub(crate) fn held_for(self) -> Duration {
        self.held_for
    }

    pub(crate) fn liveness_checks(self) -> u32 {
        self.liveness_checks
    }
}
