# C.7 Phase 8: Typed Mutation Settlement And Ordinary-Facade Cutover

## Outcome

Phase 8 replaces caller-driven durability orchestration with one Store-owned
mutation lifecycle. The ordinary caller may issue an idempotency key, prepare
one explicit platform-durable request, start the prepared mutation, observe or
cancel it through a handle, and consume one terminal physical fate. The caller
cannot drive WAL, data, root, settlement, retry, or acknowledgment phases.

Only exact completion constructs `PhysicalMutationAcknowledgment`. Proven no
effect and indeterminate fates remain distinct, carry their exact basis, and
have no acknowledgment construction path. Dropping a handle abandons only the
caller's observation; the Store retains and drains the started mutation.

This plan is confined to WORTH Store C.7. Unrelated repository-wide cleanup is
not part of Phase 8.

## Boundary Review

### Current authority

- `PhysicalRecordSubmission` is the ordinary facade, but it publicly exposes
  every internal transition from preparation through current-root advance.
- `PreparedPhysicalMutation` owns the admitted identity, idempotency binding,
  request fingerprint, deadline, resource shape, and planned record/root data.
- `RootPublicationPhysicalMutationMember` already carries the exact settled
  WAL, barrier, data-effect, persisted-record, and caller-observation basis
  through namespace-durable root publication.
- `PhysicalCurrentRootOwner::advance` is the sole current-root authority and
  returns the complete nonempty member set after the final physical barrier.
- the idempotency registry persists only `ProvenNoEffect`; completed and
  indeterminate fates have no terminal registry representation.
- checkpoint has a Store-managed attempt, weak submission facade, caller
  handle, typed poll/wait/cancellation, and close-time drain. Mutation has no
  equivalent lifecycle owner.
- record-serving close currently extracts publication authority before any
  mutation-lifecycle drain because no such drain exists.
- mutation deadlines are carried but are not evaluated. The admitted Signal
  monotonic clock is already available from `PhysicalStoreWorkRuntime`.

### Defects that Phase 8 must close

1. A caller can skip, reorder, repeat, or abandon durability phases.
2. No type derives per-member terminal settlement from the completed shared
   root publication.
3. No type makes acknowledgment construction exclusive to completion.
4. A dropped caller can abandon the only practical driver of settlement.
5. Close does not stop mutation admission and drain started mutations before
   publication, Signal, work, residency, and media teardown.
6. Cancellation and deadline have no complete pre-effect/effectful/terminal/
   stale/closing contract.
7. Completed-but-unobserved truth and Store-owned terminal counters do not
   exist.
8. Executed-boundary and diagnostic evidence projections do not exist, so
   their required one-way dependency direction is unenforced.
9. The living ledger has only three Phase 8 rows and therefore omits explicit
   acknowledgment authority, terminal-fate separation, completed-unobserved
   accounting, and one-way evidence projection.
10. The public API and removal inventories account for the current facade and
    WAL-only receipt, but not the full final Phase 8 public surface and its
    semantic owners.

## Compile-Time Destination

```text
physical_runtime/durability/
  mutation/
    handle.rs
    outcome.rs
    progression/completed.rs
  settlement/
    mod.rs
    acknowledgment.rs
    proven_no_effect.rs
    indeterminate.rs
    completed_unobserved.rs
  lifecycle/
    mod.rs
    managed_work.rs
    drain.rs
  evidence_projection/
    mod.rs
    executed_boundary.rs
    performance.rs
    diagnostic_fate.rs
  observation/
    counters.rs
    snapshot.rs
```

One-file directories are valid where the responsibility is already distinct
and expected to grow. They do not justify combining settlement, lifecycle,
observation, or evidence projection into a manager/coordinator/pipeline file.

## Public Contract

### Preparation

- retain explicit key issuance and `PhysicalMutationRequest` construction;
- preparation continues to return the existing six-branch `ProofOutcome`;
- evaluate the request deadline against the admitted Signal monotonic clock
  before any effect;
