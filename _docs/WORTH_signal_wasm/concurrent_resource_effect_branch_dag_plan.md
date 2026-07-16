# Milestone 11 Follow-On: Concurrent Resource Effect Branch DAG

## Goal

Replace shared-branch optimistic rollback with effect-owned native branches that
support independent siblings, explicit dependent children, arbitrary settlement
order, worker-first execution, resource-locus reconciliation, and a rebuildable
visible projection.

## Why This Milestone Exists

Milestone 10 promised speculative branch lifecycle, but the shipped resource
effect path selects the current runtime branch and captures a snapshot on it.
That makes one effect's rollback capable of crossing another effect's authority,
and worker-first authored state cannot honestly restore through that snapshot
path. Milestone 11 adds the fork-basis and materialization foundation this
correction needs, but intentionally stops before resource-line materialization.

This follow-on closes that structural gap before mutation-response
reconciliation builds more write families on top of it.

## Governing Summaries

- **Engineering Mentality:** Protect production-grade foundations from MVP
  shortcuts. This milestone starts with hostile concurrent settlement and builds
  the lower authority, proof, and boundedness surfaces before changing the demo.
- **Architectural Laws:** Protect singular authority and proof-bearing phase
  transitions. Native branches own speculative state, resource effects own
  semantic intent and dependencies, and visible optimism is a derived projection.
- **File Composition Laws:** Protect reviewable semantic units. New planning,
  execution, retirement, projection, and diagnostics responsibilities receive
  named modules instead of enlarging existing branch and resource god files.
- **Domain Structure Laws:** Protect distinctions of authority, lifecycle, and
  truth source in the physical tree. Core branch lifecycle, worker translation,
  resource effect orchestration, derived projection, demo presentation, and docs
  remain spatially distinct.
- **Performance Laws:** Protect work bounded by semantic delta. Settlement and
  reprojection are indexed by affected branch, effect dependency, and resource
  locus; no effect operation may scan the whole graph or every open effect.
- **`worth-signal` temporal/async roadmap:** Protect runtime-owned lifecycle and
  out-of-order completion convergence across replay, restore, and branches. This
  work extends that parity discipline without moving resource semantics into core.
- **`worth-signal-wasm` product roadmap:** Protect product layers as consumers of
  native truth. This is a Milestone 11 follow-on because explicit fork basis and
  merge materialization must precede resource effect concurrency, and it must
  close before mutation-response reconciliation.

## Adversarial Constraint

Starting from one canonical resource state, at least ten optimistic effects may
be admitted before any settles. They may touch disjoint loci, conflict on one
locus, or depend on speculative output from one or more earlier effects. Exactly
any subset may succeed, responses may arrive in any permutation, retries and
duplicate responses may occur, and the runtime may run worker-first or in
explicit main-thread compatibility mode.

Every supported execution must converge to the same canonical truth, the same
visible optimistic truth, the same dependency outcomes, and equivalent proof
artifacts. A failed effect must never erase, resurrect, or silently commit another
effect; no shared snapshot restore may execute; no settled branch may leak; and
work must stay bounded by affected effects and resource loci.

## Product Decision Lock

- One admitted optimistic effect owns one native speculative branch.
- Canonical branches contain confirmed truth only.
- Independent effects are siblings from an explicit canonical basis.
- Semantic dependencies are explicit resource-effect edges. Native
  `parent_branch_id` records execution ancestry and must not be promoted into an
  effect dependency claim.
- A disposable native projection branch materializes the visible fold of
  canonical truth plus applicable open effects. It is derived and rebuildable.
- Failure retires an effect branch; it does not restore a shared snapshot.
- Success performs resource-locus-aware three-way reconciliation before native
  canonical merge or canonical server-truth adoption.
- Physical response arrival order never becomes semantic write order. Server
  revision wins when present; otherwise declared causal order and stable client
  admission sequence govern.
- Exact history restore remains a separate user-directed history capability. It
  is not the ordinary rollback mechanism for optimistic effects.
- `branchNative()` must either acquire an effect-owned branch with complete
  proof or deny optimistic admission. Reusing the current branch is not an
  allowed fallback.

