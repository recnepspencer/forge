# Bounded Physical Record Access

## What This Feature Is

WORTH Store lets you give each physical Store instance an explicit memory
envelope before record serving begins. You declare the bytes, frame counts,
operation scopes, and speculative work the instance may use; Store admits that
declaration against the physical record format and gives you a sealed policy
that can enter initialization or open.

## Why You Use It

- Keep a Store instance inside a known physical-memory envelope.
- Give foreground, recovery, maintenance, verification, and blob work explicit
  operation ceilings.
- Reject incomplete or page-incompatible configuration before a buffer pool is
  constructed.

Use the canonical policy when its fixed envelope is appropriate. Supply an
explicit policy when deployment capacity or a physical adapter needs different
limits.

## Stable Entry Points

Import these from `worth_store::physical_runtime`:

- `PhysicalRecordResidencyPolicy::builder()`
- `PhysicalRecordResidencyPolicyBuilder`
- `AdmittedPhysicalRecordResidencyPolicy`
- `PhysicalRecordResidencyPolicyOutcome`
- `PhysicalRecordResidencyPolicyDenial`
- `PhysicalResidencyDimension`
- `PhysicalOperationAllocationScope`
- `PhysicalSpeculativeWorkKind`
- `PhysicalRecordInitialization::with_residency_policy(...)`
- `PhysicalRecordOpen::with_residency_policy(...)`
- `ServingPhysicalRuntime::residency_observation()`
- `PhysicalResidencyObservation`
- `PhysicalResidencyCounterSnapshot`
- `PhysicalResidencyAllocationSnapshot`
- `PhysicalResidencyAllocationEventSnapshot`
- `PhysicalRecordPressureBasis`
- `PhysicalRecordPressureEvidence`
- `PhysicalResidencyRetryPosture`
- `PhysicalRecordResidencyFailure`
- `PhysicalRecordResidencyFailureKind`
- `RecordReadSession`
- `PhysicalRecordChunkView<'session>`
- `PhysicalRecordChunkBasis`
- `RecordReadError::pressure()`
- `RecordAppendError::pressure()`
- `RecordAppendError::pressure_denial()`

`PhysicalRecordInitialization::new(...)` and `PhysicalRecordOpen::new(...)`
already carry the canonical admitted policy. An explicit declaration replaces
that policy; it does not create a second pool.

Pool construction, frame tables, eviction controls, allocation grants, and
lower residency snapshots are not application APIs. Store owns them and
publishes only read-only Store evidence.

## Core Mental Model

The physical record format owns page shape. The residency policy owns how much
live physical memory the Store instance may use. Store admits the policy
against the already-admitted format, then consumes both values when it creates
the instance's single buffer pool.

There are three forms:

1. `PhysicalRecordResidencyPolicy` starts a declaration.
2. `PhysicalRecordResidencyPolicyBuilder` is incomplete configuration and
   grants no runtime authority.
3. `AdmittedPhysicalRecordResidencyPolicy` proves that every required
   dimension was declared and that the declaration is internally consistent
   with the physical page size.

The admitted type is sealed. Callers cannot construct or forge it, and
initialize/open do not accept the raw builder. These are type boundaries, not
developer conventions.

After serving begins, `PhysicalResidencyObservation` reports the exact stable
Store identity, lifecycle generation, admitted policy, counter snapshot, and
allocation-event snapshot for that instance. The observation is evidence, not
authority: it cannot allocate, pin, evict, retry, write back, or alter limits.

Read and append pressure follows the same rule. `RecordReadDenial` and
`RecordAppendDenial` classify the result as `PhysicalPressure`; the enclosing
error exposes `PhysicalRecordPressureEvidence`. That evidence describes the
pre-effect world that rejected the work. It does not prove that a later retry
is safe and cannot be exchanged for runtime authority.

An operation scope identifies which physical activity owns temporary bytes. It
does not express priority, fairness, tenant identity, or semantic authority.
Speculative kinds distinguish prefetch, read-ahead, and write-behind capacity;
they do not create background workers or retry policy.

## How It Executes

