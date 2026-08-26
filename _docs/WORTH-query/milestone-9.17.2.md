# Milestone 9.17.2: Composite Runtime-World History And Coordinated Publication

> **Product posture:** This milestone establishes memory-resident Runtime
> Bridge composition authority. It makes no persistence, restart, or physical
> database claim.

## Goal And Roadmap Placement

Consume the exact owner-basis and branch-local MVCC contracts from Milestone
9.17.1 and establish one Runtime Bridge-owned product branch as a mutable
reference to an immutable single-parent composite runtime-world commit. Each
commit names the exact Relational and Signal bases that constitute one product
world. Bridge alone coordinates owner results and moves product currentness.

Milestone 9.17.1 owns component truth and produces exact owner bases. This
milestone owns composition truth but does not publish the public Query branch
workflow. Milestone 9.17.3 carries performed composite authority through Query.

## Central Claim

No product branch moves unless every operation-named component result is valid
for one exact expected composite head and Runtime Bridge wins one atomic
compare-and-publish transition. Unchanged components remain at their exact
carried bases. Partial preparation, stale heads, cancellation, foreign bases,
and losing races never become product-current or publicly observable as a
performed composite world.

The claim is false if:

- Relational or Signal currentness alone defines the product world;
- Bridge selects an ambient or latest component head;
- equal ids, ordinals, versions, digests, or descriptors substitute for
  owner-admitted bases;
- one component result becomes visible before composite publication;
- a losing product-head race silently rebases or retries under new authority;
- unchanged components are opportunistically refreshed;
- cancellation after an owner-local performed result is called rollback;
- Query or a physical adapter can mint a composite commit or move a product
  head; or
- composite state is serialized or persisted as part of this milestone.

## Ownership Lock

| Responsibility | Owner |
| --- | --- |
| Relational branches, commits, candidates, performed publications, conflicts, and retention | Relational |
| Signal branches, bases, owner-local advancement, definition compatibility, and retention | Signal |
| Exact component correspondence and compatibility admission | Runtime Bridge |
| Immutable composite commits, product branch references, head generations, parentage, retention, and coordinated publication | Runtime Bridge |
| Product workflow admission and public projection | Query in Milestone 9.17.3 |
| Portable branch/reference vocabulary | Foundational; descriptive only |
| Phase progression and concrete owner-specialized witnesses | Proof beneath private owner types |
| Persistence, restart recovery, physical residency, replication, and distributed publication | Worth Store integration |

Component authority is neither absorbed nor restamped. Bridge may consume
owner-issued artifacts through owner facades, but it cannot inspect candidate
internals, reconstruct a basis from representation, or mutate owner state.

## Current Boundary

After 9.17.1, Relational and Signal can each issue exact admitted bases, retain
them for named obligations, and perform owner-local advancement. Runtime Bridge
already owns cross-runtime correspondence and lowering, but it lacks:

- an immutable composite commit identity and parentage contract;
- a product branch reference distinct from component branches;
- exact component-change posture;
- a compare-and-publish progression over one expected product head;
- retention closure from product commits into exact component bases; and
- typed partial-preparation and cancellation settlement.

## Adversarial Courtroom

Two product branches begin at one composite commit and share the same immutable
Signal basis. Their Relational branches diverge. The court then runs:

1. one Relational-only publication;
2. one Signal-only publication;
3. one combined publication;
4. two writers racing the same product head;
5. one writer blocked on an unrelated product branch while the other commits;
6. successful Relational preparation followed by Signal denial;
7. successful owner-local work followed by a stale product head;
8. cancellation before preparation, between owner calls, before product-head
   movement, and after the movement linearization point;
9. substitution of an equal-ordinal basis from another branch, runtime,
   definition, or owner; and
10. retention pressure while live product branches and prepared publications
    still reference shared component bases.

