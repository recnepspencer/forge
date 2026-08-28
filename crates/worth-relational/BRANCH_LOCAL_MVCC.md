# Relational Branch-Local MVCC

`worth-relational` publishes truth by moving one explicit branch reference from
one complete immutable root to another. A branch name, commit ID, version,
snapshot handle, serialized descriptor, or ambient idea of "main" is not enough
to authorize that movement.

The runnable public-facade example is
[`examples/branch_local_mvcc.rs`](./examples/branch_local_mvcc.rs).

## Mental model

Keep four things distinct:

- A **branch identity** selects one runtime-owned mutable reference cell.
- A **basis descriptor** describes one exact observed target and generation. It
  is portable data, not authority.
- An **admitted basis** is the owner-issued, retained authority to read or begin
  work from that exact observation.
- A **branch root** is the complete immutable state selected by the reference.
  Readers never observe a partially installed root.

The ordinary write progression is:

```text
explicit identity
  -> observe exact owner basis
  -> detached branch-bound transaction
  -> validated proposal
  -> prepared single-use candidate
  -> branch-local compare-and-publish
  -> performed owner commit
  -> settlement and exact successor basis
```

`main_branch_identity()` is an explicit owner observation helper. Governed
operations never accept `None`, a string named `"main"`, or a missing target as
permission to choose it.

## Stable public entry points

Import through `worth_relational::facade`:

- `branch` owns identities, descriptors, admitted bases, fork outcomes,
  lifecycle outcomes, and external retention leases.
- `mvcc` owns exact observations, branch-bound transactions, prepared
  candidates, publication outcomes, and the independently borrowable
  publication port.
- `snapshots` opens a pinned read only from an exact owner observation.
- `history` and `publication` inspect canonical committed artifacts; they do
  not provide a second branch-currentness authority.

The primary runtime methods are:

- `main_branch_identity`, `branch_identity`, and `observe_branch`;
- `readmit_branch_basis` and `readmit_retained_branch_basis`;
- `begin_branch_transaction` and `prepare_branch_transaction`;
- `publication_port().compare_and_publish(...)` and
  `settle_performed_publication(...)`;
- `observe_fork_source` and `fork_branch`;
- `retain_component_basis` and `release_component_basis`; and
- `archive_branch` and `delete_branch`.

## Exact reads

`observe_branch` returns a serializable `RelationalBranchBasisDescriptor` and
an `AdmittedRelationalBranchBasis`. Use the admitted basis's
`RelationalBranchObservation` to open a snapshot:

```rust,no_run
use worth_relational::facade::runtime::RelationalRuntime;

fn open_exact_read(runtime: &mut RelationalRuntime) {
    let identity = runtime.main_branch_identity();
    let (_descriptor, basis) = runtime
        .observe_branch(&identity)
        .expect("owner observation");
    let snapshot = runtime
        .snapshots()
        .snapshot_for_observation(&basis.observation())
        .expect("exact pinned read");
    runtime
        .snapshots()
        .release_snapshot(&snapshot)
        .expect("single release");
}
```

Moving the branch later does not change that snapshot. A transported descriptor
must pass `readmit_branch_basis`; deserialization cannot restore authority.

## Detached transactions and preparation

`begin_branch_transaction` validates runtime identity, branch identity,
lifecycle, exact currentness, and retention before returning a detached
`BranchBoundRelationalTransaction`. The transaction owns its overlay, declared
read/write footprint, and retained basis. It does not borrow
`&mut RelationalRuntime` for its lifetime.

Preparation performs the fallible work before effects: schema and invariant
validation, footprint validation, canonical commit assembly, immutable-root
materialization, and resource checks. A
`PreparedRelationalCommitCandidate` is opaque, runtime-affine, branch-bound,
single-use, and retained. Preparing, discarding, expiring, or losing a
candidate does not move a public reference.

The convenience `transaction.commit(&mut runtime)` follows the same
prepare/publish/settle path. Integration owners that need the explicit
linearization result use the publication port described in
[`OWNER_COMPONENT_PORT.md`](./OWNER_COMPONENT_PORT.md).