1. Admit the physical record format.
2. Declare every residency dimension with nonzero values.
3. Call `admit(format)`.
4. Handle the typed outcome.
5. Attach only the admitted policy to initialization or open.
6. Store constructs one lower buffer pool for that Store identity.
7. During serving, inspect only Store-owned residency and pressure evidence.

Admission rejects:

- any omitted dimension;
- a byte category above `total_bytes`;
- a scope above `operation_bytes`;
- pinned, dirty, or speculative frame counts above `frame_entries`;
- resident, operation, or dirty-replacement capacity smaller than one physical
  page.

## Small Example

Use the canonical admitted policy by constructing the request normally:

```rust
use worth_store::physical_runtime::{
    AdmittedPhysicalRecordFormat, PhysicalRecordAccessPolicy,
    PhysicalRecordFormatDeclaration, PhysicalRecordOpen,
};

let format = AdmittedPhysicalRecordFormat::admit(
    PhysicalRecordFormatDeclaration::builder().admit().unwrap(),
);
let access = PhysicalRecordAccessPolicy::builder().admit(format).unwrap();
let request = PhysicalRecordOpen::new(format, access);
```

This is the minimal honest use because `PhysicalRecordOpen` already contains
an admitted policy. There is no raw default policy that can bypass admission.

## Real Example

This example declares a complete instance envelope and attaches the admitted
result to an open request:

```rust
use std::num::{NonZeroU32, NonZeroU64};
use worth_store::physical_runtime::{
    AdmittedPhysicalRecordFormat, PhysicalOperationAllocationScope as Scope,
    PhysicalRecordAccessPolicy, PhysicalRecordFormatDeclaration,
    PhysicalRecordOpen, PhysicalRecordResidencyPolicy,
    PhysicalSpeculativeWorkKind as Speculation, RecordReadError,
    ServingPhysicalRuntime,
};

let bytes = |value| NonZeroU64::new(value).unwrap();
let frames = |value| NonZeroU32::new(value).unwrap();

let format = AdmittedPhysicalRecordFormat::admit(
    PhysicalRecordFormatDeclaration::builder().admit().unwrap(),
);
let access = PhysicalRecordAccessPolicy::builder().admit(format).unwrap();

let residency = PhysicalRecordResidencyPolicy::builder()
    .total_bytes(bytes(65_536))
    .resident_bytes(bytes(16_384))
    .metadata_bytes(bytes(8_192))
    .frame_entries(frames(8))
    .pinned_frames(frames(8))
    .pin_leases(frames(2))
    .dirty_frames(frames(4))
    .dirty_replacement_bytes(bytes(16_384))
    .operation_bytes(bytes(16_384))
    .scope_bytes(Scope::ForegroundRead, bytes(16_384))
    .scope_bytes(Scope::ForegroundWrite, bytes(16_384))
    .scope_bytes(Scope::Recovery, bytes(16_384))
    .scope_bytes(Scope::Scrub, bytes(16_384))
    .scope_bytes(Scope::Maintenance, bytes(16_384))
    .scope_bytes(Scope::Verification, bytes(16_384))
    .scope_bytes(Scope::Blob, bytes(16_384))
    .speculative_frames(Speculation::Prefetch, frames(8))
    .speculative_frames(Speculation::ReadAhead, frames(8))
    .speculative_frames(Speculation::WriteBehind, frames(4))
    .admit(format)
    .into_result()
    .expect("deployment residency policy must fit the admitted format");

let request =
    PhysicalRecordOpen::new(format, access).with_residency_policy(residency);

fn inspect_residency(serving: &ServingPhysicalRuntime) {
    let observation = serving.residency_observation();
    let metadata = observation
        .allocations()
        .for_dimension(
            worth_store::physical_runtime::PhysicalResidencyDimension::MetadataBytes,
        );

    assert_eq!(observation.store_identity(), serving.store_identity());
    assert_eq!(
        metadata.active_units(),
        observation.counters().metadata_bytes(),
    );
}

fn inspect_read_pressure(error: &RecordReadError) {
    if let Some(pressure) = error.pressure() {
        eprintln!(
            "residency pressure: scope={:?} dimension={:?} requested={} limit={}",
            pressure.scope(),
            pressure.dimension(),
            pressure.requested(),
            pressure.limit(),
        );
    }
}
```

