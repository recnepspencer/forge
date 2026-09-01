# Signal Owner Services

## What This Feature Is

Signal owner services are the future shared-borrow entry points for working on
one owner-managed branch without borrowing the whole `SignalRuntime` mutably.
Phase 3 installed the real owner root, registry, independent branch cells, and
managed-reference admission described here. The Phase 4 shared-contract gate
now supplies the bounded admission, cleanup, retention-reservation, lineage,
retirement-recovery, and transfer seams that the three service lanes consume.
It does not deliver the public services, bundle, facade, or legacy cutover. The
public methods in the contract matrix remain unavailable until their later
implementation and publication gates.

## Why You Use It

- Keep a stable reference to a live branch while its exact state changes.
- Run unrelated branch work through independently synchronized owner cells.
- Carry exact bases and retention obligations without turning them into branch
  lifecycle authority.
- Receive typed owner, basis, cancellation, capacity, and lifecycle outcomes.

## Stable Entry Points

`worth_signal::facade::branch` currently exports the owner-issued
`ManagedSignalBranchReference` vocabulary and its admission denial, along with
the existing basis, snapshot, retention, retirement, and outcome types.

The following remain **not public availability claims** after the Phase 4
shared-contract gate:

- `SignalOwnerServicePorts`
- `SignalBranchBasisPort`
- `SignalBranchMutationPort`
- `SignalBranchLifecyclePort`
- `SignalRuntime::owner_component_services`

Their exact contracts are frozen below so the service lanes cannot omit an
inherited operation or improvise a weaker input. Phase 5 will make the bundle
and ports composition-facing after the methods delegate to the installed owner
kernel.

## Installed Shared-Contract Gate

The owner admits at most 64 operations. Admission reserves the packed
lifecycle phase/count before publishing one record into a fixed owner-owned
64-slot table. Each record carries the real `ThreadId` and its atomic hold
posture; there is no TLS semantic ledger, hashed thread identity, global thread
registry, owner-global executor, or allocation on record release. Capacity and
closing denial are pre-effect, and reservation unwind returns both the count
and exact slot.

An admission is thread-affine and cannot be sent or shared with another thread.
Before any metadata, registry, or branch-cell lock, the owner scans its bounded
record table and rejects same-thread/same-owner reentry if any admission on that
thread already holds owner metadata or a branch cell. This includes a fresh
admission, an earlier idle admission, and a different target branch. A hold
borrows its published admission, so its record cannot disappear while the hold
is live. Other-thread contention on one cell continues to serialize normally.
Record-scan work is reported separately as `admission_records_scanned`; it is
not branch-registry work and is never represented as zero-cost bookkeeping.

`Open -> Closing` fences new admissions while already admitted synchronous
calls retain their exact reservations and outcomes. Explicit close first
rejects any live admission on its executing thread, before changing phase, and
otherwise waits without missed wakeups. Root destruction requests close without
waiting. One cleanup claimant drains registry and metadata in actual batches of
at most 64, drops detached heavy state outside their locks, preserves the
retention obligation ledger and bounded retirement receipts, and publishes
`Closed` only after cleanup finishes. `close_batches` counts nonempty cleanup
batches rather than lifecycle phase changes. The last admitted release performs
cleanup when root destruction initiated close; no background worker owns this
transition. Cleanup ownership is an unwind-safe claim: a panic while processing
an actual batch releases the claim and wakes waiting closers, so one of them can
resume the remaining batches and publish `Closed`.

Direct admitted or external retention acquisition on the sealed owner borrows
the operation's existing exact admission and an owner-admitted exact basis; a
serializable basis descriptor is never retention authority. Acquisition neither
creates a second admission nor consults a separate open flag. Foreign or expired
admissions, basis-owner mismatch, and executing-thread owner reentry are rejected
before retention-ledger contact. Consequently, an operation admitted before the
`Open -> Closing` fence may still reserve and convert its lawful outputs while
`Closing`, whereas fresh work cannot obtain an admission during `Closing` or
`Closed`. The raw retention registry remains the lifecycle-agnostic obligation
owner beneath this admitted boundary, but it has no reachable descriptor-only
acquisition entry point.

