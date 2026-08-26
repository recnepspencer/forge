# Milestone 9.17.3: Query Product-Branch Carriage, Facade, And Certification

## Goal

Make the composite product branch established by
[Milestone 9.17.2](./milestone-9.17.2.md) the sole ordinary Query branch-world
authority from declaration through terminal publication and every later
observation. Carry the exact product branch, composite head, Relational basis,
Signal basis, correspondence, freshness, and attempt binding through:

- planning and lowering;
- provider sessions and read sets;
- proposals and invariant execution;
- effects and owner preparation;
- commit, publication, and aftermath;
- receipts and causal inspection;
- history, live delivery, preview, recovery, and certification; and
- public `worth-query-decl` / `worth-query-host` branch workflows;
- execution-owned persistent-runtime startup/recovery/readiness, reexported by
  the provider-neutral host facade; and
- existing-outbox dispatch admission gated by the performed composite
  publication rather than an owner-local Relational commit.

Delete the Relational-only product-branch assumption, one-to-one component
branch derivation, ambient Signal selection, and any public or internal lane
that can reconstruct composite authority from weaker parts.

This milestone is the final implementation slice of the
[Milestone 9.17 umbrella](./milestone-9.17.md). Its closure unlocks
[Milestone 9.18](./milestone-9.18.md).

## Roadmap Placement

Milestone 9.16 proves the authenticated and authorized ordinary Query front
door. Milestone 9.16.1 makes branch affinity structurally mandatory through the
provider-session path. Milestone 9.16.2 makes packages freshly reconstructible
and establishes the PostgreSQL adapter and persistent Query-host facades.
Milestone 9.17.1 makes
component bases, Relational branch-local MVCC, and both component recoveries
durable. Milestone 9.17.2 makes composite history/product currentness durably
recoverable. This milestone completes the Query and dispatch cutover without
granting authority to records, SQL rows, snapshots, or leases.

9.17.3 performs the audience cutover:

```text
public branch workflow declaration
    -> Query product-branch intent
    -> Bridge-admitted exact composite basis
    -> Query plan and provider session
    -> read set / proposal / invariants
    -> component preparation intent
    -> Bridge coordinated publication
    -> performed composite commit
    -> Query terminal / receipt / aftermath / live delivery
    -> history, inspection, recovery, and future correction
```

No Query convenience path may stop after selecting a Relational branch and
silently fill the Signal component later.

## Current Boundary

- Query already has branch-affine planning, provider sessions, decision read
  sets, proposals, invariant execution, effects, terminals, receipts,
  publication, history/live/preview/recovery capabilities, and lower-runtime
  routing envelopes.
- Some Query branch affinity currently derives Bridge truth branch identity
  directly from Relational branch identity. That representation cannot express
  product branches sharing Signal bases or independently advancing components.
- Branch identity appears across many Query packages and surfaces. The migration
  is broad even though the new semantic destination is narrow.
- 9.17.2 supplies the sole Bridge-owned product branch, composite basis,
  immutable history, coordinated publication, retention, durable product-head
  store, and owner-first recovery facades.
- `worth-foundational` supplies portable canonical identity, locators,
  branch/commit descriptions, correspondence descriptions, provenance,
  diagnostics, support, and performance vocabulary for boundary artifacts.
- `worth-proof` supplies the phase, basis, freshness/readmission, binding,
  structural-fact, checked-outcome, and performed-effect progression beneath
  private Query carriers.

The missing end-to-end law is:

> Every Query operation that claims branch-affine product truth must consume
> one Bridge-admitted exact composite basis and retain it unchanged or advance
> it through the one Bridge publication progression until the terminal public
> artifact is observed.

## Adversarial Courtroom

Run a real installed application through the public Query composition root with
four product branches:

1. two branches share one immutable Signal basis and diverge in Relational;
2. one branch advances only Relational;
3. one branch advances only Signal; and
4. one branch advances both components.

Exercise ordinary reads, mutations, workflows, invariant evaluation, live
subscriptions, preview, history, causal inspection, response-loss recovery,
and aftermath on those branches. Concurrently:

- present equal-ordinal foreign component bases;
- swap a valid component or composite artifact after Query admission;
- advance the product head between plan, provider session, proposal,
  invariants, owner preparation, and publication;