## Phase Plan

### Phase 1: Native Branch Retirement And Lifecycle Proof

Freeze branch disposal as native lifecycle authority so effect-owned branches
can end without retaining heavy graph state or losing their audit lineage.

**Relevant subsystems**

- `worth-signal` branch manager, ancestry index, mutation ledger, and snapshots
- branch lifecycle receipts, diagnostics, history, and complexity accounting

**Relevant APIs**

- `RuntimeCore::fork_branch_with_snapshot(...)`
- new `plan_branch_retirement(...)` and `retire_branch(...)` facade operations
- sealed `PlannedBranchRetirement` and `BranchRetirementReceipt` proof types

**Warnings**

- Retirement is not deletion from history. Heavy mutable state may be reclaimed,
  while identity, fork basis, terminal reason, and closeout digest remain retained.
- The current branch, canonical branch, merge participant, or branch with
  unresolved native children must not be retired through an unchecked call.
- Do not add retirement branches to the already oversized `merge_runtime.rs`;
  branch lifetime deserves its own module and tests.

**Test requirements**

- `Native Branch Retirement Residue Test`: retire thousands of settled sibling
  branches and prove live branch-state count returns to baseline while receipts
  and lineage remain queryable.
- `Native Branch Retirement Dependency Denial Test`: attempt to retire current,
  canonical, merge-inflight, and parent-with-live-child branches and assert typed,
  side-effect-free denials.
- `Native Branch Retirement Replay Honesty Test`: export or replay retained
  history after heavy state reclamation and prove terminal lineage digests match.

**Engineering decisions**

- Maintain reverse child membership as a branch-lifecycle index so retirement
  eligibility does not require scanning every known branch.
- Retirement reasons are a sealed family including rejected effect, merged
  effect, superseded effect, dependency cancellation, and projection rebuild.
- Lifecycle receipts carry exact reclaimed breadth and retained-proof breadth.

**Open questions**

- None.

### Phase 2: Atomic Branch-Targeted Transaction Execution

Make a declared branch and expected head the mandatory target of speculative
mutation, eliminating create/switch/apply/switch choreography as an authority
path.

**Relevant subsystems**

- `worth-signal` transaction admission, lowering, execution, and branch state
- `worth-signal-wasm` runtime core transaction and branch facades
- branch-targeted receipts and stale-basis denials

**Relevant APIs**

- new `BranchTargetedTransactionRequest`
- new `plan_branch_targeted_transaction(...)`
- new `execute_branch_targeted_transaction(...)`
- `BranchTargetedTransactionReceipt` carrying branch and head before/after proof

**Warnings**

- An implementation that temporarily switches the ambient active branch and can
  yield between steps is not atomic and is not admitted.
- The executor consumes a lowered plan. It must not rediscover branch identity,
  expected head, transaction scope, or policy during execution.
- A stale expected head is a typed denial, never an implicit rebase.

**Test requirements**

- `Branch-Targeted Interleaving Isolation Test`: adversarially interleave writes
  for ten branches and prove every mutation lands only on its declared target.
- `Branch-Targeted Stale Head Denial Test`: advance a target after planning and
  prove execution denies without mutating the target or ambient visible branch.
- `Branch-Targeted Active Branch Preservation Test`: execute success and failure
  paths against stored branches and prove the caller's active branch is unchanged.

**Engineering decisions**

- Request, validated request, lowered plan, executed receipt, and denial are
  distinct proof-bearing types.
- Transaction locality and mutation ledger recording remain native core work.
- Counters expose planned operations, executed operations, stale-head denials,
  target switches avoided, and touched-node breadth.

**Open questions**

- None.

### Phase 3: Worker-First Live Branch Authority

Expose explicit fork basis, targeted transactions, and retirement through the
worker-owned runtime, and make the worker's live graph head—not cached import
context—the source of branch acquisition truth.

**Relevant subsystems**

- Rust worker runtime shell branch and transaction commands
- worker/main-thread request and response envelopes
- worker-first authored graph publication and handle mapping
- package history facade and main-thread compatibility adapter