- carry a non-public start capability in each prepared value so
  `PreparedPhysicalMutation::start(self)` can consume it exactly once;
- `PreparedPhysicalMutation::execute(self)` is exactly `start().wait()`.

### Handle

`PhysicalMutationHandle` exposes only:

- immutable mutation identity;
- immutable idempotency identity;
- immutable request fingerprint and deadline observation;
- typed `poll`;
- consuming `wait`;
- typed `request_cancellation`.

`Drop` has no cancellation or settlement effect. The Store attempt is owned by
the lifecycle owner and worker, not by the handle.

### Terminal fate

- `CompletedPhysicalMutation` is constructed only by matching one
  `RootPublicationPhysicalMutationMember` from the completed root publication
  to the started attempt.
- `PhysicalMutationAcknowledgment` is a consuming projection available only on
  `CompletedPhysicalMutation`.
- `ProvenNoEffectPhysicalMutation` is constructible only while the exact
  idempotency binding remains pre-seal and no WAL member exists.
- `IndeterminatePhysicalMutation` preserves the exact last trustworthy phase,
  completed breadth, effect/inspection posture, identity, idempotency, and
  fingerprint needed by C.8. It exposes no retry or acknowledgment method.
- post-seal not-started continuations are Store settlement work, not a new
  mutation retry. If exact continuation cannot lawfully complete, the result
  is indeterminate, never proven no effect.

### Cancellation and deadline

- cancellation accepted before group seal terminalizes the exact binding as
  proven no effect;
- once the group is sealed, WAL-complete settlement owns forward progress and
  cancellation reports effectful settlement rather than no effect;
- terminal cancellation returns the existing terminal observation;
- stale and runtime-closing outcomes remain distinct;
- deadline uses only the admitted Signal monotonic clock;
- deadline is absent from request fingerprint and idempotency lease identity;
- deadline elapsed before group seal follows the same exact no-effect
  terminalization path with a distinct cause;
- deadline after possible effect cannot rewrite fate.

## Store-Owned Lifecycle

The mutation lifecycle owner maintains:

- `accepting` admission posture;
- one managed attempt per mutation identity;
- owned join handles for every worker;
- exact started, completed, proven-no-effect, indeterminate, observed,
  completed-but-unobserved, cancellation, and worker-panic counts;
- terminal observations retained long enough for same-runtime idempotent
  callers to join the exact attempt;
- an explicit stop-and-drain transition used by Store close.

Start registers or joins the exact mutation identity under the lifecycle lock.
Only the fresh registration spawns a worker. Duplicate prepared values for the
same live identity join that attempt; they cannot create another effect lane.

The worker consumes the existing proof-carrying progression in order:

1. pre-effect deadline/cancellation settlement;
2. WAL group planning, reservation, append, and seal;
3. WAL durability barrier;
4. exact data dispatch and settlement;
5. exact settled-group join;
6. root planning, candidate durability, replacement, and namespace sync;
7. current-root advance;
8. per-member completed settlement, idempotency terminalization, Store
   observation finalization, then waiter notification.

No waiter may observe terminal state before the Store registry and counters
are finalized.

Close performs this order:

1. stop checkpoint and mutation admission;
2. mark managed attempts runtime-closing;
3. request only still-lawful pre-effect cancellation;
4. join every mutation worker;
5. extract publication residue and current-root ownership;
6. stop work admission and finish the existing Signal/work/residency/media
   shutdown protocol.

## Evidence Projection

- `StoreExecutedBoundaryReceiptEvidence` is constructible only from a borrowed
  or consumed `PhysicalMutationAcknowledgment` and carries descriptive
  identity, fingerprint, binding, durability-policy, and completed-breadth
  facts.
- no admission, progression, settlement, root, or acknowledgment constructor
  accepts executed-boundary evidence;
- proven-no-effect and indeterminate types project only separately named
  diagnostic evidence;
- performance evidence consumes counters and completed observation only; it
  is not a completion authority.

## Ledger Corrections

