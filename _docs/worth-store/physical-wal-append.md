# Physical WAL Append

## What This Feature Is

Physical WAL append records a prepared Store mutation in the write-ahead log
before any data page is allowed to depend on it. Use it after durable mutation
preparation when you need an exact, inspectable WAL frame. The returned value
proves that the frame was appended; it does not yet prove that the frame
survived a power loss.

## Why You Use It

- Persist one mutation identity, idempotency lease, request fingerprint, and
  ordered redo set in one framed WAL member.
- Preserve exact WAL segment, byte-range, and LSN-range identity for later
  durability and data-ordering work.
- Distinguish a proven pre-effect rejection from an append whose physical fate
  needs inspection.

## Stable Entry Points

- `PhysicalRecordServing::record_submission()`
- `PhysicalRecordSubmission::issue_idempotency_key(...)`
- `PhysicalRecordSubmission::prepare_durable_append(...)`
- `PhysicalRecordSubmission::append_prepared_wal(...)`
- `PhysicalRecordSubmission::wal_observation()`

The principal result types are `PhysicalWalAppendOutcome`,
`WalAppendedPhysicalMutation`, `WalRangeReservedPhysicalMutation`, and
`PhysicalWalObservation`.

Do not call lower backend durability runtimes or path-bound WAL planners.
Those surfaces are mechanism or certification infrastructure, not an
alternative ordinary append API.

## Core Mental Model

Preparation decides what the mutation means. WAL reservation then assigns that
immutable meaning one member, one ordered redo sequence, one LSN range, and one
byte range. The Store carries those facts through Signal readiness, scheduler
admission, executor dispatch, the filesystem media effect, and exact
settlement.

`WalAppendedPhysicalMutation` means the complete requested bytes were observed
written at the declared range. It is intentionally weaker than a durable
mutation: no file barrier, data dispatch, root publication, completion, or
acknowledgment follows from append alone.

## How It Executes

```text
PreparedPhysicalMutation
  -> Store-owned WAL reservation
  -> Signal dependency readiness
  -> scheduler resource admission
  -> Store executor
  -> C.4 scheduled exact artifact append
  -> exact Store settlement
  -> WalAppendedPhysicalMutation
```

The scheduler admits resources but does not settle the mutation. The backend
performs the bytes effect but does not promote Store progression. Store
settlement checks the exact work identity, byte range, and payload digest
before constructing the appended type.

## Small Example

Assume `submission`, `placement`, `request`, and `batch` were obtained from an
admitted serving Store:

```rust
use worth_store::physical_runtime::{
    PhysicalRecordSubmission, PhysicalWalAppendFailureCause, PhysicalWalAppendOutcome,
    PhysicalWalReservationDenial, PreparedPhysicalMutation, WalAppendedPhysicalMutation,
    WalRangeReservedPhysicalMutation,
};

enum WalAppendDecision {
    Appended(WalAppendedPhysicalMutation),
    ReservationDenied {
        prepared: PreparedPhysicalMutation,
        cause: PhysicalWalReservationDenial,
    },
    ProvenNoEffect {
        prepared: PreparedPhysicalMutation,
        cause: PhysicalWalAppendFailureCause,
    },
    Inspect(WalRangeReservedPhysicalMutation),
}

fn append_one_wal_member(
    submission: &PhysicalRecordSubmission,
    prepared: PreparedPhysicalMutation,
) -> WalAppendDecision {
    match submission.append_prepared_wal(prepared) {
        PhysicalWalAppendOutcome::Appended(appended) => WalAppendDecision::Appended(appended),
        PhysicalWalAppendOutcome::ReservationDenied { prepared, cause } => {
            WalAppendDecision::ReservationDenied { prepared, cause }
        }
        PhysicalWalAppendOutcome::ProvenNoEffect { prepared, cause } => {
            WalAppendDecision::ProvenNoEffect { prepared, cause }
        }
        PhysicalWalAppendOutcome::Indeterminate { reserved } => {
            WalAppendDecision::Inspect(reserved)
        }
    }
}
```

