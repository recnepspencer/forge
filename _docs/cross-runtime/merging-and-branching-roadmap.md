# Cross-Runtime Merging, Branching, Collaboration, And Offline Roadmap

## Purpose

This roadmap defines the implementation order for WORTH's semantic-Git
capabilities: long-running branches, semantic diff, deterministic merge,
governed conflict resolution, collaboration, offline work, causal convergence,
selective replication, and the lifecycle machinery that lets those features
remain honest over years of evolution.

The product goal is stronger than versioned values. Two semantic worlds must be
able to diverge and later reconcile without losing node identity, aspect
meaning, relationships, invariants, policy, definition history, or derived
behavior. The same model must eventually support interactive collaboration,
offline editing, opt-in CRDT domains, partial replicas, and future domains whose
merge rules are not known today.

This is a cross-runtime roadmap. It is ordered by implementation dependency,
not by the repository containing each edit. A milestone may require changes in
Worth Query, Worth Relational, Worth Signal, Worth Runtime Bridge, Worth Store,
Worth Foundational, or Worth Proof when those changes are prerequisites for one
coherent collaboration model.

This roadmap does not preserve the current collaboration extension planned as
Phases 36 through 41 of Query Milestone 9.3.8. Their valid product obligations
are redesigned here against the complete semantic, durable, and offline
problem. Query remains the ordinary product facade, but Query-facing admission,
witness, classification, recovery, and inspection are built only after the
lower-authority merge model is real.

## Roadmap Position

```text
Query Milestone 9.3.8 Phase 35
  -> Milestones 1-10: semantic branching and merge authority
  -> Milestones 11-15: collaborative control plane and Query product surface
  -> Milestones 16-18: offline, causality, CRDT, and selective replication
  -> Milestones 19-21: lifecycle, extension, and operational truth
  -> Milestone 22: hostile universal certification
```

The Store roadmaps remain parallel authorities:

- the [Physical Database Roadmap](../worth-store/physical-database-roadmap.md)
  owns physical survival, isolation, WAL, checkpoint, recovery physics,
  integrity, and bounded storage mechanics
- the [Store Runtime And Query Integration Roadmap](../worth-store/runtime-integration-roadmap.md)
  owns the joined runtime, semantic-to-physical integration, durable publication,
  residency, recovery, Store-backed Query execution, durable historical worlds,
  replication transport, and joined certification
- this roadmap owns the semantic branch/diff/merge/collaboration model that
  those Store milestones make durable

Store Milestones 10, 15, and 16 do not compete with this roadmap. They provide
durable historical-world storage, retention/maintenance, and portable transfer
for the semantic artifacts defined here. If implementation reveals a Query,
Relational, Signal, Bridge, Foundational, or Proof prerequisite, the edit stays
in this roadmap's implementation sequence even when the source lives elsewhere.

## Document Authority And Future Spec Names

This file is the planning authority for cross-runtime branching, semantic diff,
merge, conflict resolution, collaboration, offline operation, and convergent
replication.

Future milestone specs use the
`merging-and-branching-m{number}.md` naming family in this directory. Milestone
specs must identify their required closure class and may not claim production
closure from runtime-only evidence when durable or distributed evidence is
required.

Query Milestone 9.15 may establish single-semantic-world candidate-search,
bounded-convergence, transformation-occurrence, loss, and proposal evidence
before this roadmap closes. Those artifacts are derived inputs to later
governed resolution. They may not create conflict identity, participant
authority, durable decision state, branch-aware carry-forward, or resolution
session recovery.

## Governing Summaries

- `MENTALITY.md` protects adversarial-first, foundation-first construction. Its
  strongest constraint here is that identity, definition versioning, merge
  algebra, conflict semantics, and lifecycle authority must exist before
  collaboration ergonomics make the system look finished.
- `arch_laws.md` protects autonomous authorities and proof-widening phase
  progression. Its strongest constraint here is that admission, comparison,
  planning, resolution, preparation, publication, and recovery must be typed
  transitions; a coordinator may join authorities but may not become them.
- `composition_laws.md` protects named semantic responsibilities. Its strongest
  constraint here is that branching, merge policy, conflict resolution,
  causality, replication, and inspection may not collapse into a collaboration
  manager, helpers bag, or one god merge function.
- `domain_structure_laws.md` protects truth-source and lifecycle distinctions.
  Its strongest constraint here is that authoritative semantic history,
  derived assistance, speculative branch state, replicated observations, and
  diagnostics occupy different structural spaces.
- `perf_laws.md` protects delta-bounded execution and cost-honest APIs. Its
  strongest constraint here is that ordinary diff, merge, synchronization, and
  cleanup scale with changed meaning and declared dependency closure rather
  than total world size, branch age, replica count, or diagnostic richness.
- `dx_laws.md` protects organized truth at the call site. Its strongest
  constraint here is that expensive, distributed, or human-mediated work uses
  inspectable plans and resumable sessions with explicit basis, policy, scope,
  consistency, cost, progress, cancellation, and recovery.
- the Query roadmap protects one daily-driver semantic language. Query may
  author and inspect branch workflows, but Relational, Signal, Bridge, and
  Store retain their real authorities.
- the Store runtime roadmap protects one joined runtime. This roadmap may not
  create a second semantic Store runtime, a second Relational instance, or a
  collaboration database beside the canonical commit path.

## Global Adversarial Constraint

The completed system must survive this hostile condition:

> Multiple long-running semantic worlds evolve for months across online and
> offline replicas while schemas, aspect definitions, node logic, invariants,
> policies, relationships, and derived graphs also evolve. Users concurrently
> edit overlapping and disjoint scopes, add and remove aspects, split and join
> identities, resolve conflicts manually, exchange partial replicas, reconnect
> with duplicated and reordered operation histories, and crash at every
> preparation and publication boundary. Every accepted result must preserve one
> deterministic semantic history, every unresolved ambiguity must remain typed
> and visible, every derived result must be reproducible from admitted
> authority, and no ordinary operation may require full-world reconstruction or
> global serialization merely because branches, replicas, or history exist.

The architecture has failed if it:

- treats a branch as a copied map or a mutable label rather than a basis-exact
  semantic world with durable lineage
- compares raw current values while ignoring the definition, policy, schema,
  or logic world that gave those values meaning
- reconstructs node identity from labels, content hashes, storage keys, or
  structural similarity
- merges facts before merging the definitions and policies required to
  interpret those facts
- treats whole entities as the minimum conflict unit when aspects or
  relationships are independently meaningful
- allows a custom strategy to bypass invariants, authority, determinism,
  compatibility, or lifecycle participation
- interprets "automatic merge" as silent last-writer-wins
- represents conflict as strings, paths without typed identity, or an opaque
  left/right value bag
- lets manual resolution mutate truth outside a governed, replayable session
- promotes candidate-search, convergence, transformation, loss, repair, or
  advisory evidence into a resolution command because its fields appear to
  match
- gives merge, import repair, routing, physics, assumptions, or AI advisories
  separate durable problem/decision/session state machines
- assumes a derived Signal graph can be merged as authoritative truth
- allows Query to become a second merge engine or Store to become semantic
  branch authority
- claims offline or CRDT support without replica identity, causal context,
  deduplication, tombstone law, causal stability, and bounded compaction
- garbage-collects history or tombstones while an admitted branch, session,
  replica, continuation, or audit obligation can still require them
- forces operational reads and writes through forensic replay or rich
  diagnostics
- permits a crash or retry to publish one runtime's half of a merge

## Closure Classes

This roadmap distinguishes three different claims:

- `SemanticClose`: the runtime-backed authorities, types, algorithms, laws, and
  deterministic reference models are complete. This may unblock later semantic
  milestones before the physical Store runtime exists.
- `JoinedClose`: the same capability survives durable publication, restart,
  drainage, retention, and Store-backed execution through the single joined
  runtime.
- `DistributedClose`: the capability survives replica identity, offline
  operation, duplication, reordering, partial transfer, causality, and causal
  reclamation.

A milestone may earn more than one class, but it must name them separately. A
semantic milestone is not dishonest because physical integration is later; it
is dishonest only if it calls `SemanticClose` a production durability claim.

## Product And Authority Locks

1. Worth Query is the ordinary authoring, workflow, inspection, and recovery
   facade for collaboration.
2. Worth Relational owns authoritative semantic version history, branch heads,
   MVCC visibility, authoritative change application, merge planning inputs,
   and authoritative merge publication.
3. Worth Signal owns definition-bound derived graph evaluation, invalidation,
   branch-local derived state, and post-merge derived reconciliation.
4. Worth Runtime Bridge owns the causal protocol joining authoritative truth
   publication to derived evaluation and cross-runtime continuity.
5. Worth Store owns durable bytes, physical access, durable artifact survival,
   checkpoint/recovery physics, transfer mechanics, and reclamation mechanics.
6. The Store integration boundary joins semantic preparation, physical
   durability, authority publication, Signal routing, acknowledgment, and
   recovery. It does not invent semantics owned above or below it.
7. Worth Foundational owns vocabulary only where meaning crosses a real crate,
   trust, export, support, or diagnostic boundary. It is not a bag for every
   merge noun.
8. Worth Proof owns sealed platform authority and proof progression. Generic
   markers, digests, ids, labels, receipts, and persisted representations do
   not mint authority.
9. Authoritative state, derived state, speculative state, replicated
   observations, retained acceleration, and diagnostic artifacts remain
   different types and lifecycles.
