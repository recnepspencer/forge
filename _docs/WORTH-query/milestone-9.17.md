# Milestone 9.17: Composite Runtime Branching And Branch-Local MVCC

## Governing Role

Milestone 9.17 is the governing umbrella for the ordinary composite product-
branch foundation. It is implemented through three authority-aligned
submilestones:

1. [Milestone 9.17.1](./milestone-9.17.1.md) establishes exact owner-issued
   component bases, real Relational branch-local MVCC, and durable recovery of
   both component owners.
2. [Milestone 9.17.2](./milestone-9.17.2.md) establishes Runtime Bridge-owned
   durable composite runtime-world history and coordinated publication.
3. [Milestone 9.17.3](./milestone-9.17.3.md) carries that authority through
   Query, completes owner-first PostgreSQL recovery and existing-outbox
   eligibility, cuts the public facade over, and certifies the product boundary.

The dependency order is strict:

```text
9.17.1 owner component bases and Relational MVCC
    -> 9.17.2 composite history and coordinated publication
        -> 9.17.3 Query carriage, facade, and certification
            -> 9.18 tree-based semantic undo and redo
```

This split is not three interpretations of one feature. Each submilestone owns
one distinct authority boundary and reaches a useful, independently reviewable
completion state. Milestone 9.17 closes only when all three close and the
cumulative courtroom passes through the real public Query composition root.

## Goal

Establish one ordinary product branch as a Runtime Bridge-owned reference to an
exact composite Relational-plus-Signal world commit while preserving distinct
branch, basis, version, history, and publication authority inside Relational
and Signal.

A product branch may select one Relational branch basis and a different Signal
branch basis. Two product branches may share one immutable Signal basis while
their Relational histories diverge, or may later fork and advance Signal
independently. Writers whose selected Relational branches and product heads are
independent progress independently. Writers that compete for the same owner or
composite head receive one typed committed, stale, conflict, cancelled, or
indeterminate outcome.

The resulting single-parent composite history is the sole ordinary foundation
consumed by [Milestone 9.18](./milestone-9.18.md). Semantic merge, rebase,
multi-parent publication, offline synchronization, Store-native replication,
and distributed cross-region recovery remain outside this milestone.

## Roadmap Placement

Milestone 9.16 establishes the typed, authenticated, authorized ordinary Query
front door. Milestone 9.16.1 makes branch affinity mandatory through planning,
provider sessions, read sets, proposals, invariants, commits, receipts, and
publication, while intentionally retaining conservative single-product-world
and globally coordinated Relational mechanics. Milestone 9.16.2 establishes
stable package reconstruction plus the PostgreSQL-backed ordinary durable
runtime and restart-safe existing-outbox claimant. It does not serialize branch
authority or make PostgreSQL rows a physical Store model for Query.

Milestone 9.17 replaces those implementation limits without weakening the
front door:

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
          Relational branch-local work                       Signal-local work
                       |                                                  |
                       +------------ owner-issued results ----------------+
                                                       |
                                                       v
                                  Bridge coordinated compare-and-publish
                                                       |
                                                       v
                                     new composite commit and branch head
                                                       |
                                                       v
                               Query-carried public product-branch workflow
