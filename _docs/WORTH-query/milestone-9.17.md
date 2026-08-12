# Milestone 9.17: Composite Runtime Branching And Branch-Local MVCC

## Goal

Establish one ordinary product branch as a Runtime Bridge-owned reference to an
exact composite runtime-world commit while preserving distinct branch and
version authority inside Relational and Signal. A product branch may select a
Relational branch basis and a different Signal branch basis; two product
branches may share one immutable Signal basis while their Relational histories
diverge, or may later fork and advance Signal independently.

At the same time, replace the conservative global Relational commit coordinator
with branch-local MVCC. Writers whose selected component branches are
independent progress independently. Writers that compete for the same
Relational or composite product head receive one honest committed, stale, or
conflict outcome.

This milestone establishes the single-parent composite history consumed by
[Milestone 9.18](./milestone-9.18.md). It does not define semantic merge,
rebase, multi-parent publication, offline synchronization, or distributed
recovery.

## Roadmap Placement

Milestone 9.16.1 made branch affinity mandatory across Query planning,
provider sessions, read sets, proposals, invariants, commits, receipts, and
publication. Milestone 9.16 retained a provisional one-product-branch and
global-coordinator implementation limit. Runtime Bridge Milestone 10 already
established that truth branches and Signal branches are different authorities
joined only by an explicit correspondence artifact.

Milestone 9.17 turns those facts into the ordinary branch foundation before
tree-based correction becomes a public product:

```text
owner-issued Relational branch basis ----\
                                         +--> admitted composite runtime basis
owner-issued Signal branch basis --------/             |
                                                       v
                                          composite world commit
                                                       |
                                                       v
                                          product branch reference
                                                       |
                       +-------------------------------+------------------+
                       |                                                  |
                       v                                                  v
          Relational-local MVCC work                         Signal-local work
                       |                                                  |
                       +------------ owner-issued results ----------------+
                                                       |
                                                       v
                                  Bridge coordinated compare-and-publish
                                                       |
                                                       v
                                     new composite commit and branch head
```

The complete semantic-world basis, multi-parent history, merge, rebase,
durability, and offline generalizations remain in the cross-runtime merging and
branching roadmap. That roadmap consumes this milestone rather than inventing a
second ordinary product branch.

## Current Boundary

- Relational already owns authoritative graph truth, truth versions, commit
  preparation, and branch-local history concepts.
- Signal already owns branch-local derived execution state, definition-bound
  evaluation, snapshots, restore, and branch lifecycle.
- Runtime Bridge already owns typed truth-branch to Signal-branch
  correspondence for speculative and branch-aware protocols.
- Query already carries branch affinity through its planned and admitted
  workflows but must not assemble lower-runtime branch identifiers manually or
  become a branch registry.

The missing capability is one authoritative composition-level answer to:

> Which exact Relational basis and which exact Signal basis constitute the
> current product world on this product branch?

A Relational branch head alone cannot answer that question, and a floating
Signal branch identifier cannot answer which Signal version was selected.

## Adversarial Constraint

Two product branches begin from one composite commit. Both select the same
immutable Signal branch basis but fork distinct Relational branches. A blocked
Relational transaction on the first product branch holds every resource it may
lawfully hold while the second commits. The second advances Relational while
retaining the exact Signal basis. A third product branch advances Signal while
retaining its Relational basis. A fourth operation changes both components.

At the same time:

- two Relational branches and two Signal branches have equal local version
  ordinals;
- two writers race against one composite product head;
- one component preparation succeeds before another component rejects;
- the product head advances after preparation but before publication;
- a hostile caller swaps a Relational basis, Signal basis, correspondence,
  retention pin, receipt, or equal ordinal from a neighboring branch; and
- cancellation occurs at every owner and orchestration boundary.

The independent oracle must observe:

- progress on the unrelated product branch despite the blocked writer;
- exact reuse of the immutable Signal basis without cloning or ambient
  `latest` selection;
- owner-issued advancement only in the components named by the operation;
- exactly one same-product-branch head advance;
- no product-visible half publication;
- no cross-runtime or cross-branch substitution;
- complete cleanup or typed retained-orphan posture after failed preparation;
  and
- one canonical single parent for every ordinary composite commit.

A design based on a Relational-only product branch, a one-to-one branch-id
assumption, floating component heads, a global commit lock, or best-effort
multi-runtime publication must fail this courtroom.

## Product Decision Lock

1. Relational owns Relational branch identity, branch-local truth versions,
   snapshots, transactions, authoritative commits, conflicts, retention, and
   owner-local publication.
