Canonical physical-residency authority boundaries.

An operation-scope label is vocabulary, not allocation authority:

```compile_fail
use worth_store_buffer_pool::{
    PhysicalFrameKey, PhysicalOperationAllocationScope, PhysicalResidencyPool,
};

fn load_with_scope(pool: &PhysicalResidencyPool, key: PhysicalFrameKey) {
    let _ = pool.access_frame(
        PhysicalOperationAllocationScope::ForegroundRead,
        key,
    );
}
```

A generic operation grant cannot authorize candidate mutation:

```compile_fail
use worth_store_buffer_pool::{
    OperationAllocationGrant, PhysicalCandidateFrameKey, PhysicalResidencyPool,
};

fn reserve_with_generic_grant(
    pool: &PhysicalResidencyPool,
    grant: &OperationAllocationGrant,
    candidate: PhysicalCandidateFrameKey,
) {
    let _ = pool.reserve_candidate_frames(grant, &[candidate]);
}
```

A consumed allocation grant cannot authorize later frame access:

```compile_fail
use worth_store_buffer_pool::{
    OperationAllocationGrant, PhysicalFrameKey, PhysicalResidencyPool,
};

fn load_after_drop(
    pool: &PhysicalResidencyPool,
    grant: OperationAllocationGrant,
    key: PhysicalFrameKey,
) {
    drop(grant);
    let _ = pool.access_frame(&grant, key);
}
```

A coalesced waiter has no source-loading authority:

```compile_fail
use worth_store_buffer_pool::PhysicalFrameFaultWaiter;

fn forge_second_source(waiter: PhysicalFrameFaultWaiter) {
    let _ = waiter.load(|_| Ok::<_, ()>(()));
}
```

A bounded coalesced waiter has the same wait-only authority:

```compile_fail
use worth_store_buffer_pool::PhysicalBoundedFrameFaultWaiter;

fn forge_second_bounded_source(waiter: PhysicalBoundedFrameFaultWaiter) {
    let _ = waiter.load(
        |_| Ok::<_, ()>(32),
        |_| Ok::<_, ()>(()),
    );
}
```

Sole fault ownership cannot be cloned:

```compile_fail
use worth_store_buffer_pool::PhysicalFrameFaultOwner;

fn fork_fault_owner(owner: PhysicalFrameFaultOwner) {
    let _second_source_authority = owner.clone();
}
```

Clean-to-dirty replacement cannot accept an external owning buffer:

```compile_fail
use worth_store_buffer_pool::PhysicalFrameLease;

fn replace_from_vec(clean: PhysicalFrameLease, bytes: Vec<u8>) {
    let _ = clean.replace_with_dirty_candidate(bytes);
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

Speculative grants are linear:

```compile_fail
use worth_store_buffer_pool::PrefetchResidencyGrant;

fn duplicate_prefetch(grant: PrefetchResidencyGrant) {
    let _duplicate = grant.clone();
}
```

A dirty-replacement reservation cannot outlive its allocation grant:

```compile_fail
use worth_store_buffer_pool::{
    ForegroundWriteAllocationGrant, PhysicalDirtyReplacementReservation,
    PhysicalFrameLease,
};

fn escape_grant<'a>(
    clean: PhysicalFrameLease,
    grant: ForegroundWriteAllocationGrant,
) -> PhysicalDirtyReplacementReservation<'a> {
    clean.begin_dirty_replacement(&grant).unwrap()
}
```

Candidate-publication authority cannot substitute for writeback authority:

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

A generic operation grant cannot authorize dirty-generation capture:

```compile_fail
use worth_store_buffer_pool::{OperationAllocationGrant, PhysicalResidencyPool};

fn capture_with_generic_grant(
    pool: &PhysicalResidencyPool,
    grant: OperationAllocationGrant,
) {
    let session = pool.begin_dirty_generation_capture().unwrap();
    let _ = pool.capture_next_dirty_generation_slice(session, grant);
}
```
