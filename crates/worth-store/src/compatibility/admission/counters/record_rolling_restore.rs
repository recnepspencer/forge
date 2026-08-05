use super::*;

impl CompatibilityAdmissionCounters {
    pub(crate) fn record_rolling_window_admission(&mut self) {
        self.rolling_window_admission_count += 1;
        self.accepted_count += 1;
    }

    pub(crate) fn record_rolling_window_rejection(&mut self) {
        self.rolling_window_rejection_count += 1;
        self.rejected_count += 1;
    }

    pub(crate) fn record_rolling_multi_writer_rejection(&mut self) {
        self.rolling_multi_writer_rejection_count += 1;
        self.record_rolling_window_rejection();
    }

    pub(crate) fn record_mixed_version_skew(&mut self) {
        self.mixed_version_skew_count += 1;
    }

    pub(crate) fn record_restore_accept(&mut self) {
        self.restore_accept_count += 1;
        self.accepted_count += 1;
    }

    pub(crate) fn record_restore_rejection(&mut self) {
        self.restore_rejection_count += 1;
        self.rejected_count += 1;
    }

    pub(crate) fn record_restore_out_of_scope_scan_rejection(&mut self) {
        self.restore_out_of_scope_scan_count += 1;
        self.record_restore_rejection();
    }

    pub(crate) fn record_restore_publication_conflict_rejection(&mut self) {
        self.restore_publication_conflict_rejection_count += 1;
        self.record_restore_rejection();
    }

    pub(crate) fn record_disaster_recovery_truth_window(&mut self) {
        self.disaster_recovery_truth_window_count += 1;
    }

    pub(crate) fn record_disaster_recovery_derived_window(&mut self) {
        self.disaster_recovery_derived_window_count += 1;
    }

    pub(crate) fn record_authoritative_partial_truth_rejection(&mut self) {
        self.authoritative_partial_truth_rejection_count += 1;
        self.rejected_count += 1;
    }

}