2. Signal owns Signal branch identity, exact Signal branch bases, definition-
   bound execution state, snapshots, restore, derived lifecycle, and
   owner-local advancement.
3. Runtime Bridge owns the admitted correspondence among exact component bases,
   composite runtime-world commits, product branch references, composite head
   generations, and cross-runtime publication orchestration. This is
   composition authority, not authority over either runtime's internal truth.
4. Query owns branch workflow declarations, fresh admission, typed progression,
   public DX, and outcome projection. It cannot mint component bases, composite
   commits, or product branch heads.
5. `ProductBranchId`, `RelationalBranchId`, and `SignalBranchId` are distinct
   meanings and types. Equality, matching text, or equal ordinals grants no
   correspondence.
6. A composite runtime basis names exact owner-issued Relational and Signal
   branch versions. No component may be selected by an ambient current branch,
   unqualified branch id, or `latest` lookup during execution or replay.
7. A composite world commit is immutable composition truth. It binds one exact
   ordinary parent, the exact component bases, component-change posture,
   correspondence proof, authoring provenance, and publication outcome.
8. A product branch is a mutable compare-and-publish reference to one composite
   world commit. Component branch heads alone do not define product currentness.
9. Branch creation consumes one exact retained composite commit and an explicit
   component plan. A component may reuse an exact immutable basis or fork from
   it through that component owner's authority; implicit shared mutability is
   forbidden.
10. Reusing one Signal basis across several product branches is lawful only as
    immutable exact-basis reuse. Any Signal mutation requires an owner-issued
    advance or fork and produces a new composite basis.
11. Owner-local results are candidates until Runtime Bridge admits their
    correspondence and wins one atomic composite-head compare-and-publish.
    Prepared or immutable component artifacts cannot independently become the
    product branch's current world.
12. A component that is unchanged by an operation remains at its exact prior
    basis. The bridge may not refresh it opportunistically.
13. Failed or losing preparations produce no product-head movement. Any
    retained immutable candidate has a typed orphan/retention posture and a
    bounded owner-managed lifecycle; it is never silently current.
14. Per-branch coordinators may share mechanical storage, but no global lock may
    serialize independent Relational branches, Signal branches, or product
    branches.
15. Retention pins name exact composite commits and their component bases.
    Deleting or archiving a product reference cannot reclaim a component basis
    still required by another product branch, snapshot, transaction,
    correction, publication, or recovery obligation.
16. Derived Signal caches and diagnostics are not component basis authority.
    Basis-equivalent rebuilds may replace them without moving the product head.
17. Merge, rebase, multi-parent publication, tags, offline synchronization,
    durable restart, and distributed recovery remain governed by the
    cross-runtime merging and branching roadmap.

## Destination Topology

```text
worth-relational/
    branch/
        identity.rs
        creation.rs
        lifecycle.rs
    history/
        commit.rs
        parentage.rs
        retention.rs
    mvcc/
        branch_version.rs
        snapshot.rs
        transaction.rs
        conflict.rs
        coordinator.rs
        publication.rs

worth-signal/
    existing branch authority
        owner-issued branch basis
        snapshot and restore
        derived execution lifecycle

worth-runtime-bridge/
    runtime_world/
        basis/
            component_basis.rs
            correspondence.rs
            compatibility.rs
        history/
            composite_commit.rs
            parentage.rs
            retention.rs
        branch/
            identity.rs
            reference.rs
            creation.rs
            lifecycle.rs
        publication/
            preparation.rs
            coordination.rs
            compare_and_publish.rs
            outcome.rs

worth-query-execution/
    basis/
        runtime_world.rs
        product_branch.rs

worth-query-decl/ and worth-query-host/
    branch workflow facade over admitted product branches

worth-query-certification/
    runtime_world_branching/
        shared_signal_basis.rs
        independent_progress.rs
        component_substitution.rs
        coordinated_publication.rs
        retention.rs
```

The stable structural axes are component authority inside each runtime and
composition authority inside Runtime Bridge. Query remains the audience facade.
Forbidden destinations include a Query-local branch registry, a Relational
field containing an ambient Signal head, a generic `branch_manager`, and a
second composite-history implementation inside the later merge roadmap.

## Phase Plan

### Phase 1: Owner-Issued Component Basis Contracts

Freeze the exact Relational and Signal basis artifacts admitted into ordinary
composition. Each includes runtime, branch, version/generation, lifecycle, and
retention identity sufficient to reject copied, stale, foreign, or derived
substitutes. This phase does not move authority out of either runtime.

### Phase 2: Composite Basis, Commit, And Product Branch Authority