10. Definitions merge before facts; identity and correspondence resolve before
    value merge; authoritative aspects merge before relationships and global
    invariants; derived state reconciles only after authoritative publication.
11. CRDT behavior is explicit per semantic family. WORTH does not declare all
    data eventually consistent or force non-convergent invariants into a CRDT
    shape.
12. Concurrent writers are admitted by planned read/write/conflict footprints,
    basis, and authority. A branch label alone proves neither safety nor
    conflict.
13. Public product and platform crates consume Query through the legal audience
    facades. Replay/reconstruction remains certification-only unless an
    ordinary recovery contract explicitly owns the operation.
14. The current `crates/worth-store` implementation is not treated as the
    collaboration substrate or compatibility authority. Durable work targets
    the replacement Store/runtime boundaries established by the active Store
    roadmaps.
15. Candidate sets, search completeness, optimality claims, convergence
    evidence, transformation records, correspondence suggestions, loss ledgers,
    repair proposals, and advisories are derived evidence. None can authorize
    resolution or publication without an admitted governed-resolution command.
16. Milestones 10-11 establish the one reusable governed-resolution lifecycle
    for merge conflicts and later domain problems such as foreign-model repair,
    physical routing, assumption review, and engineering advisories. Merge is
    the first mandatory specialization, not permission to encode the shared
    lifecycle in merge-shaped value bags or Query-local session state.

## Canonical Semantic Merge Order

Every merge family must lower into this order unless a milestone spec proves a
narrower family does not depend on an earlier layer:

```text
admit bases, authority, and policy
  -> reconcile schema, aspect definitions, node logic, and merge policy
  -> resolve identity, lineage, and correspondence
  -> compare and merge authoritative aspects
  -> compare and merge relationships and topology
  -> validate local and global invariants
  -> produce canonical authoritative merge effect
  -> durably publish the new multi-parent version
  -> reconcile Signal definitions and rebuild derived state
  -> publish Query result, subscriptions, audit, and recovery posture
```

Execution may parallelize proven-independent work inside a stage. It may not
reorder semantic dependencies because a different order is mechanically
convenient.

## Convergence Policy Families

The platform must support at least these explicit families without flattening
their distinct guarantees:

- strict/manual: concurrent semantic overlap always requires governed human or
  domain resolution
- deterministic three-way: base, left, and right plus a versioned resolver
  produce one deterministic result or typed conflict
- join-semilattice: state merges by an admitted associative, commutative, and
  idempotent join
- operation-convergent: causally identified operations converge under a
  declared operation algebra and delivery contract
- single-writer: one authority owns mutation and replicas observe or propose
  without pretending to merge
- derived/rebuildable: state is discarded and recomputed from admitted
  authoritative inputs rather than merged as truth

These are compatibility and execution contracts, not enum labels alone. Every
declared family must state identity, ordering, determinism, conflict,
invariant, migration, retention, offline, and proof behavior.

## Cross-Cutting Artifact Law

Every branch, diff, merge, conflict, session, offline, causal, or replication
artifact family must declare at birth:

- authority class: authoritative, derived durable, speculative, replicated
  observation, ephemeral, or diagnostic
- canonical identity, semantic version, and definition-world basis
- branch, version, parent, tenant, workspace, and policy scope where applicable
- source authority and the exact evidence that admitted it
- whether it can be rebuilt, and from which canonical inputs
- compatibility and migration posture
- ordinary, reconstructive, forensic, and synchronization cost contracts
- retention, pinning, tombstone, causal-stability, and reclamation participation
- export, import, partial-replication, authenticity, key, and custody posture
- typed unavailable, indeterminate, conflict, and recovery outcomes

No later cleanup milestone exists to classify an already-proliferated artifact
graph retroactively.

## Target Responsibility Topology

The initial topology below is strongly opinionated about responsibilities but
does not grant new crate names or freeze exact file cuts. Milestone 1 must map
it onto legal platform crates and boundary configuration before implementation.

```text
Relational authority
  semantic_world_basis/         basis and version-world truth
  branch_graph/                 commits, heads, parents, and ancestry
  semantic_change/              canonical authoritative deltas
  definition_world/             schema, aspect, logic, invariant, policy versions
  correspondence/               identity lineage and admitted correspondence
  merge_planning/               typed three-way plans and strategy lowering
  conflict_resolution/          merge loci and publication sufficiency

Signal authority
  definition_world/             derived-node and dependency definitions
  branch_derivation/            branch-local derived graphs and invalidation
  merge_reconciliation/         post-authority rebuild and continuity

Runtime Bridge authority
  collaboration_protocol/       cross-runtime causal phase progression
  merge_publication/            publication-to-derivation continuity
  collaboration_recovery/       typed incomplete-transition recovery

Governed resolution control-plane authority
  resolution_problem/           admitted problem and alternative lifecycle
  resolution_decision/          commands, decisions, supersession, stale rejection
  resolution_session/           roles, progress, checkpoint, recovery, disposal
  specialization/               sealed merge/domain semantic-owner handoffs

Query product surface
  collaboration_context/        admitted world, branch, policy, and strategy context
  comparison/                   semantic diff and explanation declarations
  merge_workflow/               plan, resolve, publish, inspect, and recover
  resolution_workflow/          inspect, propose, decide, defer, replan, and recover
  offline_workflow/             capsule, sync, readmission, and partial-world DX

Store integration authority
  semantic_commit_record/        versioned commit and parent representation
  resolution_session_record/     governed decisions and workflow checkpoints
  replica_journal_record/        causal operation and frontier representation
  semantic_merge_commit/         durability/publication/acknowledgment join
  semantic_merge_recovery/       checkpoint, tail, idempotency, and readmission
  offline_capsule_transfer/      capsule export, import, and resume mechanics
  replica_transfer/              replication and anti-entropy mechanics
  semantic_history_reclaim/      retention, stability, compaction, and GC

Certification
  semantic_git_oracles/          independent reference semantics
  schedule_exploration/         deterministic interleaving and crash search
  convergence_courtroom/        offline, CRDT, partial-replica, and lifecycle proof
```

Directories exist only when the responsibility is real. No `common`, `shared`,
`helpers`, `manager`, or collaboration-wide type bag is allowed.
Milestone 1 must assign the governed-resolution control-plane authority to a
legal crate/package boundary without defaulting it into Query, Relational,
Store, or Foundational merely because those authorities participate.

## Product DX Target

The common path should read like semantic intent while the advanced path keeps
all expensive and authority-changing decisions inspectable:

```rust
let feature_branch = world
    .branches()
    .fork(admitted_basis, "feature/geometry")
    .await?;

let comparison = world
    .compare(feature_branch.head(), main.head())
    .scope(assembly_scope)
    .explain()
    .await?;

let merge_plan = world
    .merge(feature_branch.head(), main.head())
    .policy(assembly_collaboration_policy)
    .plan()
    .await?;

let published = match merge_plan.outcome() {
    MergeOutcome::Ready(ready) => ready.publish().await?,
    MergeOutcome::Conflicted(conflicts) => {
        let session = conflicts.open_resolution_session().await?;
        resolve_with_domain_ui(session).await?.publish().await?
    }
};
```

This is conceptual DX, not a frozen API. The shipped surface must additionally
expose bases, definition worlds, consistency, authority, policy, strategy,
read/write/conflict footprints, estimated semantic and physical work, session
progress, cancellation, durability, synchronization, and recovery before the
caller crosses the relevant boundary.

## Milestone Plan

## Milestone 1: Collaboration Laws And Ownership

### Goal

Freeze the semantic vocabulary, authority map, dependency direction, lifecycle
states, closure classes, and enforcement boundaries for the entire program
before any crate widens its collaboration API.

### Hard Problem

Branch and merge concepts already exist independently in Relational, Signal,
Bridge, Query, Foundational, and Store planning. If those local concepts are
extended independently, WORTH will acquire several plausible but incompatible
definitions of branch basis, strategy, conflict, publication, and recovery.

### Must Ship

- an authority matrix for every canonical and derived collaboration artifact
- a legal package and dependency home for governed-resolution problem,
  decision, and session progression that sits above participating semantic
  domains without acquiring their publication authority
- a typed lifecycle from raw request through admission, comparison, planning,
  conflict, resolution, preparation, publication, reconciliation, and recovery
- a vocabulary decision for world, basis, branch, commit, change, diff, merge,
  conflict, resolution, replica, frontier, tombstone, and stability
- an explicit migration map for Query Milestone 9.3.8 Phases 36 through 41 and
  Store runtime Milestones 10, 15, and 16
- boundary-check rules preventing Query from owning merge truth, Store from
  owning semantic visibility, Signal from publishing authority, and ordinary
  crates from consuming certification replay
- support-matrix rows for runtime-backed, Store-backed, offline, CRDT, and
  partial-replica modes
- a spec template requiring closure class, authority/derivation, DX, cost, and
  hostile proof sections for every later milestone

### Authority And Derivation

This milestone creates architectural contracts, not a new runtime authority.
Foundational receives only genuinely shared vocabulary. Proof receives only
sealed progression or authority types. Existing crates retain their domain
truth.

### DX Target

An implementer must be able to answer "who proves this, who stores it, who may
project it, and who may recover it?" from the roadmap and generated boundary
context without searching source history.

### Proof Obligations