```

Multi-parent history, merge, rebase, Store-native replication, and offline
generalizations remain in the cross-runtime roadmap. PostgreSQL durability is
not deferred: every 9.17 authority owner extends the runtime-level adapter as it
lands, so ordinary restart guarantees never regress.

## Current Boundary

- Relational owns authoritative graph truth, branch and commit identity,
  snapshots, transaction conflict meaning, history, and retention, but its
  current transaction entry and commit coordination remain globally mutable.
- Signal owns branch-local derived execution state, exact snapshot and restore
  posture, definition-bound evaluation, and branch lifecycle. Its existing
  branch basis is materially stronger than a raw branch id or numeric version.
- Runtime Bridge owns cross-runtime correspondence and protocol admission, but
  does not yet own an ordinary composite commit graph or product-branch
  reference store.
- Query carries branch affinity, but some current surfaces derive product truth
  identity from Relational branch identity and therefore cannot represent an
  independently selected Signal basis honestly.
- `worth-foundational` already owns portable branch, commit, parentage,
  correspondence, canonical-basis, locator, and boundary vocabulary. Those
  values describe cross-boundary meaning but grant no runtime authority.
- `worth-proof` already owns phase progression, checked outcomes, assumption
  basis, freshness downgrade, readmission, performed-effect law, and fixed-
  shape composition. It is the substrate for compiler-visible progression,
  not a replacement runtime or history engine.

The missing product fact is:

> Which exact owner-issued Relational basis and exact owner-issued Signal basis
> constitute the current product world selected by this product branch?

Neither a Relational branch head nor a floating Signal branch identifier can
answer that question alone.

## Cumulative Adversarial Courtroom

Two product branches begin from one composite commit. They share one immutable
Signal basis and fork distinct Relational branches. A Relational writer on the
first branch blocks while holding every resource it may lawfully hold. The
second branch commits. A third operation advances Signal while retaining
Relational. A fourth changes both components.

At the same time:

- different component branches carry equal local version ordinals;
- two writers race one Relational branch head;
- two writers race one product branch head;
- one component preparation succeeds before another component rejects;
- a product head advances after preparation but before publication;
- a hostile caller swaps a component basis, correspondence, expected head,
  retention pin, receipt, or equal ordinal from a neighboring runtime or
  branch;
- a trust-boundary round trip weakens basis freshness and demands owner
  readmission;
- cancellation occurs before preparation, between component preparations,
  before composite publication, and after owner-local immutable work exists;
- process loss occurs after owner-local outbox commit, before and after durable
  product-head CAS, during owner-first recovery, and after external send before
  acknowledgement; and
- Query history, inspection, recovery, and live paths attempt to fall back to
  Relational-only or ambient Signal selection.

The independent oracle must observe:

- unrelated Relational and product branches progress despite the blocked
  writer;
- same-branch races retain exact typed stale/conflict posture;
- immutable Signal basis reuse performs no graph copy, evaluation, or ambient
  `latest` lookup;
- only operation-named components advance;
- every successful ordinary composite commit has exactly one canonical parent;
- exactly one winning product-head movement occurs;
- no public or historical observation sees a half-current product world;
- no copied, stale, foreign, equal-ordinal, or representation-equal artifact
  substitutes for owner authority;
- failed preparation leaves complete cleanup or a typed bounded retained-
  candidate posture; and
- the same exact product-world basis reaches Query planning, execution,
  publication, receipts, history, live observation, and recovery;
- a fresh process reconstructs both component owners, Bridge product heads,
  Query carriage, and pending dispatch before readiness; and
- an existing Relational outbox fact is dispatchable only when its exact
  composite publication is performed.

A Relational-only product branch, one-to-one component branch assumption,
floating head lookup, global commit mutex, best-effort multi-runtime
publication, or Query-restamped authority must fail this courtroom.

## Product Decision Lock

1. Relational owns Relational branch identity, versions, snapshots,
   transactions, conflicts, authoritative commits, retention, and owner-local
   publication.
2. Signal owns Signal branch identity, exact bases, snapshots, restore,
   definition-bound execution state, derived lifecycle, and owner-local
   advancement.
3. Runtime Bridge owns admitted exact component correspondence, immutable
   composite runtime-world commits, product branch references, composite head
   generations, and cross-runtime publication orchestration. It owns
   composition currentness, not component truth.
4. Query owns branch workflow declaration, fresh admission, public progression,
   DX, and outcome projection. It cannot mint a component basis,
   correspondence, composite commit, or product-head movement.
5. Product, Relational, and Signal branch identities are distinct meanings and
   types. Equal text, digests, or version ordinals grant no correspondence.
6. A composite basis names exact owner-issued component bases. No component is
   selected by ambient branch state or a `latest` lookup.
7. A composite world commit is immutable composition truth with one ordinary
   parent, exact component bases, component-change posture, correspondence,
   provenance, and publication outcome.
8. A product branch is a mutable compare-and-publish reference to one composite
   commit. Component heads do not independently define product currentness.
9. Component reuse is exact immutable-basis reuse. Component mutation requires
   owner-issued advancement or fork authority.
10. Unchanged components remain at the exact carried basis and are never
    opportunistically refreshed.
11. Owner-local results remain candidates until Bridge compatibility admission
    and composite-head compare-and-publish succeed.
12. Failed or losing preparation moves no product head. Retained immutable
    candidates have typed bounded owner-managed lifecycle posture.
13. Independent branches are never serialized by an ordinary global commit
    lock. Shared mechanical storage cannot erase logical ownership.
14. Retention pins exact composite commits and every exact component basis
    needed by live branches, snapshots, transactions, corrections,
    publication, or recovery.
15. `worth-foundational` vocabulary remains descriptive and canonical. A
    canonical digest, locator, branch descriptor, or correspondence value
    cannot become operational admission authority.
16. `worth-proof` supplies progression, basis, freshness, readmission,
    structural-fact, and performed-effect law beneath stronger owner-specific
    types. Generic Proof carriers cannot replace those types.
17. Every operational phase transition consumes the exact private-minted
    predecessor. Illegal ordering, raw candidate promotion, stale proof reuse,
    and weaker-type substitution are compiler-rejected where public API shape
    permits and otherwise fail typed before effects.
18. Derived Signal caches, Query projections, diagnostics, and certification
    artifacts have zero component or composite currentness authority.
19. Merge, rebase, multi-parent publication, tags, offline synchronization,
    Store-native replication, and distributed recovery remain governed
    elsewhere. PostgreSQL durable restart is required across this umbrella.
20. Owner-defined durable artifacts remain distinct: Relational and Signal own
    component recovery; Runtime Bridge owns composite history/currentness
    recovery; Query owns fresh dispatch admission; PostgreSQL owns only physical
    representation and cross-owner lifecycle ordering.
21. Query's existing outbox payload remains in the Relational component commit,
    but only performed Bridge composite publication can make it product-
    dispatchable after 9.17.3.

## Authority-Aligned Milestone Partition

| Submilestone | Sole governing outcome | Primary owner | Explicit exclusion |
| --- | --- | --- | --- |
| [9.17.1](./milestone-9.17.1.md) | Exact component bases, concurrent Relational branch-local MVCC, and durable component recovery | Relational and Signal, each for its own basis/artifact | No composite product branch or public Query workflow |
| [9.17.2](./milestone-9.17.2.md) | Durable composite history, product branch references, and coordinated compare-and-publish | Runtime Bridge | No Query-owned history or public facade cutover |
| [9.17.3](./milestone-9.17.3.md) | End-to-end Query carriage, owner-first recovery, existing-outbox composite gating, facade, and certification | Query as audience/admission facade and certification owner | No new component/composition or physical-storage authority |

No submilestone may claim the next submilestone's product. In particular,
9.17.1 does not ship a product branch, 9.17.2 does not claim public Query
completion, and 9.17.3 does not repair missing owner-local MVCC or Bridge
history with facade glue.

## Governing Destination Topology

```text
worth-relational/
    branch/
    history/
    mvcc/
    durability/