**Relevant APIs**

- worker commands for `forkBranch`, `applyTransactionToBranch`, and `retireBranch`
- async package facade operations with typed availability and denial results
- live `WorkerBranchBasisReceipt` including authored graph generation

**Warnings**

- Retaining the cached snapshot named in the original bug is not a fix; it can
  still predate authored publications and omit the state needed by the effect.
- The JS authored registry may retain identity mappings, but it must not become a
  parallel branch-state authority or reconstruct speculative truth by convention.
- Worker unavailability must deny branch speculation explicitly; it must not
  silently fall back to shared main-thread rollback.

**Test requirements**

- `Worker Authored Signal Fork Inclusion Test`: publish form and resource-line
  authored signals, fork from the live head, mutate the child, and prove the
  child contains every required authored handle without unknown-signal failure.
- `Worker And Compatibility Branch Command Parity Test`: run the same explicit
  fork, targeted write, merge, rejection, and retirement sequence in both modes
  and compare state and proof digests.
- `Worker Cached Import Basis Rejection Test`: hold a stale import context while
  publishing new signals and prove effect branch acquisition still uses the live
  worker authority or returns a typed stale-basis denial.

**Engineering decisions**

- Worker commands are batchable self-describing envelopes, not scalar bridge
  chatter for each node or lifecycle field.
- Authored graph generation is part of branch-basis equivalence proof.
- Main-thread compatibility lowers through the same request and result contracts.

**Open questions**

- None.

### Phase 4: Resource Effect Dependency And Fork Planning

Admit each optimistic effect through an explicit plan that distinguishes
semantic dependency from native execution ancestry and chooses an exact fork
basis before any speculative state is constructed.

**Relevant subsystems**

- resource effect identity, causal sequencing, retry lineage, and locality proof
- effect-branch acquisition, dependency indexing, and lifecycle typestates
- native explicit fork-basis facade from Milestone 11

**Relevant APIs**

- `ResourceEffectDependencyDeclaration`
- `ResourceEffectDependencySet`
- `ResourceEffectBranchAcquisitionPlan`
- lifecycle states `Planned -> Forked -> Applied -> Pending`

**Warnings**

- A later admission is not automatically a child of an earlier admission.
  Dependency must be declared or proven from semantic input use.
- Native ancestry answers "which state was copied?" Effect dependency answers
  "which speculative truth must exist for this intent to remain valid?" These
  fields must have distinct types even when they carry the same branch ID.
- Multi-parent effect dependency is legal. It forks from a proof-bearing derived
  basis containing the declared parents; it is not encoded by forging multiple
  native parents.

**Test requirements**

- `Sibling Effect Acquisition Equivalence Test`: admit ten independent effects
  from one canonical generation and prove ten effect-owned sibling branches with
  distinct identities and the same declared canonical basis.
- `Dependent Effect Basis Proof Test`: admit an edit that consumes an optimistic
  create and prove its dependency set and fork basis include that create's visible
  contribution without claiming native ancestry is the dependency authority.
- `Effect Dependency Cycle And Missing Parent Denial Test`: deny cyclic,
  self-dependent, unknown, retired, and generation-incompatible dependency sets
  before branch construction.

**Engineering decisions**

- Maintain effect-to-branch, dependency, reverse-dependent, and locus indexes as
  framework-owned lifecycle state.
- Effect admission produces a single canonical envelope from which diagnostics,
  history, projection, and settlement records derive.
- `branchNative()` no longer reports `selectedExistingBranch`; successful
  acquisition reports `effectOwnedBranch` with exact fork and dependency proof.

**Open questions**

- None.

### Phase 5: Resource-Locus Three-Way Reconciliation

Lower native merge proof and one effect envelope into an executable resource
reconciliation plan that updates only the declared resource locus.

**Relevant subsystems**

- native scoped merge/cherry-pick proof and materialization foundation
- compiled response lenses and resource effect loci
- line value materialization for fields, items, membership, JSON paths, regions,
  entity stores, collections, creates, updates, and removals

