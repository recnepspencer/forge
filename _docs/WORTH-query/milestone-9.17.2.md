# Milestone 9.17.2: Composite Runtime-World History And Coordinated Publication

> **Status:** Planned and ready for implementation. No production
> implementation claim is made by this specification.
>
> **Product posture:** This milestone establishes the memory-resident Runtime
> World composition authority in the dedicated `worth-runtime-world` owner
> crate. It makes no persistence, restart, physical database, merge, or public
> Query workflow claim.
>
> **Entry gate:** [Milestone 9.17.1.2](./milestone-9.17.1.2.md) closed at
> accepted production revision `95c9aa7455`. Its Relational bundle and
> independently borrowable Signal services are frozen prerequisites for this
> milestone.

## Goal And Roadmap Placement

Consume the exact Relational and Signal owner contracts established by
Milestones 9.17.1, 9.17.1.1, and 9.17.1.2 and establish one composition owner
for product branches. A product branch is a mutable reference to one immutable
single-parent composite commit. Each composite commit binds:

- one exact owner-issued Relational basis;
- one exact owner-issued Signal basis;
- the exact installed Bridge correspondence basis under which the two
  components form one interpretable runtime world;
- one ordinary parent when the commit is not a root;
- one owner-issued commit occurrence identity; and
- the exact component evidence and publication-attempt provenance that
  produced it.

Relational and Signal continue to own component truth, branches, publication,
settlement, and retention. The existing `worth-runtime-bridge` crate continues
to own installed semantic correspondence and ordinary Bridge routing. The new
`worth-runtime-world` crate is the cross-domain composition owner above those
participants. Query consumes its facade in Milestone 9.17.3 but cannot mint a
composite commit, move a product reference, or repair an owner guarantee.

This placement resolves the existing legal dependency graph:

```text
worth-relational -----------\
worth-signal ----------------+--> worth-runtime-world --> Query audiences
worth-runtime-bridge --------/
worth-foundational ---------->
worth-proof ----------------->
```

`worth-relational` already depends on `worth-runtime-bridge` for presentation
contracts. Therefore `worth-runtime-bridge` must not acquire a reverse
dependency on Relational, and composite ownership must not be implemented in
`crates/worth-runtime-bridge/src/runtime_world/`.

## Central Claim

No product branch moves unless every operation-named component result is
valid for one exact expected product-head observation and the Runtime World
owner wins one atomic compare-and-publish transition. Unchanged components
remain at their exact carried bases. Readers observe the complete old or
complete new composite basis, never a mixed world.

Owner-local effects are not falsely called cross-owner atomic. If one owner
performs before a sibling owner denies, cancellation arrives, settlement is
deferred, or the product-head comparison loses, the product reference remains
unchanged and the Runtime World owner retains a bounded,
`ProductUnpublishedOwnerEffects` recovery record. That record says exactly
which owner effects occurred, which obligations remain, and which next actions
are legal. It is not a product commit and it is not rollback.

The claim is false if:

- Relational or Signal currentness alone defines the product world;
- a component is selected through ambient `main`, `current`, or `latest`;
- an equal id, ordinal, version, digest, descriptor, receipt, or memory layout
  substitutes for owner admission;
- a component result becomes product-current before the product reference
  moves;
- owner-local movement is hidden after a losing product race;
- a stale attempt rebases or retries under a new expected head;
- unchanged components are refreshed or contacted opportunistically;
- a global Bridge, history, component-owner, or Signal runtime lock serializes
  unrelated product branches;
- one composite commit creates one owner retention lease per repeated use of
  the same exact component basis;
- history, observations, or recovery state grow without an installed bound;
- Query, an adapter, a persisted representation, or a diagnostic projection
  mints composition authority; or
- this milestone introduces a codec, backend, checkpoint, recovery cursor,
  physical runtime, or restart promise.

## Current Boundary And Required Correction

The completed owner milestones provide:

- independently borrowable Relational preparation, fork, publication,
  settlement, observation, retention, and lifecycle services;
- exact Relational and Signal admitted bases, with transport descriptors
  carried only as non-authorizing diagnostics;
- owner-local branch movement with complete performed/no-movement outcomes;
- Relational pending-settlement recovery installed before movement;
- exact current and historical component retention; and
- terminal retention obligations even when a capability or owner is lost.

The pre-gate code still leaves Relational basis/lifecycle calls and Signal
mutation on owner roots; 9.17.1.2 closes both service gaps. Because Relational
already depends on Runtime Bridge, this milestone resolves the remaining Cargo
cycle by composing above Bridge. Neither move changes component authority.

## Ownership And Truth Lock

| Responsibility | Sole owner | Explicit non-owner |
| --- | --- | --- |
| Relational branches, bases, candidates, commits, publication, settlement, and retention | Relational | Runtime World, Bridge, Query |
| Signal branches, bases, definition compatibility, owner-local operations, and retention | Signal | Runtime World, Bridge, Query |
| Installed semantic correspondence and its immutable configuration basis | Runtime Bridge | Relational, Signal, Runtime World, Query |
| Exact product component binding, composite commits, product references, retained partial effects, and coordinated publication | Runtime World composition owner | component owners, Query, Store |
| Product workflow admission and public projections | Query in 9.17.3 | Runtime World, component owners |
| Portable branch, reference, and boundary vocabulary | Foundational | every operational owner |
| Concrete phase and authority carriage | Proof beneath owner-sealed types | generic caller markers |
| Persistence, restart recovery, physical effect fate, and hydration | Store integration | this milestone |

The authoritative objects are component owner state inside Relational and
Signal, immutable composite commits and mutable product reference cells inside
Runtime World, and live retained-partial records for owner effects not yet
represented by a product commit.

History indexes, ancestry accelerators, inspection rows, cost reports,
diagnostics, and Query projections are derived. Destroying them must not change
which product commit a branch selects or which retained partial obligations
remain unsettled.

## Semantic Identities And Non-Substitution

The following are distinct sealed meanings even if their representations
match:

- `RuntimeWorldOwnerIdentity` identifies one live composition owner instance;
- `ProductBranchIdentity` identifies one mutable product reference for the
  lifetime of that owner and is never reused;
- `ProductBranchReferenceGeneration` changes only when that reference moves;
- `RuntimeWorldBootstrapAttemptIdentity` identifies the one attempt that may
  establish this owner's root commit and first product reference;
- `CompositeCommitIdentity` identifies one immutable commit occurrence and is
  not derived from content;
- `CompositeBasisIdentity` identifies the exact component/correspondence tuple
  for comparison and reuse;
- `CompositePublicationAttemptIdentity` identifies one bounded execution
  attempt; and
- `ProductUnpublishedOwnerEffectsIdentity` identifies one retained recovery
  obligation after at least one owner effect.