worth-signal/
    branch/
    durability/

worth-runtime-bridge/
    runtime_world/
        basis/
        history/
        branch/
        publication/
        durability/

worth-query-execution/
    basis/
        runtime_world.rs
        product_branch.rs

worth-query-decl/ and worth-query-host/
    branch workflow facade over admitted product branches

worth-query-certification/
    runtime_world_branching/

worth-runtime-postgres/
    owner/
        relational/
        signal/
        runtime_world/
        query_package/
    dispatch/
    runtime/
```

The submilestone specs define the populated files and ownership within these
stable axes. Forbidden destinations include a Query-local branch registry, a
Relational field containing an ambient Signal head, a generic
`branch_manager`, a Proof-owned runtime workflow engine, a Foundational-owned
operational branch handle, or a second composite-history implementation in the
later merge roadmap. SQL inside an authority owner, Query-owned physical runtime
composition, and adapter-minted performed publication are also forbidden.

## Cumulative Performance Contract

- Relational commit coordination scales with contention and state touched on
  the selected Relational branch, not total branches or unrelated writers.
- Composite publication scales with the fixed component count plus the
  components changed by the operation; it never scans all runtimes, branches,
  or history.
- Product branch creation is O(component bindings), O(1) for the ordinary
  Relational-plus-Signal composition, plus declared retention work.
- Exact Signal basis reuse performs zero graph copy, evaluation, cache
  duplication, or hidden owner advancement.
- Single-component operations perform exact-zero preparation in unchanged
  components beyond carried-basis validation required for publication.
- Ancestry traversal, historical comparison, retention reclamation, and orphan
  cleanup remain explicit history or maintenance lanes.
- Component publication and composite publication expose exact durable bytes,
  barriers, CAS attempts, and recovery work; unrelated branches contribute zero
  synchronous semantic coordination.
- Pending dispatch lookup is indexed and joins one outbox locator to one exact
  performed composite publication; it does not scan histories.
- Receipts expose exact owner preparation, component basis validation,
  composite comparison, same-branch retry, unrelated-branch wait, retained-
  candidate, cleanup, and Query carriage counters.

## Must Preserve

- Milestone 9.16.1's single graph-obligation and provider-session progression;
- Milestone 9.16's authentication, authorization, disclosure, invariant,
  compare-and-commit, recovery, aftermath, and publication contracts;
- Milestone 9.16.2's package reconstruction, canonical PostgreSQL durability,
  runtime-level facade, and existing-outbox claim guarantees, while keeping
  branch authority outside archives, SQL rows, and dispatch leases;
- Relational and Signal component authority;
- Bridge composition authority without component-truth absorption;
- Query's inability to mint lower authority;
- Foundational canonical and boundary vocabulary without authority promotion;
- Proof phase, freshness, readmission, performed-effect, and structural-fact
  law beneath owner-specific types; and
- certification-only replay.

## Explicit Non-Goals

- application undo, redo, inverse, or compensation semantics;
- semantic diff, merge, or rebase policy;
- multi-parent commits, tags, or best-common-ancestor logic;
- offline replicas and synchronization;
- distributed cross-region atomic publication/recovery; and
- treating derived Signal output or Query materialization as component truth.

## Allowed Debt

- Store-native graph persistence, replication, distributed atomic publication,
  and cross-region recovery remain explicit Store/cross-runtime handoffs.
  PostgreSQL composite restart is not debt.
- Semantic merge, rebase, multi-parent history, and offline synchronization
  remain explicit cross-runtime roadmap work.
- No runtime-backed global coordinator, floating component head, Relational-
  only product branch, ambient Signal selection, half-publication lane, raw
  authority constructor, or callable compatibility path may remain debt.

## Parallelization And Store Dependency

The three submilestones are sequential because each downstream owner consumes
the upstream authority contract. Within a submilestone, inventories,
documentation scaffolds, independent oracle construction, and non-overlapping
owner-local work may proceed in parallel only after their shared binding and
phase contracts freeze. Facade cutover and cumulative certification close last.

The 9.17 sequence is not blocked on `worth-store`. PostgreSQL durable component
and composite restart are required local closure. Store-native graph execution,
replication, distributed atomic recovery, and joined Store-provider
certification remain explicit Store or cross-runtime work.

## Documentation Deliverables

- this umbrella remains the single governing decision record and links every
  submilestone;
- each submilestone owns its current boundary, courtroom, decision lock,
  destination topology, phase plan, proof obligations, and handoff;
- public Query branch documentation lands only in 9.17.3 after the real facade
  exists;
- `worth-foundational` and `worth-proof` docs are linked where their shared
  vocabulary or progression law is used, without teaching them as operational
  owners; and
- successor documents identify 9.17.3 as the final implementation prerequisite
  while retaining 9.17 as the semantic umbrella.

## Umbrella Acceptance

Milestone 9.17 closes only when:

- 9.17.1 proves exact component bases and independent Relational branch
  progress without a global ordinary commit coordinator, plus exact
  PostgreSQL recovery of both component owners;
- 9.17.2 proves exact composite correspondence, immutable single-parent
  history, product branch references, retention, durable product-head CAS,
  owner-first recovery, and no-half-publication compare-and-publish;
- 9.17.3 proves the complete authority reaches every Query boundary through
  the public facade, gates existing-outbox dispatch on performed composite
  publication, and deletes the Relational-only/ambient-Signal lane;
- the cumulative courtroom runs through real owner facades and the real Query
  composition root with an independent component/composite-history oracle;
- public invalid phase order and raw/weak authority substitution are
  mechanically denied using compiler-visible owner-specific types;
- default and admitted parallel lanes converge on identical composite truth;
- real-PostgreSQL fresh-process courts recover the exact package, component
  bases, composite heads, Query state, and pending dispatch before readiness;
- documentation, dependency enforcement, facade inventory, residue scans,
  structural counters, and runtime evidence agree; and
- no submilestone closeout claims the umbrella before all downstream
  integration evidence exists.

The decisive product observation is the product branch head and exact
owner-observed component bases. A green Relational commit, Bridge unit test, or
Query facade compile alone is not umbrella closure.

## Handoff

[Milestone 9.18](./milestone-9.18.md) begins only after 9.17.3 and therefore the
entire 9.17 umbrella close. It consumes exact product branches, immutable
single-parent composite commits, component bases, ancestry, retention, and
coordinated compare-and-publish authority, owner-first PostgreSQL recovery, and
performed-publication-gated aftermath to define tree-based semantic undo and
redo as freshly admitted composite history. It may not create another
composition owner, treat Relational history as the whole product world, or
weaken independent component and branch progress.
