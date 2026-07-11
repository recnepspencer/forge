#![doc = include_str!("api_compile_fail_proofs.md")]
#![forbid(unsafe_code)]

mod access_policy_lifecycle;
mod admission;
mod allocation;
mod background_work;
mod background_work_budget;
mod background_work_class;
mod budget;
mod budget_units;
mod buffer_pool_evidence_source;
mod dirty_pages;
mod entry;
mod eviction;
mod lease_identity;
mod page_lease;
mod physical_entry_facts;
mod pinned_frame_view;
mod pinned_page_lease;
mod pinning;
mod record_access;
mod record_view;
mod residency;
mod residency_vocabulary;
mod s6_queue_work;
mod speculative_work;
mod streaming_allocation;

#[cfg(test)]
mod buffer_pool_evidence_source_tests;

pub use access_policy_lifecycle::{AccessPolicyBufferLifecycle, AccessPolicyBufferLifecycleKind};
pub use admission::{AdmittedBufferPoolEntry, BufferPoolAdmission};
pub use allocation::{
    AllocationAdmission, AllocationDenial, AllocationGrant, AllocationReceipt, AllocationRequest,
    AllocationRequestKind, FixedMetadataGrant,
};
pub use background_work::{
    AdmittedBackgroundEnvelope, BackgroundEnvelopeAdmission, BackgroundEnvelopeCounterSnapshot,
    BackgroundEnvelopeDenialKind, BackgroundEnvelopeRequest, BackgroundEnvelopeRequestBuilder,
    BackgroundMemoryInterferenceReport,
};
pub use background_work_budget::BackgroundWorkBudgetSnapshot;
pub use background_work_class::BackgroundWorkClass;
pub use budget::BufferPoolBudget;
pub use budget_units::{
    BudgetUnitDenial, CopiedByteCount, DirtyByteCount, DirtyPageBudget, DirtyPageCount,
    MaterializedByteCount, PinnedPageBudget, PinnedPageCount, ResidentByteCount,
    ResidentMemoryBudget,
};
pub use buffer_pool_evidence_source::{
    BufferPoolCounterSnapshot, BufferPoolEvidenceSourceDenial, BufferPoolExecutedEvidenceSource,
};
pub use dirty_pages::{
    DirtyPageAccessOrigin, DirtyPageCounterSnapshot, DirtyPageIdentity, DirtyPageState,
    DirtyPublicationPlan, DirtyPublicationReceipt, DirtyShutdownPosture, DirtyShutdownReport,
};
pub use entry::{S2PhysicalResidencyEntry, S2PhysicalResidencyEntryBuilder};
pub use eviction::{
    EvictionCandidateSet, EvictionCounterSnapshot, EvictionPlan, EvictionPressure,
    EvictionProtectionReason, EvictionProtectionSummary, EvictionReceipt, FrameProtectionReceipt,
    ProtectedFrameDenial,
};
pub use forge_store_budgets::{
    AllocationByteBudget, AllocationCounterSnapshot, AllocationEnvelopeDeclaration,
    AllocationEnvelopeSet, AllocationScope, FixedMetadataReservation, ScopeAllocationCounters,
};
pub use lease_identity::{LeaseEpoch, LeaseScope, PageLeaseId};
pub use page_lease::PageLease;
pub use physical_entry_facts::S2PhysicalEntryFacts;
pub use pinned_frame_view::PinnedFrameView;
pub use pinned_page_lease::PinnedPageLease;
pub use pinning::{
    LeaseLeakReport, PinLifecycleCloseoutReport, PinLifecycleCounterSnapshot, UnpinnedPageReceipt,
};
pub use record_access::{RecordCopyCounterSnapshot, RecordViewDenial, RecordViewDenialKind};
pub use record_view::{
    BoundedCopyRecordView, RecordViewAccess, RecordViewAdmission, RecordViewMaterializationProfile,
    ZeroCopyRecordView,
};
pub use residency::{
    BufferPoolEntryDenial, BufferPoolEntryDenialKind, ResidentFrameAdmission, ResidentFrameBytes,
    ResidentFrameCounterSnapshot, ResidentFrameDenial, ResidentFrameDenialKind,
    ResidentFrameGeneration, ResidentFrameHitMissReport, ResidentFrameIdentity,
    ResidentFrameLoadRequest, ResidentFrameResidence, ResidentFrameShortcutAttempt,
    ResidentFrameSize, ResidentFrameSlot, ResidentFrameTable, ResidentFrameTableCapacity,
    ResidentFrameToken, ResidentGenerationSeparationProof,
};
pub use residency_vocabulary::{ResidencyAuthorityTerm, ResidencyVocabulary};
pub use s6_queue_work::{
    BufferPoolQueueExecutionDeclaration, BufferPoolQueueExecutionKind, BufferPoolQueueGroupingScope,
};
pub use speculative_work::{
    PrefetchAdmission, PrefetchPlan, PrefetchRequest, PrefetchWindow, ReadAheadAdmission,
    ReadAheadPlan, ReadAheadRequest, SpeculativePhysicalWorkAdmission,
    SpeculativePhysicalWorkDenial, SpeculativePhysicalWorkDenialKind, SpeculativePhysicalWorkKind,
    SpeculativeResidencyDenial, SpeculativeWorkBudgetSnapshot, SpeculativeWorkCounterSnapshot,
    SpeculativeWorkReplayIdentity, SpeculativeWorkRequestDenial, WriteBehindAdmission,
    WriteBehindPlan, WriteBehindRequest,
};
pub use streaming_allocation::{streaming_allocation_kind, streaming_window_allocation_receipt};