**Relevant APIs**

- `ResourceEffectReconciliationRequest`
- sealed `PlannedResourceEffectReconciliation`
- `ResourceLocusMaterializationPlan`
- typed conflict, mapping-unavailable, stale-basis, and policy-denial results

**Warnings**

- Native per-aspect conflict proof does not splice fields inside a resource-line
  value. Whole checkpoint-node replacement is forbidden for a narrower admitted
  resource locus.
- Reconciliation consumes captured base fragment, current canonical fragment,
  effect intent, native merge proof, and optional canonical server result. It may
  not infer missing identity or topology from current UI state.
- Every locus advertised as branch-native must either have an admitted
  materialization strategy or be denied before optimistic application.

**Test requirements**

- `Disjoint Same-Node Resource Locus Preservation Test`: change `title` and
  `status` through sibling effects stored in one value node, settle them in every
  order, and prove both successful loci survive.
- `Same-Locus Conflict Determinism Test`: issue conflicting writes to one field,
  vary response order, and prove server revision or declared admission policy
  yields one stable result and conflict artifact.
- `Dependent Create Then Edit Reconciliation Test`: confirm, transform, or reject
  the create and prove the child edit rebases only when identity and locus mapping
  remain valid.
- `Unsupported Locus Pre-Admission Denial Test`: prove an unsupported topology
  never mutates a speculative branch and never falls back to whole-line adoption.

**Engineering decisions**

- Native merge remains policy authority; resource reconciliation is a declared
  materialization strategy consuming its proof.
- Locus planners are organized by topology responsibility, not accumulated in
  `resource_branch_capabilities.ts` or a generic helper file.
- Result counters include lookup, traversal, reconstruction, changed-locus,
  downstream invalidation, and fallback breadth.

**Open questions**

- None.

### Phase 6: Derived Optimistic Projection Branch

Materialize one disposable native branch as the visible fold of canonical truth
and all currently applicable effects, without turning that projection into a
second authority.

**Relevant subsystems**

- resource line visible-selection proof
- effect dependency and locus indexes
- native branch fork, targeted transaction, and retirement
- projection invalidation and incremental materialization

**Relevant APIs**

- `ResourceOptimisticProjectionPlan`
- `ResourceOptimisticProjectionReceipt`
- visible-selection kind `derivedEffectProjectionBranch`
- projection rebuild and affected-locus refresh operations

**Warnings**

- Sibling branches cannot all be the single active visible branch. The projection
  branch solves presentation without changing which branches own speculative
  intent.
- Projection state must be destroyable and reproducible from canonical truth plus
  open effect envelopes. No caller may merge it into canonical as effect truth.
- Removing one effect must not trigger a whole-line or all-effect rebuild when
  its locality proof admits a narrower affected surface.

**Test requirements**

- `Optimistic Projection Destruction And Rebuild Equivalence Test`: destroy the
  projection after ten mixed effects and rebuild a byte-equivalent visible value
  and proof digest from canonical plus open envelopes.
- `Projection Locality After Random Failure Test`: fail five randomly selected
  siblings and prove only their affected loci and reverse dependents are
  revisited, with exact structural counters.
- `Projection Never Becomes Canonical Test`: attempt to use a projection receipt
  as canonical merge authority and prove the type or admission boundary rejects it.

**Engineering decisions**

- Maintain a locus-to-open-effect ordered index and reverse dependency index.
- Projection order uses topological dependency order, then server revision when
  available, then stable client admission sequence as the final tie-breaker.
- Dense/broad effect profiles may choose an explicitly measured rebuild strategy;
  the result must disclose the chosen strategy and breadth.

**Open questions**

- None.

### Phase 7: Arbitrary-Order Settlement And Dependency Closeout

Close confirmed, rejected, duplicated, retried, and superseded effects through a
single lifecycle coordinator whose decisions are independent of response arrival
order.

**Relevant subsystems**

- async request generation, server correlation, and idempotency authority
- resource effect settlement planner and dependency closeout policies
- canonical reconciliation, projection refresh, and branch retirement

**Relevant APIs**

