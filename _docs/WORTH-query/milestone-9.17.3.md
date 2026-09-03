# Milestone 9.17.3: Query Product-Branch Carriage, Facade, And Certification

> **Product posture:** This milestone completes the public in-memory Query
> product-branch workflow. Persistence and restart recovery remain Worth Store
> integration work.

## Goal And Roadmap Placement

Carry the exact Runtime World product-branch basis and performed composite
publication from Milestone 9.17.2 through every Query phase that consumes or
projects product currentness. Publish one coherent Query host facade for branch
creation, selection, mutation, reads, history, live observation, inspection,
aftermath, and existing-outbox eligibility. Delete Relational-only product
identity and ambient Signal selection.

Milestones 9.17.1 through 9.17.1.2 own the exact component bases, owner
services, and branch-local progress contracts. Milestone 9.17.2 owns composite
history and product-head movement in `worth-runtime-world`. Query owns admission,
phase carriage, public DX, and projection; it cannot restamp lower authority.
Closing this milestone closes the 9.17 umbrella and unlocks tree-based semantic
undo/redo in Milestone 9.18.

## Central Claim

Every public Query operation on a product branch resolves one exact
Runtime World-admitted composite basis before planning, carries it without
rediscovery through execution, and constructs committed Query outcomes only
from performed Runtime World composite publication. One-shot, history, live,
inspection, aftermath, and
outbox eligibility all report the same product branch, composite commit, and
owner component bases.

The claim is false if:

- Query derives product identity from a Relational branch or commit;
- Signal basis is selected from ambient current state;
- a later phase re-resolves latest instead of consuming carried proof;
- a plan, session, proposal, invariant result, effect, or terminal from another
  composite basis can be paired successfully;
- owner-local performed work becomes a committed Query outcome before Runtime
  World
  product publication;
- history or live views infer product currentness from component history;
- an outbox fact becomes dispatch-eligible when its exact composite publication
  failed or lost the product-head race;
- Query owns another branch registry or calls private owner mechanics; or
- a physical runtime, recovery barrier, or persistent dispatch coordinator is
  introduced in this milestone.

## Ownership Lock

| Responsibility | Owner |
| --- | --- |
| Component truth, bases, branches, and owner-local publication | Relational and Signal |
| Installed Relational-to-Signal semantic correspondence | Base Runtime Bridge |
| Composite commits, product branches, currentness, and coordinated publication | `worth-runtime-world` composition owner |
| Product-branch intent, admission, carried execution affinity, public outcomes, and Query projections | Query |
| Existing outbox payload, correlation, and idempotency identity | Query aftermath meaning co-committed in the Relational component |
| Eligibility to attempt an external effect in the live runtime | Query after consuming exact performed composite publication |
| External exactly-once consequence | External owner through stable idempotency behavior |
| Persistence, restart-safe discovery, durable claims, recovery, and physical reads | Worth Store integration |

`worth-query-host` remains an audience facade. It reexports public Query
capabilities but implements no product history, branch selection, publication,
or dispatch policy.

## Current Boundary

After 9.17.2, the Runtime World facade can resolve a product branch to an exact composite basis,
coordinate owner work, publish a new composite commit, and issue performed
authority. Query still has branch-bearing surfaces that may use Relational
identity, ambient Signal context, or weaker descriptors. The migration must
cover the complete causal chain:

Query also still owns its unit-typed Signal runtime inside
`BridgeOwnedSignalRuntime`, whose ordinary paths use construction-only Signal
graph access. That root cannot simply issue 9.17.1.2 services and continue on
the old lane. This milestone must refactor or replace that composition root
once so Runtime Bridge and Runtime World operate against one owner-compatible
sealed Signal state. The three frozen branch ports do not expose Bridge's
conditional graph execution; Phase 1 must either prove that lane is no longer
needed or freeze one concrete Signal-owned Bridge execution service before
carriage parallelizes. A consumer adapter, second Signal graph, or simultaneous
legacy graph access after sealing is forbidden.

```text
declaration -> normalization -> admission -> planning -> access plan
    -> provider session -> observations/read set -> proposal -> invariants
    -> effect lowering -> Runtime World publication -> terminal -> public projections
```

No post-terminal surface may retain the old identity lane after cutover.

## Adversarial Courtroom

The cumulative Supply Chain world creates two product branches sharing one
Signal basis and diverging in Relational history. It executes Relational-only,
Signal-only, and combined operations while:

- racing the same product head;
- blocking an unrelated branch writer;
- making each carried binding axis stale between adjacent Query phases;
- substituting equal-version component and composite artifacts;
- cancelling before owner preparation, between owner calls, before Runtime World
  publication, and after performed publication;
- forcing partial owner work and product-unpublished-owner-effects posture;
- opening one-shot, live, history, inspection, and aftermath observations at
  adversarial times; and