- deserialize or restore a previously admitted Query artifact;
- cancel at every Query-to-Bridge and Query-to-owner transfer;
- remove or bypass one carriage field in each phase;
- force diagnostics tiers to vary while operational truth remains equal;
- kill the process after a Relational outbox commit but before composite CAS,
  after composite CAS but before Query response, and after external send but
  before acknowledgement;
- race independent product branches and one shared product head;
- request history and live observations while partial component preparation
  exists; and
- attempt the retired Relational-only or ambient-Signal path through internal,
  facade, host, test, and certification surfaces.

The independent product oracle must observe:

- the exact same product branch, composite head, component bases, and
  correspondence across every phase that does not perform a head advance;
- a newly performed composite basis only after the Bridge terminal publication;
- exact component retention for single-component operations;
- no Query observation of half-prepared component state;
- same-head stale/conflict and independent-branch progress identical to the
  lower-owner courts;
- one-shot, live, history, preview, inspection, recovery, and aftermath
  agreement on product-world identity;
- boundary-crossed artifacts requiring Query/Bridge readmission rather than
  retaining ambient trust;
- operational receipts equal across diagnostics tiers while only lawful
  sidecars differ;
- public callers unable to construct, pair, restamp, or promote component or
  composite authority; and
- exact-zero residue for Relational-only product identity and ambient Signal
  selection;
- an owner-local outbox is never dispatched unless its exact composite
  publication is performed; and
- a fresh process reconstructs owner state, Bridge product currentness, Query
  carriage, and pending dispatch before readiness; and
- semantic dispatch aftermath appears only through a subsequent performed
  composite commit, while operational attempt rows never move product state.

## Product Decision Lock

1. Query carries composition authority; it does not own or recreate it.
2. The public branch-selection concept is a product branch, not a Relational
   branch with implied Signal state.
3. Every admitted branch workflow resolves through the Bridge facade to one
   exact composite basis before lower-runtime work.
4. Product branch id, composite head generation/commit, Relational basis,
   Signal basis, correspondence, operation/attempt, provider session, and
   freshness/readmission posture are explicit binding axes.
5. Query artifacts may project human-readable component information, but only
   private Query carriers retaining the Bridge authority may progress.
6. Query cannot mint an exact component basis, admit component
   correspondence, append composite history, or move a product head.
7. Plans consume admitted current composite basis. Executors do not rediscover
   branch selection or fill missing components.
8. Provider sessions, read sets, proposals, and invariants remain bound to the
   exact composite basis used for their observations. A head change yields a
   typed stale/rebind outcome, not silent retargeting.
9. Effects and publication intents name the expected composite head and exact
   component plan. They cannot address raw lower-runtime branches outside the
   admitted Bridge progression.
10. A Query committed outcome exists only after consuming Bridge performed
    composite publication. An owner-local commit alone is not a product commit.
11. History, live, preview, inspection, recovery, and aftermath derive product-
    world identity from the same canonical terminal/composite artifacts.
12. One-shot and live observation cannot use different branch-selection
    semantics. Preview and recovery cannot retain the old one-to-one identity
    shortcut.
13. Public branch creation expresses component reuse/fork intent but never
    exposes raw owner ids as authority. Advanced intent remains typed and is
    lowered by Query into Bridge requests.
14. Query readmission after serialization, restore, replay, or trust-boundary
    carriage requires current Query and Bridge owner checks. A digest or prior
    receipt cannot restore authority.
15. Foundational canonical identity and boundary envelopes remain descriptive.
    They are used for stable portable artifact identity, not operational minting.
16. Proof progression remains below private Query artifacts. Public generic
    proof types and caller-selected authority markers cannot open the facade.
17. Every public successor transition is compiler-visible. Invalid phase order,
    raw component pairing, weaker-type substitution, reused performed evidence,
    and direct Bridge/internal entry are unrepresentable or fail to compile.
18. Diagnostics, support, and certification sidecars never affect branch
    selection, publication, or operational outcome.
19. The old branch lane is deleted, not retained as compatibility debt.
20. Tree correction begins only in 9.18 after this complete product-carriage
    boundary closes.
21. The persistent Query-host facade invokes recovery in package, component-
    owner, Runtime Bridge, Query-installation, and dispatch-reconciliation
    order. It cannot mint recovered authority; `worth-runtime-postgres` supplies
    only physical owner implementations.
22. Query's private `PerformedProductPublication` carrier now retains the exact
    Bridge performed composite publication. The 9.16.2 Relational-only source
    is deleted with the old product-world lane.