An exact branch retirement reservation is also the canonical acquisition fence
for that branch. Retention acquisition takes the short metadata guard before the
retention ledger: an acquisition that inserts first is counted by the later
retirement reservation, while a reservation that installs first rejects both
admitted-output and external acquisition before ledger contact. The guard is
dropped before cell work, and unrelated branches remain independently
retainable. Retirement samples admitted/reserved and external counts together
after installing the metadata reservation, so `RetainedAdmittedBasis` and
`RetainedComponentBasis` report their distinct exact obligations.

Retirement planning on the sealed owner consumes the caller's existing
operation admission and exact admitted basis; it never performs a second
admission. The owner validates the basis runtime and definition before registry
contact, observes lineage/merge metadata before the retention ledger, releases
those short guards, and then contacts the canonical branch cell once for its
real handle and complete current observation. Planning creates only the
existing linear `PlannedSignalBranchRetirement`: it installs no retirement
reservation, receipt reservation, registry posture, or retention fence.
Snapshot-aware planning accepts exactly one baseline admitted lease plus each
unique owner-issued snapshot retention identity; duplicate snapshot handles do
not inflate the allowance, and runtime or branch mismatch remains a distinct
typed denial. `reserve_retirement` and the checked target-cell execution still
recheck lineage, retention, complete basis, and sole-holder custody at the
canonical effect boundary; planning does not replace that fence.

The canonical retention owner also exposes private lane-ready reservations:

- advance reserves one admitted output;
- capture reserves two admitted outputs from one snapshot plus a refreshed
  basis;
- restore reserves one admitted output;
- fork reserves one destination admitted output and rebinds its pending
  reservation to the owner-issued child before that child is published.

These reservations consume the existing 4096 admitted-lease capacity before
movement. Pending slots are not reported as issued authority. Conversion is
infallible after movement even if close has begun, while cancellation, denial,
panic, or unused drop returns exactly the remaining slots. The reservation
methods pair the actual owner, borrowed operation admission, and checked
canonical cell before executing that cell's corresponding operation. Their
sealed ready conversions accept no unrelated raw outcome and cannot outlive the
admitted synchronous call. The reservation objects live outside the retention
mutex and require no metadata lock across cell work. They are private kernel
seams, not descriptors, public constructors, or a second graph/head authority
table.

A snapshot reservation binds capacity to both the selected branch and its exact
installed cell incarnation before movement. Foreign-owner validation precedes
that custody check; a sibling or replaced same-id cell is rejected before cell
contact. Snapshot movement installs the matching metadata packet before the
faultable `AfterCanonicalMovement` seam, so an unwind cannot expose a cell head
whose snapshot state is absent from owner metadata.

Owner selection now carries the actual current branch from the sealed
partition; it never infers the minimum id, and the selected pointer names the
same canonical cell stored by the registry. A short metadata reservation
linearizes fork lineage against retirement before cell acquisition. Retirement
also reserves one of 4096 compact receipt slots before effect, fences and checks
live admitted/pending plus external retention before registry removal, and
preconstructs the exact receipt at the movement boundary. A performed receipt
remains recoverable from cell
custody across a caller unwind; no-effect cancellation returns capacity and
reopens only a still-live cell. Retired inert cells are removed and never
relabeled live.

Fork destination installation is recoverable until the caller receives the
owner-issued destination handle and admitted basis. The exact registry
incarnation and uncommitted lineage remain under rollback guards through
`ForkDestinationInstallation` and `OutcomeConstruction`; unwind removes only
that incarnation and returns lineage, retention, live, and reservation capacity.
Successful conversion commits lineage and disarms registry rollback exactly
once, without reconstructing a handle from descriptive identity.

Restore selection now validates a complete `ReadyBranchLifecycleTransfer`
before moving outgoing state or removing the stored target. Its commit is
infallible, preserving raw local restore and unknown-portable import behavior
without a partial fallback lane.

## Core Mental Model

A managed branch reference and an exact basis answer different questions.