Independent component and composite-history oracles must observe exact
single-parent history, one winner per expected head, no half-current product
world, exact unchanged-component reuse, independent branch progress, and
complete obligation transfer or cleanup. Deleting the product-head comparison,
publishing after only one owner succeeds, resolving latest, accepting a foreign
basis, or releasing a component pin early must turn the court red.

## Product Decision Lock

1. Product, Relational, and Signal branch identities are distinct types and
   meanings.
2. A composite basis contains exact owner-admitted Relational and Signal bases
   plus admitted correspondence; it selects nothing ambiently.
3. A composite commit is immutable and has exactly one ordinary parent.
4. A product branch is a mutable reference to one composite commit and carries
   an independently changing reference generation.
5. Component change posture is explicit: retain exact basis, fork/advance, or
   reject. Omitted does not mean refresh.
6. Bridge prepares compatibility and retention before the first owner effect.
7. Owner-local performed results remain non-current candidates until Bridge
   compare-and-publish succeeds.
8. The product-head comparison uses the exact expected branch observation, not
   commit id or version alone.
9. Losing same-head races return typed stale/conflict outcomes and do not
   rebase automatically.
10. Cancellation is no-effect only before the first owner effect. After an
    owner performs, settlement reports retained candidate, compensatable,
    reconcilable, irreversible, or performed posture honestly.
11. Product-head movement is the sole composition-currentness linearization
    point. Readers see the complete old or complete new composite basis.
12. Retaining a product commit retains every exact component basis needed to
    interpret it. Reclamation is maintenance work.
13. Bridge owns bounded orphan/retained-candidate lifecycle and exposes typed
    inspection; no invisible queue or unbounded history is permitted.
14. Performed Bridge publication is private authority consumed by Query 9.17.3.
15. All state in this milestone is memory-resident. No codec, backend port,
    SQL row, checkpoint, recovery cursor, or application-composition crate is
    introduced.

## Compiler-Enforced Progression

```text
ProductBranchIntent
    -> ResolvedExpectedProductHead
    -> AdmittedCompositeBasis
    -> LoweredComponentPublicationPlan
    -> BridgePreparedCompositePublication
    -> ComponentPublicationSettlement
    -> BridgeExecutionReadyPublication
    -> BridgeCompositePublicationOutcome
         Performed(PerformedCompositePublication)
         Stale(ProductHeadConflict)
         Rejected(CompositePublicationDenial)
         Cancelled(CompositeCancellation)
         Retained(OwnerPerformedCompositeNotCurrent)
```

Every transition consumes its exact predecessor. Public compiler evidence must
deny raw basis construction, cross-head pairing, component-result promotion,
phase skipping, duplicate performed-witness use, and a generic authority marker
at any governed owner facade.

## Destination Topology

```text
crates/worth-runtime-bridge/src/runtime_world/
    basis/
        descriptor.rs
        admission.rs
        correspondence.rs
    history/
        commit.rs
        identity.rs
        parentage.rs
        catalog.rs
        retention.rs
    branch/
        identity.rs
        reference.rs
        observation.rs
        lifecycle.rs
    publication/
        intent.rs
        preparation.rs
        component_plan.rs
        settlement.rs
        comparison.rs
        outcome.rs
        retained_candidate.rs
    inspection/
        history.rs
        retention.rs
        cost.rs
    facade.rs

crates/worth-runtime-bridge/tests/runtime_world_certification/
    world.rs
    oracle.rs
    component_reuse.rs
    partial_preparation.rs
    same_head_race.rs
    independent_progress.rs
    substitution.rs
    cancellation.rs
    retention.rs
    cost.rs
```

Existing repository naming may refine literal leaves, but basis, immutable
history, mutable product reference, publication progression, and inspection
must remain distinct responsibility axes. Forbidden placement includes Query-
owned product heads, Relational-owned Signal correspondence, a generic
`branch_manager`, Bridge access to private owner storage, persistence adapters,
or facade files implementing publication behavior.

## Ordered Phase Plan

### Phase 1: Freeze Composite Basis And Correspondence