23. The existing Query outbox payload remains co-committed in the exact
    Relational component commit. Dispatch admission joins it to the performed
    composite commit that selected that Relational basis; no copied payload or
    database lease substitutes for either authority.
24. A failed Signal preparation, stale product head, cancellation, failed CAS,
    or orphaned owner candidate cannot become externally dispatchable.
25. Restart dispatch remains at-least-once under the stable idempotency key and
    fenced lease. Exactly-once external behavior still requires the external
    owner to honor idempotency.
26. PostgreSQL dispatch attempts and outcomes are operational delivery truth,
    not product-branch state. They authorize no workflow mutation.
27. If completed, acknowledged, unresolved, or recovery aftermath changes
    workflow state, Query submits a new aftermath operation through the same
    Bridge composite publication progression. No direct post-dispatch
    Relational mutation may bypass product history.
28. Lowering an operation with an external dispatch effect declares the
    Relational outbox write in its component program before preparation. A
    nominally Signal-only domain change with such an effect is therefore a
    combined component publication; no post-publication hidden outbox write is
    permitted.

## Compiler-Enforced Progression

The ordinary public path must lower into one private Query progression:

```text
product branch workflow declaration
    -> normalized product branch intent
    -> Bridge-resolved product branch basis
    -> Query-admitted composite basis
    -> planned branch-affine operation
    -> provider-session-bound operation
    -> proposal and invariant-ready operation
    -> lowered composite publication intent
    -> Bridge execution-ready publication
    -> Bridge performed composite transition
    -> Query committed terminal
    -> public receipt / aftermath / history / live consequence
```

Each phase consumes its predecessor. Later phases accept the strongest carried
artifact rather than repeating branch or component selection. Public compile-
fail evidence must prove callers cannot:

- construct an admitted composite Query basis from ids, descriptors, digests,
  or component bases;
- build a plan from an unresolved product branch;
- pair a plan, session, proposal, invariant result, or publication intent from
  different composite heads;
- publish from owner-local candidate or Bridge-prepared evidence;
- invoke internal Bridge/Relational/Signal paths through the ordinary facade;
- reuse an execution/performed witness; or
- advance history/live/aftermath from a non-terminal or stale artifact.

Use Proof binding and freshness law inside private Query types, not as the
public API. Use Foundational canonicalization to identify portable cases,
reports, receipts, and boundary envelopes, not to reconstruct operational
authority.

## Destination Topology

```text
worth-query-execution/
    basis/
        runtime_world.rs
        product_branch.rs
        readmission.rs
    domain_computation/
        planning/product_branch.rs
        provider_session/product_branch.rs
        proposal/product_branch.rs
        invariant/product_branch.rs
        publication/product_branch.rs
        application_aftermath/performed_product_publication.rs
        application_aftermath/composite_dispatch_admission.rs
        terminal/product_branch.rs

worth-query-decl/
    branch/
        intent.rs
        selection.rs
        creation.rs
        inspection.rs
        mutation.rs

worth-query-host/
    facade.rs                    # audience reexports only

worth-query-execution/
    persistent_runtime/recovery/
        owner_progression.rs
        composite_readmission.rs
        dispatch_reconciliation.rs
        readiness.rs

worth-query-publication/
    runtime_world/
        receipt.rs
        aftermath.rs
        live_delivery.rs

worth-query-certification/
    runtime_world_branching/
        world.rs
        oracle.rs
        shared_signal_basis.rs
        component_divergence.rs
        independent_progress.rs
        same_head_race.rs
        substitution.rs
        coordinated_publication.rs
        lifecycle.rs
        facade.rs
        residue.rs

worth-runtime-postgres/
    dispatch/
        publication_locator_index.rs
        reconciliation.rs
        claim.rs

worth-runtime-postgres-certification/
    composite_recovery/
        owner_first_restart.rs
        failed_product_publication.rs
        response_loss.rs
        dispatch_crash_matrix.rs
```

Actual package decomposition must follow the repository's legal authority graph
from Milestone 9.13.2. The stable requirement is that basis, execution
carriage, declaration facade, host lifecycle, publication projection, and
certification remain distinct owners. Forbidden placement includes a Query
branch registry, a `branch_helpers.rs`, direct component-store access in Query,
SQL inside Query, dispatch admission inside the PostgreSQL adapter, an adapter-
minted performed carrier, or certification-only production constructors.

## Phase Plan