- `ManagedSignalBranchReference` identifies one Signal owner and one installed
  branch-cell incarnation. It lets that owner revalidate where to observe or
  readmit. It does not say which generation or snapshot is current.
- `AdmittedSignalBranchBasis` proves one exact owner-admitted state and carries
  its admission retention. It is the expected input for compare-and-move.
- `SignalBranchRetentionLease` keeps one exact target available. It does not
  keep the branch reference current or the runtime alive.

The managed reference contains a concrete `worth-proof` carrier specialized to
a sealed Signal marker. Its owner and cell identities, weak lifecycle binding,
and proof fields are private. It has no public constructor, deserialization,
raw-id conversion, generic authority parameter, or adapter route. Cloning it
copies only the same weak reference contract: no runtime, cell, snapshot, basis,
or lease becomes strongly retained.

Existing owner-root compatibility helpers first compare the reference's sealed
runtime, lifecycle, and weak-allocation affinity with their own identity. This
preserves their inherited `ForeignOwner`-before-admission precedence. Future
weak ports first upgrade their owner binding; upgrade failure is exactly
`OwnerUnavailable` and never fabricates a branch id. After a successful upgrade,
the same affinity checks precede registry contact. A matching owner admits once
and carries that single admission through subordinate helpers; helpers do not
admit a second time. Retirement and replacement are
`BranchLifecycleEnded`/`BranchIncarnationReplaced`. Descriptive branch identity
equality is never consulted as proof.

## Frozen Bundle Contract

The future issuance shape is:

```rust,ignore
pub fn owner_component_services(
    &mut self,
) -> Result<SignalOwnerServicePorts<D, I, E, Ctx, T>, SignalOwnerServiceIssuanceDenial>
```

Issuance consumes the legacy canonical partition once and leaves
`SignalRuntime` as the sole non-cloneable strong owner root. The composition-
capable issuance method is available only when `D`, `I`, `E`, `Ctx`, and `T`
satisfy `Send + Sync + 'static` in addition to their existing operation bounds
(`D` remains `Copy + Ord + Debug`; `I` and `T` remain `Copy + Ord`). This is an
issuance capability fence, not permission to retain `E`, `Ctx`, or a callback.
Each operation borrows its caller-owned `Ctx` synchronously, and `F` itself has
no added `Send`, `Sync`, or `'static` bound.

The bundle accessors are exactly `basis_port()`, `mutation_port()`, and
`lifecycle_port()`. The bundle and every returned port are cloneable weak
bindings. They do not expose constructors, close authority, cells, registries,
test control, or a generic service trait.

## Frozen Method Matrix

Every receiver is `&self`. `SignalOwnerCancellationToken` is explicit only for
operations that can cross the pre-movement cancellation cutoff. The canonical
owner column names where the decision remains after the public method exists.

### Basis port