- boundary checker rejects representative authority inversions
- compile-fail cases reject caller-minted collaboration authority
- every existing branch/merge surface is classified as canonical, derived,
  compatibility-only, or scheduled for replacement
- no retained term has two incompatible meanings across public facades

### Closeout Gate

Earn `SemanticClose` only when the dependency graph and type progression make
the planned authority split mechanically enforceable. Documentation agreement
alone does not close this milestone.

## Milestone 2: Canonical Semantic World Basis

### Goal

Define one proof-bearing basis that identifies the complete semantic world in
which a read, change, diff, merge, resolution, replay, or synchronization act is
interpreted.

### Hard Problem

A branch head or Relational version is insufficient when schema, aspect
definitions, node logic, invariants, merge policy, tenant policy, or Signal
definitions can change. Two value graphs with the same apparent data may mean
different things under different definition worlds.

### Must Ship

- a sealed semantic-world basis carrying runtime identity, branch/version
  basis, definition-world basis, policy basis, and required causal posture
- explicit current, retained historical, speculative, imported, and
  support-only basis states
- typed basis compatibility, equivalence, ancestry, and drift classifications
- admission and readmission transitions that cannot promote ids, labels,
  digests, or receipts into current authority
- basis-use receipts sufficient to explain exactly which world every result
  observed or attempted to change
- canonical basis identity that is stable under formatting and diagnostic
  changes while diverging on semantic changes
- zero-ambient-context law for tenant, branch, policy, and definition meaning

### Authority And Derivation

Relational owns the authoritative truth-version component. Each definition
authority owns its component. Query carries and projects the admitted composite
basis. Foundational may carry the shared locator and compatibility vocabulary;
it may not decide currentness.

### DX Target

Callers select a world through a typed Query context and can inspect why that
world is current, historical, stale, incomplete, or incompatible without
assembling lower-runtime ids manually.

### Proof Obligations

- same semantic world reached through all admitted public lanes produces the
  same basis identity
- changing any meaning-bearing component changes compatibility posture
- swapped-runtime, wrong-tenant, stale-policy, pruned-version, and forged-digest
  cases fail before observation or mutation
- basis admission and comparison cost have named exact counters

### Closeout Gate

Earn `SemanticClose` when every runtime-backed collaboration operation consumes
the composite proof-bearing basis. Earn `JoinedClose` later when the basis
survives Store restart and fresh readmission without persisted authority
promotion.

## Milestone 3: Durable Branch And Commit Graph

### Goal

Create the canonical semantic version DAG and branch-reference model required
for arbitrary historical forks, multi-parent merges, concurrent writers, and
long-running branch retention.

### Hard Problem

An in-memory branch copied from a current head cannot support arbitrary bases,
crash recovery, retained readers, concurrent publication, or offline ancestry.
Conversely, a physical Store DAG cannot decide semantic parenthood or branch
truth.

### Must Ship

- canonical commit identity, ordered parent identity, world basis, authoring
  authority, semantic effect identity, and publication metadata
- branch creation from any admitted retained basis, not only the current head
- atomic compare-and-publish branch-head movement with typed stale-head outcome
- multi-parent commits without flattening parent meaning into one predecessor
- tags or named references with authority and retention semantics distinct from
  mutable branch heads
- branch deletion, archival, protection, and pinning contracts
- branch-local concurrent preparation and narrow publication coordination
- ancestry, common-ancestor, generation, reachability, and retained-base indexes
  whose authoritative versus derived status is explicit
- canonical durable record contracts consumed later by Store integration

### Authority And Derivation

Relational owns semantic commit and branch truth. Store later preserves the
records and atomic publication evidence. Ancestry accelerators are derived and
rebuildable. Query exposes workflows but cannot move heads directly.

### DX Target

Forking names the exact basis and scope. Publication returns `Published`,
`StaleHead`, `PolicyChanged`, `AuthorityChanged`, or `Indeterminate` with typed
recovery rather than a boolean.

### Proof Obligations

- randomized DAG operations agree with an independent reference model
- concurrent disjoint branches progress without a runtime-global lock
- same-branch overlapping publications cannot lose updates
- ancestry and fork cost are sublinear in total world size and carry exact
  structural counters
- forged parent, reordered parent, missing parent, and cross-runtime parent
  attempts fail closed

### Closeout Gate

Earn `SemanticClose` when the runtime-backed DAG and reference model agree under
concurrency. The word durable earns `JoinedClose` only in Milestone 14 after
Store publication and recovery evidence exists.

## Milestone 4: Semantic Change And Diff Model

### Goal

Define canonical semantic change and structured diff artifacts over nodes,
aspects, relationships, definitions, policy, and topology.

### Hard Problem

Raw before/after values cannot distinguish deletion from absence, absence from
unmaterialized data, identity replacement from editing, definition change from
value change, or authoritative change from derived recomputation. A generic
path/value patch would discard exactly the meaning later merge policy needs.

### Must Ship

- canonical change families for node lifecycle, aspect lifecycle and value,
  relationship lifecycle, ordering/topology, definition, invariant, and policy
- explicit unchanged, added, removed, replaced, unknown, unavailable, pruned,
  and not-materialized states
- normalized semantic deltas with stable ordering and exact round-trip laws
- scoped diff planning that declares bases, aspect/relationship selection,
  correspondence requirements, and materialization needs before execution
- separate authoritative diff and derived impact/explanation artifacts
- summary indexes that narrow candidate scope without becoming diff authority
- query-shaped diff projection that preserves the full underlying semantic
  classification

### Authority And Derivation

Owning runtimes emit canonical effects at commit. Relational compares
authoritative worlds. Signal supplies derived impact only. Query shapes the
result. Store persists canonical effects and optional rebuildable acceleration.

### DX Target

A caller can ask "what meaning changed between these worlds in this scope?"
and receive typed node-, aspect-, relation-, definition-, and policy-level
changes with basis and ambiguity intact.

### Proof Obligations

- applying a canonical delta to its admitted basis reconstructs the target
  authoritative world
- inverse deltas restore the source where the operation family is reversible
- direct diff equals accumulated canonical commit effects after normalization
- narrow diff work scales with candidate semantic delta plus declared
  dependency closure, not total graph size
- unknown, pruned, and unmaterialized inputs never masquerade as deletion

### Closeout Gate

Earn `SemanticClose` when diff is a canonical typed semantic artifact and every
later merge input can depend on it without interpreting raw storage or Query
projection shapes.

## Milestone 5: Versioned Definition Worlds

### Goal

Make schema, aspect meaning, node logic, invariants, merge policy, and relevant
Signal definitions versioned semantic inputs that can themselves diverge,
compare, migrate, conflict, and reconcile.

### Hard Problem

Long-running branches can add an aspect, change its type, replace node logic,
alter an invariant, or attach a different merge policy. Merging values under
whichever definitions happen to be current would reinterpret history and make
results depend on execution time.

### Must Ship

- stable identity and semantic versioning for aspect definitions, schemas,
  node definitions, invariants, policy, and merge/convergence contracts
- explicit compatible, migratable, conflicting, unavailable, and unsupported
  definition transitions
- definition-world diff and merge plans that execute before instance data
- migration functions with deterministic inputs, version identity, scope,
  reversibility posture, and failure topology
- exact rules for an aspect added on one branch, added differently on both,
  removed while edited elsewhere, or retyped while populated
- node-logic divergence rules covering equivalent replacement, incompatible
  replacement, dependency changes, and rebuild obligations
- historical interpretation through the definitions admitted at the original
  basis, not ambient current registries
- compatibility matrices for runtime, Store records, offline capsules, and
  mixed-version replicas

### Authority And Derivation

Each canonical definition family stays with its semantic owner. Relational
binds authoritative data to the admitted definition world. Signal owns derived
definition execution. Store persists versioned definitions but never interprets
their semantic compatibility independently.

### DX Target

Definition differences appear as first-class merge input. Users see whether a
value conflict is real, requires migration, or cannot be interpreted until its
definition conflict is resolved.

### Proof Obligations

- historical values retain their original meaning after current definitions
  evolve
- deterministic migrations produce identical canonical outputs across replay,
  Store restore, and offline import
- aspect-add/remove/retype and node-logic divergence matrices cover every
  three-way combination
- a definition conflict prevents dependent fact merge and derived evaluation
- definition lookup and compatibility counters expose depth and breadth

### Closeout Gate

Earn `SemanticClose` only when no merge path can compare or apply dependent
facts under unresolved or ambient definitions.

## Milestone 6: Identity, Lineage, And Correspondence

### Goal

Preserve authoritative node and aspect identity across forks, renames, moves,
copies, splits, joins, replacements, imports, and independently-created but
potentially-corresponding structures.

### Hard Problem

Semantic merge cannot rely on names, content hashes, storage keys, or shape
similarity to decide sameness. Yet strict identity alone cannot express a
deliberate split, join, replacement, or correspondence between structures that
were created independently.

### Must Ship

- authority-preserving identities for node, aspect instance, relationship,
  definition, branch, commit, replica, and resolution session
- explicit lineage operations for fork, rename, move, replace, split, join,
  copy-with-new-identity, import, and retirement
- typed correspondence claims distinct from identity, with exact, admitted,
  ambiguous, advisory, rejected, and unknown posture
- provenance and decision evidence for human- or policy-admitted correspondence
- cardinality-aware one-to-one, one-to-many, many-to-one, and many-to-many
  correspondence plans