### Phase 1: Query Composite Basis And Carriage Inventory

Inventory every branch-bearing Query type, constructor, transition, facade,
lower-runtime request, receipt, history/live/preview/recovery surface, and
test/world compiler. Classify the current authority axis and identify every
Relational-only, derived-identity, ambient-Signal, raw-id, or weaker-artifact
lane.

Install the private Query composite basis carrier and Bridge readmission entry.
Freeze complete binding axes and public compiler denials before broad migration.

### Phase 2: Planning Through Invariant Carriage

Carry the exact admitted composite basis through normalization, admission,
planning, access plans, provider sessions, observations, decision read sets,
proposals, and invariant execution. Remove every default-to-main, branch-id
derivation, current-head relookup, and raw lower-runtime pairing on these paths.

Each transition consumes the exact predecessor; no phase re-reads ambient
product currentness except through named freshness validation.

### Phase 3: Effect, Owner Preparation, Publication, And Terminal Carriage

Lower Query effects into Bridge component plans bound to the expected composite
head. Consume only 9.17.2 publication progression and terminal evidence.
Construct committed Query terminals and publications only from performed
composite transitions. Preserve typed stale, conflict, cancellation, denial,
partial-preparation, response-loss, and recovery outcomes.

### Phase 4: Composite Recovery, Aftermath, And Existing-Outbox Cutover

Cut every post-terminal and observational surface to canonical composite
identity. Complete the runtime-level owner-first recovery barrier. Replace the
9.16.2 Relational publication source inside Query's private performed-product-
publication carrier with Bridge performed composite publication. Join the
existing Relational outbox fact to that exact composite commit before claim
admission. Prove live/one-shot parity, history, preview, inspection, response-
loss recovery, aftermath identity, and the full dispatch crash matrix. Prevent
partial or orphan owner candidates from entering observation or dispatch.

### Phase 5: Public Facade And Host Cutover

Publish product branch selection, explicit component reuse/fork branch
creation, inspection, mutation, history, and recovery through
`worth-query-decl` and `worth-query-host`. Keep the common path semantic and the
advanced component plan explicit. Remove public raw lower-runtime ids and
direct Bridge/Relational/Signal entry from the ordinary Query journey.

Preserve the stable `WorthQueryHost::open_persistent` audience entry while
replacing its internal provider-bundle progression with the completed component
and Runtime Bridge owner set. The 9.16.2 provider bundle must already be a
compiler-visible construction progression: adding these required owner
providers intentionally breaks incomplete composition roots without moving or
duplicating the facade. The PostgreSQL adapter populates only their physical
implementations. Query execution owns opening and recovery; `worth-query-host`
only reexports that capability. Readiness remains unavailable until exact
package, owner, composite, Query, and dispatch reconciliation all close.

Delete the old Relational-only and ambient-Signal paths atomically with the
last covered consumer migration. No compatibility authority lane survives.

### Phase 6: Executable Documentation And Support Contract

Update public branch, history, live, recovery, aftermath, and application-world
guides. Teach product branch versus component branch meaning, exact basis,
shared immutable Signal basis, component-specific advancement, stale
currentness, cancellation, partial preparation, and recovery. Compile every
ordinary and advanced example against the real facade.

Support/profile surfaces describe the completed PostgreSQL-backed composite
durable lane and distinguish it from Store-native replication, merge/rebase,
multi-parent, offline synchronization, and distributed-recovery non-goals.

### Phase 7: End-To-End Hostile Certification And Umbrella Closure

Run the cumulative 9.17 courtroom through a causally complete installed
application world and real Query composition root. Use independent component
and composite-history oracles, real Bridge and owner facades, default and
admitted parallel lanes, diagnostics-tier twins, cancellation at every
transfer, exact counter/slopes, compiler misuse probes, facade inventory,
dependency enforcement, and exact-zero residue.

9.17.3 cannot close from local Query unit fixtures alone. Its closeout must
also cite the frozen 9.17.1 and 9.17.2 authority evidence and prove no later
Query phase reopened those guarantees.

## DX Target