| Exact method | Inputs | Output | Cancellation | Canonical owner | Named case / executable lane |
| --- | --- | --- | --- | --- | --- |
| `issue_managed_branch_reference` | `&AdmittedSignalBranchBasis` | `Result<ManagedSignalBranchReference, ManagedSignalBranchReferenceAdmissionDenial>` | none; bounded admission only | owner lifecycle + registry cell incarnation | future `signal_owner_services`: healthy `provenance::issues_managed_reference_from_real_basis`, denials `provenance::managed_reference_issuance_denial_matrix`; current kernel healthy `managed_reference::owner_issued_reference_reenters_one_live_cell_without_retaining_exact_state` |
| `observe_current` | `&ManagedSignalBranchReference` | `Result<AdmittedSignalBranchBasis, SignalBranchBasisObservationDenial>` | none; read-only target-cell contact | checked target cell + admission retention registry | future `signal_owner_services`: healthy `provenance::managed_reference_observes_current_after_movement`, denials `provenance::managed_reference_observation_denial_matrix`; current kernel healthy/movement `managed_reference::canonical_movement_stales_exact_basis_without_staling_managed_reference`, denials `equal_looking_branch_numbers_from_another_owner_are_denied_by_affinity`, `retirement_invalidates_the_reference_for_the_consumed_branch_incarnation`, and owner replacement tests |
| `readmit_exact` | `&ManagedSignalBranchReference`, `&SignalBranchBasisDescriptor` | `Result<AdmittedSignalBranchBasis, SignalBranchBasisReadmissionDenial>` | none; read-only compare | checked target cell + admission retention registry | future `signal_owner_services`: healthy `provenance::managed_reference_readmits_exact_descriptor`, denials `provenance::managed_reference_readmission_denial_matrix`; current kernel `managed_reference::owner_issued_reference_reenters_one_live_cell_without_retaining_exact_state` executes the checked-cell observation substep, while `transaction_panic_quarantines_managed_readmission_without_unknown_branch` executes its terminal quarantine mapping; descriptor comparison and new admitted-basis issuance remain unexecuted until Phase 4 |
| `compare_current_exact` | `&AdmittedSignalBranchBasis` | `Result<(), SignalBranchBasisReadmissionDenial>` | none; read-only compare | concrete basis authority + its target cell | future `signal_owner_services`: healthy `provenance::admitted_basis_matches_current`, denials `provenance::compare_current_exact_denial_matrix`; current kernel `managed_reference::canonical_movement_stales_exact_basis_without_staling_managed_reference` executes the stale no-callback/no-movement denial |
| `readmit_retained_exact` | `&SignalBranchBasisDescriptor`, `&SignalBranchRetentionLease` | `Result<AdmittedSignalBranchBasis, SignalBranchRetainedReadmissionDenial>` | none; read-only retained-target admission | issuing retention ledger + exact target admission | future `signal_owner_services`: healthy `lifecycle::retained_exact_readmission_after_movement`, denials `lifecycle::retained_exact_readmission_denial_matrix`; current root `branch_retention_lifecycle::a_live_obligation_readmits_its_exact_retained_target` executes healthy and foreign/descriptor denials |
| `retain_exact` | `&AdmittedSignalBranchBasis` | `Result<SignalBranchRetentionLease, SignalBranchRetentionAcquisitionDenial>` | pre-effect, no token | retention registry | future `signal_owner_services`: healthy `lifecycle::retains_historical_exact_basis`, denials `lifecycle::exact_retention_acquisition_denial_matrix`; current root `branch_retention_lifecycle::an_exact_obligation_pins_a_real_historical_admitted_target` and `branch_retention_contract::retention_capacity_denies_before_unbounded_growth` execute both postures |
| `release_exact` | `SignalBranchRetentionLease` | `SignalBranchRetentionReleaseOutcome` | terminal and non-cancellable | weak port admission, then issuing retention ledger | future `signal_owner_services`: healthy/denial `lifecycle::weak_port_release_and_direct_lease_terminal_matrix`; current root `branch_retention_lifecycle::explicit_release_returns_governed_exact_target_evidence`, `a_foreign_release_hands_the_live_obligation_back`, and `an_obligation_outlives_its_owner_and_records_owner_loss` execute lease terminal postures, while weak-port owner loss remains unexecuted until Phase 4 |
| `owner_lifecycle_observation` | none | `SignalOwnerLifecycleObservation` | not applicable | owner lifecycle | future `signal_owner_services`: healthy/terminal `lifecycle::weak_port_observes_open_closing_closed`; current kernel `lifecycle::close_drains_admitted_work_and_monotonically_denies_late_admission`, `root_destruction::root_drop_inside_admitted_callback_requests_close_without_self_deadlock`, and `foreign_owner_cannot_admit_or_close_the_lifecycle` execute lifecycle posture |
| `owner_service_cost_snapshot` | none | `Result<SignalOwnerServiceCostSnapshot, SignalOwnerUnavailable>` | not applicable | owner counters | future `signal_owner_services`: healthy `cost::basis_operations_report_exact_structural_deltas`, denial `cost::closed_basis_port_reports_owner_unavailable`; current kernel `managed_reference::owner_issued_reference_reenters_one_live_cell_without_retaining_exact_state` proves the one-lookup/one-contact managed observation cost |