Retain the existing Phase 8 API, lifecycle, and Signal rows. Add explicit rows
for:

- completion-only acknowledgment construction and exact completed breadth;
- proven-no-effect versus indeterminate separation, including the no-retry and
  no-acknowledgment boundary;
- completed-but-unobserved accounting and observer-finalization order;
- one-way executed-boundary and diagnostic evidence projections.

Add a Phase 8 ledger validator and exact Phase 8 source-closure identity. The
validator must reject omission, stale proof, phase reassignment, and any Phase
9 start while a Phase 8 row is unresolved.

## Cleanup And Cutover

- narrow `PhysicalRecordSubmission` to key issuance and preparation;
- move start/wait/cancel/settlement ownership to durability mutation and
  lifecycle modules;
- make WAL/data/root phase-driving methods crate-private and unavailable to
  ordinary downstream callers;
- retain explicit certification seams only under certification authority and
  name them as certification operations;
- replace ordinary tests, examples, binaries, and support fixtures with the
  final prepared/start/wait API;
- narrow `DurableAckReceipt` to its WAL-boundary fact; it must not look like the
  final physical acknowledgment;
- delete obsolete acknowledgment aliases and any convenience wrapper that
  hides idempotency, durability request, deadline, cancellation, or
  indeterminate fate;
- update public API, removal, and authority inventories from fresh scoped
  discovery after the cutover, not from guessed counts.

## Phase 8 Evidence

- compiled ordinary caller example covering all six admission branches and all
  three terminal fates;
- public UI failures for raw completion construction, acknowledgment from
  proven-no-effect or indeterminate, phase skipping, reverse evidence
  projection, and old phase-driving facade access;
- real handle drop before start observation, before WAL effect, after WAL
  append, after WAL durability, during data settlement, during root
  publication, and after completion;
- cancellation at the same boundaries with exact typed outcomes;
- monotonic deadline elapsed before effect and deadline elapsed after possible
  effect;
- duplicate live handle joins and no second effect;
- acknowledgment-delivery loss recorded as completed-but-unobserved;
- stale-generation and runtime-closing observations;
- close with work in every lifecycle phase and with all caller handles dropped;
- independent artifact/root observation proving a completed acknowledgment's
  breadth;
- mutation tests that remove drain, terminal-registry-before-notify,
  completion-only acknowledgment, effectful-cancellation fencing, and one-way
  evidence direction.

## Ordered Implementation Slices

1. Add the missing ledger rows, Phase 8 validator skeleton, and locked
   destination/API identities as `OPEN` evidence.
2. Add terminal settlement, completed progression, observation, and one-way
   evidence types with private constructors and compile-fail attacks.
3. Add managed attempt, handle, Store lifecycle registry, pre-effect deadline
   and cancellation, and close-time drain.
4. Move the existing proof-carrying progression behind the worker and derive
   per-member terminal outcomes from completed root publication.
5. Persist completed and indeterminate idempotency fates and join duplicate
   live callers without a second effect lane.
6. Cut ordinary phase-driving methods from the public facade and migrate every
   ordinary consumer; preserve only explicit certification seams.
7. Reconcile API, authority, removal, and source identities from scoped current
   source; complete every Phase 8 ledger row only after its independent
   evidence passes.
8. Run holistic Phase 8 QA and reopen every affected guarantee after each
   defect correction. Phase 9 may begin only when all Phase 8 rows are proved
   against the exact current source closure.

## Initial Touched-File Inventory

This list is expanded only when a discovered Phase 8 dependency requires it:

- this plan;
- C.7 living closure ledger;
- Phase 8 ledger validator and source identity;
- durability mutation handle/outcome/completed progression;
- durability settlement/lifecycle/evidence-projection/observation modules;
- mutation idempotency fate and registry owners;
- prepared mutation and publication director lifecycle;
- serving runtime and Store shutdown protocol;
- physical-runtime public exports;
- Phase 8 tests and compile-fail cases;
- C.7 public API, authority, and removal inventories.