- creating an outbox occurrence in an owner-local Relational result whose
  composite publication subsequently fails.

An independent product-world oracle must observe exact phase affinity, one
winner, no half-current projection, correct component reuse, independent branch
progress, and no dispatch eligibility for the failed composite publication.
Dropping a carriage field, resolving latest, promoting owner evidence, bypassing
Runtime World, deriving identity from Relational, or enabling outbox work from row
existence must turn the court red.

## Product Decision Lock

1. Query product-branch identity is the exact Runtime World product branch, not a
   Relational alias.
2. Query resolves one exact composite basis before planning and carries it
   through the entire operation.
3. Every phase consumes the exact private-minted predecessor and preserves all
   binding axes needed by later phases.
4. Same-looking plans, sessions, attempts, proposals, or receipts from different
   composite heads are non-substitutable.
5. Lower runtimes are reached only through their audience facades and exact
   Runtime World-owned progression.
6. A Query committed terminal consumes performed composite publication. Owner-
   local candidate or performed evidence is insufficient.
7. Product-unpublished permits settlement, cleanup, inspection, or a fresh
   Query-admitted operation referencing the record without deriving authority;
   every outcome keeps typed next actions.
8. One-shot, history, live, inspection, preview where supported, and aftermath
   project the same canonical product-world identity.
9. Existing Query live views retain their exact patch granularity. Product-
   branch carriage changes the basis attached to patches; it does not create a
   second streaming abstraction.
10. An outbox occurrence is eligible only when its exact containing Relational
    basis participates in performed composite publication.
11. External-effect plans include the Relational outbox footprint before owner
    preparation; a Signal-only domain change with an external effect becomes a
    combined component operation.
12. Workflow-visible dispatch aftermath re-enters as a new ordinary composite
    operation; no sideband Relational mutation creates hidden product history.
13. Query does not persist dispatch attempts, leases, outcomes, or retry state
    here. Live-process dispatch retains existing bounded lifecycle and stable
    idempotency semantics only.
14. Public branch creation exposes explicit exact component retain/fork intent
    because only the caller can choose that product meaning.
15. Raw component ids, generic authority markers, internal Bridge or Runtime
    World handles, and compatibility aliases are absent from the ordinary
    public facade.
16. All application state remains memory-resident. A process loss loses the
    current world; no restart guarantee is implied.

## Compiler-Enforced Progression

```text
ProductBranchWorkflowIntent
    -> NormalizedProductBranchIntent
    -> RuntimeWorldResolvedProductBranchBasis
    -> QueryAdmittedCompositeBasis
    -> ProductBranchExecutionPlan
    -> ProductBranchProviderSession
    -> ProductBranchProposal
    -> ProductBranchInvariantResult
    -> RuntimeWorldPublicationOutcome
         PerformedCompositePublication -> QueryCommittedProductBranchTerminal
             -> receipt / history / live / aftermath / dispatch eligibility
         NoEffect / ProductUnpublished -> typed Query terminal
```

Compiler evidence denies raw composite-basis construction, unresolved planning,
cross-basis pairing, phase skipping, duplicate performed-witness use, direct
owner publication, and post-terminal projection from any weaker artifact.

## Destination Topology

```text
workspaces/worth-query/crates/worth-query-execution/src/
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
        terminal/product_branch.rs
        application_aftermath/
            performed_product_publication.rs
            composite_dispatch_admission.rs

workspaces/worth-query/crates/worth-query-declaration/src/branch/
    intent.rs
    selection.rs
    creation.rs
    inspection.rs
    mutation.rs

workspaces/worth-query/crates/worth-query-publication/src/runtime_world/
    receipt.rs
    aftermath.rs
    live_delivery.rs

workspaces/worth-query/crates/worth-query-host/src/
    facade.rs

workspaces/worth-query/crates/worth-query-certification/src/
    runtime_world_branching/
        world.rs
        oracle.rs
        shared_signal_basis.rs
        component_divergence.rs
        same_head_race.rs
        coordinated_publication.rs
        outbox_eligibility.rs
        facade.rs
        residue.rs
```

Existing package decomposition follows the Milestone 9.13.2 authority graph.
Forbidden placement includes a Query branch registry, generic helper/bucket
modules, direct component storage access, physical-runtime composition,
persistent recovery modules, adapter-owned dispatch admission, or facade files
implementing behavior.

## Ordered Phase Plan

### Phase 1: Composite Basis And Carriage Inventory

Inventory every branch-bearing Query type, constructor, transition, facade,
lower-runtime request, receipt, history/live/preview/inspection surface, and
fixture. Install the private admitted composite-basis carrier and compiler
denials before broad migration. Freeze the one-graph
`BridgeOwnedSignalRuntime` cutover seam and its exact service ownership before
parallel carriage work begins.

### Phase 2: Planning Through Invariant Carriage