Equal component bases may appear in multiple distinct commit occurrences.
Equal composite bases do not collapse history, parentage, operation occurrence,
or reference movement. Digests may prove canonical comparison but never mint
any identity above.

Every `ProductBranchObservation` compares the complete owner identity, branch
identity, lifecycle incarnation, reference generation, and selected composite
commit. Comparing only a branch name, commit id, or generation is forbidden.
Identity or generation exhaustion is a typed pre-effect denial.

## Composite Commit Contract

`CompositeRuntimeWorldCommit` is immutable and contains exactly:

- its owner-issued `CompositeCommitIdentity`;
- `Root` or one `OrdinaryParent` identity;
- the exact `CompositeRuntimeWorldBasis`;
- explicit Relational and Signal change posture;
- exact owner publication/fork identities for every changed component;
- the exact Bridge correspondence-basis identity;
- exact occurrence provenance: the root bootstrap attempt or one composite
  publication attempt;
- caller correlation accepted as descriptive, non-authorizing boundary
  provenance.

The root-admitted correspondence basis is inherited by every descendant.
Publication compares its installed generation and denies drift pre-effect as
`CompositeCorrespondenceRebindRequired`; changing correspondence requires a
new Runtime World owner and bootstrap.
The canonical `PerformedCompositePublication` envelope binds the immutable
commit to the product-reference movement that selected it. The commit does not
contain that later movement and can therefore be materialized before the
product-head compare-and-publish. Query 9.17.3 derives receipts, history, live
identity, aftermath, and outbox eligibility from the commit plus this one
performed-publication artifact; it may not create an operation-to-commit
authority table beside them.

This milestone admits one ordinary parent only. Parentage is placed beneath a
stable `history/parentage/` axis. Later multi-parent work may add an ordered
parent-set sibling or versioned commit family there, but no multi-parent API,
placeholder, optional vector, or merge behavior is created now.

## Root Bootstrap

`RuntimeWorldOwner` construction creates an empty composition owner. It does
not inspect ambient component heads, infer a default branch, or manufacture a
root. Exactly one `bootstrap_root` operation may establish the owner graph. It
requires:

- one exact owner-issued admitted Relational basis;
- one exact owner-issued admitted Signal basis;
- one basis admitted through the Bridge Runtime World correspondence port;
- one validated initial `ProductBranchCreationIntent`; and
- installed history, branch, pin, attempt, and metadata budgets.

Bootstrap first validates owner/runtime/definition correspondence, reserves the
root commit occurrence and first branch cell, and acquires the two exact owner
retention obligations through the unique pin registry. It then installs the
root commit, first product reference, and canonical creation record in one
Runtime World critical section. It performs no component mutation.

The terminal topology is `PerformedRuntimeWorldBootstrap` or
`NoEffectRuntimeWorldBootstrap`. No-effect guarantees that any acquired lease
was released and no commit or product reference became visible. Performed is
linear and carries the admitted root basis, root commit, first branch
observation, exact pin accounting, and bootstrap occurrence. A second
bootstrap attempt, foreign basis, incompatible correspondence, identity or
capacity exhaustion, cancellation, or owner loss is a typed pre-effect denial.

Every later commit and product branch in this milestone descends from this one
root. Multiple roots, root import, orphan adoption, and root replacement are
not hidden bootstrap modes; later import or merge work must introduce their
own admitted semantics.

## Product Branch And Observation Contract

A product branch is one independently synchronized reference cell. Its public
observation is an admitted, managed object rather than a tuple:

```text
ProductBranchIdentity
    -> ProductBranchObservation
    -> AdmittedCompositeRuntimeWorldBasis
```

Admission pins the selected composite commit and its exact component bases for
the observation lifetime. Clones share one internal observation obligation;
they do not acquire a new owner lease or re-read a head. Serialization, if a
later boundary adds it, must discard operational admission.

Readers resolve the branch cell and commit under one publication-compatible
visibility boundary. A reader can observe the old cell or the new cell. It
cannot observe a moved reference whose commit is absent from history or whose
component pins are not yet installed.

Product branch creation starts from an admitted retained composite basis and
requires one explicit posture per component:

- `ReuseExact` retains the exact component basis without owner movement;
- `ForkExact` asks that component owner to create a distinct component branch
  from the exact admitted source; or
- `ForkAndAdvance` forks and then performs owner-local work declared by the
  admitted creation plan.

Omission is invalid. Two `ReuseExact` postures may select the existing commit.
Creation otherwise uses publication's pre-effect reservation and
Relational-then-Signal order. Each fork creates a new composition occurrence
under the source commit even when content is equal; a performed fork followed
by sibling denial terminates as `ProductUnpublishedOwnerEffects`.

Product branch retirement removes only the product reference and releases its
composition obligations. It never assumes a component branch is exclusive and
never deletes a component reference as a side effect. Owner-created component
branches carry explicit custody records and produce typed owner-retirement work
through owner lifecycle services; denied retirement remains bounded managed
work rather than a hidden leak.

## Component Change And Execution Order

Every publication plan contains exactly one specialized posture for each
component:

```text
RelationalComponentPlan
    RetainExact
    PublishPrepared
    ForkThenPublish

SignalComponentPlan
    RetainExact
    AdvanceExact
    ForkThenAdvance
```

Plans remain owner-specialized. Relational plans carry owner-issued candidates.
Signal advance posture is admitted during preparation, but its mutation and
caller-owned runtime context enter as a `SignalExecutionBorrow` only for the
synchronous owner call. The attempt never retains, clones, erases, or registers
either value.

For a combined change, canonical effect order is:

1. prepare every fallible compatibility, correspondence, budget, retention,
   history-slot, and attempt-record requirement;
2. prepare the Relational candidate without component movement;
3. recheck the exact product-head observation;
4. perform and settle Relational publication;
5. recheck the exact product-head observation;
6. perform the Signal owner operation;
7. recheck and compare-and-publish the product reference; and
8. issue the performed composite artifact.

Relational moves first because its owner installs recoverable settlement before
movement. Signal moves last because its canonical operation returns its final
successor basis synchronously and has no separate external settlement phase.
This order minimizes the least-recoverable residual without pretending either
owner can roll the other back.

Every step branches on its owner outcome. Relational no-movement ends as
`NoEffectCompositePublication`; Relational performed-but-unsettled ends as
`ProductUnpublishedOwnerEffects` without calling Signal; a stale product head
after settled Relational work also stops before Signal; and Signal denial or a
lost final product CAS after any owner movement ends as
`ProductUnpublishedOwnerEffects`. Only fully settled required Relational work
and performed required Signal work may reach the final product CAS.

