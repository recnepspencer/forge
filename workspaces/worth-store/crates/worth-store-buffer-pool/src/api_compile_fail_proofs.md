Store-owned S.2 buffer-pool entry vocabulary.

Raw physical page ids are not ordinary S.2 entry authority:

```compile_fail
use worth_store_buffer_pool::PageLease;
use worth_store_physical_format::PhysicalPageId;

let page_id = PhysicalPageId::from_raw(7).unwrap();
let _lease = PageLease::new(page_id);
```

S.2 residency entry cannot be opened from raw page ids:

```compile_fail
use worth_store_buffer_pool::S2PhysicalResidencyEntry;
use worth_store_physical_format::PhysicalPageId;

let page_id = PhysicalPageId::from_raw(7).unwrap();
let _entry = S2PhysicalResidencyEntry::from_raw_page_id(page_id);
```

Foundational/profile labels cannot substitute for Store residency authority:

```compile_fail
use worth_store_buffer_pool::S2PhysicalResidencyEntry;

let _entry = S2PhysicalResidencyEntry::from_foundational_profile("platform-grade");
```

S.1 durable physical generations cannot prove S.2 resident-frame validity:

```compile_fail
use worth_store_buffer_pool::ResidentFrameTable;
use worth_store_physical_format::PhysicalGeneration;

let physical_generation = PhysicalGeneration::from_raw(1).unwrap();
let _resident = ResidentFrameTable::resident_frame_from_physical_generation(
    physical_generation,
);
```

S.2 resident-frame generations cannot validate persisted physical references:

```compile_fail
use worth_store_buffer_pool::ResidentFrameGeneration;
use worth_store_physical_format::PhysicalReferenceAuthority;

let resident_generation: ResidentFrameGeneration = todo!();
let _cell = PhysicalReferenceAuthority::s1().validate_page_slot(
    todo!(),
    resident_generation,
);
```

Resident byte size is derived from admitted S.1 header facts, not supplied
by ordinary callers:

```compile_fail
use worth_store_buffer_pool::ResidentFrameSize;

let _forged = ResidentFrameSize::bytes(4096).unwrap();
```

The resident-frame table is authority state and cannot be forked by clone:

```compile_fail
use worth_store_buffer_pool::ResidentFrameTable;

let table: ResidentFrameTable = todo!();
let _forked_authority = table.clone();
```

Pinned frame views cannot be forged from raw resident bytes:

```compile_fail
use worth_store_buffer_pool::PinnedFrameView;

let raw = b"resident bytes without pin";
let _view = PinnedFrameView::new(raw);
```

Access-policy lifecycle proof cannot be minted without a live
`PinnedPageLease`:

```compile_fail
use worth_store_buffer_pool::AccessPolicyBufferLifecycle;

let _proof = AccessPolicyBufferLifecycle::pinned_s2_lease();
```

`PinnedFrameView` cannot outlive its `PinnedPageLease`:

```compile_fail
use worth_store_buffer_pool::{PinnedFrameView, PinnedPageLease};

fn escape_view<'a>(pinned: &'a PinnedPageLease<'a>) -> PinnedFrameView<'static> {
    pinned.view().unwrap()
}
```

Leak reports cannot be promoted into normal unpin receipts:

```compile_fail
use worth_store_buffer_pool::{LeaseLeakReport, UnpinnedPageReceipt};

let leak: LeaseLeakReport = todo!();
let _receipt = UnpinnedPageReceipt::from_leak_report(leak);
```

Dirty publication receipts do not prove WAL, fsync, checkpoint, recovery,
or durability semantics:

```compile_fail
use worth_store_buffer_pool::DirtyPublicationReceipt;

let receipt: DirtyPublicationReceipt = todo!();
let _durable = receipt.proves_fsync_durability();
```

Dirty publication plans are lowered scheduling authority and cannot be
replayed after the scheduler consumes them:

```compile_fail
use worth_store_buffer_pool::{DirtyPublicationPlan, ResidentFrameTable};

fn schedule_twice(table: &mut ResidentFrameTable, plan: DirtyPublicationPlan) {
    let _ = table.record_dirty_write_scheduled(plan);
    let _ = table.record_dirty_write_scheduled(plan);
}
```