Carry the exact basis through normalization, admission, planning, access plans,
provider sessions, observations, read sets, proposals, and invariant execution.
Remove default-main, derived identity, ambient Signal, and current-head relookup.

### Phase 3: Effect, Publication, And Terminal Carriage

Lower Query effects into exact Runtime World component plans, consume only 9.17.2
publication progression, and construct committed terminals only from performed
composite transitions. Preserve every losing, no-effect,
product-unpublished-owner-effects, and cancellation outcome.

### Phase 4: Projection And Existing-Outbox Cutover

Cut receipts, history, live views, inspection, preview where supported,
aftermath, and live-process outbox eligibility to canonical composite identity.
Prove exact patch granularity remains unchanged and failed composite publication
cannot dispatch its owner-local outbox.

### Phase 5: Public Facade And Legacy Deletion

Publish product branch selection, explicit component reuse/fork creation,
mutation, reads, history, live, inspection, and aftermath through declaration
and host facades. Delete the Relational-only/ambient-Signal lane with its last
consumer; no compatibility authority remains.

### Phase 6: Documentation And Cumulative Certification

Compile the ordinary and advanced public examples, run the cumulative 9.17
court through the real Query composition root, and close facade, dependency,
line-cap, formatting, boundary, generated-context, counter, and residue proof.

## Caller DX Target

```rust
let branch = app
    .branches()
    .fork(app.current_world())
    .components(|components| {
        components
            .fork_relational()
            .reuse_exact_signal_basis()
    })
    .create()?;

let committed = app
    .on_branch(branch)
    .transaction()
    .apply(admitted_change)
    .commit()?
    .require_committed()?;

assert_eq!(committed.product_branch(), branch.id());
```

The caller chooses product intent and component retain/fork posture. Query
carries exact owner identities and the Runtime World owner orchestrates
publication; callers never wire component runtimes or manufacture bases.

## Performance And Resource Contract

- Basis admission and carriage are O(1) in fixed component and binding axes.
- Later phases do not rediscover or rehash a basis already carried within the
  same trust boundary.
- A single-component operation executes only the changing owner plus fixed
  correspondence validation and Runtime World publication work.
- One-shot, history, live, inspection, and aftermath never scan histories to
  infer currentness.
- Existing live patch computation remains bounded by semantic delta and the
  query's declared delivery granule.
- Diagnostic richness and certification identity remain sidecar/cold work.
- Counters distinguish basis resolution/readmission, phase carriage, freshness
  checks, owner contacts, composite publication, projections, outbox admission,
  fallback use, and retained resources.

## Documentation Deliverables

- Product branch versus component branch mental model.
- Ordinary branch creation, selection, mutation, read, history, and live guide.
- Advanced exact component retain/fork guide.
- Stale/conflict/cancellation/partial-preparation outcome reference.
- Live patch and aftermath product-world identity guide.
- Migration guide deleting Relational-only and ambient-Signal assumptions.
- Explicit current-limits section stating that state is memory-resident and
  restart durability begins with Worth Store integration.
- Executable public examples and API reference.

## Must Preserve

- every 9.16 authorization, provider-session, invariant, aftermath, and
  publication guarantee;
- every 9.16.2 portable package and fresh-readmission guarantee;
- every 9.17.1 component-authority and independent-progress guarantee;
- every 9.17.1.1 and 9.17.1.2 owner-service, retention, lifecycle, and
  independent-branch progress guarantee;
- every 9.17.2 explicit-bootstrap, composite-history, retention, and
  no-half-publication guarantee;
- Query as audience/admission facade rather than history/currentness owner;
- exact existing outbox payload and idempotency identity;
- existing Query live-view patch granularity and backpressure policy;
- diagnostic noninterference and certification-only replay.

## Explicit Non-Goals

- persistence, PostgreSQL, physical runtime composition, restart recovery,
  durable dispatch claims/outcomes, paging, or database fetches;
- semantic undo/redo, merge, rebase, multi-parent history, or tags;
- offline synchronization or distributed publication; and
- a public lower-runtime orchestration API.

## Acceptance And Handoff

Milestone 9.17.3 closes when the real public Query composition root proves exact
composite carriage through every phase and projection; shared Signal basis with
divergent Relational histories; Relational-only, Signal-only, and combined
operations; independent progress; one-winner same-head races; typed partial and
cancellation outcomes; no half-current observation; existing-outbox eligibility
only after performed composite publication; unchanged live patch precision;
compiler-denied minting/phase skipping/cross-basis pairing; executable docs;
exact counters; dependency/facade enforcement; and zero legacy identity lanes.

Milestone 9.18 receives exact Query-selected product branches and composite
heads, immutable single-parent composite commits, owner-observed component
bases, Runtime World coordinated publication, retention, public history/inspection/
aftermath, typed outcome carriage, and performed-publication-gated outbox
eligibility. It may add freshly admitted correction semantics but may not create
a second history owner or introduce persistence as part of undo/redo.