Prepare the mutation first with `prepare_durable_append`. The example begins at
the consuming append boundary so each non-success branch can retain the exact
value it owns. Reservation and proven-no-effect denials both prove that this
attempt changed no WAL bytes, but retry eligibility is cause-specific. Retry
only after a transient cause is removed while the same runtime remains
authoritative. `PublicationAuthorityReleased` and `StaleRuntime` retain the
preparation for lifecycle or recovery handling; a reopened runtime must reject
that old runtime identity. An indeterminate result retains a reserved mutation
for inspection and must not be retried as though no bytes changed.

## Real Example

Use the settlement and observation together when recording operational
evidence:

```rust
fn inspect_wal_append(
    submission: &PhysicalRecordSubmission,
    appended: &WalAppendedPhysicalMutation,
) -> Result<(), &'static str> {
    let declaration = appended.reserved().declaration();
    let settlement = appended.settlement();

    assert_eq!(settlement.range(), declaration.artifact_range());
    assert_eq!(
        settlement.payload_digest(),
        declaration.payload_digest(),
    );

    let wal = submission
        .wal_observation()
        .ok_or("the serving Store has released its publication authority")?;

    assert!(wal.appended_frames() >= 1);
    assert_eq!(
        wal.last_lsn_end(),
        Some(declaration.lsn_range().end_exclusive().get()),
    );
    assert!(!wal.sealed_for_inspection());
    Ok(())
}
```

The declaration is the planned identity. The settlement is the exact completed
media effect. The observation is a bounded, read-only summary; it cannot append,
retry, unseal, or advance a mutation.

## How It Relates To Other Features

- Durable mutation preparation supplies the exact request and idempotency
  meaning consumed by WAL reservation.
- The physical work runtime supplies Signal, scheduling, execution, effect
  fate, and settlement without becoming WAL authority.
- A later WAL barrier consumes the appended mutation before data dispatch can
  become legal.
- Bounded WAL verification independently inspects a segment after the writer is
  no longer authoritative.

## Inspection And Debugging

`wal_observation()` reports the current segment and generation, frames and
bytes appended by this runtime, the valid byte prefix, the last assigned LSN
end, and whether allocation is sealed for inspection.

Safe-denial causes remain layer-specific. For example,
`SubmissionDeferred` reports the exact bounded work-capacity dimension and
limit, while `SchedulerReservationDenied` retains the exact scheduler
admission denial. `DependencyBlocked` retains both Signal's admission class
and its exact blocking condition rather than collapsing every dependency
posture into one label. Release or reduce the named pressure before retrying
the returned preparation. Do not reinterpret deferral as media failure.

An indeterminate append seals later allocation. Do not infer no effect from a
timeout, dropped completion, or partial write. Preserve the returned reserved
mutation and use the recovery or certification inspection path appropriate to
the deployment.

## Anti-Patterns

- Do not treat `WalAppendedPhysicalMutation` as file durability or caller
  acknowledgment.
- Do not dispatch page writes from a raw LSN comparison or WAL observation.
- Do not bypass Store through `StoreDurabilityRuntime`, a path-bound planner,
  or a direct filesystem append.
- Do not rebuild a request fingerprint from allocated WAL identity; allocation
  is attempt state, not request equivalence.
- Do not retry an indeterminate append as though it were proven no effect.

## Current Limits

- This surface stops after exact append. WAL barrier, WAL-before-data, and
  pageLSN authority are separate later transitions.
- Group commit, checkpoint capture, root publication, and physical
  acknowledgment are not provided by this API.
- Opening a Store that already contains WAL bytes currently seals ordinary WAL
  allocation for inspection; reconstruction and safe continuation belong to
  the recovery progression.

## Related Docs

- [Bounded Physical Record Access](./bounded-physical-record-access.md)
- [C.7 Durable Publication Join](./physical-reconstruction-c7-durable-publication-join.md)
- [C.7 Phase 3 Implementation Plan](./physical-reconstruction-c7-phase-3-implementation-plan.md)
