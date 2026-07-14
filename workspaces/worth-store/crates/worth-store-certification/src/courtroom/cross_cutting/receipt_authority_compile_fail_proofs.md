Foundational residency boundary receipts are descriptive artifacts only.

They cannot be passed back into Store as resident-frame authority:

```compile_fail
use worth_store_buffer_pool::ResidentFrameTable;
use worth_store_certification::CompletedResidencyBoundaryReceipt;

fn needs_store_authority(_: ResidentFrameTable) {}
let receipt: CompletedResidencyBoundaryReceipt = unimplemented!();
needs_store_authority(receipt);
```

They cannot be passed back into Store as dirty-state authority:

```compile_fail
use worth_store_buffer_pool::DirtyPageState;
use worth_store_certification::CompletedResidencyBoundaryReceipt;

fn needs_dirty_authority(_: DirtyPageState) {}
let receipt: CompletedResidencyBoundaryReceipt = unimplemented!();
needs_dirty_authority(receipt);
```

They cannot be passed back into Store as lease authority:

```compile_fail
use worth_store_buffer_pool::PageLease;
use worth_store_certification::CompletedResidencyBoundaryReceipt;

fn needs_lease_authority(_: PageLease<'_>) {}
let receipt: CompletedResidencyBoundaryReceipt = unimplemented!();
needs_lease_authority(receipt);
```

They cannot be passed back into Store as allocation authority:

```compile_fail
use worth_store_buffer_pool::AllocationReceipt;
use worth_store_certification::CompletedResidencyBoundaryReceipt;

fn needs_allocation_authority(_: AllocationReceipt) {}
let receipt: CompletedResidencyBoundaryReceipt = unimplemented!();
needs_allocation_authority(receipt);
```

They cannot be passed back into Store as view-admission authority:

```compile_fail
use worth_store_buffer_pool::RecordViewAdmission;
use worth_store_certification::CompletedResidencyBoundaryReceipt;

fn needs_view_authority(_: RecordViewAdmission) {}
let receipt: CompletedResidencyBoundaryReceipt = unimplemented!();
needs_view_authority(receipt);
```

Provenance attachments are descriptive evidence, not resident-frame authority:

```compile_fail
use worth_store_buffer_pool::ResidentFrameTable;
use worth_store_certification::BufferPoolProvenanceAttachment;

fn needs_store_authority(_: ResidentFrameTable) {}
let provenance: BufferPoolProvenanceAttachment = unimplemented!();
needs_store_authority(provenance);
```

Provenance attachments are not dirty-state authority:

```compile_fail
use worth_store_buffer_pool::DirtyPageState;
use worth_store_certification::BufferPoolProvenanceAttachment;

fn needs_dirty_authority(_: DirtyPageState) {}
let provenance: BufferPoolProvenanceAttachment = unimplemented!();
needs_dirty_authority(provenance);
```

Provenance attachments are not lease authority:

```compile_fail
use worth_store_buffer_pool::PageLease;
use worth_store_certification::BufferPoolProvenanceAttachment;

fn needs_lease_authority(_: PageLease<'_>) {}
let provenance: BufferPoolProvenanceAttachment = unimplemented!();
needs_lease_authority(provenance);
```

Provenance attachments are not allocation authority:

```compile_fail
use worth_store_buffer_pool::AllocationReceipt;
use worth_store_certification::BufferPoolProvenanceAttachment;

fn needs_allocation_authority(_: AllocationReceipt) {}
let provenance: BufferPoolProvenanceAttachment = unimplemented!();
needs_allocation_authority(provenance);
```

Provenance attachments are not view-admission authority:

```compile_fail
use worth_store_buffer_pool::RecordViewAdmission;
use worth_store_certification::BufferPoolProvenanceAttachment;

fn needs_view_authority(_: RecordViewAdmission) {}
let provenance: BufferPoolProvenanceAttachment = unimplemented!();
needs_view_authority(provenance);
```

Layout posture reports are descriptive evidence, not resident-frame authority:

```compile_fail
use worth_store_buffer_pool::ResidentFrameTable;
use worth_store_certification::ZeroCopyLayoutPostureReport;

fn needs_store_authority(_: ResidentFrameTable) {}
let layout: ZeroCopyLayoutPostureReport = unimplemented!();
needs_store_authority(layout);
```

Layout posture reports are not dirty-state authority:

```compile_fail
use worth_store_buffer_pool::DirtyPageState;
use worth_store_certification::ZeroCopyLayoutPostureReport;

fn needs_dirty_authority(_: DirtyPageState) {}
let layout: ZeroCopyLayoutPostureReport = unimplemented!();
needs_dirty_authority(layout);
```

Layout posture reports are not lease authority:

```compile_fail
use worth_store_buffer_pool::PageLease;
use worth_store_certification::ZeroCopyLayoutPostureReport;

fn needs_lease_authority(_: PageLease<'_>) {}
let layout: ZeroCopyLayoutPostureReport = unimplemented!();
needs_lease_authority(layout);
```

Layout posture reports are not allocation authority:

```compile_fail
use worth_store_buffer_pool::AllocationReceipt;
use worth_store_certification::ZeroCopyLayoutPostureReport;

fn needs_allocation_authority(_: AllocationReceipt) {}
let layout: ZeroCopyLayoutPostureReport = unimplemented!();
needs_allocation_authority(layout);
```