## Linearization and conflict

`RelationalPublicationPort::compare_and_publish` compares the candidate's full
expected observation with the selected branch cell inside one bounded
branch-local critical section. It installs one complete next root and advances
that reference's generation exactly once.

The terminal outcomes are intentionally distinct:

- `Performed` means the branch reference moved and carries the canonical commit
  plus successor admitted basis. The owner must still settle durability and
  optional projections.
- `Stale` means another movement won; it carries complete expected and observed
  descriptors and moves nothing for this candidate.
- `Denied` means owner, runtime, branch, or lifecycle admission failed.
- `Interrupted` means cancellation or timeout won before linearization.
- `Deferred` means bounded capacity, retention, reservation, or candidate
  lifetime prevented movement.
- `Failed` means an owner invariant or required publication step failed before
  movement.

Same-reference contenders have one winner. Different branch cells do not share
an ordinary publication lock; global ID and patch-position reservations are
reported separately and do not become branch serialization.

## Cancellation

Cancellation is observed at named boundaries. Before the critical section it
returns `Interrupted` and the old reference remains current. Once the owner has
crossed the linearization point, cancellation cannot turn performed work into
a failure: `PerformedRelationalCommit::late_interruption()` records the event,
and settlement must still complete or enter its typed recovery lane.

Prepared candidates and performed-but-unsettled commits retain their required
roots until their terminal path transfers or releases the obligation. Dropping
work is not a substitute for reporting success.

## Fork, retention, and lifecycle

Fork is a separate owner transition. `observe_fork_source` issues a linear,
fork-only source basis. `fork_branch` consumes it, creates a fresh reference
cell, and shares the exact immutable source root and canonical commit artifact.
It does not clone authoritative truth or infer a source from a branch label.

Every admitted observation, snapshot, transaction, candidate, performed
settlement, and external component holder has a bounded retention obligation.
Use `retain_component_basis` when another component must keep an exact basis
resident, and consume the lease through `release_component_basis`. The lease
does not claim the retained basis is still current; it only preserves the
immutable target for exact readmission.

Archive advances lifecycle metadata and denies new ordinary work. Delete is
forbidden for the main branch and either removes an unretained branch or
returns `WaitingForActiveOperations`. Shared ancestors survive deletion while
another branch or lease retains them. Reclamation scans belong to the explicit
maintenance lane, never transaction admission or ordinary publication.

## Structural sharing and cost

Branch roots use persistent copy-on-write regions. Fork performs constant
reference/cell/retention work and copies zero authoritative truth bytes and
zero canonical commit envelopes. A write materializes only touched regions and
root paths; unchanged regions, schema registries, and ancestry remain shared by
allocation identity.

Inspection keeps these claims separable:

- logical bytes reachable from each branch;
- unique physical authoritative bytes;
- reused and newly materialized regions;
- copied truth and commit-envelope counts;
- branch-local wait/contact counters; and
- ordinary work versus maintenance reconstruction work.

Ordinary observation, transaction admission, and publication do not scan
unrelated branches or retained history. Historical reconstruction and
reclamation are explicit cold paths.

## Authority boundaries

- Foundational branch-reference values describe exact meaning but never own
  currentness, liveness, or movement.
- Proof carriers encode progression law, while Relational's concrete
  owner-sealed marker types decide which values can open governed operations.
- Signal owns its independent component basis and graph state.
- Runtime Bridge may retain and coordinate owner-issued component bases, but
  it may not mint Relational authority or mutate a Relational branch cell.
- Query carries component or composite artifacts supplied by their owners; it
  does not become component history authority.

## Current limits

The owner catalog, immutable roots, branch cells, and retention accounting are
memory-resident. Restart durability for this owner model is deferred to Worth
Store integration. This milestone does not provide composite Relational plus
Signal currentness, cross-owner atomic publication, automatic rebase, semantic
merge, distributed movement, or offline synchronization.

For the semantic certification model and its reusable extension rules, see
[`TESTING_WORLDS.md`](./TESTING_WORLDS.md).