- correspondence invalidation when definitions, structure, or policy move
  beyond the admitted basis
- stable lineage queries and derived acceleration indexes

### Authority And Derivation

Identity comes only from the owning authority. Lineage records are
authoritative semantic history. Structural fingerprints and similarity matches
are derived evidence and can never promote themselves into identity. Query
projects correspondence and ambiguity without deciding it.

### DX Target

A merge plan can say "these are the same node," "these nodes descended from
one source," "these may correspond," or "no lawful correspondence is known"
using distinct types and recovery paths.

### Proof Obligations

- rename and move never create new identity accidentally
- copy, split, and join never preserve an identity whose cardinality changed
- ambiguous structural matches remain advisory until admitted by authority
- lineage traversal and correspondence narrowing have named breadth counters
- digest collisions, label reuse, wrong-runtime ids, and stale correspondence
  claims cannot open merge authority

### Closeout Gate

Earn `SemanticClose` when authoritative sameness and admitted correspondence
are sufficient for all later aspect and relationship merge planning without
heuristic identity reconstruction.

## Milestone 7: Authoritative Aspect Merge

### Goal

Implement deterministic, aspect-granular three-way merge for authoritative
state with explicit policy, exact conflict loci, and no whole-entity fallback
for independently meaningful aspects.

### Hard Problem

Entities are aspect-rich. One branch may change geometry while another changes
workflow state; one may remove an aspect while another edits it; both may add
an aspect under evolved definitions. Whole-node conflict is too broad, while a
generic value merge is too weak to preserve domain meaning.

### Must Ship

- base/left/right aspect merge planning bound to the complete semantic-world
  bases and admitted correspondence
- add/add, add/absent, edit/edit, edit/remove, remove/remove, replace/edit, and
  migration-aware case classification
- aspect-family strategies for strict, deterministic three-way,
  join-semilattice, operation-convergent, single-writer, and derived/rebuildable
  posture where each is semantically lawful
- typed exact conflict loci carrying node identity, aspect identity, definition
  identity, field or substructure identity where admitted, and basis evidence
- support for structured scalar, record, sequence, map, set, and domain-native
  aspect semantics without flattening all aspects into one universal value bag
- strategy planning and lowering before merge execution
- canonical merge effect and explanation derived from one plan
- no-op and already-integrated detection with explicit equivalence basis

### Authority And Derivation

Relational owns authoritative aspect merge execution. Domain-declared strategy
definitions contribute policy through admitted contracts but do not bypass
Relational publication. Query authors and inspects; Store persists effects.

### DX Target

Developers declare merge behavior with the aspect definition. Users see a
narrow semantic conflict such as "fastener preload edited differently" rather
than an opaque entity conflict or serialized value pair.

### Proof Obligations

- exhaustive generated three-way case matrices for every admitted aspect
  family
- disjoint aspect edits merge without conflict and without scanning untouched
  aspects
- conflict order and canonical merge output are deterministic across execution
  order, process, restore, and replay
- strategy selection is basis- and version-bound; ambient registry changes
  cannot alter an existing plan
- unsupported custom aspect semantics fail before execution

### Closeout Gate

Earn `SemanticClose` when every authoritative aspect family either has a
versioned lawful merge contract or fails typed before planning, and aspect-level
conflicts preserve all meaning required for later resolution.

## Milestone 8: Relationship, Topology, And Invariant Merge

### Goal

Extend semantic merge across relationships, ordering, topology, referential
integrity, and local or global invariants after identity and aspect meaning are
resolved.

### Hard Problem

Individually valid branch edits can combine into an invalid world: duplicated
ownership, broken references, cycles, impossible geometry, violated capacity,
or cross-branch uniqueness collisions. Relationship meaning is directional and
often depends on merged node identity and aspects.

### Must Ship

- typed relationship and topology change families preserving direction,
  cardinality, ordering, role, and definition identity
- three-way relationship merge after correspondence resolution
- sequence and ordered-neighborhood merge with stable element identity
- referential-integrity and lifecycle handling for deleted, replaced, split,
  joined, or unavailable endpoints
- invariant declarations with explicit scope, dependency contract, severity,
  authority, and evaluation basis
- local incremental validation and explicit global coordination plans
- success, advisory, violation, and indeterminate invariant outcomes
- repair proposal artifacts distinct from authorized resolution commands
- post-merge world validation before authoritative publication

### Authority And Derivation

Relational owns authoritative relationships and invariant-gated publication.
Domain authorities own invariant definitions. Signal may compute derived
evidence but cannot satisfy an authoritative invariant by assertion. Query
exposes plan, impact, and recovery.

### DX Target

The merge plan explains which relationship rings, ordered neighborhoods, or
global scopes need coordination before execution and returns domain-shaped
violations rather than a late generic transaction failure.

### Proof Obligations

- relationship merge matrices cover endpoint deletion, identity split/join,
  concurrent reparenting, ordering edits, and definition drift
- property tests prove published worlds satisfy every admitted hard invariant
- cross-branch global uniqueness and quota scenarios coordinate at the narrowest
  true authority boundary
- relationship traversal and invariant breadth match the declared plan counters
- repair suggestions cannot publish without resolution authority

### Closeout Gate

Earn `SemanticClose` when no authoritative merge can publish a world that
violates an admitted hard invariant and global coordination is explicit rather
than hidden inside aspect execution.

## Milestone 9: Merge Algebra And Convergence Contracts

### Goal

Turn merge and convergence claims into versioned algebraic contracts with
machine-checked laws, compatibility rules, and strategy-specific cost and
failure semantics.

### Hard Problem

Calling a resolver "deterministic" or a type "CRDT" does not prove convergence.
Associativity, commutativity, idempotence, causal delivery assumptions,
invariant closure, and migration compatibility differ by family and can be
silently invalidated by later code changes.

### Must Ship

- a versioned convergence contract for every admitted strategy family
- declared algebraic laws and preconditions rather than one universal trait
- canonical ordering, equality, identity, and normalization contracts
- deterministic resolver sandbox inputs excluding ambient clock, random,
  network, mutable registry, or process-local state
- law-check harnesses for associativity, commutativity, idempotence,
  permutation invariance, replay equivalence, and invariant preservation where
  each law is claimed
- compatibility rules for strategy version changes and branch histories created
  under earlier contracts
- explicit non-convergent, manual-only, single-writer, or unsupported posture
- exact cost counters for normalization, law-required metadata, and merge work

### Authority And Derivation

Domain policy may select from admitted convergence families. The owning runtime
proves and executes the contract. Foundational may carry cross-boundary strategy
identity and verdict vocabulary but not executable policy. Certification owns
independent law oracles.

### DX Target

Strategy declarations make guarantees and limitations visible before use. A
developer cannot opt into `operation-convergent` without also declaring causal,
deduplication, tombstone, migration, and invariant posture.

### Proof Obligations

- generated values and operation histories falsify every claimed law
- resolver outputs are byte-identical under process, platform-supported build,
  input permutation, replay, and Store restore where ordering should not matter
- a mutation to a certified resolver that breaks one law makes CI red
- incompatible strategy evolution rejects or migrates before merge
- negative controls prove the harness detects deliberately non-convergent
  strategies

### Closeout Gate

Earn `SemanticClose` when every automatic merge or convergence claim is backed
by a named executable contract and no unclassified fallback strategy remains.

## Milestone 10: Conflict And Governed Resolution Model

### Goal

Make semantic conflicts and the shared governed-resolution problem model
durable, typed, composable, manually resolvable, and safe against stale bases,
partial decisions, derived-proposal promotion, and authority drift. Merge
conflict is the first mandatory specialization of the shared model.

### Hard Problem

Detecting conflict is not enough. Real users must inspect the exact semantic
problem, choose among base/left/right/custom/domain repair options, defer some
conflicts, collaborate on decisions, and resume later without applying a
resolution to a world that has since changed.

The same lifecycle will later govern foreign-model repairs, physical routing
alternatives, assumption review, and engineering advisories. If this milestone
hard-codes base/left/right as the shared representation or treats Query 9.15
candidate evidence as authority, every later domain will build a second
resolution system.

### Must Ship

- a typed conflict taxonomy for definition, identity, aspect, relationship,
  topology, invariant, policy, strategy, authority, basis, and availability
  conflicts
- a canonical governed-resolution problem contract carrying problem family,
  semantic locus, exact basis, policy, authority requirement, domain payload
  contract, lifecycle, and publication owner without requiring merge vocabulary
- stable conflict identity derived from semantic locus and exact compared
  bases, not display text or list position
- typed alternatives, domain repair proposals, and resolution requirements
  whose derived search, feasibility, completeness, optimality, convergence,
  transformation, and loss evidence remains distinct from resolution authority
- explicit unresolved, proposed, admitted, rejected, superseded, applied, and
  invalidated resolution states
- resolution commands that carry resolver authority, reason, scope, policy,
  expected plan identity, and expected target head
- partial-resolution plans that preserve remaining conflicts without silently
  publishing an incomplete authoritative merge
- replan and carry-forward rules when branch heads, definitions, policy, or
  authority change
- deterministic canonical ordering and grouping for UI, API, audit, and replay
- an admitted specialization boundary through which later domain problems
  reuse the same command and decision lifecycle while retaining their own
  candidate, comparator, repair, invariant, and publication semantics

### Authority And Derivation