Dirty page state cannot be fabricated from raw resident or physical ids:

```compile_fail
use worth_store_buffer_pool::DirtyPageState;
use worth_store_physical_format::PhysicalPageId;

let page_id = PhysicalPageId::from_raw(9).unwrap();
let _dirty = DirtyPageState::from_raw_page_id(page_id);
```

Eviction candidates cannot be fabricated from persisted physical ids:

```compile_fail
use worth_store_buffer_pool::EvictionCandidateSet;
use worth_store_physical_format::PhysicalPageId;

let page_id = PhysicalPageId::from_raw(9).unwrap();
let _candidate = EvictionCandidateSet::from_physical_page_id(page_id);
```

Eviction plans are consumed by execution and cannot be replayed:

```compile_fail
use worth_store_buffer_pool::{EvictionPlan, ResidentFrameTable};

fn evict_twice(table: &mut ResidentFrameTable, plan: EvictionPlan) {
    let _ = table.record_eviction(plan);
    let _ = table.record_eviction(plan);
}
```

Heap allocation success cannot be promoted into allocation admission:

```compile_fail
use worth_store_buffer_pool::AllocationGrant;

let _grant = AllocationGrant::from_heap_success(4096);
```

Allocation admission is authority state and cannot be forked by clone:

```compile_fail
use worth_store_buffer_pool::AllocationAdmission;

let admission: AllocationAdmission = todo!();
let _forked_authority = admission.clone();
```

An operation-scope label is vocabulary, not physical allocation authority:

```compile_fail
use worth_store_buffer_pool::{
    PhysicalFrameKey, PhysicalOperationAllocationScope, PhysicalResidencyPool,
};

fn load_with_scope(
    pool: &PhysicalResidencyPool,
    key: PhysicalFrameKey,
) {
    let _ = pool.access_frame(
        PhysicalOperationAllocationScope::ForegroundRead,
        key,
    );
}
```

Candidate residency cannot be reserved from a raw operation-scope label:

```compile_fail
use worth_store_buffer_pool::{
    PhysicalCandidateFrameKey, PhysicalOperationAllocationScope, PhysicalResidencyPool,
};

fn reserve_candidate_with_scope(
    pool: &PhysicalResidencyPool,
    candidate: PhysicalCandidateFrameKey,
) {
    let _ = pool.reserve_candidate_frames(
        PhysicalOperationAllocationScope::ForegroundWrite,
        &[candidate],
    );
}
```

A generic operation grant—even one whose runtime label is
`ForegroundWrite`—is insufficient mutation authority:

```compile_fail
use worth_store_buffer_pool::{
    OperationAllocationGrant, PhysicalCandidateFrameKey, PhysicalResidencyPool,
};

fn reserve_candidate_with_generic_grant(
    pool: &PhysicalResidencyPool,
    grant: &OperationAllocationGrant,
    candidate: PhysicalCandidateFrameKey,
) {
    let _ = pool.reserve_candidate_frames(grant, &[candidate]);
}
```

A consumed operation allocation grant cannot authorize later admission:

```compile_fail
use worth_store_buffer_pool::{
    OperationAllocationGrant, PhysicalFrameKey, PhysicalResidencyPool,
};

fn load_after_grant_drop(
    pool: &PhysicalResidencyPool,
    grant: OperationAllocationGrant,
    key: PhysicalFrameKey,
) {
    drop(grant);
    let _ = pool.access_frame(&grant, key);
}
```

A candidate batch retains the exact grant that admitted its metadata. Per-frame
progression has no second-grant parameter through which another scope or
allocation can be substituted:

```compile_fail
use worth_store_buffer_pool::{
    OperationAllocationGrant, PhysicalCandidateBatchReservation,
    PhysicalCandidateFrameKey,
};

fn substitute_candidate_grant(
    batch: &mut PhysicalCandidateBatchReservation<'_>,
    other: &OperationAllocationGrant,
    candidate: PhysicalCandidateFrameKey,
) {
    let _ = batch.reserve_next(other, candidate);
}
```

Coalesced waiters have no source-loading authority:

```compile_fail
use worth_store_buffer_pool::PhysicalFrameFaultWaiter;

fn forge_second_source(waiter: PhysicalFrameFaultWaiter) {
    let _ = waiter.load(|_| Ok::<_, ()>(()));
}
```

Sole fault ownership cannot be forked:

```compile_fail
use worth_store_buffer_pool::PhysicalFrameFaultOwner;

fn fork_fault_owner(owner: PhysicalFrameFaultOwner) {
    let _second_source_authority = owner.clone();
}
```

Loading identities are observations, not constructible authority:

```compile_fail
use worth_store_buffer_pool::{PhysicalFrameLoadingIdentity, PhysicalResidencyIncarnation};

fn forge_loading_identity(pool: PhysicalResidencyIncarnation) {
    let _forged = PhysicalFrameLoadingIdentity {
        pool,
        ordinal: 1,
    };
}
```

Clean-to-dirty replacement cannot allocate or submit an external owning
`Vec`:

```compile_fail
use worth_store_buffer_pool::PhysicalFrameLease;

fn replace_from_external_vec(clean: PhysicalFrameLease, bytes: Vec<u8>) {
    let _ = clean.replace_with_dirty_candidate(bytes);
}
```

Dirty replacement cannot be authorized by a raw scope label:

```compile_fail
use worth_store_buffer_pool::{
    PhysicalFrameLease, PhysicalOperationAllocationScope,
};

fn replace_with_scope(clean: PhysicalFrameLease) {
    let _ = clean.begin_dirty_replacement(
        &PhysicalOperationAllocationScope::ForegroundWrite,
    );
}
```

A generic operation grant cannot authorize prefetch even when its runtime scope
is foreground-read:

```compile_fail
use worth_store_buffer_pool::{
    OperationAllocationGrant, PhysicalResidencyPool,
};
use worth_store_physical_format::RecordFrameCoordinate;

fn prefetch_with_erased_scope(
    pool: &PhysicalResidencyPool,
    grant: OperationAllocationGrant,
    coordinate: RecordFrameCoordinate,
) {
    let _ = pool.admit_prefetch(grant, coordinate);
}
```

Foreground-write authority cannot substitute for foreground-read authority:

```compile_fail
use worth_store_buffer_pool::{
    ForegroundWriteAllocationGrant, PhysicalResidencyPool,
};
use worth_store_physical_format::RecordFrameCoordinate;

fn prefetch_with_write_authority(
    pool: &PhysicalResidencyPool,
    grant: ForegroundWriteAllocationGrant,
    coordinate: RecordFrameCoordinate,
) {
    let _ = pool.admit_prefetch(grant, coordinate);
}
```

Caller-bound frame keys cannot choose or forge the Store identity admitted by
prefetch:

```compile_fail
use worth_store_buffer_pool::{
    ForegroundReadAllocationGrant, PhysicalFrameKey, PhysicalResidencyPool,
};

fn prefetch_with_caller_key(
    pool: &PhysicalResidencyPool,
    grant: ForegroundReadAllocationGrant,
    frame: PhysicalFrameKey,
) {
    let _ = pool.admit_prefetch(grant, frame);
}
```

Prefetch authority cannot be passed to a read-ahead frame entrypoint:

```compile_fail
use worth_store_buffer_pool::{
    PhysicalResidencyPool, PrefetchResidencyGrant,
};

fn substitute_speculative_kind(
    pool: &PhysicalResidencyPool,
    grant: &PrefetchResidencyGrant,
) {
    let _ = pool.access_read_ahead_frame(grant);
}
```

Prefetch authority also cannot construct read-ahead queue evidence:

```compile_fail
use worth_store_buffer_pool::{
    BufferPoolQueueDeclarationContext, BufferPoolReadQueueExecutionDeclaration,
    PrefetchResidencyGrant,
};

fn lower_prefetch_as_read_ahead(
    grant: &PrefetchResidencyGrant,
    context: BufferPoolQueueDeclarationContext,
) {
    let _ = BufferPoolReadQueueExecutionDeclaration::read_ahead(grant, context);
}
```