`readmit_exact` is not descriptor-only readmission: the managed reference must
admit the exact owner and checked cell before the descriptor is compared there. The
compatibility method `SignalRuntime::readmit_signal_branch_basis(descriptor)`
stays owner-root-only and is not a composition input. `readmit_retained_exact`
uses a different inherited authority route: the concrete lease already binds
the issuing owner and exact historical target, so it takes the descriptor and
lease without a managed reference and performs no currentness comparison.
`compare_current_exact` first validates the admitted basis's receiving-owner,
definition, and branch affinity, then compares its complete exact state against
that one canonical cell. The named future denial matrix must cover same-Rust-type
foreign authority and exact-state drift separately.

Managed-reference admission is mapped without flattening. Matching-owner loss
remains the existing top-level `OwnerUnavailable`; foreign affinity, an ended or
replaced incarnation, retirement already visible during lookup, and invariant
failure are carried by the additive `ManagedReferenceDenied` variant on
observation/readmission denials. After admission returns its checked cell, a
retirement race remains the cell's existing top-level `RetirementInProgress` or
`RetiredBranch` observation denial. A Phase 4 method must continue from that
exact checked cell; a second raw-id lookup is not an acceptable substitute.

### Typed installed-cell posture

Once a registry lookup has returned an installed target cell, its posture is
not flattened into `UnknownBranch` or a diagnostic string. `advance_exact`,
`fork_exact`, `capture_exact`, `restore_exact`, and retirement planning or
execution preserve the cell result as the operation-specific
`RetirementInProgress { branch_id }`, `RetiredBranch { branch_id }`,
`QuarantinedBranch { branch_id }`, or `OwnerCellMisuse { branch_id }` denial.
Managed exact readmission likewise preserves retirement-in-progress, retired,
quarantined, cell-misuse, and owner-invariant postures in
`SignalBranchBasisReadmissionDenial`.

`UnknownBranch` remains reserved for actual absence at registry lookup. Owner
invariants and expired retirement custody retain their distinct typed posture;
they are not converted to absence. A quarantined
cell is the terminal contained-panic posture and cannot be readmitted, moved, or
retired. It retains its registry membership and one configured live-branch
capacity slot until the Signal owner root is destroyed; the current kernel has
no quarantine purge path. Unrelated cells remain available. `OwnerCellMisuse`
remains the typed cell-misuse meaning. Owner-wide executing-thread reentry is
the operation-specific additive `OwnerReentry` meaning and does not require a
fabricated branch id. Current executable mapping evidence is
`cell_posture_outcomes::every_operation_preserves_reachable_cell_posture_without_unknown_fallback`
for every operation mapping and
`managed_reference::transaction_panic_quarantines_managed_readmission_without_unknown_branch`
for the real contained-panic path.

Two inherited public root methods remain explicit compatibility surfaces rather
than disappearing into this port contract:

- `validate_signal_basis_compatibility(&AdmittedSignalBranchBasis,
  &AdmittedSignalBranchBasis) -> Result<(),
  SignalBranchBasisCompatibilityDenial>` compares two admitted artifacts. It
  does not compare either artifact with current owner state, so it is distinct
  from `compare_current_exact`. Current healthy and denial evidence is
  `branch_basis_contract::snapshot_and_restore_each_move_the_exact_reference`.
- `signal_component_retention_terminal_counts() ->
  SignalBranchRetentionTerminalCounts` is root inspection of recorded release,
  drop, and owner-loss totals. Current evidence is
  `branch_retention_lifecycle::explicit_release_returns_governed_exact_target_evidence`,
  `dropping_an_obligation_is_the_same_terminal_release`, and
  `an_obligation_outlives_its_owner_and_records_owner_loss`.

Both receivers remain `&self`. Phase 4/5 compatibility delegation must preserve
their behavior and denial meanings, but neither becomes a fourth port or a
second currentness/retention authority lane.

`release_exact` first upgrades the weak port. If that fails, it returns
`SignalBranchRetentionReleaseOutcome::Denied` with the still-live lease and
`SignalBranchRetentionReleaseDenial::OwnerUnavailable`. Calling
`SignalBranchRetentionLease::release` directly (or dropping it) remains the
lease's distinct terminal route and may report terminal owner loss.