The governed-resolution control-plane authority owns problem identity,
alternative/decision progression, supersession, and stale-basis rejection.
Relational owns whether an authoritative merge conflict is resolved
sufficiently to prepare publication. Human and domain decisions enter as
governed commands. Query owns the ergonomic workflow and presentation.
Diagnostics and suggested repairs are not authority.

For a non-merge domain specialization, the owning semantic authority decides
whether the admitted decisions are sufficient to prepare its ordinary
authoritative operation. The shared resolution model owns problem/decision
progression and stale-basis rejection, not domain truth or publication.

### DX Target

A user can resolve one aspect, accept an invariant-aware domain repair for
another, defer a topology conflict, and later resume with a precise report of
which decisions remain valid or became stale.

The same public shape can later inspect a STEP repair set or routing tradeoff
without exposing merge-only base/left/right fields or allowing a candidate
artifact to masquerade as an approved command.

### Proof Obligations

- every conflict kind round-trips without losing typed locus or basis
- stale plan, stale head, changed policy, revoked authority, and changed
  definition invalidate only the decisions whose proof basis moved
- conflict ordering and identity remain stable across restart and projection
- custom resolutions re-enter invariant validation and cannot bypass policy
- no partially resolved plan reaches authoritative publication
- candidate, convergence, transformation, loss, correspondence, and advisory
  artifacts from Query 9.15 remain derived until admitted by a resolution
  command carrying current authority and expected basis
- one hostile non-merge specialization proves the shared problem, alternative,
  command, stale-basis, and invariant-readmission contracts contain no
  merge-specific representation while using the same production lifecycle

### Closeout Gate

Earn `SemanticClose` when manual resolution is a complete governed semantic
transition rather than an out-of-band value override, and when merge conflict
is proven to be one specialization of a reusable lifecycle rather than the
definition of resolution itself.

## Milestone 11: Durable Collaborative Resolution Sessions

### Goal

Wrap governed problem inspection and manual resolution in a framework-owned,
durable, multi-participant session lifecycle with progress, leases, decisions,
cancellation, recovery, and explicit publication authority. Merge sessions are
the first production specialization; later admitted domain sessions reuse the
same control plane.

### Hard Problem

Long-running resolutions outlive requests and processes. Multiple people may
inspect or propose decisions while branches continue moving. Persisting a bag
of chosen values is insufficient to recover authority, staleness, participant
roles, decision provenance, or the plan that those choices were meant to
resolve.

A domain repair or routing session may not involve two branch heads, but it has
the same hard lifecycle problem: its source world, definitions, assumptions,
policy, candidate evidence, target generation, participant authority, and
publication owner can drift independently while the session remains open.

### Must Ship

- session identity, governed problem identity, exact semantic-world and
  specialization-plan basis, participant roles, policy basis, lease state,
  progress frontier, and lifecycle typestates
- separate read/inspect, propose, approve, reject, replan, cancel, abandon, and
  publish capabilities
- durable append-only decision records with actor authority, reason, affected
  conflict identities, and supersession links
- optimistic concurrent proposals with deterministic collision classification
- session checkpoint and bounded journal sufficient for restart
- explicit stale-session, partially-stale, reauthorization-required,
  replanning-required, cancelled, abandoned, and indeterminate outcomes
- framework-owned disposal and retention participation
- Query-facing progress streams with backpressure and lifecycle control
- specialization-neutral checkpoints and journals that retain typed domain
  decisions and evidence references without serializing merge-only value bags

### Authority And Derivation

The governed-resolution control-plane authority owns session workflow state,
not semantic truth. Relational still decides whether a resolved merge plan can
prepare. Store persists session records. Query exposes participant and operator
workflows. A persisted role or past approval does not restore current authority
without readmission.

For non-merge specializations, the owning semantic authority replaces
Relational's merge-sufficiency role for the final preparation decision; Store,
Query, participant readmission, and session authority remain unchanged.

### DX Target

Opening a conflict returns a session handle, not a blocking merge call. The
handle supports inspect, propose, approve, checkpoint, replan, publish, cancel,
and recover with typed next actions.

Opening an admitted repair, routing, or advisory problem returns the same class
of handle while preserving domain-shaped alternatives and outcomes. Callers
never select a different session framework based on problem domain.

### Proof Obligations

- two participants resolving overlapping and disjoint conflict sets converge
  or receive typed collision without lost decisions
- crash at every append/checkpoint boundary resumes the same session state
- authority revocation and policy drift force fresh admission
- abandoned sessions cannot pin history forever without declared retention
  policy
- session memory, journal, participant, and progress counters remain bounded
- merge-conflict and one non-merge domain session share lifecycle, authority,
  staleness, checkpoint, recovery, and retention oracles while keeping their
  problem payloads, invariants, and publication owners distinct
- a Query 9.15 candidate or repair artifact with matching serialized fields
  cannot restore a session, approve a decision, or publish an outcome

### Closeout Gate

Earn `SemanticClose` for the session state machine and reference persistence
model across merge and one hostile non-merge specialization. Earn `JoinedClose`
only after the replacement Store runtime durably recovers sessions, decisions,
and pins without promoting persisted authority.

## Milestone 12: Signal Definition And Derived-World Reconciliation

### Goal

Make Signal's definition world and branch-local derived state reconcile
correctly after authoritative semantic merge without treating derived values as
merge authority.

### Hard Problem

Two branches may have different Signal nodes, dependency graphs, evaluation
logic, scheduling policy, or cached derived results. Merging cached outputs can
preserve stale or definition-incompatible state; discarding everything can make
ordinary merges unbounded and destroy live continuity.

### Must Ship

- versioned Signal node, dependency, scheduling, invalidation, and delivery
  definitions bound into the semantic definition world
- definition diff and compatibility classification before derived evaluation
- branch-local Signal graph identity and lifecycle
- a reconciliation plan derived from authoritative merge effect plus reconciled
  Signal definitions
- exact keep, invalidate, recompute, migrate, suppress, discontinue, and
  unsupported classifications for derived resources
- bounded incremental rebuild with dense fallback when invalidation breadth
  makes sparse tracking more expensive
- subscription continuity decisions for preserved, rebound, reset, terminated,
  or support-only live resources
- derived acceleration checkpoints classified as rebuildable rather than truth

### Authority And Derivation

Signal owns derived definitions, evaluation, and resource lifecycle.
Relational's published merge effect is the authoritative input. Bridge carries
the causal handoff. Query projects continuity and recovery. Store may persist
declared acceleration but cannot make it authoritative.

### DX Target

The merge plan previews which derived nodes and subscriptions remain valid,
which rebuild, and why. A divergent node-logic conflict is resolved at the
definition layer before any output is trusted.

### Proof Obligations

- destroying all post-merge derived state and rebuilding from authority yields
  the same canonical outputs as admitted incremental reconciliation
- divergent node logic, dependency edits, cycles, removed inputs, and policy
  drift fail or reconcile deterministically
- derived state from one branch never leaks into another basis
- rebuild breadth matches the authoritative semantic delta plus declared
  dependency closure
- live resources cannot claim continuity when their definition or basis changed

### Closeout Gate

Earn `SemanticClose` when all derived merge behavior is expressed as
definition reconciliation plus rebuild from authority, never value-level
authority promotion.

## Milestone 13: Cross-Runtime Merge Protocol

### Goal

Define one typed protocol that joins Relational merge preparation, Bridge
causality, Signal reconciliation, Store durability preparation, Query outcome,
and recovery without collapsing their authorities.

### Hard Problem

A semantically valid Relational merge is not a complete runtime transition. The
Store must durably record it, the branch head must publish atomically, Signal
must observe the exact canonical effect, Query must acknowledge honestly, and
recovery must know which phases happened after a crash.

### Must Ship

- a proof-widening protocol such as admitted -> compared -> planned -> resolved
  -> invariant-valid -> prepared -> durable -> authority-published ->
  derivation-reconciled -> acknowledged
- one immutable cross-runtime merge summary derived once at the batch boundary
- lowered per-authority plans that executors consume without re-deciding policy,
  strategy, artifact richness, or coordination scope
- explicit abort, rollback, retry, compensate, replan, and indeterminate
  transitions for every phase where those outcomes are honest
- idempotency identity and duplicate suppression across retries
- move-only phase packets unless a real second observer justifies a clone
- structured boundary envelopes and exact counters at each authority crossing
- a protocol compatibility contract for rolling versions

### Authority And Derivation

The protocol coordinator owns sequencing only. It cannot mint Relational,
Signal, Store, Query, or operator authority. Each phase consumes a proof from
the prior authority and returns a sealed artifact sufficient for the next
phase.

### DX Target

Ordinary users see one merge session and outcome. Advanced users and operators
can inspect the lowered plan, phase progress, cost, durability, derived rebuild,
and recovery state without reaching into runtime internals.

### Proof Obligations

- compile-fail tests make skipped, repeated-illegal, and out-of-order phase
  transitions uncallable
- deterministic schedule exploration covers all legal interleavings with
  concurrent readers and writers
- executors contain no strategy or policy rediscovery
- boundary envelopes reconstruct the operation without querying producers
- abort and retry never duplicate semantic effect or derived publication

### Closeout Gate

Earn `SemanticClose` when the complete protocol executes against deterministic
runtime-backed and fake durable authorities. Production publication remains
unearned until Milestone 14.

## Milestone 14: Crash-Safe Merge Publication And Recovery

### Goal