Single-component operations execute only their changing owner. An unchanged
component incurs no execution call, latest lookup, compatibility rediscovery,
or new owner lease when its exact basis is already pinned by Runtime World.

No Runtime World lock, product-reference lock, history lock, or pin-registry
lock may be held while calling a component owner. Owner calls occur only from
named execution phases after Runtime World preparation locks have been
released.

## Pre-Effect Reservation And Attempt Authority

Before the first owner effect, Runtime World installs one bounded
`CompositePublicationAttempt` containing:

- exact expected product-head observation;
- admitted predecessor composite basis;
- explicit per-component plan;
- reserved composite commit identity and history slot;
- reserved product-unpublished recovery capacity;
- all predecessor and prospective retention obligations obtainable before
  effects;
- cancellation/deadline policy and the last safe no-effect point;
- canonical owner execution order; and
- structural counters initialized before execution.

Capacity for the attempt record, potential retained-partial record, commit
metadata, and required Runtime World pins is reserved before owner effects. An
owner may still return an owner-local bounded-capacity denial according to its
own contract, but Runtime World never discovers that its own bookkeeping or
history capacity is exhausted after an owner has moved.

The attempt is linear. Every terminal path consumes it into exactly one of:

- `NoEffectCompositePublication`;
- `ProductUnpublishedOwnerEffects`;
- `PerformedCompositePublication`; or
- a branch-creation/retirement terminal governed by the same resource rules.

Dropping a pre-effect attempt performs deterministic local cleanup. Dropping an
attempt after any owner effect cannot erase the attempt; ownership first moves
into the preinstalled retained-partial registry, and Drop only abandons the
caller capability while owner recovery remains addressable.

## Product-Unpublished Owner Effects

`ProductUnpublishedOwnerEffects` means at least one component owner performed
work and no product reference moved. It is never called a prepared candidate,
rollback, conflict-only result, or failed commit.

It records:

- the exact expected and last observed product heads;
- a typed progress row for each component: untouched, prepared, performed,
  settlement pending, or settled;
- every performed owner occurrence and successor basis;
- every still-live component and composite retention obligation;
- the cause: sibling denial, owner settlement pending, cancellation after
  effect, stale product head, owner loss, or product publication loss;
- legal next actions: resume owner-local settlement, expose the exact record
  for a new Query-admitted operation, abandon after owner-safe cleanup, or
  inspect only; and
- deadline, age, count, and byte-accounting posture.

Resume consumes a fresh recovery capability issued from the retained record
and may perform only owner settlement or owner-safe cleanup. It cannot call an
unperformed sibling owner or move a product reference. Continuing or adopting
owner-local movement is always a new Query-admitted operation in 9.17.3 with a
fresh expected product head and current authority; that new operation names
the retained record as exact evidence but does not derive authority from it.

The registry is bounded by installed count and Runtime World metadata-byte
budgets. Exhaustion rejects before owner effects. Age expiry makes the record
eligible for governed cleanup; it never causes a background thread to discard
owner settlement, release a still-required component basis, or move a product
reference. The owner lifecycle can enumerate every record and terminate or
report it during close.

## Publication And Outcome Topology

The compiler-visible progression is:

```text
ProductBranchIntent
    -> ResolvedExpectedProductHead
    -> AdmittedCompositeRuntimeWorldBasis
    -> LoweredOwnerComponentPlan
    -> ReservedCompositePublicationAttempt
    -> OwnerExecutionSettlement
    -> CompositePublicationReady
    -> RuntimeWorldPublicationOutcome
         Performed(PerformedCompositePublication)
         NoEffect(NoEffectCompositePublication)
         ProductUnpublished(ProductUnpublishedOwnerEffects)
```

`NoEffectCompositePublication` guarantees that no component owner moved and no
product reference moved. It distinguishes stale expected head, rejection,
cancellation, deadline, owner denial, and internal pre-effect failure.

`ProductUnpublishedOwnerEffects` guarantees that the product reference did not
move and that at least one named owner effect occurred. Its recovery record is
the only ordinary continuation authority.

`PerformedCompositePublication` proves that the exact reserved commit was
installed and the expected product reference moved once. It carries the
canonical commit, old and new reference observations, exact component results,
late-cancellation posture, retention transfer, and cost counters. It is linear
private authority consumed by Query 9.17.3. A cloneable inspection projection
may describe it but cannot authorize a Query committed terminal.

Cancellation is no-effect only through the last safe point before the first
owner effect. Between owner calls or before product CAS it produces
`ProductUnpublishedOwnerEffects`. After product CAS, movement wins and the
performed artifact carries late-cancellation evidence.

There is no indeterminate in-memory product-reference movement: the Runtime
World owner either observes its local reference cell move or not move. Owner
settlement may remain pending or unavailable, which is represented by the
retained partial rather than by inventing an indeterminate product head.

## Atomic Product Publication

Each product branch has an independently borrowable reference cell. The final
critical section:

1. compares the complete expected `ProductBranchObservation`;
2. validates the reserved history slot and already-installed component pins;
3. materializes the immutable commit into that reserved slot;
4. emits the canonical reference-movement record; and
5. changes the branch cell to the new commit and next generation.

These steps are one reader-visible critical section for that branch. Every
fallible Runtime World allocation or capacity check happens earlier. Failure
or panic before reference movement leaves the old head selected and the
attempt recoverable. Once the cell moves, commit and movement evidence are
already available and the performed artifact can be reconstructed by the live
owner if its caller capability is lost.

The branch cell is the composition-currentness linearization point. History
insertion alone, a reserved commit identity, complete owner settlement, or a
diagnostic event does not make a product world current.

## Retention And Reclamation

Runtime World owns a unique component-basis pin registry. The key is the full
owner-issued exact basis identity, never a hash alone. One registry entry holds
at most one external owner retention lease for that exact basis and accounts
for Runtime World dependents by semantic class:

- product branch heads;
- retained composite history;
- admitted observations;
- active publication attempts;
- product-unpublished owner effects; and
- explicit historical inspection or correction obligations.

Repeated composite commits that reuse one Signal or Relational basis increment
Runtime World dependency counts; they do not acquire one owner lease per
commit. The last dependent releases the owner lease exactly once. A denied
foreign release preserves and rebinds the still-live lease according to the
component owner's contract.

Concurrent first use is single-flight per exact basis: one claimant installs a
bounded acquiring reservation, releases the registry lock, and alone calls the
owner. Contenders join that reservation. Success installs one lease; denial
removes the reservation and wakes contenders with the same typed result.

Composite commit records contain exact descriptors and owner occurrence
identities, not operational owner leases. The history retention graph owns the
pins needed to interpret retained records.