Layout posture reports are not view-admission authority:

```compile_fail
use worth_store_buffer_pool::RecordViewAdmission;
use worth_store_certification::ZeroCopyLayoutPostureReport;

fn needs_view_authority(_: RecordViewAdmission) {}
let layout: ZeroCopyLayoutPostureReport = unimplemented!();
needs_view_authority(layout);
```

Materialization profile reports are descriptive evidence, not resident-frame authority:

```compile_fail
use worth_store_buffer_pool::ResidentFrameTable;
use worth_store_certification::MaterializationProfileReport;

fn needs_store_authority(_: ResidentFrameTable) {}
let profile: MaterializationProfileReport = unimplemented!();
needs_store_authority(profile);
```

Materialization profile reports are not dirty-state authority:

```compile_fail
use worth_store_buffer_pool::DirtyPageState;
use worth_store_certification::MaterializationProfileReport;

fn needs_dirty_authority(_: DirtyPageState) {}
let profile: MaterializationProfileReport = unimplemented!();
needs_dirty_authority(profile);
```

Materialization profile reports are not lease authority:

```compile_fail
use worth_store_buffer_pool::PageLease;
use worth_store_certification::MaterializationProfileReport;

fn needs_lease_authority(_: PageLease<'_>) {}
let profile: MaterializationProfileReport = unimplemented!();
needs_lease_authority(profile);
```

Materialization profile reports are not allocation authority:

```compile_fail
use worth_store_buffer_pool::AllocationReceipt;
use worth_store_certification::MaterializationProfileReport;

fn needs_allocation_authority(_: AllocationReceipt) {}
let profile: MaterializationProfileReport = unimplemented!();
needs_allocation_authority(profile);
```

Materialization profile reports are not view-admission authority:

```compile_fail
use worth_store_buffer_pool::RecordViewAdmission;
use worth_store_certification::MaterializationProfileReport;

fn needs_view_authority(_: RecordViewAdmission) {}
let profile: MaterializationProfileReport = unimplemented!();
needs_view_authority(profile);
```

External callers cannot construct resident-memory performance receipts from
copied counter fields:

```compile_fail
use worth_store_buffer_pool::ResidentFrameCounterSnapshot;
use worth_store_certification::ResidentMemoryPerformanceReceipt;

let counters: ResidentFrameCounterSnapshot = unimplemented!();
let _report = ResidentMemoryPerformanceReceipt::from_executed_counters(counters);
```

External callers cannot construct allocation performance receipts from copied
counter fields:

```compile_fail
use worth_store_buffer_pool::AllocationCounterSnapshot;
use worth_store_certification::AllocationEnvelopePerformanceReceipt;

let counters: AllocationCounterSnapshot = unimplemented!();
let _report = AllocationEnvelopePerformanceReceipt::from_executed_counters(counters);
```

External callers cannot construct copy performance receipts from copied counter
fields:

```compile_fail
use worth_store_buffer_pool::RecordCopyCounterSnapshot;
use worth_store_certification::CopyMaterializationPerformanceReceipt;

let counters: RecordCopyCounterSnapshot = unimplemented!();
let _report = CopyMaterializationPerformanceReceipt::from_executed_counters(counters);
```

External callers cannot construct layout posture reports from copied counter
fields:

```compile_fail
use worth_store_buffer_pool::RecordCopyCounterSnapshot;
use worth_store_certification::ZeroCopyLayoutPostureReport;

let counters: RecordCopyCounterSnapshot = unimplemented!();
let _report = ZeroCopyLayoutPostureReport::from_executed_copy_counters(counters);
```

External callers cannot construct materialization profile reports from copied
counter fields:

```compile_fail
use worth_store_buffer_pool::BufferPoolCounterSnapshot;
use worth_store_certification::{FoundationalEvidenceProfile, MaterializationProfileReport};

let counters: BufferPoolCounterSnapshot = unimplemented!();
let _report = MaterializationProfileReport::from_executed_counters(
    FoundationalEvidenceProfile::full(),
    counters,
);
```

External callers cannot construct provenance attachments from copied counter
fields:

```compile_fail
use worth_store_buffer_pool::BufferPoolCounterSnapshot;
use worth_store_certification::BufferPoolProvenanceAttachment;

let counters: BufferPoolCounterSnapshot = unimplemented!();
let _provenance = BufferPoolProvenanceAttachment::from_executed_counters(counters);
```

External callers cannot reassemble a completed boundary receipt from report
pieces:

```compile_fail
use worth_store_certification::{
    AllocationEnvelopePerformanceReceipt, BufferPoolProvenanceAttachment,
    CompletedResidencyBoundaryReceipt, CopyMaterializationPerformanceReceipt,
    MaterializationProfileReport, ResidentMemoryPerformanceReceipt,
    ZeroCopyLayoutPostureReport,
};

let _receipt = CompletedResidencyBoundaryReceipt::from_distinct_reports(
    unimplemented::<ResidentMemoryPerformanceReceipt>(),
    unimplemented::<AllocationEnvelopePerformanceReceipt>(),
    unimplemented::<CopyMaterializationPerformanceReceipt>(),
    unimplemented::<ZeroCopyLayoutPostureReport>(),
    unimplemented::<MaterializationProfileReport>(),
    unimplemented::<BufferPoolProvenanceAttachment>(),
);
```