The format remains authoritative for page size. The admitted residency value
proves configuration completeness and consistency. The request retains that
proof until Store consumes it to construct the pool. The application never
receives frame-table, eviction, or allocation-grant authority.

In production code, match the policy outcome and report
`PhysicalRecordResidencyPolicyDenial` rather than using `expect`.
After open, `inspect_residency` uses only Store-owned evidence. The pressure
handler reads the pre-effect basis without treating it as permission to retry.
Append failures expose the same evidence through `RecordAppendError::pressure`.

## Reading Records Without Owning Them

`PhysicalRecordReader::open` and `open_external` return a
`RecordReadSession`. The session owns the read lifecycle, operation allocation,
logical cursor, and at most one current resident frame. It is not an owning
record buffer.

Borrow decoded payload directly when the consumer can finish with one chunk
before advancing:

```rust
use worth_store::physical_runtime::{
    RecordReadSession, RecordStreamFailure,
};

fn consume(mut session: RecordReadSession) -> Result<(), RecordStreamFailure> {
    while let Some(chunk) = session.next_chunk()? {
        let basis = chunk.basis();
        consume_payload(
            chunk.bytes(),
            chunk.logical_range(),
            basis.record(),
            basis.frame_coordinate(),
        );
    }
    Ok(())
}
```

An inline record yields one chunk. An extent yields one decoded payload chunk
per resident frame. `bytes()` excludes frame headers, extent metadata, and
neighboring inline slots. `logical_range()` names the exact range of the
logical record represented by those bytes.

The chunk mutably borrows its session. The compiler therefore rejects
advancing, copying from, moving, or dropping the session while either the
chunk or its borrowed byte slice remains live. The chunk and its basis have no
public constructors. A caller cannot fabricate them from bytes, coordinates,
generations, counters, or pressure evidence.

`PhysicalRecordChunkBasis` is observation only:

- `store_identity()` identifies the stable physical Store;
- `store_generation()` identifies its admitted serving lifecycle;
- `record()` identifies the logical physical record;
- `frame_coordinate()` identifies the exact durable artifact range. The
  artifact identity carries its durable generation.

The basis does not expose the pool incarnation, frame lease, allocation grant,
pin, eviction, fault, writeback, retry, or mutation authority.

Use `read_next` when caller-owned storage is required:

```rust
fn copy_bounded(
    mut session: RecordReadSession,
    target: &mut [u8],
) -> Result<(), RecordStreamFailure> {
    assert!(
        !target.is_empty(),
        "a bounded-copy buffer must distinguish progress from end of record",
    );
    loop {
        let count = session.read_next(target)?;
        if count == 0 {
            break;
        }
        consume_copy(&target[..count]);
    }
    Ok(())
}
```

`read_next` never allocates an owning result. It copies only into the supplied
slice and records the actual nonzero copy operation, byte count, and maximum
copy width. `next_chunk` records no copy. The methods share one cursor, so
interleaving them neither repeats nor skips bytes.

Store deliberately provides no whole-record `read_to_end`, `to_vec`,
`into_bytes`, `Arc<[u8]>`, or compatibility conversion. A caller that chooses
to accumulate chunks owns and budgets that separate allocation.

Chunk projection adds no Signal family, Foundational fact, or `worth-proof`
authority. A real cold fault still uses the existing private `ReadFault`
topology; a resident hit and borrowing decoded bytes create no synthetic work.
The safety proof is the concrete `RecordReadSession` ownership boundary plus
the Rust borrow lifetime.

## How It Relates To Other Features

- Format admission happens first because residency must fit at least one page.
- Placement and access policies remain separate. They do not imply memory
  capacity and cannot substitute for residency admission.
- Scheduler capacity controls admitted physical work. It does not own memory.
- Recovery, scrub, maintenance, verification, and blob code may receive scoped
  allocation outcomes in later C.6 work, but they do not receive the pool.