Install Runtime Bridge-owned correspondence admission, immutable single-parent
composite commits, product branch references, explicit component reuse/fork
plans, and exact composite retention. Make floating component selection and
one-to-one branch-id assumptions unrepresentable.

### Phase 3: Relational Branch-Local MVCC

Replace the global Relational coordinator with branch-qualified snapshots,
transactions, conflict evidence, and atomic owner-local publication candidates.
Equal ordinals on different Relational branches remain non-equivalent, and
unrelated branches progress independently.

### Phase 4: Coordinated Composite Publication

Build the Runtime Bridge orchestration progression from exact expected
composite head through owner preparation, compatibility, component outcomes,
and one atomic composite compare-and-publish. Define cancellation, stale head,
component rejection, partial preparation, retained orphan, and indeterminate
outcomes without exposing a half-current product world.

### Phase 5: Query Carriage And Public Facade Cutover

Carry the admitted composite branch and exact component basis through every
Query plan, provider session, read set, proposal, invariant, effect, commit,
receipt, publication, history read, and recovery boundary. Publish ordinary
branch creation, selection, inspection, and mutation through
`worth-query-decl` and `worth-query-host`; delete Relational-only product branch
assumptions and ambient Signal selection.

### Phase 6: Lifecycle, Documentation, And Hostile Certification

Document product branches as exact component compositions, including shared
Signal bases, component-specific forks, stale correspondence, cancellation,
retention, and orphan cleanup. Certify the complete adversarial courtroom
through the real Query composition root and owner facades. Mutation of exact
component selection, correspondence admission, branch-local coordination, or
composite compare-and-publish must turn the evidence red.

## DX Target

```rust
let proposal = app
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
    .on_branch(proposal)
    .transaction()
    .apply(admitted_change)
    .commit()
    .await?;
```

The ordinary caller selects semantic intent and an admitted product branch, not
raw lower-runtime ids. Advanced branch creation makes component reuse or fork
posture explicit because only the caller can choose that product meaning.

## Performance Contract

- Relational commit coordination scales with contention on the selected
  Relational branch, not total product branches or unrelated writers.
- Composite publication scales with the fixed number of participating
  component owners plus the components changed by the operation; it does not
  scan all runtimes, branches, or history.
- Product branch creation is O(number of component bindings), O(1) for the
  current Relational-plus-Signal composition, plus declared retention work.
- Reusing an exact Signal basis performs no graph copy, evaluation, or cache
  duplication.
- Ordinary single-component work performs exact-zero preparation in unchanged
  components beyond validating the carried basis proof required for composite
  publication.
- Ancestry traversal, historical comparison, and orphan reclamation are
  explicit historical or maintenance lanes and never hide inside ordinary
  commit accessors.
- Counters distinguish owner preparation, component basis checks, composite
  head comparisons, same-branch retries, unrelated-branch waits, retained
  candidates, and cleanup breadth.

## Must Preserve

- Milestone 9.16.1's single graph-obligation and provider-session progression;
- Milestone 9.16 authentication, authorization, disclosure, invariant,
  compare-and-commit, recovery, aftermath, and publication contracts;
- Relational ownership of authoritative graph and truth-version history;
- Signal ownership of definition-bound derived execution branches and state;
- Runtime Bridge's inability to fabricate component currentness or internal
  owner authority;
- Query's inability to mint component bases, correspondence, composite commits,
  conflicts, or product-head movement; and
- cert-only replay.

## Explicit Non-Goals

- application undo, redo, inverse, or compensation semantics;
- semantic diff, merge, or rebase policy;
- multi-parent commits, tags, or cross-runtime best-common-ancestor logic;
- offline replicas and synchronization;
- durable Store-backed restart and distributed atomic publication; and
- treating derived Signal outputs as authoritative component truth.

## Acceptance Evidence

Milestone 9.17 closes only when shared-Signal-basis, divergent-component,
Relational-only, Signal-only, combined-component, independent-progress,
same-head-race, equal-ordinal substitution, stale-correspondence,
partial-preparation, cancellation, retention, branch-creation, facade,
documentation, dependency, residue, and structural-cost evidence agree.

The decisive product observation is the composite branch head and exact
owner-observed component bases. A green Relational commit alone is not closure.

## Handoff

[Milestone 9.18](./milestone-9.18.md) consumes exact product branches,
composite commits, component bases, single-parent ancestry, correction
retention, and coordinated compare-and-publish authority to define tree-based
semantic undo and redo as newly admitted composite history. It may not create a
second composition owner, treat Relational history as the whole product world,
or weaken independent component and branch progress.