Define the exact composite-basis descriptor, owner-admission progression, and
component correspondence without product-head mutation. Prove foreign, stale,
equal-ordinal, and mixed-owner substitution fails.

### Phase 2: Immutable Composite History And Product References

Install single-parent composite commits, distinct mutable product references,
exact observations, lifecycle, and retention closure into component bases.
Prove branch creation and exact component reuse without copying owner truth.

### Phase 3: Prepare Component Publication

Lower explicit retain/fork/advance posture into an owner-call plan, acquire
obligations, validate compatibility, and freeze cancellation points before
effects. Prove single-component work contacts no unchanged owner beyond the
fixed validation required for publication.

### Phase 4: Coordinated Compare-And-Publish

Consume owner settlements, compare the exact expected product head, move the
product reference once, and issue performed composite authority. Prove same-
head race, partial preparation, post-effect failure, and cancellation topology
without half-publication or automatic rebase.

### Phase 5: Retention, Inspection, Documentation, And Certification

Close retained-candidate cleanup, history/component obligations, bounded
memory, public owner facade, executable docs, adversarial certification,
dependency enforcement, line caps, and residue. Freeze the exact performed
artifact and readmission surface Milestone 9.17.3 may consume.

## Performance And Resource Contract

- Composite basis admission and product-head comparison are O(1) in the fixed
  component count.
- Composite publication contacts only operation-named changing owners plus
  fixed Bridge validation; unchanged owner execution is exactly zero.
- Independent product branches contribute zero synchronous waits or owner
  contacts to one another.
- Branch creation retains component bases without graph or truth copying.
- History traversal, retention scans, and orphan cleanup are explicit
  maintenance lanes.
- Counters expose owner contacts, basis validations, obligation acquire/release,
  candidate creation/retention/cleanup, comparison attempts, stale outcomes,
  performed movements, and unique retained bytes.
- Product branches, retained candidates, history, and obligations have explicit
  count/byte/age budgets; exhaustion rejects before new owner effects.

## Documentation Deliverables

- Runtime Bridge product-world mental model: component truth versus composition
  currentness.
- Composite commit and product branch reference guide.
- Exact component retain/fork/advance guide.
- Partial-preparation, stale-head, cancellation, and retained-candidate outcome
  reference.
- Retention and inspection guide with memory-resident limits.
- Executable owner-facade examples consumed by 9.17.3.

Every document must say plainly that state is memory-resident and restart
durability begins with Worth Store integration.

## Must Preserve

- every 9.17.1 owner basis, branch isolation, independent progress, retention,
  and performed-publication guarantee;
- distinct Relational, Signal, Bridge, and Query authority;
- Foundational descriptive vocabulary without authority promotion;
- concrete owner-specialized Proof carriers;
- one canonical owner artifact for each performed component publication;
- Query's existing outbox payload inside the Relational component result;
- ordinary versus history/maintenance cost separation; and
- certification-only replay.

## Explicit Non-Goals

- Query public branch workflow or complete Query carriage;
- persistence, PostgreSQL, Worth Store adapters, checkpoints, restart recovery,
  physical residency, or application composition;
- semantic merge, rebase, multi-parent history, tags, or offline sync;
- undo/redo or compensation product semantics; and
- distributed publication or replication.

## Acceptance And Handoff

Milestone 9.17.2 closes when the real Runtime Bridge owner facade and independent
component/composite-history oracles prove exact correspondence, immutable
single-parent commits, distinct product references, explicit component posture,
independent progress, one-winner same-head races, no half-publication, typed
partial settlement, bounded retention, compiler-enforced progression, exact
counters, executable docs, dependency direction, and zero legacy composition
authority.

Milestone 9.17.3 receives exact product-branch observation, admitted composite
basis, performed composite publication, history/inspection projections, and
typed losing/partial outcomes. It may carry and project those artifacts but may
not mint them, rebuild them from component ids, move product heads, or repair a
missing owner/Bridge guarantee with Query facade logic.
