#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BridgeExecutionBasisReadmissionCounters {
    preflight_check_count: usize,
    reservation_check_count: usize,
    signal_attempt_admission_count: usize,
    signal_attempt_check_count: usize,
    signal_queue_binding_count: usize,
    abort_count: usize,
    commit_count: usize,
}

impl BridgeExecutionBasisReadmissionCounters {
    pub(super) fn checked_preflight(&mut self) {
        self.preflight_check_count += 1;
    }

    pub(super) fn checked_reservation(&mut self) {
        self.reservation_check_count += 1;
    }

    pub(super) fn admitted_signal_attempt(&mut self) {
        self.signal_attempt_admission_count += 1;
    }

    pub(super) fn checked_signal_attempt(&mut self) {
        self.signal_attempt_check_count += 1;
    }

    pub(super) fn bound_signal_queue(&mut self) {
        self.signal_queue_binding_count += 1;
    }

    pub(super) fn aborted(&mut self) {
        self.abort_count += 1;
    }

    pub(super) fn committed(&mut self) {
        self.commit_count += 1;
    }

    pub const fn preflight_check_count(self) -> usize {
        self.preflight_check_count
    }

    pub const fn reservation_check_count(self) -> usize {
        self.reservation_check_count
    }

    pub const fn signal_attempt_admission_count(self) -> usize {
        self.signal_attempt_admission_count
    }

    pub const fn signal_attempt_check_count(self) -> usize {
        self.signal_attempt_check_count
    }

    pub const fn signal_queue_binding_count(self) -> usize {
        self.signal_queue_binding_count
    }

    pub const fn abort_count(self) -> usize {
        self.abort_count
    }

    pub const fn commit_count(self) -> usize {
        self.commit_count
    }
}