Bind the cross-runtime protocol to the replacement Store runtime so a merge is
durable, atomically publishable, restart-safe, idempotently recoverable, and
honestly acknowledged.

### Hard Problem

Crashes can occur before durable append, after append but before branch-head
publication, after head publication but before Signal routing, after derived
reconciliation but before acknowledgment, or during recovery itself. No phase
may disappear, duplicate, or leave different runtimes believing different
truth.

### Must Ship

- versioned Store records for merge plan identity, canonical effect, ordered
  parents, branch publication intent, session decisions, definition basis, and
  protocol progress
- WAL/checkpoint integration through the physical Store's real durability path
- atomic or recoverably joined semantic commit and branch-head publication
- exact acknowledgment rule distinguishing failed, rejected, committed,
  indeterminate, and recovered outcomes
- restart classification and idempotent completion or rollback for every crash
  window
- fresh Query, operator, policy, and runtime authority readmission after restart
- checkpoint-plus-bounded-tail reconstruction without full history replay
- corruption, missing-record, version-skew, wrong-tenant, wrong-key, and
  quarantine outcomes
- joined retention pins for active merge and resolution sessions

### Authority And Derivation

Store proves byte survival and physical publication. Relational proves semantic
head truth. Bridge proves causal continuation. Signal proves derived
reconciliation. Query reports the joined outcome. Recovery may reconstruct
proofs from authoritative records but cannot restore obsolete authority tokens.

### DX Target

An indeterminate merge returns a recovery handle that can inspect, wait,
resume, or escalate. It never asks callers to infer success from a timeout or
search logs for a commit id.

### Proof Obligations

- deterministic crash injection before and after every durable and semantic
  transition
- every acknowledged merge survives; every unacknowledged durable merge is
  classified and completed or rejected idempotently
- recovered Query results and Signal outputs equal uninterrupted execution
- corruption and incompatible record versions fail before head publication
- restart work is bounded by checkpoint plus declared tail and exposes exact
  counters

### Closeout Gate

Earn `JoinedClose` only against the production Store boundaries established by
Store runtime Milestones 2, 3, 7, 8, and 10. A mock WAL or alternate local
database cannot close this milestone.

## Milestone 15: Query Semantic-Git Product Surface

### Goal

Expose the complete admitted collaboration model as one coherent Query-facing
daily-driver API for branch, compare, merge, conflict, resolution, publication,
inspection, and recovery.

### Hard Problem

Query must feel like the beginning of the platform without becoming a second
merge engine or a bag of lower-runtime handles. The six concepts formerly
planned as Query 9.3.8 Phases 36 through 41 are necessary, but only as
projections over the real lower-authority model built in Milestones 1 through
14.

### Must Ship

- Query-facing collaboration context over admitted semantic-world basis,
  branch/workspace, definition, policy, authority, and strategy posture
- strategy admission and compatibility projected from lower-authority
  contracts
- one sealed collaboration witness retaining exact basis, authority, policy,
  strategy, and protocol posture
- semantic compare and diff declarations with aspect-, relationship-,
  definition-, topology-, and policy-level result shaping
- branch creation, mutation, merge planning, conflict preview, resolution
  session, publication, and post-merge inspection workflows
- specialization-neutral governed-resolution authoring, inspection, proposal,
  approval, deferral, replan, cancellation, recovery, and publication
  projection over the Milestones 10-14 authorities
- typed readmission classifications including replayable, current, stale,
  rebind-required, authority-mismatch, merge-inspection-required,
  semantic-conflict, unavailable, and indeterminate
- recovery briefs and next-action handles over the real protocol state
- ordinary, checked, proof-visible, helper, grouped, continuation, and
  contribution-composed parity where those public lanes exist
- legal audience-facade routing through `worth-query-decl` and
  `worth-query-host`; certification-only replay through `worth-query-replay`

### Authority And Derivation

Query owns declaration, orchestration DX, result shaping, explanation, and
recovery projection. It lowers to Relational, Signal, Bridge, Store integration,
Foundational, and Proof authorities and may not reinterpret their verdicts.
Candidate/search/convergence/transformation evidence from Query 9.15 may be
projected into this workflow but cannot replace its conflict, decision, session,
publication, or recovery authorities.

### DX Target

A serious domain can implement a semantic-Git workflow without importing lower
runtime internals, minting authority bags, correlating unrelated receipts, or
losing access to advanced plan and recovery detail.

### Proof Obligations

- all equivalent Query lanes converge on the same lower-authority plan,
  witness, conflict, outcome, and recovery identity
- hostile tests prove Query cannot synthesize basis, strategy, merge,
  publication, or recovery authority
- ergonomic helpers are projections over one canonical workflow, not parallel
  execution paths
- documentation, goldens, support matrices, inventories, and compile-fail
  boundaries teach the exact public path
- runtime-backed and Store-backed Query semantic oracles agree
- merge and non-merge resolution workflows use one Query surface and one lower
  control plane while preserving distinct semantic owners and publication paths

### Closeout Gate

Earn `SemanticClose` for runtime-backed public parity and `JoinedClose` for the
durable workflow. This milestone is the new home for the valid product intent
of Query 9.3.8 Phases 36 through 41; it may not depend on those old phases being
implemented first.

## Milestone 16: Offline Branch Capsules And Synchronization

### Goal

Let an admitted user take a bounded semantic world offline, continue working on
a local branch, and later synchronize through quarantine and fresh admission
without treating an exported snapshot as current authority.

### Hard Problem

Offline work crosses trust, time, version, policy, custody, and causality
boundaries. A capsule can be authentic yet stale, complete yet unauthorized,
or physically intact yet semantically uninterpretable after definitions and
policy evolve.

### Must Ship

- immutable capsule manifest over base semantic-world basis, included scope,
  dependency closure, definition world, policy grant, replica identity,
  compatibility window, custody, key, and expiry posture
- snapshot-plus-operation-journal representation with bounded resume
- local offline branch identity distinct from authoritative server branch head
- explicit offline capability grant naming allowed operations, scope,
  consistency, expiry, and whether publication may ever be automatic
- deterministic export, interruption-safe resume, and exact integrity graph
- import quarantine, authenticity/integrity verification, compatibility
  migration, dependency validation, and fresh authority readmission
- duplicate and reordered capsule/journal delivery handling
- synchronization plan showing upload, download, merge, conflicts, policy
  changes, cost, and recovery before mutation
- zero authority restoration from persisted Query or operator handles

### Authority And Derivation

Store owns capsule bytes and transfer mechanics. Relational owns imported
semantic admission and branch integration. Query owns offline workflow DX.
Security authorities own capability and custody. A capsule is a portable
observation plus proposed history, not current authority.

### DX Target

Users explicitly `take_offline`, work through a local branch handle, inspect a
`sync_plan`, and receive synchronized, conflicted, reauthorization-required, or
quarantined outcomes with resumable progress.

### Proof Obligations

- interrupted export/import resumes idempotently
- duplicated, reordered, truncated, corrupted, expired, wrong-tenant,
  wrong-key, and revoked-policy capsules cannot publish
- offline edits made under old definitions remain interpretable or fail with an
  exact migration/conflict requirement
- sync work scales with exchanged semantic delta plus dependency closure
- source, capsule, and freshly admitted replica agree on canonical included
  truth

### Closeout Gate

Earn `DistributedClose` only after Store runtime Milestone 16 provides the real
portable transport and the complete offline edit/import/merge journey passes
against production boundaries.

## Milestone 17: Replica Causality And CRDT Runtime

### Goal

Add the causal substrate required for local-first and operation-convergent
domains while preserving strict semantic merge and invariant-gated modes for
domains that are not lawful CRDTs.

### Hard Problem

Offline replicas create concurrent operations rather than only branch-tip value
differences. Correct convergence needs stable replica and operation identity,
causal context, duplicate suppression, tombstones, stability, and migration
rules. A vector clock alone does not solve semantic conflicts or invariants.

### Must Ship

- stable admitted replica identity and non-reusable operation identity
- causal context/frontier representation with exact happened-before,
  concurrent, duplicate, missing-dependency, and unknown classification
- operation journal contracts with deterministic canonical application
- idempotent deduplication and bounded received-operation indexing
- tombstone and remove semantics explicit per operation-convergent family
- causal stability evidence from declared replica membership or an admitted
  alternative model
- per-aspect and per-relationship opt-in to operation-convergent semantics
- bridges between operation histories and canonical semantic commits
- invariant compatibility classification: preserved, coordination-required,
  repairable, or forbidden under the selected CRDT family
- replica retirement, epoch change, and lost-replica recovery contracts

### Authority And Derivation

The causal runtime orders and deduplicates observations; it does not decide
domain meaning. Relational applies admitted semantic operations. Domain
convergence contracts define lawful algebra. Store persists journals and
frontiers. Query exposes synchronization and conflict posture.

### DX Target

Developers opt a semantic family into a named convergence contract rather than
adding a `crdt: true` flag. Users can see pending dependencies, concurrency,
stability, coordination requirements, and repair actions.

### Proof Obligations

- permutation, duplication, batching, delay, partition, reconnect, and restart
  produce the same canonical result for every admitted convergent family
- negative controls prove strict/manual families do not silently converge
- operation identity survives retry and rejects identity reuse with different
  content
- causal metadata and dedupe growth have explicit boundedness and compaction
  contracts
- invariant-preserving claims are property-checked under randomized concurrent
  histories

