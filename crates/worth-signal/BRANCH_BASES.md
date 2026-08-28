# Signal Branch Bases

## What This Feature Is

Signal branch bases let you name one exact, live branch state without exposing
mutable graph internals. A basis says which runtime, branch, Signal definition,
snapshot posture, and reference generation you observed. Use it when work must
fork, restore, advance, merge, or retain a particular Signal branch without silently
falling back to whatever branch is current later.

## Why You Use It

- Fork background or preview work from the exact state you inspected.
- Reject a stale request after another transaction, snapshot, restore, or merge
  moves the branch reference.
- Carry a descriptive basis through JSON and ask the owning runtime to readmit
  it before it can operate.
- Keep a component basis resident while another owner still needs it.

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

A **retention lease** keeps the referenced component state live. Leases are
bounded, runtime-affine, and single-release. Branch retirement is denied while
external leases remain. Retirement planning consumes the branch's unique
admitted basis into a linear plan; sibling clones deny planning until only one
holder remains, and execution releases that final internal obligation before
reclamation.

## How It Executes

1. Observe a known branch through its owning runtime.
2. Borrow the admitted basis for fork, restore, merge, compatibility, or mutation.
3. Signal compares the complete live observation before effects.
4. A successful reference-moving operation returns a new admitted basis.
5. If the basis crosses a transport boundary, send only its descriptor and
   readmit it at the destination owner.
6. Acquire a lease when another component must keep the basis resident, then
   release that exact lease when finished.
7. To retire a branch, pass its sole admitted basis by value. A successful plan
   owns that basis until retirement executes.

Owner, definition, lifecycle, snapshot, and generation failures remain
distinct so callers can choose retry, refresh, or rejection deliberately.
Fork validates its owner identity before allocating a catalog entry. Fork,
restore, advance, and canonical merge distinguish basis denials before effects
from failures that occur only after the owner performed the operation.

## Small Example

```rust
use worth_signal::facade::{SignalGraph, SignalRuntime};

let mut runtime = SignalRuntime::<(), (), (), (), ()>::builder(SignalGraph::new())
    .with_kernel_defaults()
    .build();

let main = runtime.current_branch();
let main_basis = runtime.observe_signal_branch_basis(main)?;
let fork = runtime.fork_signal_branch("preview", &main_basis)?;

let preview = fork.created_branch();
let preview_basis = fork.created_basis();
assert_ne!(preview.id, runtime.current_branch().id);
assert_eq!(preview_basis.observation().generation().get(), 0);
```

This is the smallest honest fork: the runtime observes the source, validates
that exact basis, creates the branch, and returns the new branch with its own
admitted basis.

## Real Example

```rust
use worth_signal::facade::{SignalGraph, SignalRuntime};

let mut runtime = SignalRuntime::<(), (), (), (), ()>::builder(SignalGraph::new())
    .with_kernel_defaults()
    .build();
let main = runtime.current_branch();
let main_basis = runtime.observe_signal_branch_basis(main)?;
let (preview, preview_basis) = runtime
    .fork_signal_branch("what-if", &main_basis)?
    .into_parts();

// Keep this exact component basis alive for a downstream holder.
let lease = runtime.retain_signal_component_basis(&preview_basis)?;

// A no-op transaction is used here only to show the authority flow. Real
// applications stage their normal Signal mutations in this closure.
let advanced = runtime.advance_signal_branch(
    &mut (),
    &preview_basis,
    |_transaction| Ok(()),
)?.into_basis();
assert!(advanced.observation().generation().get()
    > preview_basis.observation().generation().get());

// Transport carries a weak descriptor. The owner must readmit it.
let encoded = serde_json::to_vec(advanced.descriptor())?;
let descriptor = serde_json::from_slice(&encoded)?;
let readmitted = runtime.readmit_signal_branch_basis(descriptor)?;

runtime.validate_signal_basis_compatibility(&advanced, &readmitted)?;
let _release = runtime.release_signal_component_basis(lease);
let _ = preview;
```

The runtime is authoritative throughout. The JSON payload never becomes
authority by deserialization, and retaining the old basis does not pretend it
is still current after `advance_signal_branch`; it only keeps its component
state resident.

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
- Do not forget to release component leases; retained leases intentionally
  block retirement.
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
promises component residency and consume the exact lease through
`release_signal_component_basis`. A lease keeps an immutable component state
available but does not claim that its branch reference is still current.

All Signal branch catalog state, admitted bases, stored branch snapshots, and
retention accounting remain memory-resident. Restart durability is deferred to
Worth Store integration; the port does not expose a serializable authority
token or persistence promise.

## Related Docs

- [Signal documentation index](./DOCS.md)
- [Snapshots, branches, and history](./docs/guides/snapshots-branches-and-history.md)
- [Foundational branch references](../worth-foundational/docs/branching-merging-and-commit-vocabulary/branch-references.md)
- [Authority and workflow contracts](../worth-proof/docs/features/authority-and-workflow-contracts.md)