- Signal is used only after the private serving-residency capability reports a
  real miss. That miss reuses the existing `ReadFault` family with the exact
  root, artifact, frame, or scan projection basis. A hit and a coalesced waiter
  create no Signal request, scheduler admission, executor command, or media
  effect. Residency admission, counters, allocation events, and pressure
  evidence neither import Signal into the pool nor create a Signal family.
- Bounded artifact reads are classified by the pool before file-length
  discovery. Only the move-owned bounded fault owner may run the canonical
  metadata and exact-read work; hits and typed waiters receive the resolved
  exact lease without source authority. A composite record `open` may still
  perform its separate eager segment- or extent-completeness metadata
  validation. That validation is not bounded-frame source work and is counted
  independently.
- Every bounded terminal publication wakes participants already sleeping on
  the loading identity. Collision, pool-close, and rejected-completion paths
  retain the shared typed terminal and notify as one transition; a waiter
  cannot remain blocked behind terminal state.
- A bounded loading identity includes its admitted request limit. Only an
  equal-limit caller becomes a coalesced waiter. Another limit receives a
  pre-effect `BoundedLoadLimitConflict` carrying the active and requested
  limits, then may retry after resolution. Store projects that lower conflict
  as `PhysicalRecordResidencyFailureKind::FrameLoadConflict`; it does not call
  an unobserved length a mismatch.
- Candidate frames declare whether they cover one artifact fragment or the
  complete artifact. Store derives that declaration exhaustively from the
  candidate role: manifest blocks, root manifests, and catalog candidates are
  complete artifacts; inline pages and extent chunks are fragments. A complete
  candidate reserves its artifact alias while dirty, so bounded access denies
  before source work. Clean publication turns the reserved alias into a
  zero-source hit. Cancellation, discard, eviction, and catalog identity
  promotion remove or retarget the alias atomically. A fragment can never
  satisfy a bounded whole-artifact read merely because its offset is zero.
- Candidate declaration/order failures project as
  `PhysicalRecordResidencyFailureKind::CandidateContractConflict`, while
  complete-artifact promotion failures project as
  `PhysicalRecordResidencyFailureKind::FrameIdentityConflict`. Promotion
  revalidates offset-zero coverage and target artifact-alias availability
  before detaching the source, so a denial preserves both frame-table indexes
  and all residency accounting.
- Identity promotion may replace an exact target only when that target is
  resident, unpinned, and clean. A retained failed-loading target returns its
  exact `FrameLoadTerminated` terminal until the final waiter reconciles it.
  A live loading or candidate-reserved target returns
  `FrameIdentityOccupied`, which Store projects as
  `PhysicalRecordResidencyFailureKind::FrameIdentityConflict`. Neither path
  removes the source, steals in-progress authority, or releases resident
  accounting a second time.
- `worth-proof` supplies the Store policy-admission denial transition. The
  admitted policy is then retained by the Store owner. The lower buffer-pool
  crate has no direct `worth-proof` dependency or source-level proof API and
  receives only the admitted lower limits inside Store's private runtime
  boundary. Lower physical-owner dependencies retain their own transitive
  governed proof dependencies; those do not grant proof authority to the pool.
- Foundational is not used to classify physical pressure or frame residency.
  Those facts carry no semantic truth. Store uses the already-admitted
  Store-native projection basis only when it constructs real `ReadFault` work;
  no cache/residency aspect enters the pool. A dedicated Foundational
  frame-writeback basis belongs to C.6 Phase 5 and is not part of the current
  observation or pressure API.

## Inspection And Debugging

Before attaching the policy, inspect its public getters to confirm the admitted
values. If admission is denied, match the typed denial:

- `MissingRequiredDimension` names the omitted declaration.
- `CategoryExceedsTotal` identifies the byte category above the global
  declaration.
- `ScopeExceedsOperation` identifies the oversubscribed operation scope.
- `CountExceedsFrameEntries` identifies the frame-count relationship.
- `PageExceeds*` identifies which byte category cannot hold one admitted page.

During serving, call `residency_observation()`:

- `store_identity()`, `store_generation()`, and `admitted_policy()` identify
  the exact instance and envelope being observed.
