#![doc = include_str!("api_compile_fail_proofs.md")]
#![forbid(unsafe_code)]

#[cfg(feature = "legacy-s2-models")]
mod admission;
#[cfg(feature = "legacy-s2-models")]
mod allocation;
#[cfg(feature = "legacy-s2-models")]
mod background_work;
#[cfg(feature = "legacy-s2-models")]
mod background_work_budget;
#[cfg(feature = "legacy-s2-models")]
mod background_work_class;
mod budget;
mod budget_units;
#[cfg(feature = "legacy-s2-models")]
mod buffer_pool_evidence_source;
#[cfg(feature = "legacy-s2-models")]
mod dirty_pages;
#[cfg(feature = "legacy-s2-models")]
mod entry;
#[cfg(feature = "legacy-s2-models")]
mod eviction;
#[cfg(feature = "legacy-s2-models")]
mod lease_identity;
#[cfg(feature = "legacy-s2-models")]
mod page_lease;
#[cfg(feature = "legacy-s2-models")]
mod physical_entry_facts;
mod physical_residency;
#[cfg(feature = "legacy-s2-models")]
mod pinned_frame_view;
#[cfg(feature = "legacy-s2-models")]
mod pinned_page_lease;
#[cfg(feature = "legacy-s2-models")]
mod pinning;
#[cfg(feature = "legacy-s2-models")]
mod record_access;
#[cfg(feature = "legacy-s2-models")]
mod record_view;
#[cfg(feature = "legacy-s2-models")]
mod residency;
#[cfg(feature = "legacy-s2-models")]
mod residency_vocabulary;
#[cfg(feature = "legacy-s2-models")]
mod speculative_work;
#[cfg(feature = "legacy-s2-models")]
mod streaming_allocation;

#[cfg(all(test, feature = "legacy-s2-models"))]
mod buffer_pool_evidence_source_tests;

