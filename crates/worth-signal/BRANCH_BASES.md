# Signal Branch Bases

## What This Feature Is

Signal branch bases let you name one exact, live branch state without exposing
mutable graph internals. A basis says which runtime, branch, Signal definition,
snapshot posture, and reference generation you observed. Use it when work must
fork, restore, advance, merge, or retain a particular Signal branch without silently
falling back to whatever branch is current later.

A **component obligation** (a retention lease) is the second half of that story.
It lets one component keep an exact past state available while the branch itself
moves on without it.

## Why You Use It

- Fork background or preview work from the exact state you inspected.
- Reject a stale request after another transaction, snapshot, restore, or merge
  moves the branch reference.
- Carry a descriptive basis through JSON and ask the owning runtime to readmit
  it before it can operate.
- Keep one exact component state available to a downstream owner after the
  branch reference has moved past it.

## Stable Entry Points

Import owner artifacts from `worth_signal::facade::branch` and call these
methods on `SignalRuntime`:

- `observe_signal_branch_basis`
- `readmit_signal_branch_basis`
- `fork_signal_branch`
- `restore_signal_branch`
- `validate_signal_basis_compatibility`
- `retain_signal_component_basis`
- `release_signal_component_basis`
- `readmit_retained_signal_branch_basis`
- `signal_component_retention_terminal_counts`
- `advance_signal_branch`
- `merge` for guided planning and `merge_branch` for the full-branch shortcut
- `capture_signal_branch_snapshot`
- `reconstruct_signal_branch_snapshot` for pristine-runtime construction only
- `plan_signal_branch_retirement` and `retire_signal_branch`

`SignalBranchBasisDescriptor` is the transport form. It is descriptive data,
not permission to operate. `AdmittedSignalBranchBasis` is the runtime-issued
form accepted by governed operations.

`SignalSnapshotV1` is likewise portable data, not restore authority.
`capture_signal_branch_snapshot` returns an owner-bound
`AdmittedSignalBranchSnapshot`; ordinary `restore_signal_branch` accepts only
that admitted form. A fresh, pristine runtime may import portable snapshot
data through the distinct `reconstruct_signal_branch_snapshot` construction
lane, which validates its empty live basis and issues a new owner-bound
snapshot.

An admitted snapshot carries its own bounded branch-retention obligation.
Snapshot clones share that one obligation, and branch retirement remains
denied until the final admitted snapshot holder is dropped. Consuming the
admitted form into portable `SignalSnapshotV1` data releases that authority
and its retention. Owners that coordinate snapshot disposal with retirement
may use the snapshot-release planning lanes: planning borrows each concrete
admitted snapshot, validates its runtime and branch identity, and leaves every
snapshot usable on denial. The owner must release the declared snapshots
before executing the returned retirement plan; execution revalidates that no
undeclared or cloned authority still retains the branch.

Heavy branch snapshot states are also governed by a runtime-wide stored-
snapshot budget (`maximum_stored_branch_snapshots` on the runtime builder).
Capacity is checked before capture moves a branch reference. Exhaustion is a
typed `SnapshotCapacityExhausted` denial on the canonical capture lane, and
branch retirement reclaims that branch's stored states for subsequent use.

The older transaction-head tuple and strong basis artifact are engine details,
not production facade contracts.

## Core Mental Model

The `SignalRuntime` owns live branch truth. Its branch manager is the one
operational catalog for branch handles and reference generations. Graph
diagnostics may display that catalog, but they do not provide a second source
of currentness.

A **basis** is an exact observation plus owner proof. Cloning an admitted basis
shares the same immutable admission; it does not inspect the graph, reacquire a
snapshot, or run evaluation. That shared admission also carries one internal
retention obligation: every `Arc` clone shares it, and the obligation is
released only after the last clone is dropped.

A **descriptor** is a serializable copy of the observation. Serialization
deliberately drops operational authority. The receiving runtime checks owner,
definition, lifecycle, snapshot availability, and exact reference generation
before issuing a new admitted basis.

Observation and readmission each acquire exactly one bounded admission. If the
retention registry is full, observation reports
`SignalBranchBasisObservationDenial::RetentionUnavailable` and readmission
reports `SignalBranchBasisReadmissionDenial::UnavailableRetention`; neither
misclassifies the branch as unknown.

