Background pacing cannot be forged from raw labels or diagnostic artifacts.

```compile_fail
use worth_store_io_scheduler::background_pacing::BackgroundIdleCapacityLease;

let raw_label = "compaction";
let _lease: BackgroundIdleCapacityLease = raw_label;
```

Later semantic lifecycle receipts are not S.6 pacing authority.

```compile_fail
use worth_store_blob_chunks::BlobDedupeReceipt;
use worth_store_io_scheduler::background_pacing::BackgroundIoPressureShape;

let receipt: BlobDedupeReceipt = todo!();
let _shape: BackgroundIoPressureShape = receipt;
```

Worker-local state cannot mint visible debt.

```compile_fail
use worth_store_io_scheduler::background_pacing::BackgroundIoDebt;

let worker_local_queue_depth = 4_u64;
let _debt: BackgroundIoDebt = worker_local_queue_depth;
```
