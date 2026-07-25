#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiHostObservationRetentionSnapshot {
    pub(crate) retained_reports: usize,
    pub(crate) retained_bytes: usize,
    pub(crate) retained_report_limit: usize,
    pub(crate) retained_byte_limit: usize,
    pub(crate) quarantined_batches: usize,
    pub(crate) quarantined_bytes: usize,
    pub(crate) quarantine_count_limit: usize,
    pub(crate) quarantine_byte_limit: usize,
}