```rust
let persistence = WorthRuntimePostgres::connect(postgres_configuration)?;
let providers = PersistentQueryRuntimeProviders::builder()
    .relational_durability(persistence.relational_durability())
    .query_package_archives(persistence.query_package_archives())
    .runtime_stream_catalog(persistence.runtime_stream_catalog())
    .dispatch_coordination(persistence.dispatch_coordination())
    .signal_durability(persistence.signal_durability())
    .runtime_world_durability(persistence.runtime_world_durability())
    .build()?;
let app = WorthQueryHost::open_persistent(
    providers,
    signed_release,
    host_runtime_bindings,
)?;
app.wait_until_ready()?;

let branch = app
    .branches()
    .fork(world.head())
    .components(|components| {
        components
            .fork_relational()
            .reuse_exact_signal_basis()
    })
    .create()
    .await?;

let outcome = app
    .on_branch(branch)
    .transaction()
    .apply(admitted_change)
    .commit()
    .await?;

let committed = outcome.require_committed()?;
assert_eq!(committed.product_branch(), branch.id());
```

The ordinary caller chooses product intent and one admitted product branch.
Advanced branch creation exposes component retain/fork intent because only the
caller can choose that product meaning. Neither path exposes component ids as
authority or asks callers to orchestrate runtimes.

## Performance Contract

- Query basis admission and carriage are O(1) in the fixed component count and
  declared binding axes.
- Later phases do not rediscover, reconstruct, hash, or re-resolve a basis
  already carried within the same trust boundary.
- A single-component operation contacts only the changing owner plus fixed
  validation/publication boundaries; unchanged owner execution count is zero.
- One-shot, history, live, preview, inspection, and aftermath projections do
  not scan product branch or component history to infer currentness.
- Public branch selection is one indexed product-reference lookup plus owner/
  Bridge freshness validation; it is not a cross-runtime search.
- Pending dispatch selection is one indexed lookup plus exact outbox/composite-
  publication readmission; it never scans component or composite history.
- Diagnostic richness and certification identity remain sidecar/cold work and
  contribute exact zero operational branch-selection or publication work.
- Counters distinguish basis resolution, readmission, phase carriage,
  freshness checks, planner/session/proposal/invariant transitions, owner
  contacts, composite publication, history/live projection, recovery,
  outbox/publication joins, lease attempts, diagnostics, and residue/fallback
  use.

## Proof Portfolio

The proof portfolio must include:

- a causally complete installed application world with public product branch
  creation, selection, mutation, read, history, live, inspection, recovery, and
  aftermath journeys;
- shared Signal basis plus divergent Relational branch scenarios;
- Relational-only, Signal-only, and combined-component changes;
- independent progress and same-product-head race parity with lower courts;
- one-axis drift across every Query/Bridge/component/session/attempt binding;
- stale-between-every-phase scenarios;
- cancellation and response loss at every Query-to-owner/Bridge transfer;
- process kills after owner-local outbox commit, after performed composite CAS,
  after send, and before dispatch acknowledgement, all against real PostgreSQL;
- a mutation probe that writes aftermath directly to Relational after dispatch
  and must be rejected or exposed as non-current by the independent product
  oracle;
- a mutation probe that drops the Relational outbox footprint from an external-
  effect component program and must fail before owner preparation;
- exact no-half-publication observation across one-shot/live/history paths;
- checkpoint/serialization trust-boundary downgrade and readmission;
- diagnostics-tier twins with identical operational truth/receipts and lawful
  sidecar variation;
- public compile-pass journeys and consolidated compile-fail authority/phase
  cases;
- facade/dependency scans proving ordinary consumers cannot import internal
  lower-runtime construction paths;
- exact-zero Relational-only, branch-id-derived, ambient-Signal, raw pairing,
  compatibility-authority, and test-only bypass residue;
- scale-sensitive exact counters over unrelated branches, history, consumers,
  live observers, and diagnostics; and
- mutations that drop one carriage field, re-resolve latest, restamp a basis,
  publish from owner evidence, bypass Bridge CAS, or retain the old facade.

World fixtures must derive expected product/component history independently
from declared scenario actions. They may not ask Query or Bridge which work was
expected and then use that answer to select the nodes or phases they observe.

## Documentation Deliverables

- product branch mental model and component authority guide;
- ordinary branch creation, selection, mutation, history, and recovery guide;
- advanced exact component retain/fork guide;
- shared Signal basis and independent component advancement guide;
- stale/conflict/cancellation/partial-preparation/recovery outcome reference;
- live, preview, inspection, and aftermath product-world identity guide;
- migration guide deleting Relational-only and ambient-Signal assumptions;
- operator guide for owner-first PostgreSQL recovery, readiness, dispatch
  fencing, and unresolved external outcomes;