The in-memory history budget has exact commit-count and Runtime World
metadata-byte limits. Reachable ancestry is not silently truncated because
9.18 consumes the complete retained single-parent tree. Reclamation may remove
only commits unreachable from every product reference and explicit obligation,
in bounded maintenance batches. If the budget is full and no eligible node can
be reclaimed, new branch creation or publication is denied before owner
effects as `CompositeHistoryCapacityExhausted`.

Component-state byte totals remain owner-scoped. Runtime World reports:

- exact Runtime World metadata bytes;
- unique component-basis pin counts;
- owner lease acquisition and release counts;
- Relational byte observations only under Relational's declared metric scope;
  and
- Signal capacity and obligation counts without relabeling them as exact
  Signal resident bytes.

No counter claims a cross-owner total that the owners do not expose.

## Lifecycle And Construction

`RuntimeWorldOwner` is a non-cloneable managed owner. It constructs cloneable
weak observation, publication, recovery, and lifecycle ports. Ports share the
live owner state but cannot keep a closed owner alive. Calls after close return
typed `RuntimeWorldOwnerUnavailable`.

Request-local `RuntimeBridge::fork_managed_request_lane` never clones, resets,
or forks product history; Query composition explicitly carries one Runtime
World port beside any request-local Bridge execution lane.

Construction is compiler-total. The application composition root must supply:

- the base Bridge correspondence-basis port;
- Relational preparation, fork, publication, settlement, observation,
  retention, and lifecycle services;
- Signal basis, mutation, and lifecycle services from 9.17.1.2;
- installed history, attempt, retained-partial, observation, branch, and pin
  budgets; and
- an explicit clock only for attempt deadlines and cleanup eligibility.

Adding or omitting one required subsystem breaks every construction site. The
clock cannot affect product meaning, identity, parentage, or authority.

Close stops new admission, waits only for declared in-flight critical sections,
settles or exposes every retained owner obligation, releases product/reference
pins, and returns a terminal report. It does not keep owners alive through a
strong cycle and does not manufacture success when a component owner is lost.

## Owner-Facing DX Contract

The integration path must be expressible through the real facade without
private imports:

```rust,no_run
use worth_runtime_world::facade::{
    CompositeComponentIntent, CompositeExecutionBorrow, ProductBranchCreationIntent,
    RuntimeWorldBootstrapIntent, RuntimeWorldOwner, RuntimeWorldPublicationOutcome,
};

let world = RuntimeWorldOwner::builder()
    .with_bridge_correspondence(bridge.runtime_world_correspondence_port())
    .with_relational_services(relational.owner_component_services())
    .with_signal_services(signal.owner_component_services())
    .with_budgets(runtime_world_budgets)
    .with_clock(runtime_world_clock)
    .build()?;

let bootstrapped = world.lifecycle_port().bootstrap_root(
    RuntimeWorldBootstrapIntent::new(
        ProductBranchCreationIntent::named("main")?,
        initial_relational_basis,
        initial_signal_basis,
        installed_correspondence_basis,
    ),
)?;
let product_branch = bootstrapped.product_branch();

let expected = world
    .observation_port()
    .observe_product_branch(product_branch)?;

let prepared = world.publication_port().prepare(
    expected,
    CompositeComponentIntent::relational_only(relational_change),
)?;

match world.publication_port().execute(
    prepared,
    CompositeExecutionBorrow::without_signal(),
    cancellation,
)? {
    RuntimeWorldPublicationOutcome::Performed(performed) => {
        query_handoff.accept(performed)?;
    }
    RuntimeWorldPublicationOutcome::NoEffect(no_effect) => {
        inspect_no_effect(no_effect);
    }
    RuntimeWorldPublicationOutcome::ProductUnpublished(retained) => {
        recovery_queue.retain(retained.recovery_handle());
    }
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

Signal-changing execution instead borrows its caller context and lowered
mutation through `CompositeExecutionBorrow::signal`; the borrow ends with the
synchronous call. Builder grouping may refine, but callers never supply a raw
owner runtime, generic authority marker, component-id string, or private
history handle.

## Destination Dependency And Module Topology

The current slice creates this package and populated topology:

```text
crates/worth-runtime-world/                         [create]
    Cargo.toml                                      [create: includes test-operation-control]
    README.md                                       [create]
    COMPOSITE_HISTORY.md                            [create]
    COORDINATED_PUBLICATION.md                      [create]
    RETENTION_AND_RECOVERY.md                       [create]
    src/
        lib.rs                                      [create: private topology + facade]
        facade.rs                                   [create: sole public aggregation]
        basis/
            composite.rs                           [create]
            admission.rs                           [create]
            equivalence.rs                         [create]
        identity/
            owner.rs                               [create]
            branch.rs                              [create]
            commit.rs                              [create]
            bootstrap.rs                           [create]
            attempt.rs                             [create]
        history/
            commit.rs                              [create]
            catalog.rs                             [create]
            parentage/
                ordinary_parent.rs                 [create]
            retention.rs                           [create]
            reclamation.rs                         [create]
        branch/
            bootstrap.rs                           [create]
            reference_cell.rs                      [create]
            observation.rs                         [create]
            creation.rs                            [create]
            retirement.rs                          [create]
        publication/
            intent.rs                              [create]
            component_plan.rs                      [create]
            reservation.rs                         [create]
            owner_execution.rs                     [create]
            product_comparison.rs                  [create]
            performed.rs                           [create]
            no_effect.rs                           [create]
        recovery/
            product_unpublished.rs                 [create]
            progress.rs                            [create]
            continuation.rs                        [create]
            cleanup.rs                             [create]
        retention/
            unique_component_pin.rs                [create]
            dependency_counts.rs                   [create]
            obligation_transfer.rs                 [create]
        lifecycle/
            owner.rs                               [create]
            ports.rs                               [create]
            close.rs                               [create]
        inspection/
            history.rs                             [create]
            retention.rs                           [create]
            recovery.rs                            [create]
            cost.rs                                [create]
    examples/
        runtime_world_publication.rs                 [create: executable contract]
    tests/
        runtime_world_certification.rs               [create: one integration target]
        runtime_world_certification/
            world.rs                                [create: court assembly only]
            world/
                definition.rs                       [create: semantic world inputs]
                compiler.rs                         [create: public-facade compilation]
                relational.rs                       [create: real owner fixture]
                signal.rs                           [create: real owner fixture]
                bridge.rs                           [create: installed correspondence]
                observation.rs                      [create: neutral observations]
            oracle.rs                               [create: pure oracle assembly only]
            oracle/
                state.rs                            [create: test-local world state]
                transition.rs                       [create: independent progression]
                comparison.rs                       [create: neutral comparison]
            cases/
                provenance_and_bootstrap.rs         [create]
                branch_lifecycle.rs                 [create]
                component_plans.rs                  [create]
                publication_outcomes.rs             [create: sequential outcomes]
                recovery.rs                         [create]
                retention_history.rs                [create]
                substitution.rs                     [create]
                model_sequences.rs                  [create]
                cost.rs                             [create]
                facade.rs                           [create]
                operation_control.rs                [create: feature-gated assembly]
                operation_control/
                    concurrency.rs                  [create]
                    publication_boundaries.rs       [create]
                    recovery_boundaries.rs          [create]
            ui/                                     [create: grouped pass/fail fixtures]

