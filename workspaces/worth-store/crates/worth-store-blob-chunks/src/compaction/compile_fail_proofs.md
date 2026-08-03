An unpaced compaction basis cannot enter planning.

```compile_fail
use worth_store_blob_chunks::{BlobCompactionAuthority, BlobCompactionIntentBasis};

fn plan_unpaced(
    authority: &BlobCompactionAuthority,
    basis: BlobCompactionIntentBasis,
) {
    let _ = authority.plan_compaction(basis);
}
```

One scheduler lease cannot pace two compaction intents.

```compile_fail
use worth_store_blob_chunks::BlobCompactionIntentBasis;
use worth_store_io_scheduler::BackgroundIdleCapacityLease;

fn pace_twice(
    first_basis: BlobCompactionIntentBasis,
    second_basis: BlobCompactionIntentBasis,
    lease: BackgroundIdleCapacityLease,
) {
    let _first = first_basis.with_scheduler_pacing(lease);
    let _second = second_basis.with_scheduler_pacing(lease);
}
```

A blob consumer cannot self-admit compaction pacing from an unpaced basis.

```compile_fail
use worth_store_blob_chunks::{BlobCompactionIntent, BlobCompactionIntentBasis};

fn self_admit(basis: BlobCompactionIntentBasis) {
    let _intent = BlobCompactionIntent::admitted_compaction(basis);
}
```