### Residency Is Not Currentness

Retention and readmission ask the runtime two different questions about the
same state, and the answers are independent:

```text
readmit_signal_branch_basis(descriptor)
    "Does this branch still select this state?"
    -> ReferenceMismatch once any operation moves the reference.

retain_signal_component_basis(&basis)
    "Is this exact immutable state still available?"
    -> Ok, and it never consults the branch reference at all.
```

An obligation over a historical target is therefore ordinary, not exceptional.
`SignalBranchRetentionAcquisitionDenial` has no stale-basis variant on purpose:
every refusal it can give is a fact about the owner, the branch lifecycle, the
Signal definition, or the continued availability of that exact target. How
current the target is never enters the decision.

A lease is consequently **not** evidence that its branch reference is current,
and it is not a private copy of the state. It is an obligation the owner
honors: that exact immutable state stays available, and branch retirement stays
denied, until the obligation ends.

### An Obligation Names One Exact Target

`SignalBranchRetentionLease` is deliberately not `Clone`. The obligation is the
value, so it cannot be duplicated and cannot be released twice. It authorizes
exactly the target it was opened over: `readmit_retained_signal_branch_basis`
refuses any other descriptor with `DescriptorMismatch`, and refuses a runtime
that did not issue it with `ForeignRetention`.

The same exact target may carry several independent obligations. Release
receipts keep them distinct: `remaining_target_leases` counts obligations still
held over that one exact target, while `remaining_branch_leases` counts
obligations still held over any target of the same branch.

### Acquire Before You Lose It

`readmit_retained_signal_branch_basis` is the only route back to an admitted
basis for a state the branch no longer selects, and it requires a live
obligation. Ordinary readmission answers the currentness question and refuses.

So retain **before** you drop the last admitted basis for a state you will
need. Once no admitted basis and no obligation remain, that exact target is no
longer reachable, and branch retirement is free to reclaim it.

### Both Terminal Paths End the Obligation

An obligation reaches a terminal state exactly once, by either path:

- `release_signal_component_basis` consumes it and returns a
  `SignalBranchRetentionReleaseReceipt`: the released target, the terminal
  outcome, and both remaining counts.
- Dropping it takes the same exactly-once terminal path, recorded as a dropped
  release instead of fabricating a receipt nobody asked for.

Forgetting to release is therefore not a leak. Both paths discharge identical
accounting and both unblock retirement; only explicit release yields governed
evidence. `signal_component_retention_terminal_counts` reports the owner's view
as `explicit_releases`, `dropped_releases`, `owner_loss_releases`, and
`unknown_lease_defenses`.

An obligation may also outlive the runtime that issued it. Its
`owner_posture()` becomes `Lost`, releasing it reports
`SignalBranchRetentionTerminalOutcome::OwnerUnavailable` rather than `Released`,
and `owner_terminal_counts()` stays readable so the loss remains observable.
The owner ledger survives the runtime; the retained state does not. A lost
owner cannot hand the target back, so `readmit_retained_signal_branch_basis`
reports `UnavailableRetainedTarget`.

Branch retirement is denied while external obligations remain. Retirement
planning consumes the branch's unique admitted basis into a linear plan;
sibling clones deny planning until only one holder remains, and execution
releases that final internal obligation before reclamation.

## How It Executes

1. Observe a known branch through its owning runtime.
2. Borrow the admitted basis for fork, restore, merge, compatibility, or mutation.
3. Signal compares the complete live observation before effects.
4. A successful reference-moving operation returns a new admitted basis.
5. If the basis crosses a transport boundary, send only its descriptor and
   readmit it at the destination owner.
6. Retain an exact basis when another component must keep that state
   available. This is legitimate before or after the branch reference moves
   past it, because retention decides availability rather than currentness.
7. End the obligation by releasing it for a receipt, or by dropping it for the
   same terminal accounting without one.
8. To retire a branch, pass its sole admitted basis by value once no external
   obligation remains. A successful plan owns that basis until retirement
   executes.

Owner, definition, lifecycle, snapshot, and generation failures remain
distinct so callers can choose retry, refresh, or rejection deliberately.
Fork validates its owner identity before allocating a catalog entry. Fork,
restore, advance, and canonical merge distinguish basis denials before effects
from failures that occur only after the owner performed the operation.

