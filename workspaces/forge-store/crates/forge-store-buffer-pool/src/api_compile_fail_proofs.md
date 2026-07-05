Store-owned S.2 buffer-pool entry vocabulary.

Raw physical page ids are not ordinary S.2 entry authority:

```compile_fail
use forge_store_buffer_pool::PageLease;
use forge_store_physical_format::PhysicalPageId;

let page_id = PhysicalPageId::from_raw(7).unwrap();
let _lease = PageLease::new(page_id);
```

S.2 residency entry cannot be opened from raw page ids:

```compile_fail
use forge_store_buffer_pool::S2PhysicalResidencyEntry;
use forge_store_physical_format::PhysicalPageId;

let page_id = PhysicalPageId::from_raw(7).unwrap();
let _entry = S2PhysicalResidencyEntry::from_raw_page_id(page_id);
```

Foundational/profile labels cannot substitute for Store residency authority:

```compile_fail
use forge_store_buffer_pool::S2PhysicalResidencyEntry;

let _entry = S2PhysicalResidencyEntry::from_foundational_profile("platform-grade");
```

S.1 durable physical generations cannot prove S.2 resident-frame validity:

```compile_fail
use forge_store_buffer_pool::ResidentFrameTable;
use forge_store_physical_format::PhysicalGeneration;

let physical_generation = PhysicalGeneration::from_raw(1).unwrap();
let _resident = ResidentFrameTable::resident_frame_from_physical_generation(
    physical_generation,
);
```

S.2 resident-frame generations cannot validate persisted physical references:

```compile_fail
use forge_store_buffer_pool::ResidentFrameGeneration;
use forge_store_physical_format::PhysicalReferenceAuthority;

let resident_generation: ResidentFrameGeneration = todo!();
let _cell = PhysicalReferenceAuthority::s1().validate_page_slot(
    todo!(),
    resident_generation,
);
```

Resident byte size is derived from admitted S.1 header facts, not supplied
by ordinary callers:

```compile_fail
use forge_store_buffer_pool::ResidentFrameSize;

let _forged = ResidentFrameSize::bytes(4096).unwrap();
```

The resident-frame table is authority state and cannot be forked by clone:

```compile_fail
use forge_store_buffer_pool::ResidentFrameTable;

let table: ResidentFrameTable = todo!();
let _forked_authority = table.clone();
```

Pinned frame views cannot be forged from raw resident bytes:

```compile_fail
use forge_store_buffer_pool::PinnedFrameView;

let raw = b"resident bytes without pin";
let _view = PinnedFrameView::new(raw);
```

Access-policy lifecycle proof cannot be minted without a live
`PinnedPageLease`:

```compile_fail
use forge_store_buffer_pool::AccessPolicyBufferLifecycle;

let _proof = AccessPolicyBufferLifecycle::pinned_s2_lease();
```

`PinnedFrameView` cannot outlive its `PinnedPageLease`:

```compile_fail
use forge_store_buffer_pool::{PinnedFrameView, PinnedPageLease};

fn escape_view<'a>(pinned: &'a PinnedPageLease<'a>) -> PinnedFrameView<'static> {
    pinned.view().unwrap()
}
```

Leak reports cannot be promoted into normal unpin receipts:

```compile_fail
use forge_store_buffer_pool::{LeaseLeakReport, UnpinnedPageReceipt};

let leak: LeaseLeakReport = todo!();
let _receipt = UnpinnedPageReceipt::from_leak_report(leak);
```

Dirty publication receipts do not prove WAL, fsync, checkpoint, recovery,
or durability semantics:

```compile_fail
use forge_store_buffer_pool::DirtyPublicationReceipt;

let receipt: DirtyPublicationReceipt = todo!();
let _durable = receipt.proves_fsync_durability();
```

Dirty publication plans are lowered scheduling authority and cannot be
replayed after the scheduler consumes them:

```compile_fail
use forge_store_buffer_pool::{DirtyPublicationPlan, ResidentFrameTable};

fn schedule_twice(table: &mut ResidentFrameTable, plan: DirtyPublicationPlan) {
    let _ = table.record_dirty_write_scheduled(plan);
    let _ = table.record_dirty_write_scheduled(plan);
}
```

Dirty page state cannot be fabricated from raw resident or physical ids:

```compile_fail
use forge_store_buffer_pool::DirtyPageState;
use forge_store_physical_format::PhysicalPageId;

let page_id = PhysicalPageId::from_raw(9).unwrap();
let _dirty = DirtyPageState::from_raw_page_id(page_id);
```

Eviction candidates cannot be fabricated from persisted physical ids:

```compile_fail
use forge_store_buffer_pool::EvictionCandidateSet;
use forge_store_physical_format::PhysicalPageId;

let page_id = PhysicalPageId::from_raw(9).unwrap();
let _candidate = EvictionCandidateSet::from_physical_page_id(page_id);
```

Eviction plans are consumed by execution and cannot be replayed:

```compile_fail
use forge_store_buffer_pool::{EvictionPlan, ResidentFrameTable};

fn evict_twice(table: &mut ResidentFrameTable, plan: EvictionPlan) {
    let _ = table.record_eviction(plan);
    let _ = table.record_eviction(plan);
}
```

Heap allocation success cannot be promoted into allocation admission:

```compile_fail
use forge_store_buffer_pool::AllocationGrant;

let _grant = AllocationGrant::from_heap_success(4096);
```

Allocation admission is authority state and cannot be forked by clone:

```compile_fail
use forge_store_buffer_pool::AllocationAdmission;

let admission: AllocationAdmission = todo!();
let _forked_authority = admission.clone();
```

Fixed metadata exemptions cannot be forged from variable diagnostics:

```compile_fail
use forge_store_buffer_pool::FixedMetadataReservation;

let diagnostic_bytes = String::from("variable diagnostic payload");
let _reservation = FixedMetadataReservation::from_diagnostic_payload(diagnostic_bytes);
```

Zero-copy record views cannot be forged from raw bytes:

```compile_fail
use forge_store_buffer_pool::ZeroCopyRecordView;

let raw = b"record bytes without a pinned lease";
let _view = ZeroCopyRecordView::new(raw, todo!(), todo!());
```

Zero-copy record views cannot outlive the pinned lease they borrow through:

```compile_fail
use forge_store_buffer_pool::{
    PinnedPageLease, RecordViewMaterializationProfile, ZeroCopyRecordView,
};
use forge_store_physical_format::FramedRecordView;

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
use forge_store_buffer_pool::ZeroCopyRecordView;

let view: ZeroCopyRecordView<'_> = todo!();
let heap_bytes = Vec::<u8>::new();
let _bounded = view.bounded_copy(heap_bytes);
```

Physical record views do not materialize semantic domain objects:

```compile_fail
use forge_store_buffer_pool::ZeroCopyRecordView;

struct DomainRecord;

let view: ZeroCopyRecordView<'_> = todo!();
let _domain: DomainRecord = view.into_domain_record();
```

Dirty mutation cannot regain table authority while a zero-copy record view
is live:

```compile_fail
use forge_store_buffer_pool::{
    RecordViewMaterializationProfile, ResidentFrameTable, ResidentFrameToken,
};
use forge_store_physical_format::FramedRecordView;

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
use forge_store_buffer_pool::{
    DirtyPageIdentity, RecordViewMaterializationProfile, ResidentFrameTable,
    ResidentFrameToken,
};
use forge_store_physical_format::FramedRecordView;

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
use forge_store_buffer_pool::{
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
use forge_store_buffer_pool::{BufferPoolCounterSnapshot, BufferPoolExecutedEvidenceSource};

let copied_report_counters: BufferPoolCounterSnapshot = todo!();
let _source = BufferPoolExecutedEvidenceSource::from_counters(copied_report_counters);
```

