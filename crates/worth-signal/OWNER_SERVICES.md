# Signal Owner Services

## What This Feature Is

Signal owner services are the future shared-borrow entry points for working on
one owner-managed branch without borrowing the whole `SignalRuntime` mutably.
Phase 3 has installed the real owner root, registry, independent branch cells,
and managed-reference admission described here. The public service methods in
the contract matrix are frozen for Phase 4 and Phase 5 implementation; they are
not available yet.

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

The following are **not public availability claims** in Phase 3:

- `SignalOwnerServicePorts`
- `SignalBranchBasisPort`
- `SignalBranchMutationPort`
- `SignalBranchLifecyclePort`
- `SignalRuntime::owner_component_services`

Their exact contracts are frozen below so Phase 4 cannot omit an inherited
operation or improvise a weaker input. Phase 5 will make the bundle and ports
composition-facing after the methods delegate to the installed owner kernel.

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

At every use, the receiving owner first compares the reference's sealed runtime,
lifecycle, and weak-allocation affinity with its own identity. A mismatch is
`ForeignOwner` before either owner admits work or contacts a registry. A matching
owner then admits its own lifecycle, looks up the target branch, and compares
the installed cell incarnation. A closed matching owner is `OwnerUnavailable`;
retirement and replacement are
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

`UnknownBranch` remains reserved for absence at registry lookup. A quarantined
cell is the terminal contained-panic posture and cannot be readmitted or moved.
`OwnerCellMisuse` is a typed owner-kernel invariant denial: a lawful public call
creates one fresh owner admission and must not reach it. Current executable
evidence is
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

`ValidatedSignalBranchName` names the frozen `fork_exact` input, but it is not a
Phase 3 facade export. The facade gate that publishes `fork_exact` must publish
the owner validator and its sealed validated value together; accepting an
unvalidated `String` is not a compatible implementation of this row.

### Lifecycle port

| Exact method | Inputs | Output | Cancellation | Canonical owner | Named case / executable lane |
| --- | --- | --- | --- | --- | --- |
| `plan_retirement_exact` | `AdmittedSignalBranchBasis`, `SignalBranchRetirementReason` | `TransitionOutcome<PlannedSignalBranchRetirement, SignalBranchRetirementDenial>` | pre-effect planning; no token | target cell + retention/lineage metadata | future `signal_owner_services`: healthy `lifecycle::plan_retirement_exact_requires_linear_basis`, denials `lifecycle::plan_retirement_exact_denial_matrix`; current root `branch_lifecycle_retirement::retirement_reclaims_heavy_state_and_retains_compact_closeout_proof` and `retirement_denies_current_and_parent_branches_with_live_native_children` |
| `plan_retirement_releasing_snapshots_exact` | `AdmittedSignalBranchBasis`, `&[&AdmittedSignalBranchSnapshot]`, `SignalBranchRetirementReason` | `TransitionOutcome<PlannedSignalBranchRetirement, SignalBranchRetirementDenial>` | pre-effect planning; no token | target cell + snapshot retention | future `signal_owner_services`: healthy `lifecycle::plan_retirement_releasing_exact_snapshots`, denials `lifecycle::snapshot_release_retirement_denial_matrix`; current root `branch_snapshot_retirement::declared_snapshot_release_permits_retirement_only_after_authority_is_dropped`, `snapshot_release_allowance_rejects_cross_branch_authority`, and `snapshot_release_allowance_rejects_foreign_runtime_authority` |
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
same typed unavailable posture. An explicit owner close waits for admitted work
to drain. Root destruction only requests close and returns without waiting, so
destruction from inside an admitted callback cannot wait on itself; the last
admission release performs the terminal `Closing` to `Closed` transition.

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

- Phase 3 exposes managed-reference vocabulary and the private owner kernel,
  not public port methods or the aggregate bundle.
- Existing `SignalRuntime` convenience methods have not yet been delegated to
  the owner services.
- Signal services decide only Signal component truth. Product currentness and
  composite publication belong to Milestone 9.17.2.
- Persistence, restart, replay, correction, and merge are outside this contract.

## Related Docs

- [`BRANCH_BASES.md`](./BRANCH_BASES.md)
- [`../worth-relational/OWNER_COMPONENT_PORT.md`](../worth-relational/OWNER_COMPONENT_PORT.md)
- [`../../_docs/WORTH-query/milestone-9.17.1.2.md`](../../_docs/WORTH-query/milestone-9.17.1.2.md)
