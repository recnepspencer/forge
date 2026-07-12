use crate::runtime::allocation_frame_dispatch::UiAllocationFrameMailboxStoragePosture;

/// Exact structural work completed by the bounded dispatcher.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiAllocationFrameDispatcherCounters {
    ingress_count: u64,
    frame_count: u64,
    late_ingress_count: u64,
    duplicate_count: u64,
    backpressure_denial_count: u64,
    terminal_denial_count: u64,
    identity_lookup_count: u64,
    sequence_lookup_count: u64,
    canonical_drain_count: u64,
    mailbox_order_comparison_count: u64,
    mailbox_canonical_write_count: u64,
    retry_ledger_comparison_count: u64,
    retry_ledger_write_count: u64,
    mailbox_storage_posture: UiAllocationFrameMailboxStoragePosture,
    mailbox_high_watermark: u16,
}

impl UiAllocationFrameDispatcherCounters {
    pub(in crate::runtime::allocation_frame_dispatch) fn empty(
        mailbox_storage_posture: UiAllocationFrameMailboxStoragePosture,
    ) -> Self {
        Self {
            ingress_count: 0,
            frame_count: 0,
            late_ingress_count: 0,
            duplicate_count: 0,
            backpressure_denial_count: 0,
            terminal_denial_count: 0,
            identity_lookup_count: 0,
            sequence_lookup_count: 0,
            canonical_drain_count: 0,
            mailbox_order_comparison_count: 0,
            mailbox_canonical_write_count: 0,
            retry_ledger_comparison_count: 0,
            retry_ledger_write_count: 0,
            mailbox_storage_posture,
            mailbox_high_watermark: 0,
        }
    }

    pub fn ingress_count(self) -> u64 {
        self.ingress_count
    }
    pub fn frame_count(self) -> u64 {
        self.frame_count
    }
    pub fn late_ingress_count(self) -> u64 {
        self.late_ingress_count
    }
    pub fn duplicate_count(self) -> u64 {
        self.duplicate_count
    }
    pub fn backpressure_denial_count(self) -> u64 {
        self.backpressure_denial_count
    }
    pub fn terminal_denial_count(self) -> u64 {
        self.terminal_denial_count
    }
    pub fn identity_lookup_count(self) -> u64 {
        self.identity_lookup_count
    }
    pub fn sequence_lookup_count(self) -> u64 {
        self.sequence_lookup_count
    }
    pub fn canonical_drain_count(self) -> u64 {
        self.canonical_drain_count
    }
    pub fn mailbox_order_comparison_count(self) -> u64 {
        self.mailbox_order_comparison_count
    }
    pub fn mailbox_canonical_write_count(self) -> u64 {
        self.mailbox_canonical_write_count
    }
    pub fn retry_ledger_comparison_count(self) -> u64 {
        self.retry_ledger_comparison_count
    }
    pub fn retry_ledger_write_count(self) -> u64 {
        self.retry_ledger_write_count
    }
    pub fn mailbox_capacity(self) -> u16 {
        self.mailbox_storage_posture.admitted_capacity()
    }
    pub fn mailbox_high_watermark(self) -> u16 {
        self.mailbox_high_watermark
    }
    pub fn mailbox_storage_posture(self) -> UiAllocationFrameMailboxStoragePosture {
        self.mailbox_storage_posture
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn record_accepted(
        &mut self,
        mailbox_len: u16,
    ) {
        self.ingress_count += 1;
        self.mailbox_high_watermark = self.mailbox_high_watermark.max(mailbox_len);
    }
    pub(in crate::runtime::allocation_frame_dispatch) fn record_frame(&mut self) {
        self.frame_count += 1;
    }
    pub(in crate::runtime::allocation_frame_dispatch) fn record_late(&mut self) {
        self.late_ingress_count += 1;
    }
    pub(in crate::runtime::allocation_frame_dispatch) fn record_duplicate(&mut self) {
        self.duplicate_count += 1;
    }
    pub(in crate::runtime::allocation_frame_dispatch) fn record_backpressure(&mut self) {
        self.backpressure_denial_count += 1;
    }
    pub(in crate::runtime::allocation_frame_dispatch) fn record_terminal_denial(&mut self) {
        self.terminal_denial_count += 1;
    }
    pub(in crate::runtime::allocation_frame_dispatch) fn record_identity_lookup(
        &mut self,
        slots_scanned: u64,
    ) {
        self.identity_lookup_count += slots_scanned;
    }
    pub(in crate::runtime::allocation_frame_dispatch) fn record_sequence_lookup(
        &mut self,
        slots_scanned: u64,
    ) {
        self.sequence_lookup_count += slots_scanned;
    }
    pub(in crate::runtime::allocation_frame_dispatch) fn record_canonical_drain(&mut self) {
        self.canonical_drain_count += 1;
    }
    pub(in crate::runtime::allocation_frame_dispatch) fn record_mailbox_insert(
        &mut self,
        comparisons: u64,
        canonical_writes: u64,
    ) {
        self.mailbox_order_comparison_count += comparisons;
        self.mailbox_canonical_write_count += canonical_writes;
    }
    pub(in crate::runtime::allocation_frame_dispatch) fn record_retry_ledger_work(
        &mut self,
        comparisons: u64,
        writes: u64,
    ) {
        self.retry_ledger_comparison_count += comparisons;
        self.retry_ledger_write_count += writes;
    }
}