crates/worth-runtime-bridge/src/
    correspondence/
        runtime_world_admission.rs                  [create: admit existing Bridge meaning]
    facade/
        runtime_world.rs                            [create: curated admission port]

crates/worth-runtime-world/src/correction/           [9.18 committed successor;
                                                      do not create now]
crates/worth-runtime-world/src/history/parentage/
    ordered_parent_set.rs                            [cross-runtime committed
                                                      successor; do not create now]
```

The dominant axes are:

- base Bridge `correspondence/`: the existing installed semantic
  correspondence meaning plus a narrow Runtime World admission port, excluding
  component state and product history;
- Runtime World `basis/`: the exact composite of owner bases under admitted
  Bridge meaning, excluding mapping implementation;
- `history/`: immutable commit occurrences and retention, excluding mutable
  reference policy;
- `branch/`: mutable product references and lifecycle, excluding commit
  contents;
- `publication/`: forward phase progression to product movement, excluding
  recovery of already-performed owner effects;
- `recovery/`: product-unpublished owner-effect lifecycle, excluding ordinary
  publication and certification replay;
- `retention/`: unique operational component obligations, excluding history
  navigation; and
- `inspection/`: derived read-only projections, excluding operational handles.

The stable facade is `worth_runtime_world::facade`. Internal modules are
private or `pub(crate)`. The facade aggregates but implements no publication,
history, retention, or recovery behavior.

Forbidden destinations include:

- `worth-runtime-bridge/src/runtime_world/`;
- Query-owned product references or history;
- a generic `branch_manager`, `runtime_world_manager`, `helpers`, `common`, or
  `shared` bucket;
- Relational-owned Signal correspondence;
- Bridge access to private owner storage;
- a global mutex around any component runtime or the whole composition owner;
- replay, persistence, codec, Store adapter, or physical-runtime modules.

A listed file that would exceed 400 lines splits by its named semantic
responsibilities; it may not become a catch-all or require an exemption.

## Dependency Enforcement

Mechanical dependency checks must enforce:

- `worth-runtime-world` may depend on public facades of Relational, Signal,
  Runtime Bridge, Foundational, and Proof;
- `worth-runtime-world` may not depend on any Query package, Store package,
  replay facade, certification crate, UI crate, or application crate;
- `worth-runtime-bridge`, `worth-relational`, and `worth-signal` may not depend
  on `worth-runtime-world`;
- `worth-runtime-bridge` remains forbidden from depending on
  `worth-relational`;
- Query 9.17.3 consumes `worth-runtime-world` only through its facade; and
- no ordinary package reaches a certification-only replay surface.

Facade compile evidence must deny raw basis or commit construction,
cross-owner/cross-head pairing, phase skipping, duplicate performed-witness
use, retained-partial promotion, and generic authority-marker substitution.

## Ordered Phase Plan

Contract evolution uses serial freeze gates followed by parallel waves of two
to four implementation lanes. Only a gate may edit `Cargo.toml`, `lib.rs`,
module roots, `facade.rs`, shared phase/outcome types, or integration-target
roots. A parallel lane exclusively owns its named paths and must land real
behavior or independent evidence. If implementation disproves a contract, the
affected lanes pause while a serial correction gate revises the canonical type
once and reruns its consumers; no lane may add an adapter, alias, duplicate
trait, or provisional compatibility surface.

Every incremental-review prompt begins by limiting review to the exact last
approved-to-candidate diff and directly invalidated seams. Reviewers read that
patch once, reuse prior conclusions and evidence, and revisit only named hunks
or line ranges; they do not reopen the whole diff or reread the milestone,
crate, kernel, manifests, or dirty surface. Any expansion is announced before
inspection. Holistic review occurs only at an explicit phase or closure gate.

### Phase 1: Serial composition-contract gate

Create `worth-runtime-world`, install dependency fences and assembly, and add
the narrow Bridge admission contract. Freeze identities, budgets, owner bundle
inputs, composite basis, phase/outcome vocabulary, exact attempt progress, and
the branch/publication/recovery service seams. These are real invariant-bearing
types, not hollow scaffolding. This gate owns all shared files and records the
exclusive paths and focused commands for each following lane.

### Phase 2: Parallel foundations

- **Bridge lane:** own only Runtime Bridge correspondence admission behavior
  and its focused facade-contract proof, excluding facade aggregation;
- **basis/history lane:** own Runtime World basis, identity, immutable commits,
  catalog, parentage, and bounded history behavior;
- **retention lane:** own unique component pins, dependency counts, obligation
  transfer, and reclamation; and
- **reference lane:** own product reference cells, managed observations, and
  old-or-new reader semantics, without branch creation policy.

The lanes consume Phase 1 contracts and cannot edit assembly or facades. This
wave establishes honest independent owners before any workflow orchestrates
them.

### Phase 3: Serial bootstrap and publication gate

Integrate exact root bootstrap, owner construction, capacity installation, and
the Phase 2 services. Prove immutable history is distinct from mutable
currentness and unique pins precede retained history. Then freeze the exact
component-plan, reservation, owner-execution, product-CAS, recovery, and close
interfaces used by the next wave.

### Phase 4: Parallel product mechanics

- **branch lane:** own branch bootstrap completion, explicit reuse/fork
  creation, retirement, custody records, and branch/history capacity;
- **preparation lane:** own intent lowering, owner-specialized plans,
  compatibility and expected-head checks, reservations, pins, and linear
  pre-effect attempt installation;
- **owner/recovery lane:** own canonical Relational-then-Signal execution,
  typed settlement, product-unpublished progress, continuation, cleanup, and
  caller-loss transfer; and
- **publication/lifecycle lane:** own exact-head comparison, reserved commit
  install, reference movement, performed/no-effect issuance, owner close, and
  derived inspection.

The lanes own disjoint files under `branch`, `publication`, `recovery`,
`lifecycle`, and `inspection`; none may hold a product lock across an owner
call. Predictable Runtime World failures remain pre-effect, and continuation
toward product movement after retained partial work remains a fresh
Query-admitted operation.

### Phase 5: Serial progression and facade freeze

Assemble the full typed progression through the frozen services, audit every
cancellation and capacity edge, and resolve any contract revision at its sole
owner. Prove the three outcomes, same-head one-winner CAS, unrelated-branch
progress, bounded recovery, and no half-current product world. Freeze the sole
public facade and exact performed/recovery artifacts consumed by 9.17.3.

### Phase 6: Parallel certification and documentation

- **production-world lane:** own causal public-facade compilation, real owner
  fixtures, neutral observations, bootstrap, branch, and sequential cases;
- **independent-oracle lane:** own pure state, transitions, comparison, and
  model sequences without production semantic helpers;
- **adversarial lane:** own operation-control publication/recovery schedules,
  hostile substitutions, compiler fixtures, and race courts; and
- **operability lane:** own cost/scale evidence, facade cases, executable
  example, owner guides, and dependency/residue proof.

### Phase 7: Serial closure gate

Assemble the one intentional integration target and run its default and
feature-gated selections plus owner, compiler, scale, documentation,
formatting, lint, line-cap, boundary, and generated-context checks. A
review-only or merge-only assignment does not count as a parallel lane, and no
unresolved contract fork may cross the 9.17.3 handoff.

## Performance And Resource Contract

Name these scale axes in implementation and evidence:

- `B`: live product branches;
- `H`: retained composite commits;
- `U`: unique exact component bases pinned by Runtime World;
- `A`: active publication attempts;
- `P`: retained product-unpublished records;
- `O`: active admitted observations; and
- `W`: concurrent writers.

Required bounds are:

- product-head observation and comparison are O(1) in `B` and `H`;
- basis admission is O(1) in the fixed two-component cardinality;
- branch creation is O(1) plus only operation-named owner fork work;
- ordinary publication is O(changed components) plus O(1) product publication;
- unchanged component execution and new owner-lease acquisition are exactly
  zero when the unique basis pin already exists;
- one branch operation acquires no other product branch's reference cell;
- no ordinary operation scans `B`, `H`, `U`, `A`, `P`, or `O`;
- history traversal is explicit O(requested span) work with a caller-visible
  bound, never a cheap-looking getter;
- reclamation is explicit maintenance bounded by the requested batch;
- retained-partial cleanup cannot export unbounded work to a background queue;
  and
- total retained Runtime World metadata, attempts, observations, branches,
  commits, and recovery records stay within installed limits.

Counters expose at least product cells touched, history slots reserved and
installed, owner contacts by component, expected-head rechecks, unique pin
hits/acquisitions/releases, no-effect outcomes, owner-effect progress,
retained-partial creation/resume/cleanup, CAS attempts/wins/losses, observations,
and reclamation breadth.

The deterministic court must use controlled schedules rather than timing to
prove unrelated progress and same-head races. The scheduled scale profile must
measure structural slopes across `B`, `H`, `U`, `A`, and `W` with named runtime
configuration, cold/warm posture, repetitions, variance, and percentiles.

## Adversarial Courtroom

The decisive real-facade court starts two product branches from one commit. It
parks one owner call while an unrelated branch publishes to convict global
locking. On one product branch, Relational-only and Signal-only attempts share
one expected head; both owner effects must linearize and park before product
CAS. Exactly one becomes `Performed`; independent owner observations require
the loser to be `ProductUnpublished`. A combined-versus-single variant proves
owner-phase losers are classified from actual movement, not predicted winner
identity. This schedule convicts a per-branch lock held across owner calls.

The pure history oracle owns semantic state, not outcome classification. Direct
owner counters and Runtime World observations feed the outcome table below.
The remaining bootstrap, creation, recovery, retention, substitution,
capacity, compiler, and scale obligations belong to their named families.

## Test Evidence Architecture

The certification target follows the established Supply Chain and Bank World
standard: a causally compiled real world, an independently authored oracle,
direct evidence from every external owner crossed by the claim, and hostile
twins that differ in one relevant fact.

The production side uses only the real public facades of Runtime World,
Relational, Signal, and Runtime Bridge. The expected side uses a pure
`CompositeWorldOracle` over test-local semantic identifiers, maps, sets, and
transition rules. Production observations are converted at the boundary into
neutral values before comparison. The oracle must not call or copy Runtime
World history, publication, digest, comparison, pinning, retention,
reclamation, or recovery algorithms.

World declaration/compiler failures, component-owner failures, Runtime World
denials, observation failures, and oracle disagreements have distinct test
errors. Every hostile family proves a healthy twin at the same boundary. A
denial after invalid setup, an absent owner call, or production-comparator reuse
is not evidence.

## Canonical Composite Court World

`CompositeSupplyChainCourt` is a compact but semantically credible port,
voyage, and manifest world. Its compiler:

1. installs component state in a real Relational owner through its public
   facade;
2. installs the corresponding cargo-routing derivation in a real Signal owner
   through its public facade;
3. installs and admits a real base-Bridge correspondence basis; and
4. creates a real Runtime World owner and performs explicit root bootstrap from
   those exact admitted bases.

The court must not copy the private test kit of another crate or replace an
owner with a mock. Its scope is the composition contract, so the component
world is intentionally smaller than the Relational Supply Chain certification
world while still crossing the real owner boundaries.

The `CompositeWorldOracle` models independently:

- explicit bootstrap and one root occurrence;
- immutable commit occurrences with exactly one ordinary parent;
- product branch identity, lifecycle incarnation, head, and generation;
- exact Relational, Signal, and correspondence bindings;
- `ReuseExact`, `ForkExact`, and `ForkAndAdvance` component postures;
- unique component-pin dependency classes for heads, history, observations,
  active attempts, and retained partials; and
- bounded attempt, history, reclamation, and recovery state.

Runtime World public history, reference, observation, retention, recovery, and
cost projections provide product evidence. Relational and Signal public owner
observations and counters independently establish component contact, movement,
and settlement; a composite result cannot establish any of them.

## Required Runtime World Scenario Families

The one certification target must contain these named families:

1. **Provenance and bootstrap:** prove one healthy explicit root with zero owner
   mutation calls and unchanged component heads; then attempt
   duplicate, foreign-owner, foreign-runtime, incompatible-correspondence,
   cancelled, and each capacity-denied bootstrap. Each denial is exact
   no-effect and releases every temporary lease or reservation.
2. **Branch lifecycle and the posture matrix:** cover the full three-by-three
   Relational/Signal matrix of `ReuseExact`, `ForkExact`, and
   `ForkAndAdvance`. Omission is denied. A forked but content-equal basis creates
   a distinct component binding and composite occurrence. If one fork performs
   before its sibling denies, bounded product-unpublished custody owns the
   result. Retirement releases only obligations owned by that branch.
3. **Component plans and call exactness:** exercise retain, owner-local change,
   and `ForkThen*` for each owner. Fork creates a new owner identity and
   occurrence without moving the source. Single-owner work makes zero sibling
   contacts; combined work follows canonical order, and unsettled Relational
   performance prohibits Signal contact.
4. **Publication outcome table:** cover cancellation, preflight denial, stale
   expected head, each owner denial, owner performance followed by sibling
   denial, owner settlement followed by a late head change, Signal performance
   followed by CAS loss, CAS success followed by cancellation, capability loss
   after an owner effect, and owner loss. Assert exactly `NoEffect`,
   `ProductUnpublished`, or `Performed` from direct owner and product evidence.
5. **Deterministic concurrency:** prove unrelated progress at each owner park.
   On one head, make Relational-only and Signal-only effects both linearize and
   park before CAS; exactly one performs product movement and the other is
   product-unpublished. A combined-versus-single variant covers owner-phase
   loss. Direct owner evidence classifies losers; readers see only complete old
   or new bases.
6. **Recovery authority:** prove the retained-partial handle is owner- and
   attempt-affine; forged, copied, or foreign handles fail. Resume may settle or
   clean up only—it cannot call the sibling owner or publish. Cover lost caller
   capability, cleanup-versus-inspect/resume, owner close, clock-advanced cleanup
   eligibility without lost settlement, and complete obligation enumeration.
   Query-admitted adoption remains 9.17.3 work.
7. **Retention and history:** reuse one Signal basis across sequential and
   concurrent commits and prove one external lease. Exercise every dependency
   class, cloned observations with one shared obligation, final-drop release,
   bounded batch reclamation, unreachable-only pruning, ancestry protection,
   history-full pre-effect denial, and branch-name retire/recreate ABA defense.
8. **Authority substitution:** substitute representation-, ordinal-, digest-,
   or version-equal artifacts from another owner, branch, definition,
   correspondence, or Runtime World, including installed-correspondence drift.
   Raw identifiers and descriptors cannot admit, retain, recover, or publish.
9. **Seeded model sequences:** run deterministic sequences of bootstrap,
   branch create/retire, publish, observe/drop, retain/release, cancel, stale
   comparison, reclaim, resume, and cleanup against the real world and oracle.
   On failure, print the seed and minimal reproducing prefix.
10. **Structural cost:** vary `B`, `H`, `U`, `A`, `P`, `O`, and `W`
    independently. Assert exact-zero unrelated owner contacts, no new lease on a
    unique-pin hit, no foreign product-cell touch, bounded reclamation breadth,
    and the required ordinary-operation slopes. Do not invent Signal bytes.
11. **Facade, dependency, and documentation:** compile-pass facade-only use;
    compile-fail raw basis/commit construction, cross-head or cross-owner
    pairing, duplicate performed consumption, product-unpublished or inspection
    projection promotion, generic authority substitution, and private access.
    Run dependency fences and the public example as executable evidence.

## Outcome Classification Table

These boundary cases are mandatory because they distinguish component truth
from product truth:

| Last independently observed fact | Required outcome | Forbidden interpretation |
| --- | --- | --- |
| cancellation, preflight/stale denial, or owner denial before any owner movement | `NoEffect` | an attempt or partial record presented as movement |
| an owner moved, required sibling work is incomplete, and the product head stayed put | `ProductUnpublished` | rollback, performed product state, or implicit sibling continuation |
| Relational settled, then the product head changed before Signal | `ProductUnpublished` | calling Signal against a now-stale product intent |
| Signal moved, then final product CAS lost | `ProductUnpublished` | silently adopting owner-local movement |
| product CAS installed the reserved commit and movement envelope carrying exact changed-owner occurrence identities | `Performed` | late cancellation or an inspection projection replacing linear authority |
| caller capability disappeared after an owner effect | preinstalled retained partial | losing custody, duplicating work, or finishing the sibling/product step |

Each row needs a healthy twin, direct component-owner counters, Runtime World
attempt/recovery observations, and the oracle's expected semantic state.

## Lane And Harness Contract

All runtime evidence remains under `runtime_world_certification.rs`; directory
modules do not become additional Cargo integration targets. UI fixtures execute
as a grouped compiler family.

- The ordinary CI lane owns bootstrap denials, sequential no-effect/owner-loss
  outcomes, branch lifecycle, component plans, recovery authority,
  retention/history, substitution, facade evidence, and the Court model.
- The CI operation-control lane runs with the test-only
  `test-operation-control` feature and owns deterministic owner parks,
  cancellation/fault boundaries, same-head races, reader atomicity, and their
  healthy twins.
- The scheduled ignored lane owns longer model sequences, larger histories and
  contention, and named Court/Standard/Scale profiles with configuration,
  cold/warm posture, repetitions, variance, p50/p95/p99, and structural slopes.

The operation-control feature may only park or inject faults at named points in
the real phase progression. It may not mint authority, bypass an owner, replace
publication logic, or change production semantics. Synchronization uses
barriers or channels, bounded waits, and drop-safe release guards—not sleeps.
It is disabled by default and forwards only the corresponding Relational and
Signal test-operation-control capabilities required by the court. CI must fail
if a command intended to select a required feature-gated family executes zero
cases.

The planned commands are:

```text
cargo test -p worth-runtime-world --test runtime_world_certification
cargo test -p worth-runtime-world --features test-operation-control --test runtime_world_certification operation_control::
cargo test -p worth-runtime-world --features test-operation-control --test runtime_world_certification -- --ignored
```

The last command belongs to the scheduled lane. This milestone does not add
Docker, TCP, or process tests: Runtime World is intentionally memory-resident,
and those boundaries would be theatre here. Query's real composition root is
9.17.3; durable restart belongs to Store successors.

## Sensitivity And Teardown Contract

The suite must become red for each defective implementation below, through a
targeted hostile twin, fault placement, or review-time mutation probe:

- removing final expected-head comparison or CAS;
- inverting canonical owner order;
- holding any product or Signal-global lock across an owner call;
- acquiring one component lease per commit instead of per unique exact basis;
- resolving latest or accepting a representation-compatible descriptor;
- treating any owner effect as performed product truth;
- allowing retained-partial recovery to call a sibling owner or product CAS;
- pruning reachable ancestry;
- exposing a commit before its reference-movement envelope is complete; or
- reusing a retired product branch's lifecycle identity.

Every family uses fresh owners. Success and failure paths release observations,
leases, parks, and test faults; close Runtime World and both component owners;
and prove there are no unexplained attempts, pins, custody records, waiters, or
partial records. A drop-safe harness must unblock parked workers before
propagating a panic so an assertion failure cannot hang the suite or poison the
next case.

## QA Considerations

Architecture review must confirm the new crate is the sole composition owner,
the Cargo graph is acyclic, component owners retain their authority, and no
global lock or compatibility representation bypasses the public owner ports.

Lifecycle and concurrency review must cover capability loss, owner loss,
same-head races, unrelated branch progress, cancellation at every safe point,
retained-partial resume/cleanup races, observation versus reclamation, and
owner close.

Performance review must validate structural counters and scopes, unique-basis
deduplication, fixed-cardinality execution, bounded histories and registries,
and absence of background-cost laundering. Signal counts must not be presented
as byte truth.

Evidence review must ensure the independent oracle does not reuse production
comparison or retention logic, the real owner facades are crossed, failure
setup cannot create wrong-reason green results, compile sessions are grouped,
and the selected scale lane is proportionate.

## Documentation Deliverables

- `crates/worth-runtime-world/README.md` for integrators: construction,
  explicit root bootstrap, ownership, common publication flow, typed outcomes,
  and the explicit memory-resident limit.
- `crates/worth-runtime-world/COMPOSITE_HISTORY.md` for Query and future
  correction authors: commit occurrence versus basis equivalence, parentage,
  root bootstrap, product references, observations, branch creation, and
  retained history.
- `crates/worth-runtime-world/COORDINATED_PUBLICATION.md` for runtime
  integrators: phase progression, canonical owner order, no-effect versus
  product-unpublished versus performed outcomes, cancellation, and recovery.
- `crates/worth-runtime-world/RETENTION_AND_RECOVERY.md` for operators and
  runtime authors: unique pins, budgets, retained partials, reclamation,
  inspection, close, and owner-loss posture.
- `crates/worth-runtime-world/examples/runtime_world_publication.rs` as an
  executable facade contract covering bootstrap, ordinary, stale, and
  retained-partial handling.
- revise `crates/worth-runtime-bridge/API_OVERVIEW.md` and
  `crates/worth-runtime-bridge/REFERENCE_MAP.md` to identify the new higher
  composition owner and prevent readers from looking for product history in
  the base Bridge crate.

The example must compile and execute against the real public facade in an
ordinary package test command. Documentation must not claim durability,
restart recovery, multi-parent history, or Query public completion.

## Must Ship

- the acyclic `worth-runtime-world` composition package and enforced dependency
  direction;
- explicit one-root bootstrap from exact admitted component/correspondence
  inputs, with no ambient initial head;
- exact Bridge correspondence-basis admission;
- owner-issued, non-substitutable Runtime World, product branch, commit,
  attempt, and retained-partial identities;
- immutable root/single-parent commits and independently synchronized product
  reference cells;
- managed exact observations and unique component-basis pinning;
- explicit product branch reuse/fork creation and retirement lifecycle;
- bounded history, observation, attempt, recovery, and reclamation behavior;
- owner-specialized component plans and canonical Relational-then-Signal order;
- pre-effect attempt and recovery reservation;
- typed no-effect, product-unpublished, and performed outcomes;
- atomic product compare-and-publish and reconstructible live performed
  authority;
- honest cost scopes and structural counters; and
- one stable facade, executable docs, dependency/compile enforcement, and the
  adversarial court.

## Must Preserve

- every 9.17.1 owner basis, branch isolation, structural sharing, independent
  Relational progress, retention, and publication guarantee;
- every 9.17.1.1 Relational service, settlement recovery, exact Signal
  retention, terminal lease, facade, cost-scope, and evidence guarantee;
- every 9.17.1.2 concrete owner-service, Signal progress, lifecycle, and no-
  global-lock guarantee;
- distinct Relational, Signal, Bridge, Runtime World, Query, and Store
  authority;
- Foundational descriptive vocabulary without authority promotion;
- concrete owner-specialized Proof carriers;
- one canonical owner artifact for each performed component publication;
- Query's existing outbox payload inside the Relational component result;
- ordinary, history, recovery, maintenance, diagnostic, and certification lane
  separation; and
- certification-only replay.

## Explicit Non-Goals

- Query public branch workflow, public facade cutover, or complete Query
  carriage;
- persistence, PostgreSQL, Store adapters, checkpoints, restart recovery,
  physical residency, replication, or distributed publication;
- semantic undo/redo, compensation product semantics, merge, rebase,
  multi-parent history, tags, best-common-ancestor selection, or offline sync;
- automatic adoption of owner-local movement after a losing product race;
- cross-owner rollback; and
- total component-state byte accounting not exposed by component owners.

## Acceptance And Handoff

Milestone 9.17.2 closes only when:

- the new package and dependency checks prove one acyclic composition owner;
- exactly one explicit bootstrap establishes the initial root and product
  reference, while every duplicate, foreign, incompatible, cancelled, or
  capacity-denied bootstrap is exact no-effect;
- the real owner facade proves exact correspondence and rejects every hostile
  substitution before effects;
- product branch observations remain valid under concurrent publication and
  pin exact component bases;
- immutable root/single-parent commits and mutable product references remain
  distinct;
- branch creation supports exact reuse and owner-issued fork without ambient
  selection;
- independent branches progress without a global composition or Signal lock;
- same-head mixed-plan races overlap owner effects, produce one winner, and
  classify every loser from observed owner movement;
- partial owner effects survive caller-capability loss and converge through one
  bounded recovery record without duplicate owner work;
- repeated exact-basis reuse does not amplify owner leases by commit count;
- history and every managed registry enforce their installed bounds before new
  owner effects;
- compiler evidence denies minting, phase skipping, cross-head pairing,
  duplicate performed use, and retained-partial promotion;
- cost observations use honest scopes and exact structural counters;
- executable documentation, focused tests, the intentional integration target,
  scheduled scale evidence, formatting, lint, dirty line caps, boundary check,
  and generated context validation pass;
- the causally compiled composite court, independent oracle, outcome table,
  deterministic operation-control lane, seeded model family, sensitivity
  cases, and teardown assertions satisfy their contracts; and
- review finds no legacy or competing composition authority.

Milestone 9.17.3 receives only:

- `PerformedRuntimeWorldBootstrap` and its initial admitted branch observation
  at application construction;
- `RuntimeWorldObservationPort` and exact product branch observations;
- `AdmittedCompositeRuntimeWorldBasis`;
- owner-facing branch creation and publication intents;
- `PerformedCompositePublication` as the sole committed-terminal authority;
- `NoEffectCompositePublication` and
  `ProductUnpublishedOwnerEffects` with their typed next actions;
- immutable history and bounded inspection projections; and
- exact component/publication correlation needed to gate the existing outbox.

Query may carry, consume, and project these artifacts. It may not construct
them, rebuild them from component ids, store a competing operation-to-commit
authority map, move product heads, settle component owners, reinterpret a
retained partial as committed, or repair a missing 9.17.2 guarantee with facade
logic.