## Small Example

```no_run
use worth_signal::facade::branch::SignalBranchBasisReadmissionDenial;
use worth_signal::facade::{SignalGraph, SignalRuntime};

let mut runtime = SignalRuntime::<(), (), (), (), ()>::builder(SignalGraph::new())
    .with_kernel_defaults()
    .build();
let main_basis = runtime
    .observe_signal_branch_basis(runtime.current_branch())
    .expect("the current branch admits an owner basis");
let (_preview, forked) = runtime
    .fork_signal_branch("retained-history", &main_basis)
    .expect("an exact basis admits a fork")
    .into_parts();

// Capturing twice moves the branch reference. The first captured state stays
// stored; the branch now selects the second. `advance_signal_branch` moves the
// reference the same way when you have real mutations to stage.
let (_first, superseded) = runtime
    .capture_signal_branch_snapshot(&forked)
    .expect("the first capture succeeds")
    .into_parts();
let (_second, current) = runtime
    .capture_signal_branch_snapshot(&superseded)
    .expect("the second capture succeeds")
    .into_parts();

// The currentness question: refused, because the branch has moved on.
assert!(matches!(
    runtime.readmit_signal_branch_basis(superseded.descriptor().clone()),
    Err(SignalBranchBasisReadmissionDenial::ReferenceMismatch { .. })
));

// The residency question: granted, because that exact state is still stored.
let lease = runtime
    .retain_signal_component_basis(&superseded)
    .expect("an exact stored target stays retainable after its branch moves");
assert_ne!(
    lease.retained_target(),
    current
        .observation()
        .target()
        .as_basis()
        .expect("an owner observation carries a basis target"),
    "an obligation is never a claim about whatever the branch holds now"
);

// Holding the obligation is what makes the historical target admissible again.
let readmitted = runtime
    .readmit_retained_signal_branch_basis(lease.descriptor().clone(), &lease)
    .expect("a live obligation readmits the exact target it retains");
assert_eq!(readmitted.descriptor(), lease.descriptor());
```

Two questions, one runtime, separate answers: the branch reference moved, and
the exact state it moved off is still there.

## Real Example

```no_run
use worth_signal::facade::branch::{
    SignalBranchRetentionOwnerPosture, SignalBranchRetentionReleaseOutcome,
    SignalBranchRetentionTerminalOutcome,
};
use worth_signal::facade::{SignalGraph, SignalRuntime};

let mut runtime = SignalRuntime::<(), (), (), (), ()>::builder(SignalGraph::new())
    .with_kernel_defaults()
    .build();
let main_basis = runtime
    .observe_signal_branch_basis(runtime.current_branch())
    .expect("the current branch admits an owner basis");
let (_preview, forked) = runtime
    .fork_signal_branch("retained-history", &main_basis)
    .expect("an exact basis admits a fork")
    .into_parts();
let (_first, superseded) = runtime
    .capture_signal_branch_snapshot(&forked)
    .expect("the first capture succeeds")
    .into_parts();
let (_second, _current) = runtime
    .capture_signal_branch_snapshot(&superseded)
    .expect("the second capture succeeds")
    .into_parts();

// The same exact target may carry more than one independent obligation.
let first = runtime
    .retain_signal_component_basis(&superseded)
    .expect("an exact stored target stays retainable");
let second = runtime
    .retain_signal_component_basis(&superseded)
    .expect("a second obligation over the same exact target is legitimate");

let receipt = match runtime.release_signal_component_basis(first) {
    SignalBranchRetentionReleaseOutcome::Released(receipt) => receipt,
    // A denial hands the still-live obligation back rather than dissolving it.
    // Letting this binding fall out of scope would drop-release the very
    // obligation you were just told you still hold.
    SignalBranchRetentionReleaseOutcome::Denied { lease, denial } => {
        panic!("this runtime issued the obligation: {denial:?} {lease:?}")
    }
};
assert_eq!(
    receipt.outcome(),
    SignalBranchRetentionTerminalOutcome::Released
);
// One obligation still covers this exact target, and it is the only obligation
// anywhere on this branch.
assert_eq!(receipt.remaining_target_leases(), 1);
assert_eq!(receipt.remaining_branch_leases(), 1);

// Dropping the second is the same terminal path; only the receipt is forgone.
drop(second);
let counts = runtime.signal_component_retention_terminal_counts();
assert_eq!(counts.explicit_releases(), 1);
assert_eq!(counts.dropped_releases(), 1);
assert_eq!(counts.terminal_releases(), 2);
assert_eq!(counts.unknown_lease_defenses(), 0);

// An obligation may outlive its runtime. The ledger survives; the state does not.
let orphan = runtime
    .retain_signal_component_basis(&superseded)
    .expect("an exact stored target stays retainable");
let witness = runtime
    .retain_signal_component_basis(&superseded)
    .expect("a second obligation over the same exact target is legitimate");
drop(runtime);

assert_eq!(
    orphan.owner_posture(),
    SignalBranchRetentionOwnerPosture::Lost
);
let receipt = orphan.release();
assert_eq!(
    receipt.outcome(),
    SignalBranchRetentionTerminalOutcome::OwnerUnavailable,
    "owner loss is recorded distinctly from an ordinary release"
);
assert_eq!(witness.owner_terminal_counts().owner_loss_releases(), 1);
```

