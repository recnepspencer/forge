use super::background_envelope_counters::BackgroundEnvelopeCounters;
use crate::{
    AllocationAdmission, AllocationReceipt, AllocationRequest, BackgroundEnvelopeCounterSnapshot,
    BackgroundEnvelopeDenialKind, BackgroundEnvelopeRequest, BackgroundMemoryInterferenceReport,
    BackgroundWorkBudgetSnapshot, BackgroundWorkClass,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedBackgroundEnvelope {
    work_class: BackgroundWorkClass,
    resident_frames: u32,
    resident_bytes: u64,
    pinned_pages: u32,
    allocation_bytes: u64,
    copied_bytes: u64,
    streaming_object_bytes: u64,
    streaming_window_bytes: u64,
    allocation_receipt: AllocationReceipt,
    counters: BackgroundEnvelopeCounterSnapshot,
}

impl AdmittedBackgroundEnvelope {
    pub const fn work_class(self) -> BackgroundWorkClass {
        self.work_class
    }

    pub const fn allocation_scope(self) -> crate::AllocationScope {
        self.work_class.allocation_scope()
    }

    pub const fn resident_frames(self) -> u32 {
        self.resident_frames
    }

    pub const fn resident_bytes(self) -> u64 {
        self.resident_bytes
    }

    pub const fn pinned_pages(self) -> u32 {
        self.pinned_pages
    }

    pub const fn allocation_bytes(self) -> u64 {
        self.allocation_bytes
    }

    pub const fn copied_bytes(self) -> u64 {
        self.copied_bytes
    }

    pub const fn streaming_object_bytes(self) -> u64 {
        self.streaming_object_bytes
    }

    pub const fn streaming_window_bytes(self) -> u64 {
        self.streaming_window_bytes
    }

    pub const fn allocation_receipt(self) -> AllocationReceipt {
        self.allocation_receipt
    }

    pub const fn counters(self) -> BackgroundEnvelopeCounterSnapshot {
        self.counters
    }

    pub const fn proves_scrub_correctness(self) -> bool {
        false
    }

    pub const fn proves_corruption_localization(self) -> bool {
        false
    }

    pub const fn proves_wal_recovery(self) -> bool {
        false
    }

    pub const fn proves_compaction_validity(self) -> bool {
        false
    }

    pub const fn proves_blob_lifecycle_completion(self) -> bool {
        false
    }

    pub const fn proves_repair_behavior(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackgroundEnvelopeAdmission {
    counters: BackgroundEnvelopeCounters,
}

impl BackgroundEnvelopeAdmission {
    pub const fn new() -> Self {
        Self {
            counters: BackgroundEnvelopeCounters::new(),
        }
    }

    pub const fn counters(self) -> BackgroundEnvelopeCounterSnapshot {
        self.counters.snapshot()
    }

    pub fn admit(
        &mut self,
        request: BackgroundEnvelopeRequest,
        budget: BackgroundWorkBudgetSnapshot,
        allocation: &mut AllocationAdmission,
    ) -> Result<AdmittedBackgroundEnvelope, BackgroundMemoryInterferenceReport> {
        self.counters.record_attempt(
            request.resident_frames(),
            request.resident_bytes(),
            request.bounded_pin_pages() + request.indefinite_pin_pages(),
            request.allocation_bytes(),
        );
        self.reject_foreground_resident_interference(request, budget)?;
        self.reject_indefinite_pin(request)?;
        self.reject_pin_budget_interference(request, budget)?;
        self.reject_whole_object_memory(request)?;
        self.reject_streaming_envelope_window_mismatch(request)?;

        let allocation_request = allocation_request_for(request).map_err(|denial| {
            self.deny(
                request.work_class(),
                BackgroundEnvelopeDenialKind::AllocationDenied(denial),
            )
        })?;
        let grant = allocation.admit(allocation_request).map_err(|denial| {
            self.deny(
                request.work_class(),
                BackgroundEnvelopeDenialKind::AllocationDenied(denial),
            )
        })?;
        let receipt = allocation.record_allocation(grant).map_err(|denial| {
            self.deny(
                request.work_class(),
                BackgroundEnvelopeDenialKind::AllocationDenied(denial),
            )
        })?;
        self.counters.record_admitted(request);
        Ok(AdmittedBackgroundEnvelope {
            work_class: request.work_class(),
            resident_frames: request.resident_frames(),
            resident_bytes: request.resident_bytes(),
            pinned_pages: request.bounded_pin_pages(),
            allocation_bytes: request.allocation_bytes(),
            copied_bytes: request.copied_bytes(),
            streaming_object_bytes: request.streaming_object_bytes(),
            streaming_window_bytes: request.streaming_window_bytes(),
            allocation_receipt: receipt,
            counters: self.counters(),
        })
    }

    fn reject_foreground_resident_interference(
        &mut self,
        request: BackgroundEnvelopeRequest,
        budget: BackgroundWorkBudgetSnapshot,
    ) -> Result<(), BackgroundMemoryInterferenceReport> {
        if request.resident_frames() <= budget.background_available_frames() {
            return Ok(());
        }
        self.counters.record_foreground_interference();
        Err(self.deny(
            request.work_class(),
            BackgroundEnvelopeDenialKind::ForegroundResidencyInterference {
                requested_frames: request.resident_frames(),
                background_available_frames: budget.background_available_frames(),
                foreground_reserved_frames: budget.foreground_reserved_frames(),
            },
        ))
    }

    fn reject_indefinite_pin(
        &mut self,
        request: BackgroundEnvelopeRequest,
    ) -> Result<(), BackgroundMemoryInterferenceReport> {
        if request.indefinite_pin_pages() == 0 {
            return Ok(());
        }
        self.counters.record_indefinite_pin_denial();
        Err(self.deny(
            request.work_class(),
            BackgroundEnvelopeDenialKind::IndefinitePinRequested {
                requested_pages: request.indefinite_pin_pages(),
            },
        ))
    }

    fn reject_pin_budget_interference(
        &mut self,
        request: BackgroundEnvelopeRequest,
        budget: BackgroundWorkBudgetSnapshot,
    ) -> Result<(), BackgroundMemoryInterferenceReport> {
        if request.bounded_pin_pages() <= budget.pin_budget_remaining() {
            return Ok(());
        }
        Err(self.deny(
            request.work_class(),
            BackgroundEnvelopeDenialKind::PinBudgetWouldBeExceeded {
                requested_pages: request.bounded_pin_pages(),
                pinned_pages_used: budget.pinned_pages_used(),
                pinned_page_budget: budget.pinned_page_budget(),
            },
        ))
    }

    fn reject_whole_object_memory(
        &mut self,
        request: BackgroundEnvelopeRequest,
    ) -> Result<(), BackgroundMemoryInterferenceReport> {
        let Some(object_bytes) = request.whole_object_bytes() else {
            return Ok(());
        };
        self.counters.record_whole_object_attempt();
        Err(self.deny(
            request.work_class(),
            BackgroundEnvelopeDenialKind::WholeObjectMemoryRequired {
                object_bytes,
                envelope_bytes: request.allocation_bytes(),
            },
        ))
    }

    fn reject_streaming_envelope_window_mismatch(
        &mut self,
        request: BackgroundEnvelopeRequest,
    ) -> Result<(), BackgroundMemoryInterferenceReport> {
        if request.work_class() != BackgroundWorkClass::LargeRecordStreaming {
            return Ok(());
        }
        match request
            .streaming_window_bytes()
            .cmp(&request.allocation_bytes())
        {
            std::cmp::Ordering::Less => Err(self.defer(
                request.work_class(),
                BackgroundEnvelopeDenialKind::StreamingEnvelopeExceedsWindow {
                    envelope_bytes: request.allocation_bytes(),
                    window_bytes: request.streaming_window_bytes(),
                },
            )),
            std::cmp::Ordering::Equal => Ok(()),
            std::cmp::Ordering::Greater => Err(self.defer(
                request.work_class(),
                BackgroundEnvelopeDenialKind::StreamingWindowExceedsEnvelope {
                    window_bytes: request.streaming_window_bytes(),
                    envelope_bytes: request.allocation_bytes(),
                },
            )),
        }
    }

    fn deny(
        &mut self,
        work_class: BackgroundWorkClass,
        kind: BackgroundEnvelopeDenialKind,
    ) -> BackgroundMemoryInterferenceReport {
        self.counters.record_denied();
        BackgroundMemoryInterferenceReport::new(work_class, kind, self.counters())
    }

    fn defer(
        &mut self,
        work_class: BackgroundWorkClass,
        kind: BackgroundEnvelopeDenialKind,
    ) -> BackgroundMemoryInterferenceReport {
        self.counters.record_deferred();
        BackgroundMemoryInterferenceReport::new(work_class, kind, self.counters())
    }
}

impl Default for BackgroundEnvelopeAdmission {
    fn default() -> Self {
        Self::new()
    }
}

fn allocation_request_for(
    request: BackgroundEnvelopeRequest,
) -> Result<AllocationRequest, crate::AllocationDenial> {
    let scope = request.work_class().allocation_scope();
    if request.work_class() == BackgroundWorkClass::LargeRecordStreaming {
        AllocationRequest::streaming_window(scope, request.streaming_window_bytes())
    } else {
        AllocationRequest::background_work_memory(scope, request.allocation_bytes())
    }
}