### Mutation port

| Exact method | Inputs and operation-specific generic | Output | Cancellation | Canonical owner | Named case / executable lane |
| --- | --- | --- | --- | --- | --- |
| `fork_exact` | `ValidatedSignalBranchName`, `&AdmittedSignalBranchBasis`, `&SignalOwnerCancellationToken` | `Result<SignalBranchForkOutcome, SignalBranchForkOperationDenial>` | before source capture/installation; performed installation wins | source cell + bounded registry reservation | future `signal_owner_services`: healthy `fork_and_sharing::fork_exact_shares_populated_state`, denials `fork_and_sharing::fork_exact_denial_and_cancellation_matrix`; current kernel `fork_contracts::exact_fork_shares_graph_roots_and_isolates_touched_node_state` and `late_fork_cancellation_drops_preconstructed_destination_without_source_movement` |
| `advance_exact<F>` | `&AdmittedSignalBranchBasis`, `&mut Ctx`, `&SignalOwnerCancellationToken`, `F: FnOnce(&mut SignalTransaction<'_, D, I, E, Ctx, T>) -> Result<(), SignalError>` | `Result<SignalBranchAdvanceOutcome, SignalBranchAdvanceDenial>` | before canonical movement; performed movement wins | one target cell + transaction engine | future `signal_owner_services`: healthy `baseline::advance_exact_changes_semantic_output`, denials `baseline::advance_exact_stale_cancelled_and_engine_denials`; current kernel `managed_reference::canonical_movement_stales_exact_basis_without_staling_managed_reference` and `cancellation::cancellation_while_waiting_for_same_cell_denies_without_movement` |
| `capture_exact` | `&AdmittedSignalBranchBasis`, `&SignalOwnerCancellationToken` | `Result<SignalBranchSnapshotCaptureOutcome, SignalBranchSnapshotCaptureDenial>` | before capture movement; performed capture wins | target cell + snapshot registry | future `signal_owner_services`: healthy `lifecycle::capture_exact_records_snapshot_and_basis`, denials `lifecycle::capture_exact_denial_and_cancellation_matrix`; current kernel `exact_cell_contracts::exact_snapshot_and_restore_contracts_move_one_cell_and_install_metadata_between_locks` executes healthy and stale exact denial |
| `restore_exact` | `&AdmittedSignalBranchBasis`, `&AdmittedSignalBranchSnapshot`, `&SignalOwnerCancellationToken` | `Result<AdmittedSignalBranchBasis, SignalBranchRestoreDenial>` | before restore movement; performed restore wins | target cell + snapshot registry | future `signal_owner_services`: healthy `lifecycle::restore_exact_changes_canonical_observation`, denials `lifecycle::restore_exact_denial_and_cancellation_matrix`; current kernel `exact_cell_contracts::exact_snapshot_and_restore_contracts_move_one_cell_and_install_metadata_between_locks` executes healthy and mismatch denial |

`F` executes synchronously while the caller retains `&mut Ctx`; the owner does
not clone, register, return, or keep either value. Portable snapshot
reconstruction remains an owner-root construction compatibility operation and
is not a mutation-port method.

Snapshot reservations carry a checked, runtime-global identity as well as
storage capacity. Forking or restoring an older snapshot cannot reset that
identity allocator. Cancellation, denial, and unwinding return reservation
capacity without reusing its identity; exhaustion is
`SnapshotIdentityExhausted` before movement. Active-branch capture or
reconstruction stores an immutable snapshot and updates metadata, never a
second live mutable branch. These rules preserve distinct exact retention
targets across sibling captures and the handoff into owner cells.

`ValidatedSignalBranchName` names the frozen `fork_exact` input, but it is not a
Phase 3 facade export. The facade gate that publishes `fork_exact` must publish
the owner validator and its sealed validated value together; accepting an
unvalidated `String` is not a compatible implementation of this row.

### Lifecycle port