- `counters()` exposes Store-owned current/peak byte and frame counts plus hit,
  fault, eviction, writeback, copy, denial, and speculative-work totals.
- `allocations().store_identity()` identifies the same Store.
- `allocations().for_dimension(dimension)` exposes exact attempts, admissions,
  releases, denials, allocator failures, admitted/released/denied units, and
  current active units for one dimension.

For a denied read or append, inspect `pressure()` on the error. The evidence
names the Store/optional record basis, Store generation, allocation scope,
pressure dimension, requested/admitted/limit values, retry posture, and
whether an effect may have started. `pressure_denial()` gives append callers
the unit classification without embedding evidence in the denial enum.

## Anti-Patterns

- Do not treat the raw builder as an admitted configuration.
- Do not choose `total_bytes` by summing independent maxima and assume all
  combinations are executable; it is one live envelope.
- Do not use scopes as priorities, tenants, or semantic categories.
- Do not make a second pool for recovery, verification, or blob work.
- Do not import `worth-store-buffer-pool` from an ordinary application or
  future adapter.
- Do not use pressure evidence as a retry token or allocation grant.
- Do not infer mutable pool control from a residency observation.
- Do not infer semantic residency, data completeness, or durability from
  physical frame residency.

## Current Limits

- Stable now: the complete declaration vocabulary, typed admission outcome,
  sealed admitted policy, format preflight, canonical admitted policy, and the
  initialize/open type boundary. The lower runtime enforces per-scope
  operation ceilings, the aggregate operation ceiling, the shared live-byte
  envelope, dirty-replacement capacity, and fixed allocation-event publication
  before allocation or state mutation. Store publishes identity-bound,
  policy-bearing residency observation and pre-effect read/append pressure
  evidence.
- Stable now: ordinary serving reads pass through one private
  `ServingFrameResidency`, reserve one loading identity before media work,
  distinguish exact and bounded hit/fault/coalesced outcomes, execute one
  canonical source load per cold identity, select victims deterministically
  through a checked legal-victim proof, release accounting exactly once, and
  refault through the same C.5.1 physical-work path. Bounded aliases and exact
  coordinates share one preallocated frame slot; their indexes and free-slot
  storage are admitted inside the declared metadata envelope before the pool
  opens. Whole-artifact candidate declarations install the same alias without
  duplicating the resident frame, while fragment declarations remain
  ineligible. Direct media reads remain bootstrap-only.
- Stable now: `RecordReadSession` exposes lease-scoped borrowed chunks and
  explicit bounded copies through one cursor. Inline and extent views carry
  Store generation, record identity, physical frame basis, and logical range
  without exposing pool authority.
- Later C.6 phases add ordinary dirty/writeback settlement and speculative work
  lowering. They are not promised by the current read API.
- Temporary `C6*` handoff and direct drain/writeback command surfaces are not
  ordinary application APIs and compile only with certification authority. The
  handoff's frame-read API and read-capable source were deleted in Phase 3; its
  remaining writeback responsibilities are replaced in their assigned later
  phases. Certification uses a separate responsibility-named residency probe
  that cannot be constructed by ordinary callers.
- Phase 2 cleanup removed scalar/default admission bypasses, bare budget
  denials, externally supplied dirty-replacement buffers, loose lifecycle
  ownership, and lower snapshot types from the Store observation facade.
- Phase 3 cleanup removed duplicate serving loader composition, fallback
  serving reads, and temporary handoff frame-read types.
- Phase 4 cleanup removed the alternate opened-record alias. The only public
  logical read lease is `RecordReadSession`; no compatibility name or owning
  conversion remains.
- This feature does not define WAL/checkpoint order, reconstruction, integrity,
  semantic stable reads, QoS, or blob protocol.

## Related Docs

- [C.6 Buffer-Pool Runtime Join](./physical-reconstruction-c6-buffer-pool-runtime-join.md)
- [C.5.1 Physical Store Work Runtime](./physical-reconstruction-c5-1-physical-store-work-runtime.md)
- [Physical Foundation Reconstruction Roadmap](./physical-foundation-reconstruction-roadmap.md)
