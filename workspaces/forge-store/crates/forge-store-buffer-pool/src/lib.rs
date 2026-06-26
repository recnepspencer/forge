#![doc = include_str!("api_compile_fail_proofs.md")]
#![forbid(unsafe_code)]

mod admission;
mod allocation_scope;
mod background_envelope_admission;
mod background_envelope_counters;
mod background_envelope_denials;
mod background_envelope_request;
mod background_work_budget;
mod background_work_class;
mod budget;
mod budget_units;
mod buffer_pool_evidence_source;
mod dirty_counters;
mod dirty_publication;
mod dirty_state;
mod entry;
mod entry_denials;
mod eviction;
mod eviction_counters;
mod lease_identity;
mod page_lease;
mod physical_entry_facts;
mod pin_counters;
mod pin_lifecycle;
mod pinned_frame_view;
mod pinned_page_lease;
mod record_view;
mod record_view_admission;
mod record_view_conflicts;
mod record_view_counters;
mod record_view_denials;
mod residency_vocabulary;
mod resident_frame_bytes;
mod resident_frame_counters;
mod resident_frame_denials;
mod resident_frame_dirty_table;
mod resident_frame_eviction_table;
mod resident_frame_identity;
mod resident_frame_lease_table;
mod resident_frame_record;
mod resident_frame_report;
mod resident_frame_request;
mod resident_frame_source;
mod resident_frame_table;
mod speculative_work_admission;
mod speculative_work_budget;
mod speculative_work_counters;
mod speculative_work_denials;
mod speculative_work_plan;
mod speculative_work_request;

#[cfg(test)]
mod allocation_scope_tests;
#[cfg(test)]
mod background_envelope_tests;
#[cfg(test)]
mod buffer_pool_evidence_source_tests;
#[cfg(test)]
mod dirty_state_test_support;
#[cfg(test)]
mod dirty_state_tests;
#[cfg(test)]
mod entry_tests;
#[cfg(test)]
mod eviction_tests;
#[cfg(test)]
mod pin_lifecycle_tests;
#[cfg(test)]
mod record_view_admission_tests;
#[cfg(test)]
mod resident_frame_tests;
#[cfg(test)]
mod speculative_work_honesty_tests;
#[cfg(test)]
mod speculative_work_tests;

pub use admission::{AdmittedBufferPoolEntry, BufferPoolAdmission};
pub use allocation_scope::{
    AllocationAdmission, AllocationDenial, AllocationGrant, AllocationReceipt, AllocationRequest,
    AllocationRequestKind, FixedMetadataGrant,
};
pub use background_envelope_admission::{AdmittedBackgroundEnvelope, BackgroundEnvelopeAdmission};
pub use background_envelope_counters::BackgroundEnvelopeCounterSnapshot;
pub use background_envelope_denials::{
    BackgroundEnvelopeDenialKind, BackgroundMemoryInterferenceReport,
};
pub use background_envelope_request::{
    BackgroundEnvelopeRequest, BackgroundEnvelopeRequestBuilder,
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
pub use dirty_counters::DirtyPageCounterSnapshot;
pub use dirty_publication::{DirtyPublicationPlan, DirtyPublicationReceipt};
pub use dirty_state::{
    DirtyPageIdentity, DirtyPageState, DirtyShutdownPosture, DirtyShutdownReport,
};
pub use entry::{S2PhysicalResidencyEntry, S2PhysicalResidencyEntryBuilder};
pub use entry_denials::{BufferPoolEntryDenial, BufferPoolEntryDenialKind};
pub use eviction::{
    EvictionCandidateSet, EvictionPlan, EvictionPressure, EvictionProtectionReason,
    EvictionProtectionSummary, EvictionReceipt, FrameProtectionReceipt, ProtectedFrameDenial,
};
pub use eviction_counters::EvictionCounterSnapshot;
pub use forge_store_budgets::{
    AllocationByteBudget, AllocationCounterSnapshot, AllocationEnvelopeDeclaration,
    AllocationEnvelopeSet, AllocationScope, FixedMetadataReservation, ScopeAllocationCounters,
};
pub use lease_identity::{LeaseEpoch, LeaseScope, PageLeaseId};
pub use page_lease::PageLease;
pub use physical_entry_facts::S2PhysicalEntryFacts;
pub use pin_counters::PinLifecycleCounterSnapshot;
pub use pin_lifecycle::{LeaseLeakReport, PinLifecycleCloseoutReport, UnpinnedPageReceipt};
pub use pinned_frame_view::PinnedFrameView;
pub use pinned_page_lease::PinnedPageLease;
pub use record_view::{
    BoundedCopyRecordView, RecordViewAccess, RecordViewAdmission, RecordViewMaterializationProfile,
    ZeroCopyRecordView,
};
pub use record_view_counters::RecordCopyCounterSnapshot;
pub use record_view_denials::{RecordViewDenial, RecordViewDenialKind};
pub use residency_vocabulary::{ResidencyAuthorityTerm, ResidencyVocabulary};
pub use resident_frame_bytes::ResidentFrameBytes;
pub use resident_frame_counters::ResidentFrameCounterSnapshot;
pub use resident_frame_denials::{
    ResidentFrameDenial, ResidentFrameDenialKind, ResidentFrameShortcutAttempt,
};
pub use resident_frame_identity::{
    ResidentFrameGeneration, ResidentFrameIdentity, ResidentFrameSlot, ResidentFrameToken,
};
pub use resident_frame_report::{
    ResidentFrameAdmission, ResidentFrameHitMissReport, ResidentFrameResidence,
    ResidentGenerationSeparationProof,
};
pub use resident_frame_request::{ResidentFrameLoadRequest, ResidentFrameSize};
pub use resident_frame_table::{ResidentFrameTable, ResidentFrameTableCapacity};
pub use speculative_work_admission::SpeculativePhysicalWorkAdmission;
pub use speculative_work_budget::SpeculativeWorkBudgetSnapshot;
pub use speculative_work_counters::SpeculativeWorkCounterSnapshot;
pub use speculative_work_denials::{
    SpeculativePhysicalWorkDenial, SpeculativePhysicalWorkDenialKind, SpeculativeResidencyDenial,
};
pub use speculative_work_plan::{
    PrefetchAdmission, PrefetchPlan, ReadAheadAdmission, ReadAheadPlan,
    SpeculativeWorkReplayIdentity, WriteBehindAdmission, WriteBehindPlan,
};
pub use speculative_work_request::{
    PrefetchRequest, PrefetchWindow, ReadAheadRequest, SpeculativePhysicalWorkKind,
    SpeculativeWorkRequestDenial, WriteBehindRequest,
};