| Exact method | Inputs | Output | Cancellation | Canonical owner | Named case / executable lane |
| --- | --- | --- | --- | --- | --- |
| `plan_retirement_exact` | existing `&SignalOwnerOperationAdmission`, `AdmittedSignalBranchBasis`, `SignalBranchRetirementReason` | `TransitionOutcome<PlannedSignalBranchRetirement, SignalBranchRetirementDenial>` | pre-effect planning; no token | metadata then retention, followed by one target-cell contact | current kernel `retirement_planning::owner_exact_retirement_plan_preserves_pre_effect_state_and_executes_real_handle`, `owner_retirement_planning_distinguishes_current_canonical_and_live_child`, `owner_retirement_planning_checks_complete_basis_and_owner_before_registry_contact`, `owner_retirement_planning_preserves_distinct_retention_and_holder_denials`, and `owner_retirement_planning_preserves_reachable_merge_participant_denial`; future service case `lifecycle::plan_retirement_exact_requires_linear_basis` and denial matrix |
| `plan_retirement_releasing_snapshots_exact` | existing `&SignalOwnerOperationAdmission`, `AdmittedSignalBranchBasis`, `&[&AdmittedSignalBranchSnapshot]`, `SignalBranchRetirementReason` | `TransitionOutcome<PlannedSignalBranchRetirement, SignalBranchRetirementDenial>` | pre-effect planning; no token | metadata then retention, followed by one target-cell contact | current kernel `retirement_planning::snapshots::owner_snapshot_release_plan_counts_unique_owner_issued_custody_exactly`, `owner_snapshot_release_plan_denies_foreign_runtime_before_registry_contact`, and `owner_snapshot_release_plan_denies_real_wrong_branch_custody`; future service case `lifecycle::plan_retirement_releasing_exact_snapshots` and denial matrix |
| `retire_exact` | `PlannedSignalBranchRetirement`, `&SignalOwnerCancellationToken` | `TransitionOutcome<SignalBranchRetirementReceipt, SignalBranchRetirementDenial>` | before movement; performed retirement wins | target cell then short registry removal | future `signal_owner_services`: healthy `lifecycle::retire_exact_consumes_linear_plan`, denials `lifecycle::retire_exact_cancellation_and_stale_plan_matrix`; current kernel `exact_cell_contracts::exact_retirement_contract_consumes_a_linear_plan_before_registry_removal` executes performed and denied posture |
| `owner_lifecycle_observation` | none | `SignalOwnerLifecycleObservation` | not applicable | owner lifecycle | future `signal_owner_services`: healthy/terminal `lifecycle::weak_port_observes_open_closing_closed`; current kernel `lifecycle::close_drains_admitted_work_and_monotonically_denies_late_admission` and `root_destruction::root_drop_inside_admitted_callback_requests_close_without_self_deadlock` execute blocking-close and nonblocking-destruction posture |
| `owner_service_cost_snapshot` | none | `Result<SignalOwnerServiceCostSnapshot, SignalOwnerUnavailable>` | not applicable | owner counters | future `signal_owner_services`: healthy `cost::lifecycle_operations_report_exact_structural_deltas`, denial `cost::closed_lifecycle_port_reports_owner_unavailable`; current kernel `exact_cell_contracts::exact_retirement_contract_consumes_a_linear_plan_before_registry_removal` and registry cost assertions execute structural deltas |

Batch retirement remains a bounded owner-root compatibility family in this
handoff. It is intentionally not added to the composition port: 9.17.2 needs
individual component custody, and the specification permits a batch only “if
exposed.” Existing batch methods and outcomes remain available on the root and
are not deleted or reinterpreted.

## How It Executes

1. Admit the owner lifecycle.
2. Perform one short registry lookup or reservation.
3. Enter the checked target cell returned by reference admission, or at most one
   existing target cell selected by exact basis authority.
4. Compare the complete expected basis before movement.
5. Apply the canonical owner operation.
6. Release the cell before constructing the external outcome.
7. Update short retention, registry, or diagnostic accounting as required.

