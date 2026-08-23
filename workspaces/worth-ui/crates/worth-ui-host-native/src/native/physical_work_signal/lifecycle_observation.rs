use super::counters::UiNativePhysicalSignalCounters;

/// Owner-issued totals for physical cancellation and recovery progression.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiNativePhysicalSignalLifecycleObservation {
    cancellations: u64,
    recovery_schedules: u64,
    recovery_resolutions: u64,
}

impl UiNativePhysicalSignalLifecycleObservation {
    pub(super) const fn from_counters(counters: UiNativePhysicalSignalCounters) -> Self {
        Self {
            cancellations: counters.cancellations,
            recovery_schedules: counters.recovery_schedules,
            recovery_resolutions: counters.recovery_resolutions,
        }
    }

    pub const fn cancellations(self) -> u64 {
        self.cancellations
    }

    pub const fn recovery_schedules(self) -> u64 {
        self.recovery_schedules
    }

    pub const fn recovery_resolutions(self) -> u64 {
        self.recovery_resolutions
    }
}
