#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiNativePhysicalSignalCounters {
    pub(crate) admissions: u64,
    pub(crate) pending_observations: u64,
    pub(crate) completed_observations: u64,
    pub(crate) rejected_observations: u64,
    pub(crate) indeterminate_observations: u64,
    pub(crate) stale_observations: u64,
    pub(crate) retry_schedules: u64,
    pub(crate) timeout_observations: u64,
    pub(crate) cancellations: u64,
    pub(crate) supersessions: u64,
    pub(crate) recovery_schedules: u64,
    pub(crate) recovery_resolutions: u64,
}