A per-frame read-ahead authority cannot escape the aggregate grant it borrows:

```compile_fail
use worth_store_buffer_pool::{ReadAheadFrameGrant, ReadAheadResidencyGrant};

fn escape_read_ahead_frame<'grant, 'coordinates>(
    grant: ReadAheadResidencyGrant<'coordinates>,
) -> ReadAheadFrameGrant<'grant, 'coordinates> {
    grant.frame(0).unwrap()
}
```

Writeback claims require consumed foreground-write allocation authority; a raw
frame collection opens no claim:

```compile_fail
use worth_store_buffer_pool::{PhysicalFrameKey, PhysicalResidencyPool};

fn claim_without_write_authority(
    pool: &PhysicalResidencyPool,
    frame: PhysicalFrameKey,
) {
    let _ = pool.claim_writeback(&[frame]);
}
```

Speculative grants are linear and cannot be cloned:

```compile_fail
use worth_store_buffer_pool::PrefetchResidencyGrant;

fn duplicate_prefetch(grant: PrefetchResidencyGrant) {
    let _duplicate = grant.clone();
}
```

Aggregate read-ahead grants are linear too:

```compile_fail
use worth_store_buffer_pool::ReadAheadResidencyGrant;

fn duplicate_read_ahead(grant: ReadAheadResidencyGrant<'_>) {
    let _duplicate = grant.clone();
}
```

Dirty replacement also rejects a scope-erased operation grant at compile time:

```compile_fail
use worth_store_buffer_pool::{OperationAllocationGrant, PhysicalFrameLease};

fn replace_with_generic_grant(
    clean: PhysicalFrameLease,
    grant: &OperationAllocationGrant,
) {
    let _ = clean.begin_dirty_replacement(grant);
}
```

A dirty-replacement reservation cannot outlive the concrete allocation grant
it borrows:

```compile_fail
use worth_store_buffer_pool::{
    ForegroundWriteAllocationGrant, PhysicalDirtyReplacementReservation, PhysicalFrameLease,
};

fn escape_grant<'a>(
    clean: PhysicalFrameLease,
    grant: ForegroundWriteAllocationGrant,
) -> PhysicalDirtyReplacementReservation<'a> {
    clean.begin_dirty_replacement(&grant).unwrap()
}
```

Dirty-replacement reservations are move-owned and cannot be cloned:

```compile_fail
use worth_store_buffer_pool::{
    ForegroundWriteAllocationGrant, PhysicalFrameLease,
};

fn clone_replacement(clean: PhysicalFrameLease, grant: &ForegroundWriteAllocationGrant) {
    let reservation = clean.begin_dirty_replacement(grant).unwrap();
    let _copy = reservation.clone();
}
```

Possession of a dirty candidate is not authority to declare it clean:

```compile_fail
use worth_store_buffer_pool::DirtyPhysicalFrame;

fn bypass_candidate_settlement(dirty: DirtyPhysicalFrame) {
    let _ = dirty.complete_candidate_publication();
}
```

Possession of a writeback claim is not authority to declare its frames clean:

```compile_fail
use worth_store_buffer_pool::PhysicalWritebackClaim;

fn bypass_writeback_settlement(claim: PhysicalWritebackClaim) {
    let _ = claim.complete_writeback();
}
```

Candidate-publication authority cannot substitute for frame-writeback
authority:

```compile_fail
use worth_store_buffer_pool::{
    CandidateFrameCleanAuthority, PhysicalWritebackClaim,
};

fn cross_settlement_authority(
    claim: PhysicalWritebackClaim,
    candidate: &CandidateFrameCleanAuthority,
) {
    let _ = claim.complete_writeback(candidate);
}
```

Fixed metadata exemptions cannot be forged from variable diagnostics:

```compile_fail
use worth_store_buffer_pool::FixedMetadataReservation;

let diagnostic_bytes = String::from("variable diagnostic payload");
let _reservation = FixedMetadataReservation::from_diagnostic_payload(diagnostic_bytes);
```

Zero-copy record views cannot be forged from raw bytes:

```compile_fail
use worth_store_buffer_pool::ZeroCopyRecordView;

let raw = b"record bytes without a pinned lease";
let _view = ZeroCopyRecordView::new(raw, todo!(), todo!());
```

Zero-copy record views cannot outlive the pinned lease they borrow through:

```compile_fail
use worth_store_buffer_pool::{
    PinnedPageLease, RecordViewMaterializationProfile, ZeroCopyRecordView,
};
use worth_store_physical_format::FramedRecordView;

fn escape_record_view<'a>(
    pinned: &'a mut PinnedPageLease<'a>,
    framed: FramedRecordView<'a>,
) -> ZeroCopyRecordView<'static> {
    pinned
        .zero_copy_record_view(framed, RecordViewMaterializationProfile::PhysicalBytesOnly)
        .unwrap()
}
```

Bounded copies cannot be made from heap success without allocation receipt:

```compile_fail
use worth_store_buffer_pool::ZeroCopyRecordView;

let view: ZeroCopyRecordView<'_> = todo!();
let heap_bytes = Vec::<u8>::new();
let _bounded = view.bounded_copy(heap_bytes);
```

Physical record views do not materialize semantic domain objects:

```compile_fail
use worth_store_buffer_pool::ZeroCopyRecordView;

struct DomainRecord;

let view: ZeroCopyRecordView<'_> = todo!();
let _domain: DomainRecord = view.into_domain_record();
```

Dirty mutation cannot regain table authority while a zero-copy record view
is live:

```compile_fail
use worth_store_buffer_pool::{
    RecordViewMaterializationProfile, ResidentFrameTable, ResidentFrameToken,
};
use worth_store_physical_format::FramedRecordView;

fn dirty_while_record_view_live<'a>(
    table: &'a mut ResidentFrameTable,
    view_token: ResidentFrameToken,
    dirty_token: ResidentFrameToken,
    framed: FramedRecordView<'a>,
) {
    let lease = table.lease_page(view_token).unwrap();
    let mut pinned = lease.pin().unwrap();
    let view = pinned
        .zero_copy_record_view(framed, RecordViewMaterializationProfile::PhysicalBytesOnly)
        .unwrap();
    let _ = table.mark_dirty(dirty_token);
    let _ = view.physical_record_bytes();
}
```

Dirty publication cannot regain table authority while a zero-copy record
view is live:

```compile_fail
use worth_store_buffer_pool::{
    DirtyPageIdentity, RecordViewMaterializationProfile, ResidentFrameTable,
    ResidentFrameToken,
};
use worth_store_physical_format::FramedRecordView;

fn publish_while_record_view_live<'a>(
    table: &'a mut ResidentFrameTable,
    view_token: ResidentFrameToken,
    dirty_identity: DirtyPageIdentity,
    framed: FramedRecordView<'a>,
) {
    let plan = table.plan_dirty_publication(dirty_identity).unwrap();
    let lease = table.lease_page(view_token).unwrap();
    let mut pinned = lease.pin().unwrap();
    let view = pinned
        .zero_copy_record_view(framed, RecordViewMaterializationProfile::PhysicalBytesOnly)
        .unwrap();
    let _ = table.record_dirty_write_scheduled(plan);
    let _ = view.physical_record_bytes();
}
```

Speculative physical work plans are lowered scheduling evidence and cannot be
replayed after admission consumes them:

```compile_fail
use worth_store_buffer_pool::{
    ReadAheadPlan, SpeculativePhysicalWorkAdmission, AllocationAdmission,
};

fn admit_read_ahead_twice(
    admission: &mut SpeculativePhysicalWorkAdmission,
    allocation: &mut AllocationAdmission,
    plan: ReadAheadPlan,
) {
    let _ = admission.record_read_ahead_admitted(plan, allocation);
    let _ = admission.record_read_ahead_admitted(plan, allocation);
}
```

Copied evidence counter fields cannot mint a fresh executed evidence source:

```compile_fail
use worth_store_buffer_pool::{BufferPoolCounterSnapshot, BufferPoolExecutedEvidenceSource};

let copied_report_counters: BufferPoolCounterSnapshot = todo!();
let _source = BufferPoolExecutedEvidenceSource::from_counters(copied_report_counters);
```