The runtime is authoritative throughout. Retaining the superseded basis never
pretends it is still current; it only keeps that exact state available, and the
obligation ends exactly once however it ends.

The full runnable owner workflow, including the retained readmission lane, is
[`examples/branch_bases.rs`](./examples/branch_bases.rs):

```text
cargo run -p worth-signal --example branch_bases
```

## How It Relates To Other Features

- Transactions remain Signal's mutation engine. `advance_signal_branch`
  supplies exact branch admission and returns the resulting basis together
  with the transaction receipt.
- Existing Signal-local merge planning and execution consume exact admitted
  source and target bases. Successful execution returns the merge result with
  the newly admitted target basis; raw branch handles remain private engine
  inputs.
- `capture_signal_branch_snapshot` consumes an exact admitted expectation and
  returns both an owner-bound snapshot and the post-capture basis.
  `restore_signal_branch` validates both exact owner basis and snapshot owner,
  then returns the post-restore basis. Portable snapshot data must use the
  separate pristine-runtime reconstruction lane before ordinary restore can
  accept it.
- Branch retirement is the counterweight to retention. It is denied while any
  external obligation remains, and it reclaims every exact stored target that
  no obligation retains.
- Runtime Bridge may carry and reuse Signal-issued bases, but it does not mint
  them or become Signal authority.
- Foundational defines the runtime-neutral branch reference grammar. Signal
  owns liveness, readmission, retention, and all effects.
- Relational branch bases are independent owner artifacts. There is no
  combined Relational-plus-Signal authority in this milestone.

## Inspection And Debugging

Inspect the admitted basis's `observation()` when diagnosing a rejection. Its
branch identity, target basis, and generation show which axes were expected.
Readmission reports foreign owner, definition drift, retired or unknown branch,
unavailable snapshot, and reference mismatch separately.

For an obligation, ask it directly:

- `retained_target()` and `descriptor()` give the exact target as of
  acquisition, never the branch's current one.
- `owner_posture()` distinguishes `Live` from `Lost`.
- `owner_terminal_counts()` reads the issuing owner's ledger, and stays
  readable after that runtime is gone.

For the owner-side view, `signal_component_retention_terminal_counts` separates
explicit releases, dropped releases, owner-loss releases, and unknown-lease
defenses. When a release receipt surprises you, read `remaining_target_leases`
and `remaining_branch_leases` as the different questions they are.

For lifecycle failures, inspect the typed retention or retirement denial.
Do not infer liveness from graph diagnostic output; ask the runtime owner.

## Anti-Patterns

- Do not pass a `SignalBranchBasisDescriptor` to a governed operation. Readmit
  it first.
- Do not cache a raw branch ID and reconstruct expected currentness yourself.
- Do not use graph diagnostics as a second branch catalog.
- Do not treat `Clone` on an admitted basis as a new observation.
- Do not pass deserialized `SignalSnapshotV1` data directly to ordinary
  restore; only an owner-issued admitted snapshot opens that door.
- Do not read a lease as a claim that its branch reference is still current. It
  says that one exact immutable state is still available.
- Do not read a lease as durability. Once its owner is gone the ledger stays
  readable, but the retained state is not recoverable.