- support/profile entries for admitted and explicitly deferred neighbors;
- executable public examples and API reference; and
- Milestone 9.17 umbrella closeout plus the exact 9.18 handoff.

## Must Preserve

- every 9.16 and 9.16.1 public, authorization, provider-session, invariant,
  recovery, aftermath, and publication guarantee;
- every 9.16.2 package reconstruction, PostgreSQL durability foundation,
  runtime-level facade, existing-outbox, and adapter-ownership guarantee;
- every 9.17.1 component authority and independent-progress guarantee;
- every 9.17.2 composition, history, retention, and no-half-publication
  guarantee, including their PostgreSQL recovery contracts;
- Query as audience facade rather than history/currentness owner;
- Foundational portable vocabulary without authority promotion;
- Proof progression beneath private owner-specific types;
- diagnostic-tier noninterference; and
- cert-only replay.

## Explicit Non-Goals

- semantic undo/redo, inverse, or compensation acceptance;
- merge, rebase, multi-parent history, tags, or best-common-ancestor logic;
- offline synchronization;
- distributed cross-region atomic recovery;
- a public component-runtime orchestration API; and
- preserving the old branch facade as compatibility debt.

## Allowed Debt

- Store-native graph persistence, replication, and distributed recovery remain
  later owners; PostgreSQL composite restart is required here. Merge, rebase,
  multi-parent history, and offline synchronization remain cross-runtime work.
- No Relational-only product identity, derived component identity, ambient
  Signal selection, raw lower-runtime pairing, internal ordinary-facade bypass,
  or callable compatibility authority may remain debt.

## Parallelization And Store Dependency

Carriage inventories and independent world/oracle construction may proceed
alongside early private basis integration. Planning/session migration and
post-terminal projection migration may proceed in parallel only after the
private Query composite-basis contract freezes. Publication cutover precedes
facade deletion; executable docs and cumulative certification close last. This
milestone is not blocked on `worth-store`.

## Acceptance Evidence

Milestone 9.17.3 and therefore the Milestone 9.17 umbrella close only when
`Query Composite Product-Branch End-To-End Certification` in
[test-requirements.md](./test-requirements.md) passes and:

- every Query phase carries one exact admitted composite basis or its exact
  performed successor;
- every public branch workflow uses the Bridge product branch and no raw or
  Relational-only authority lane survives;
- the public facade is compiler-ordered and rejects raw minting, phase skipping,
  cross-basis pairing, stale proof reuse, and lower-runtime bypass;
- one-shot, live, history, preview, inspection, recovery, publication, and
  aftermath agree on exact product/component identity;
- owner-first fresh-process recovery reconstructs the exact package, component
  bases, composite heads, Query carriage, and pending dispatch before readiness;
- no existing outbox fact is dispatchable without its exact performed composite
  publication, and crash retries preserve one idempotency identity and current
  fence;
- workflow-visible dispatch aftermath enters through a new performed composite
  publication, never a sideband Relational mutation;
- every external-effect plan includes the Relational outbox footprint before
  owner preparation;
- no partial preparation is publicly observable;
- shared immutable Signal basis reuse and independent component advancement are
  visible and correct through public DX;
- default and admitted parallel lanes converge on identical truth, history,
  receipts, and global publication order;
- exact counters, scale slopes, diagnostics tiers, cancellation, response loss,
  retention, and cleanup match the cumulative contract;
- executable docs, support posture, dependency checks, facade inventory,
  residue scans, line-cap/formatting gates, boundary-check, agent-context, and
  hostile certification agree; and
- the closeout cites frozen owner/Bridge evidence rather than substituting
  Query-only tests for lower-authority proof.

## Handoff

[Milestone 9.18](./milestone-9.18.md) consumes the completed 9.17 umbrella:

- exact Query-selected product branches and composite heads;
- immutable single-parent composite commits and ancestry;
- exact owner-observed component bases and per-component change posture;
- Bridge coordinated compare-and-publish and terminal recovery;
- owner-first PostgreSQL recovery of exact component and composite authority;
- exact retention and branch creation from retained commits; and
- public history, inspection, aftermath, typed outcome carriage, and existing-
  outbox dispatch gated by performed composite publication.

9.18 may add freshly admitted correction semantics over this history. It may
not add a second branch registry, infer Signal state from Relational history,
weaken compiler-enforced carriage, or repair incomplete 9.17 authority inside
an undo/redo facade.