- lifecycle progression `Pending -> ResponseRecorded -> CloseoutEligible`
- terminal results `Merged`, `RejectedAndRetired`, `DependencyCancelled`,
  `RebasedAndPending`, `SupersededAndRetired`, and typed denial
- `ResourceEffectCloseoutPlan` and `ResourceEffectCloseoutReceipt`

**Warnings**

- Recording a child response before its parent settles is legal. Finalizing it
  against an unresolved basis is not.
- Parent rejection does not automatically mean child rejection. The declared
  dependency closeout policy chooses cancel or proof-bearing rebase; absence of a
  valid rebase strategy must deny rather than guess.
- Success responses carrying canonical server values outrank speculative intent,
  but only for their declared reconciled targets and server revision.

**Test requirements**

- `Ten-Effect Random Settlement Convergence Test`: for five successes and five
  failures, enumerate or property-generate response permutations and prove one
  final canonical value, one visible value, and no live settled branches.
- `Child Response Before Parent Test`: record successful and failed child
  responses first, then vary the parent outcome and prove closeout waits, rebases,
  or cancels according to typed policy without transient canonical corruption.
- `Duplicate Retry Closeout Idempotency Test`: replay confirmations and failures
  across retry lineage and prove one canonical application and typed duplicate
  receipts.
- `Settlement Failure Atomicity Test`: inject reconciliation and retirement
  failures and prove no partial canonical commit, lost projection layer, or
  terminal lifecycle lie.

**Engineering decisions**

- Plan dependency readiness and reconciliation before executing any closeout.
- Closeout execution consumes one lowered plan and emits one canonical receipt;
  diagnostics, history, UI events, and counters derive from it.
- Use request-local, dependency-local, and locus-local indexes; never scan all
  runtime branches or all resource lines to settle one effect.

**Open questions**

- None.

### Phase 8: Public Resource Effect And History Surface

Expose concurrent effect lifecycle through a stable resource facade without
requiring consumers to operate native branches directly.

**Relevant subsystems**

- resource line facade, diagnostics, history, and generated TypeScript contracts
- effect profiles and closeout matrix
- forms action integration and compatibility aliases

**Relevant APIs**

- new `line.effects()` inspection facade with targeted effect lookup and open
  effect summaries
- new `line.history().rollbackEffect(effectId)`
- retained `line.history().rollbackLastEffect()` as a convenience lowering to a
  targeted effect, never to shared snapshot restore
- enriched effect envelope branch, dependency, projection, and closeout receipts

**Warnings**

- `lastEffect` is useful diagnostics but insufficient authority for ten open
  effects. Targeted lifecycle operations require an effect ID.
- `restoreExact()` remains explicit history navigation and must not share result
  vocabulary with effect rejection or effect-branch retirement.
- Do not expose raw mutable dependency indexes, projection branches, or unbranded
  branch handles through the resource facade.

**Test requirements**

- `Concurrent Effect Public Surface Type Test`: type-check inspection, targeted
  rollback, dependency declarations, closeout receipts, and exhaustive terminal
  result handling from the packaged consumer surface.
- `Rollback Last Effect Compatibility Test`: with zero, one, and many open
  effects, prove deterministic targeting, typed no-effect behavior, and no shared
  snapshot restore call.
- `Forms Resource Action Parity Test`: run the same concurrent sibling and
  dependent form submissions through worker-first and compatibility modes and
  compare resource, form, diagnostics, and history truth.

**Engineering decisions**

- Public effect summaries are immutable derived views keyed by runtime-issued
  effect identity.
- Dependency declarations use effect identities or declared semantic input
  handles, not branch IDs supplied as untyped numbers.
- Package declarations, generated declarations, smoke examples, and runtime
  exports land in the same phase so the public surface cannot drift.

**Open questions**

- None.

### Phase 9: Demo 5 Concurrent Request Proof

### Phase 10: Feature Documentation And Migration Guidance

### Phase 11: Cross-Layer Certification And Release Gate

## Must Ship

## Must Preserve

## Acceptance Evidence

## Sequencing Notes