- Do not let a `SignalBranchRetentionReleaseOutcome::Denied { lease, .. }`
  binding fall out of scope. That denial handed you a live obligation, and
  dropping it releases the retention you were just told you still hold.
- Do not drop the last admitted basis for a state you will need before
  retaining it; ordinary readmission will refuse to give it back.
- Do not treat an unreleased lease as a leak to be defended against with
  `std::mem::forget` or a second bookkeeping layer. Drop is terminal.
- Do not create a second Signal graph for Query-installed operations hosted by
  Runtime Bridge.

## Current Limits

- Branch state, descriptors awaiting readmission, and retention accounting are
  memory-resident. Restart durability is deferred to Worth Store integration.
- Automatic rebase, distributed reference movement, and offline synchronization
  are not supported.
- Signal and Relational operations do not form a cross-owner atomic commit.
- Cross-runtime semantic merge, undo/redo, and persistent branch recovery
  remain future work.

## Owner Component Port

An integrating owner may consume only the public admitted basis, descriptor,
fork outcome, readmission/compatibility denials, retention lease outcomes,
linear retirement plan, and the admitted basis returned by capture, restore,
advance, or merge. Fork, capture, restore, and advance return typed mismatch,
retention, and no-movement outcomes rather than flattened owner strings;
owner preflight acquires the retention needed to issue the successor basis, so
basis construction after a performed canonical operation is an internal
invariant rather than a caller-visible fallible phase. It should treat a typed
no-movement outcome as leaving the old reference current; once Signal returns
the new basis, that basis is the post-operation reference. Retention transfers
through the explicit external lease object and never through a descriptor or
raw branch ID. The runtime's stable owner identity and BranchManager catalog
survive graph-state switches and snapshot restoration.

This port does not expose Signal graph mutation, legacy head tuples, private
snapshot storage, or authority minting. It is the component boundary available
to the later composite-owner work; it does not itself provide cross-owner
atomicity.

Milestone 9.17.2 may carry a `SignalBranchBasisDescriptor`, ask the Signal
owner to readmit it, retain the resulting `AdmittedSignalBranchBasis`, compare
two admitted bases through `validate_signal_basis_compatibility`, and consume
the typed owner outcome from fork, capture, restore, advance, merge, retention,
or retirement. It may not construct a basis, infer one from a snapshot ID or
digest, call the private graph/branch manager, or publish a product reference
as though Signal issued it.

Signal's canonical branch operations are synchronous and do not expose a
generic external cancellation token. A coordinating owner may honor
cancellation before invoking Signal. Once the call returns a successor admitted
basis, the Signal movement is performed and cannot be relabeled as cancelled;
a typed denial that says no movement preserves the predecessor. Milestone
9.17.2 must use this boundary in its own no-half-publication protocol rather
than attempting raw Signal rollback.

The integrating owner must acquire `retain_signal_component_basis` before it
promises component residency, and it must acquire while it still holds an
admitted basis for that exact state. Retention is available whether or not the
branch reference has already moved past the target, and it never claims that
reference is current. While the obligation is live, the owner may ask for the
exact retained basis back through `readmit_retained_signal_branch_basis`;
ordinary `readmit_signal_branch_basis` answers the currentness question and
will refuse.

The obligation ends exactly once, either through
`release_signal_component_basis` for a governed receipt or by being dropped for
the same terminal accounting. An integrating owner is therefore not required to
build leak defenses around it. A release offered to a runtime that did not
issue the obligation is denied with the live lease handed back, so the
integrator must rebind that returned lease rather than discard it. An
obligation that outlives its Signal runtime reports a `Lost` owner posture and
an `OwnerUnavailable` terminal outcome; the surviving ledger is observability,
not a promise that the retained state can still be produced.

All Signal branch catalog state, admitted bases, stored branch snapshots, and
retention accounting remain memory-resident. Restart durability is deferred to
Worth Store integration; the port does not expose a serializable authority
token or persistence promise.

## Related Docs

- [Signal documentation index](./DOCS.md)
- [Snapshots, branches, and history](./docs/guides/snapshots-branches-and-history.md)
- [Foundational branch references](../worth-foundational/docs/branching-merging-and-commit-vocabulary/branch-references.md)
- [Authority and workflow contracts](../worth-proof/docs/features/authority-and-workflow-contracts.md)