### Closeout Gate

Earn `DistributedClose` when convergence is proven under arbitrary admitted
delivery schedules and every non-convergent family retains typed coordination
or conflict rather than fallback last-writer-wins.

## Milestone 18: Selective Replication And Partial Worlds

### Goal

Support replicas that intentionally contain only admitted scopes of a semantic
world while keeping absence, dependency completeness, policy filtering, diff,
merge, and convergence honest.

### Hard Problem

Filtered replicas make absence ambiguous. A missing node may be deleted,
outside scope, redacted, not transferred, pruned, or causally not yet known.
Operations can depend on definitions, relationships, invariants, or remote
facts outside the selected scope.

### Must Ship

- typed replica scope and dependency-closure declarations
- explicit present, absent-by-domain, outside-scope, redacted, pending,
  unavailable, pruned, and unknown materialization states
- closure planning for definitions, identity lineage, relationship endpoints,
  invariant evidence, policy, blobs, and causal dependencies
- incremental scope expansion and contraction with lifecycle-safe cleanup
- anti-entropy protocols over scoped canonical identities and causal frontiers
- filtered operation admission rules preventing hidden dependencies from
  producing authoritative claims
- partial diff and merge outcomes that state exact completeness and ambiguity
- policy-preserving redaction that does not corrupt causal or identity meaning
- subscription and Query semantics for partial worlds

### Authority And Derivation

Source authorities define truth and allowed disclosure. Store transports and
materializes admitted scope. Query declares scope and shapes partial outcomes.
A partial replica cannot infer negative truth from missing materialization.

### DX Target

The scope is explicit at export, synchronization, query, and merge. A caller
can distinguish "not in this replica" from "does not exist" without consulting
transport internals.

### Proof Obligations

- partial and full replicas agree on every fact for which the partial replica
  carries complete admitted dependencies
- hidden or missing dependencies produce exact incomplete outcomes rather than
  false values
- scope expansion converges with a replica created directly at the larger scope
- redaction never leaks through identifiers, diffs, counts, conflicts, causal
  metadata, or diagnostics
- anti-entropy work is bounded by scope delta plus dependency closure

### Closeout Gate

Earn `DistributedClose` when partial worlds remain semantically honest under
offline edits, policy change, definition evolution, merge, and rejoin.

## Milestone 19: Retention, Drainage, Collaboration GC

### Goal

Define how collaboration state leaves memory, compacts on disk, expires,
archives, or is reclaimed without breaking bases, sessions, replicas,
tombstones, audit, or recovery.

### Hard Problem

Branch DAGs, merge plans, conflicts, sessions, offline replicas, operation
journals, tombstones, derived indexes, and diagnostics have different
lifecycles. One global TTL is unsafe; never deleting anything is operationally
unbounded.

### Must Ship

- artifact-family lifecycle declarations for memory residency, drainage,
  durable retention, archival, compaction, and final reclamation
- hard and soft memory budgets, high/low watermarks, tenant/branch/session
  shares, and bounded emergency behavior
- separate leases/pins for readers, branch bases, active merges, resolution
  sessions, continuations, offline grants, replicas, audit, and legal policy
- causal-stability-gated tombstone and dedupe compaction
- safe replica retirement and lost-replica policy before stability advances
- branch/tag deletion and unreachable-history GC with grace and recovery posture
- derived artifact eviction and rebuild without authoritative loss
- checkpoint/journal compaction preserving bounded recovery
- dry-run, explanation, operator approval, cancellation, progress, and recovery
  for broad maintenance
- exact denial when pressure cannot be relieved without violating a guarantee

### Authority And Derivation

Each artifact owner declares semantic survival requirements. Store executes
physical reclaim and tier movement. The integration lifecycle authority joins
pins. Query and operators configure policies through typed scoped plans; they
cannot order unsafe deletion.

### DX Target

Users configure collaboration lifecycle by semantic family and scope, inspect
why data is retained, preview reclaim effect and risk, and receive handles for
long-running maintenance.

### Proof Obligations

- memory remains bounded during active branches, sessions, offline replicas,
  sync, and background maintenance
- reclaim never deletes a still-admitted base, unresolved decision, causal
  dependency, required tombstone, or recovery record
- derived state can be destroyed and rebuilt from retained authority
- long-disconnected and retired-replica scenarios prove stability and reclaim
  policy
- ordinary reads and writes retain latency and concurrency budgets during
  drainage, compaction, and GC

### Closeout Gate

Earn `JoinedClose` and `DistributedClose` only after integration with Store
runtime Milestone 6 residency, Milestone 15 retention, and the real replica
membership/stability model.

## Milestone 20: Extension And Domain Policy SDK

### Goal

Let serious domains add semantic merge, correspondence, invariant, repair,
offline, and convergence behavior through one governed declaration model
without editing runtime internals or escaping platform laws.

### Hard Problem

No platform can anticipate every future CAD, workflow, scientific, financial,
or collaborative structure. An unconstrained custom callback, however, can
introduce nondeterminism, hidden I/O, global scans, authority forgery,
incompatible version changes, or unverifiable convergence claims.

### Must Ship

- declarative domain collaboration package covering definition identity,
  aspect and relationship semantics, merge strategy, invariant contracts,
  correspondence rules, repair proposals, offline participation, and
  convergence posture
- governed-resolution specialization declaring problem/candidate payloads,
  authority requirements, stale-basis dependencies, invariant readmission, and
  publication owner while reusing the Milestones 10-11 lifecycle
- object-style definitions for whole semantic shape and builders only for real
  proof progression
- sealed registration and compatibility admission
- deterministic execution environment with explicit external observations and
  no ambient clock, random, network, filesystem, or mutable global registry
- declared locality, read/write/conflict footprint, complexity, allocation,
  and diagnostic contracts
- version evolution and migration for installed domain packages
- law-test kit, generated case matrices, hostile fixtures, and compile-fail
  authority tests
- capability discovery, support reports, explanation, and unavailable posture
- containment so one domain package cannot inspect or mutate another domain's
  unauthorized state

### Authority And Derivation

Domain packages declare meaning and policy. Platform authorities admit, lower,
execute, and publish. Certification proves declared laws. A plugin callback or
host closure is never direct merge authority.

### DX Target

A domain author defines collaboration semantics beside the canonical aspect or
relationship definition, runs a shipped law/case harness locally, and installs
one package into Query rather than coordinating several lower-runtime
registries.

### Proof Obligations

- reference packages for at least one geometric, one workflow, and one
  operation-convergent domain
- packages cannot bypass authority, invariant, publication, retention, or
  diagnostic policy
- packages cannot install a parallel decision/session state machine or promote
  Query 9.15 candidate, search, convergence, transformation, or loss evidence
  into resolution authority
- missing declarations fail at installation rather than during first merge
- package upgrade matrices cover old branches, offline capsules, active
  sessions, and mixed-version replicas
- declared cost counters are exact under scale fixtures

### Closeout Gate

Earn all applicable closure classes only when third-party domain behavior uses
the same canonical pipeline and proof suites as built-in behavior, with no
privileged internal escape path.

## Milestone 21: Collaboration Inspection, Replay, And Audit

### Goal

Make every collaboration decision reconstructable, explainable, queryable, and
operationally recoverable without putting replay or forensic richness on the
ordinary hot path.

### Hard Problem

Semantic collaboration produces decisions across definitions, identities,
strategies, conflicts, humans, authorities, storage, derived evaluation, and
replicas. Logs cannot prove why a result occurred, while materializing a full
forensic tree on every operation would destroy performance.

### Must Ship

- one canonical decision log derived from operation and protocol envelopes
- O(1) decision identity lookup and incremental summary indexes
- inspection surfaces for bases, ancestry, definitions, correspondence,
  changes, merge plans, conflicts, resolutions, invariant verdicts, protocol
  phases, durability, derived reconciliation, causality, and retention
- exact separation of operational envelope, policy-selected diagnostics, and
  reconstructive replay
- deterministic replay and independent verification from checkpoint plus
  bounded journal
- audit queries by actor, authority, scope, branch, commit, conflict,
  resolution, replica, policy, and time basis
- redaction and disclosure controls that preserve audit integrity
- operator and AI-readable typed evidence bundles
- support-grade recovery when rich evidence is unavailable without fabricating
  stronger truth

### Authority And Derivation

Inspection and audit are projections over canonical artifacts. They cannot
change semantic truth or authorize recovery. Replay is certification-only;
ordinary recovery uses the dedicated joined-runtime contracts from Milestone
14.

### DX Target

A developer can ask "why did this aspect win, which invariant blocked the
alternative, who resolved it, what was durable at the crash, and what must
happen next?" through one typed Query/operator surface.

### Proof Obligations

- decision log alone reconstructs every authority-path decision
- minimal diagnostics and full diagnostics produce identical operational truth
- replayed canonical outputs and protocol outcomes equal original execution
- audit indexes can be destroyed and rebuilt from retained authority
- redaction removes protected content without breaking integrity or leaking
  through explanation metadata

### Closeout Gate

Earn applicable closure classes when operational, support, forensic, replay,
and audit lanes are explicitly distinct yet converge on one canonical history.

## Milestone 22: Hostile Universal Certification

### Goal

Certify the complete semantic-Git system under adversarial scale, concurrency,
definition evolution, human resolution, offline partitions, causal delivery,
partial replication, retention pressure, and crash recovery.