#[cfg(feature = "legacy-s2-models")]
pub use admission::{AdmittedBufferPoolEntry, BufferPoolAdmission};
#[cfg(feature = "legacy-s2-models")]
pub use allocation::{
    AllocationAdmission, AllocationDenial, AllocationGrant, AllocationReceipt, AllocationRequest,
    AllocationRequestKind, FixedMetadataGrant,
};
#[cfg(feature = "legacy-s2-models")]
pub use background_work::{
    AdmittedBackgroundEnvelope, BackgroundEnvelopeAdmission, BackgroundEnvelopeCounterSnapshot,
    BackgroundEnvelopeDenialKind, BackgroundEnvelopeRequest, BackgroundEnvelopeRequestBuilder,
    BackgroundMemoryInterferenceReport,
};
#[cfg(feature = "legacy-s2-models")]
pub use background_work_budget::BackgroundWorkBudgetSnapshot;
#[cfg(feature = "legacy-s2-models")]
pub use background_work_class::BackgroundWorkClass;
pub use budget::BufferPoolBudget;
pub use budget_units::{
    BudgetUnitDenial, CopiedByteCount, DirtyByteCount, DirtyPageBudget, DirtyPageCount,
    MaterializedByteCount, PinnedPageBudget, PinnedPageCount, ResidentByteCount,
    ResidentMemoryBudget,
};
#[cfg(feature = "legacy-s2-models")]
pub use buffer_pool_evidence_source::{
    BufferPoolCounterSnapshot, BufferPoolEvidenceSourceDenial, BufferPoolExecutedEvidenceSource,
};
#[cfg(feature = "legacy-s2-models")]
pub use dirty_pages::{
    DirtyPageAccessOrigin, DirtyPageCounterSnapshot, DirtyPageIdentity, DirtyPageState,
    DirtyPublicationPlan, DirtyPublicationReceipt, DirtyShutdownPosture, DirtyShutdownReport,
};
#[cfg(feature = "legacy-s2-models")]
pub use entry::{S2PhysicalResidencyEntry, S2PhysicalResidencyEntryBuilder};
#[cfg(feature = "legacy-s2-models")]
pub use eviction::{
    EvictionCandidateSet, EvictionCounterSnapshot, EvictionPlan, EvictionPressure,
    EvictionProtectionReason, EvictionProtectionSummary, EvictionReceipt, FrameProtectionReceipt,
    ProtectedFrameDenial,
};
#[cfg(feature = "legacy-s2-models")]
pub use lease_identity::{LeaseEpoch, LeaseScope, PageLeaseId};
#[cfg(feature = "legacy-s2-models")]
pub use page_lease::PageLease;
#[cfg(feature = "legacy-s2-models")]
pub use physical_entry_facts::S2PhysicalEntryFacts;
pub use physical_residency::{
    BufferPoolQueueDeclarationContext, BufferPoolQueueGroupingScope,
    BufferPoolQueueWriteDurability, BufferPoolReadQueueExecutionDeclaration,
    BufferPoolReadQueueExecutionKind, BufferPoolWritebackQueueExecutionDeclaration,
    CandidateFrameCleanAuthority, DirtyPhysicalFrame, ForegroundReadAllocationGrant,
    ForegroundWriteAllocationGrant, FrameWritebackCleanAuthority, OperationAllocationGrant,
    OperationAllocationObservation, PhysicalBoundedFrameAccess, PhysicalBoundedFrameFaultOwner,
    PhysicalBoundedFrameFaultWaiter, PhysicalBoundedFrameKey, PhysicalCandidateBatchAdmission,
    PhysicalCandidateBatchReservation, PhysicalCandidateFrameKey,
    PhysicalCandidateFrameReservation, PhysicalDirtyReplacementError,
    PhysicalDirtyReplacementReservation, PhysicalFrameAccess, PhysicalFrameFaultError,
    PhysicalFrameFaultOwner, PhysicalFrameFaultWaiter, PhysicalFrameKey, PhysicalFrameLease,
    PhysicalFrameLoadTerminal, PhysicalFrameLoadTerminalKind, PhysicalFrameLoadingIdentity,
    PhysicalOperationAllocationScope, PhysicalResidencyAllocationEventCounters,
    PhysicalResidencyAllocationEventObserver, PhysicalResidencyAllocationEventSnapshot,
    PhysicalResidencyCounters, PhysicalResidencyDenial, PhysicalResidencyDimension,
    PhysicalResidencyIncarnation, PhysicalResidencyLimits, PhysicalResidencyLimitsAdmissionDenial,
    PhysicalResidencyLimitsBuilder, PhysicalResidencyPool, PhysicalResidencyPoolOwner,
    PhysicalResidencyPressureDenial, PhysicalResidencyShutdown, PhysicalSpeculativeWorkKind,
    PhysicalWritebackClaim, PhysicalWritebackRangePosture, PrefetchResidencyGrant,
    ReadAheadFrameGrant, ReadAheadResidencyGrant, WriteBehindResidencyGrant,
};
#[cfg(feature = "legacy-s2-models")]
pub use pinned_frame_view::PinnedFrameView;
#[cfg(feature = "legacy-s2-models")]
pub use pinned_page_lease::PinnedPageLease;
#[cfg(feature = "legacy-s2-models")]
pub use pinning::{
    LeaseLeakReport, PinLifecycleCloseoutReport, PinLifecycleCounterSnapshot, UnpinnedPageReceipt,
};
#[cfg(feature = "legacy-s2-models")]
pub use record_access::{RecordCopyCounterSnapshot, RecordViewDenial, RecordViewDenialKind};
#[cfg(feature = "legacy-s2-models")]
pub use record_view::{
    BoundedCopyRecordView, RecordViewAccess, RecordViewAdmission, RecordViewMaterializationProfile,
    ZeroCopyRecordView,
};
#[cfg(feature = "legacy-s2-models")]
pub use residency::{
    BufferPoolEntryDenial, BufferPoolEntryDenialKind, ResidentFrameAdmission, ResidentFrameBytes,
    ResidentFrameCounterSnapshot, ResidentFrameDenial, ResidentFrameDenialKind,
    ResidentFrameGeneration, ResidentFrameHitMissReport, ResidentFrameIdentity,
    ResidentFrameLoadRequest, ResidentFrameResidence, ResidentFrameShortcutAttempt,
    ResidentFrameSize, ResidentFrameSlot, ResidentFrameTable, ResidentFrameTableCapacity,
    ResidentFrameToken, ResidentGenerationSeparationProof,
};
#[cfg(feature = "legacy-s2-models")]
pub use residency_vocabulary::{ResidencyAuthorityTerm, ResidencyVocabulary};
#[cfg(feature = "legacy-s2-models")]
pub use speculative_work::{
    PrefetchAdmission, PrefetchPlan, PrefetchRequest, PrefetchWindow, ReadAheadAdmission,
    ReadAheadPlan, ReadAheadRequest, SpeculativePhysicalWorkAdmission,
    SpeculativePhysicalWorkDenial, SpeculativePhysicalWorkDenialKind, SpeculativeResidencyDenial,
    SpeculativeWorkBudgetSnapshot, SpeculativeWorkCounterSnapshot, SpeculativeWorkReplayIdentity,
    SpeculativeWorkRequestDenial, WriteBehindAdmission, WriteBehindPlan, WriteBehindRequest,
};
#[cfg(feature = "legacy-s2-models")]
pub use streaming_allocation::{streaming_allocation_kind, streaming_window_allocation_receipt};
pub use worth_store_budgets::{
    AllocationByteBudget, AllocationCounterSnapshot, AllocationEnvelopeDeclaration,
    AllocationEnvelopeSet, AllocationScope, FixedMetadataReservation, ScopeAllocationCounters,
};
#[cfg(feature = "legacy-s2-models")]
pub use worth_store_physical_backend::{
    AccessPolicyBufferLifecycle, AccessPolicyBufferLifecycleKind,
};