The lock order is lifecycle, registry metadata, one target cell, then short
retention/diagnostic accounting. No call holds two existing branch cells,
metadata/global owner locks, a Runtime World lock, or a runtime-wide mutex.
`advance_exact` intentionally executes its synchronous callback while holding
the one target-cell exclusion that protects mutation; no registry or other-owner
lock crosses that callback.

Callbacks must not synchronously wait on work that needs their held cell or
nest blocking calls across owners. Cross-owner wait-cycle detection is not
provided. The Phase 4 reentry fence is owner-scoped, not a global executor or
cross-owner lock registry.

## Small Example

The Phase 3 public vocabulary can be carried and cloned, but not constructed:

```rust
use worth_signal::facade::branch::ManagedSignalBranchReference;

fn carry(reference: &ManagedSignalBranchReference) -> ManagedSignalBranchReference {
    reference.clone()
}
```

Owner issuance and service calls are deliberately absent from this example
until the bundle is public. Current runnable branch-basis and retention examples
remain in [`BRANCH_BASES.md`](./BRANCH_BASES.md).

## Real Example

There is no honest public owner-service workflow in Phase 3. The production
kernel tests build a real `SignalRuntime`, fork live branches, seal its canonical
partition into owner cells, issue a reference from a genuinely admitted basis,
and then revalidate through the registry and exact cell incarnation. Phase 5
will add the facade example only after that same path is public; no
private-module or fake-compiling example stands in for it here.

## How It Relates To Other Features

- Exact bases remain compare-and-move authority; managed references never
  replace them.
- External leases preserve exact residency, including historical state; they
  never preserve branch currentness or owner liveness.
- `SignalBranchHandle`, `SignalBranchIdentity`, descriptors, snapshot ids, and
  digests remain descriptive compatibility/inspection values.
- Relational owner ports are separate concrete owner services. Runtime World
  will coordinate both owners without restamping either authority.

## Inspection And Debugging

`SignalOwnerServiceCostSnapshot` reports owner upgrades, registry lookup and
reservation work, target-cell contacts/waits, movements, retention contacts,
fork capture/installation and copy work, diagnostic recording/omission, and
close batches. Counters are descriptive; they cannot authorize an operation.

Operational capacity denial is pre-effect. Diagnostic capacity exhaustion
records an omission/drop count and does not deny an otherwise lawful owner
operation. Closing rejects new admissions and gives every later weak call the
same typed unavailable posture. Direct lease termination linearizes under the
issuing retention ledger: a release that consumes its capability before close
may return `Released`, while a call beginning after completed close cannot
report `Live` merely because a temporary weak upgrade succeeded. Ledger entries
are not cleared at close. Explicit close rejects calls from that owner's
admitted execution thread before starting close; otherwise it waits for
admitted work and real cleanup. Root destruction requests close without waiting,
so destruction from inside an admitted callback cannot wait on itself.

## Anti-Patterns

- Do not construct authority from a `SignalBranchHandle`, branch identity,
  descriptor, generation, snapshot id, or digest.
- Do not use a managed reference as an exact basis or retention lease.
- Do not use descriptor-only readmission from composition code.
- Do not wrap `SignalRuntime` in a global mutex or define an adapter trait over
  these concrete ports.
- Do not keep `Ctx`, callbacks, cell guards, or owner admissions after the
  synchronous call returns.

## Current Limits

- The Phase 4 gate exposes no new public port methods or aggregate bundle; its
  admission, output-reservation, lineage, recovery, and ready-transfer seams
  remain private kernel contracts for the service lanes.
- Existing `SignalRuntime` convenience methods have not yet been delegated to
  the owner services.
- Signal services decide only Signal component truth. Product currentness and
  composite publication belong to Milestone 9.17.2.
- Persistence, restart, replay, correction, and merge are outside this contract.

## Related Docs

- [`BRANCH_BASES.md`](./BRANCH_BASES.md)
- [`../worth-relational/OWNER_COMPONENT_PORT.md`](../worth-relational/OWNER_COMPONENT_PORT.md)
- [`../../_docs/WORTH-query/milestone-9.17.1.2.md`](../../_docs/WORTH-query/milestone-9.17.1.2.md)