### Hard Problem

Most collaboration bugs exist at intersections: a definition changes while an
offline replica edits a removed aspect; a manual resolution races head movement
and policy revocation; a tombstone is reclaimed while a partial replica is
still causally behind; Signal rebuild and acknowledgment straddle a crash.
Component tests cannot prove the joined system.

### Must Ship

- an independent executable semantic reference model
- deterministic schedule exploration with shrinking for concurrent runtime,
  session, Store, Bridge, Signal, Query, and replica actors
- formal or model-checked protocols for branch-head publication, session
  decision progression, merge publication, replica causality, and stability-
  gated reclamation
- generated state-machine, algebraic-law, metamorphic, parity, compile-fail,
  corruption, version-skew, and resource-budget suites
- production-boundary execution of all three roadmap-spanning hostile scenarios
- reproducible transcripts, seeds, counter evidence, and offline-verifier
  bundles
- sabotage tests for forged authority, semantic hash collision, strategy drift,
  omitted dependency, stale decision, reordered operation, early tombstone GC,
  partial publication, and diagnostic-path authority leakage
- a capability matrix that links every public claim to exact proof evidence

### Authority And Derivation

Certification observes and falsifies production authorities through legal
facades. It does not replace production paths with a test-only runtime. The
reference model is an oracle, not runtime authority.

### DX Target

One command runs the bounded presubmit courtroom; explicit heavier commands run
schedule exploration, long-duration convergence, formal models, and
greater-than-memory Store trials. Failures shrink to a replayable semantic and
interleaving transcript.

### Proof Obligations

- every advertised capability, strategy, operating mode, and closure class maps
  to an executable proof
- negative controls demonstrate that every oracle detects a seeded defect
- the same hostile transcripts run runtime-backed and Store-backed where both
  modes are supported
- no test-only authority, alternate persistence path, or host-local assembly is
  required to pass
- complexity and resource counters remain within named contracts throughout
  the scenarios below

### Closeout Gate

The roadmap closes only when all three closure classes are earned for their
advertised surfaces, all hostile scenarios pass through production boundaries,
and no remaining collaboration claim relies on convention, unowned glue, or
uncertified strategy behavior.

## Roadmap-Spanning Hostile Scenarios

### Scenario A: Century-Scale Semantic Monorepo Merge

- two branches fork from a multi-million-node, aspect-rich world and evolve
  through the equivalent of years of commits
- both branches add and remove aspect definitions, migrate populated aspects,
  replace node logic, change invariants and merge policies, split and join
  identities, restructure relationships, and modify large ordered topologies
- ten readers remain pinned across historical bases while four writers per
  branch submit overlapping and disjoint work and global invariants force only
  narrow real coordination
- a multi-user resolution session resolves thousands of aspect-, identity-,
  topology-, definition-, and invariant-level conflicts while heads continue
  moving and selected authorities are revoked
- crashes occur at every Store, Relational, Bridge, Signal, Query, and session
  phase; memory pressure concurrently drains and rehydrates state
- the final published world must equal the independent semantic oracle,
  preserve every acknowledged decision, classify every stale decision, rebuild
  all derived state, and keep work proportional to semantic delta and dependency
  closure rather than total history

This scenario is mandatory from Milestone 3 onward and must close through the
joined Store runtime in Milestones 14, 19, and 22.

### Scenario B: Offline Planetary Partition And CRDT Siege

- hundreds of replicas take overlapping and disjoint partial worlds offline
  across several admitted software and definition versions
- replicas perform millions of strict, three-way, semilattice, and operation-
  convergent edits while identities split/join, policies expire, keys rotate,
  scopes change, and some replicas are lost or retired
- synchronization duplicates, drops, reorders, batches, corrupts, and delays
  operations and capsules; replicas repeatedly crash during import and merge
- strict and invariant-sensitive families must produce typed conflicts or
  coordination requirements while admitted CRDT families converge regardless
  of delivery order
- tombstone, dedupe, journal, and history compaction run under hard memory and
  disk budgets without advancing stability past any admitted lagging replica
- after quarantine, migration, reauthorization, anti-entropy, resolution, and
  repair, every full replica must converge byte-identically on canonical truth
  and every partial replica must agree on its complete admitted projection

This scenario is mandatory for Milestones 16 through 22.

### Scenario C: Divergent Logic, Partial Knowledge, And Disaster Recovery

- two long-running branches independently introduce similarly named aspects
  with different meaning, replace derived node logic with incompatible
  dependency graphs, change global invariants, and reorganize corresponding
  topology through different identity split/join histories
- a partial replica contains only one affected region, an offline user edits
  under the older definition world, and a live user resolves a related conflict
  while retention and compaction attempt to reclaim old bases
- a merge is durably appended, the primary crashes before semantic head
  publication, a replica is promoted under mixed-version conditions, and Signal
  derived checkpoints contain both valid and stale acceleration
- recovery must quarantine incompatible artifacts, reconstruct the exact
  protocol phase, preserve required history and tombstones, freshly admit all
  authority, complete or reject publication idempotently, reconcile definitions
  before facts, and rebuild derived state from canonical authority
- Query inspection must explain the complete outcome without using diagnostic
  artifacts as authority or leaking data excluded from the partial replica

This scenario is mandatory for Milestones 5, 6, and 11 through 22, with full
production closure in Milestone 22.

## Critical Path And Parallel Work

The strict semantic dependency path is:

```text
1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9 -> 10
  -> 11 -> 12 -> 13 -> 14 -> 15
  -> 16 -> 17 -> 18 -> 19 -> 20 -> 21 -> 22
```

The numbering is intentionally conservative. Some implementation can proceed
in parallel only after the shared prerequisites below are fixed:

- Milestones 3 and 4 may overlap after Milestone 2 freezes basis identity, but
  diff cannot close until commit effects and ancestry are canonical.
- Definition-world implementation in Milestone 5 and identity/correspondence
  research in Milestone 6 may overlap, but merge execution must preserve the
  canonical order defined above.
- Aspect merge law harnesses for Milestone 9 should begin during Milestone 7;
  Milestone 7 cannot claim automatic convergence it has not yet certified.
- Session UI and Query ergonomics may prototype against typed fakes after
  Milestone 10, but Milestone 11 and 15 cannot close against those fakes.
- Store record/codec design may begin with Milestones 2 through 5, but durable
  publication must use the production integration spine and cannot fork an
  alternate persistence path.
- Signal definition reconciliation may begin after Milestone 5, but full
  reconciliation consumes the canonical authoritative merge effect from
  Milestones 7 and 8.
- Offline capsule format work may begin once Milestones 2 through 6 define
  world, history, change, definition, and identity truth; synchronization may
  not close before Milestone 14's durable publication.
- CRDT law research may proceed beside Milestone 9, but replica runtime work
  consumes Milestone 16's identity, journal, quarantine, and synchronization
  contracts.
- Retention policy is designed at artifact birth throughout all milestones.
  Milestone 19 closes the joined policy only after all artifact families exist.
- Extension-kit design starts with the first built-in strategy but cannot close
  until offline, partial-replica, lifecycle, and operational obligations are
  known.
- Certification infrastructure begins in Milestone 1 and expands continuously;
  Milestone 22 remains the final integrated gate.
- Query 9.15 candidate-search, convergence, transformation, loss, and
  single-basis proposal evidence may proceed before Milestone 10. Durable
  conflict identity, participant decisions, approval/deferral, carry-forward,
  session persistence, and resolution recovery may not.
- STEP parsing/normalization/healing kernels and physical resolver/solver work
  may proceed as derived single-basis computation. Authoritative imported
  identity, repair publication, branch-aware correspondence, and durable
  advisory/manual resolution consume Milestones 2-11 according to the authority
  they require.

## Completion Standard

This roadmap is complete only when WORTH can honestly say:

- a semantic world is identified by complete proof-bearing meaning, not a raw
  branch label or value snapshot
- long-running branches preserve stable identity, definition history,
  relationships, policy, and invariants
- diff is semantic, structured, scoped, and delta-bounded
- definitions reconcile before facts and derived state never becomes merge
  authority
- aspect-, relationship-, topology-, and invariant-level merge is deterministic
  or produces exact typed conflict
- manual resolution is governed, durable, collaborative, replayable, and safe
  against stale bases and authority drift
- merge conflicts and later foreign-model repair, physical routing, assumption,
  and engineering-advisory problems reuse one governed-resolution lifecycle
  without collapsing their domain semantics or publication authorities
- one cross-runtime protocol joins semantic preparation, durable publication,
  branch-head truth, derived reconciliation, acknowledgment, and recovery
- Query is the ordinary semantic-Git facade without becoming a second truth or
  merge engine
- offline work crosses trust boundaries through capsules, quarantine, causal
  identity, migration, and fresh readmission
- CRDT behavior is opt-in, law-proven, invariant-honest, and bounded
- partial replicas never confuse missing materialization with negative truth
- retention, drainage, tombstone compaction, and GC cannot outrun live semantic
  or causal obligations
- domain extensions use one deterministic governed SDK and the same production
  path as built-ins
- every decision can be inspected and audited while replay and forensic cost
  remain off the ordinary path
- all three hostile scenarios pass with independent oracles, exact counters,
  production boundaries, and reproducible failure transcripts

Only then has WORTH earned the claim that its branching and merging model is
semantic Git rather than versioned storage with merge-shaped APIs.
