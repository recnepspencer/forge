# Forge Store Vision

## Thesis

`forge-relational` defines what truth is. `forge-store` makes it survive.

`forge-store` is a durable storage engine for graph-native truth runtimes. It
persists canonical commit envelopes, immutable snapshots, branch version graphs,
lineage, schema evolution boundaries, and retention-aware history so that
everything `forge-relational` guarantees in-memory — transactional correctness,
deterministic replay, branchable version history, structured CDC, and
identity-safe lineage — extends to durable, crash-recoverable, long-lived
storage.

It is not a general-purpose database. It is a semantics-preserving storage
engine beneath a database-grade truth runtime. `forge-relational` owns
semantics. `forge-store` owns survival.

The runtime is intended for systems where authoritative truth must outlive
process boundaries, survive crashes, span deployments, replicate across nodes,
and retain retrievable history under explicit policy — without collapsing into
a generic key-value store that discards the structural contracts the truth
runtime provides.

## What This Storage Engine Is For

`forge-store` exists for product surfaces where truth must be durable with the
same structural guarantees it has in memory.

It is meant to support:

- geometry kernels that need topology truth, rebuild history, and branch-local
  experiments to survive process restarts and session boundaries without losing
  identity, lineage, or version structure
- chip-design and simulation systems that need large version graphs, long
  analysis histories, and certification-grade replay from durable canonical
  artifacts
- AI systems that need persistent branch exploration, durable speculative
  workspaces, and retrievable decision history across agent sessions
- web and data platforms that need durable truth with real-time CDC, crash
  recovery, point-in-time reads, and multi-tenant branch isolation that
  survives deployments
- collaborative editing systems that need offline-capable branch persistence,
  durable merge history, and resumable sync from stored cursors
- workflow, node-editor, and visual-editor platforms that need transactional
  graph persistence with rollback, audit history, and version-navigable state
- incremental compiler and IR systems that need durable query caches, replayable
  state graphs, and version-navigable truth across build sessions

The technical thesis is the same across all of them:

- truth must survive crashes without corruption
- history must be durable, not accidental in-memory retention
- branches must persist cheaply, not as full copies
- replay must be possible from stored canonical artifacts
- retention and reclamation must be explicit policy
- durability must not weaken the semantic contracts of the truth runtime

## Why This Storage Engine Is Different

These are not optional add-ons. They are the capabilities that make
`forge-store` strategically different from ordinary persistence layers:

- commit-envelope-native storage
- structural delta storage instead of full-state snapshots
- branch-local delta layering with zero-copy base sharing
- version-graph-native persistence
- lineage graph persistence and historical identity resolution
- aspect-aware columnar storage organization
- schema evolution boundary persistence
- retention-policy-driven compaction and reclamation
- write-ahead logging for crash recovery of in-flight transactions
- multi-resolution snapshot materialization for point-in-time reads
- CDC cursor persistence for durable subscriber resume
- storage backend abstraction with pluggable implementations
- deterministic recovery with multiple recovery modes
- replication-ready immutable artifact publishing
- content-addressed structural blocks for cross-branch deduplication
- structural fingerprint storage for cross-branch correspondence
- persistent correspondence indexes for identity evolution tracking
- read-amplification-aware delta stack management
- basis-anchored cached analysis artifacts with accuracy classification
- hot/cold branch lifecycle tiering
- region-aware locality clustering for domain-specific performance
- simulation and analysis checkpoint lanes
- deterministic bulk-ingest and bulk-transform paths
- durable working set intelligence for adaptive performance
- edge-first and local-first replication primitives
- deterministic import/export capsules
- cross-artifact digest graphs for integrity verification
- merge-assistance durable artifacts
- store-native admission control and budget contracts
- diagnostics artifact persistence for audit and certification

If these are treated as "nice to have later," the storage engine becomes a
generic key-value store that discards the structural semantics
`forge-relational` was designed to preserve.

## Mission

`forge-store` exists to make truth durable without making it dumber.

It must answer these questions as native storage responsibilities:

- How are canonical commit envelopes stored so replay is mechanically exact?
- How are branches stored so creation is near-free and storage scales with
  delta, not full state?
- How does the version graph persist so branch ancestry, merge history, and
  parent ordering survive restarts?
- How does the lineage graph persist so identity evolution survives across
  sessions and deployments?
- How do in-flight transactions survive crashes through write-ahead logging?
- How are immutable snapshots materialized for point-in-time reads, parallel
  analysis, and replication?
- How does retention policy translate to physical compaction and space reclaim?
- How do CDC cursors persist so subscribers resume without re-reading committed
  history?
- How does schema evolution history persist so recovery and reconciliation
  know what schema was active at each commit boundary?
- How does the storage engine remain pluggable so the same truth semantics
  can be backed by different physical stores?

## Architectural Model

### Runtime stack

| Layer | Responsibility | Owns |
| --- | --- | --- |
| `forge-relational` | Truth-state semantics | identity, transactions, MVCC, diffs, CDC, lineage, schema, integrity |
| `forge-store` | Durable storage engine | commit persistence, snapshot storage, WAL, compaction, recovery, backends |
| `forge-signal` | Derived-computation runtime | dependency DAG, invalidation, recomputation, scheduling, convergence |
| Bridge / integration | Decoupled coordination | patch-to-invalidation, aspect mapping, snapshot evaluation |
| `forge-server` | Network delivery | subscriptions, cursors, delivery classes, basis negotiation, HTTP surface |

### Ownership boundary

`forge-store` owns:

- physical persistence of canonical commit envelopes
- write-ahead log management and crash recovery
- snapshot materialization and point-in-time storage
- branch delta storage and base sharing
- version graph persistence
- lineage graph storage
- CDC cursor and subscriber checkpoint persistence
- schema evolution boundary storage
- retention-driven compaction and physical space reclaim
- storage backend abstraction and pluggable implementations
- storage-level diagnostics and integrity verification
- replication-ready artifact publishing

`forge-store` does not own:

- truth semantics, transaction semantics, or identity semantics
- MVCC consistency rules or snapshot isolation behavior
- schema validation or integrity enforcement logic
- signal evaluation, dependency tracking, or reactive scheduling
- domain-specific meaning of stored entities or relations
- application-level query planning or optimization
- sync protocol delivery or subscription management

### Structural rule

`forge-store` persists exactly the canonical artifacts that `forge-relational`
produces. It does not invent its own truth representation, define its own
identity model, or add semantic behavior beyond faithful storage, recovery,
and lifecycle management of what the truth runtime commits.

Physical layouts, indexes, and derived storage structures may exist for
performance, but any such derived structure must be reconstructible from
canonical authoritative artifacts. Authority must never migrate from commit
envelopes into physical layout or acceleration structures.

### Durable artifact classification

Every durable artifact in `forge-store` must be classified into exactly one of
three categories:

**Authoritative artifacts** — the semantic truth that must survive and from
which everything else can be reconstructed:

- canonical commit envelopes
- version DAG records (branches, parents, merge points, branch heads)
- lineage events (replace, split, merge-identity)
- schema evolution boundary artifacts
- CDC cursor checkpoints (durable position contracts)

**Derived durable artifacts** — stored for performance and bounded reads, but
reconstructible from authoritative artifacts if lost:

- materialized snapshots
- compaction products
- physical indexes and aspect-local read accelerators
- structural fingerprint tables
- any physical layout optimization

**Ephemeral artifacts** — in-memory only, not persisted, discarded freely:

- runtime caches
- temporary merge working state
- transient read plans
- in-progress WAL entries prior to commit

If a new durable artifact is introduced without explicit classification, it is
a design defect.

## Principles

1. Storage persists canonical truth artifacts, not transient memory layout.
2. The stored representation must be sufficient for deterministic replay without
   the original process state.
3. Branch storage must scale with delta, not with base state size.
4. Retention and reclamation are explicit product-visible policies, not hidden
   garbage collection.
5. Recovery must reconstruct exactly the committed truth state, not an
   approximation.
6. Schema evolution boundaries must be durable so recovery and reconciliation
   know the schema at every commit.
7. CDC cursors are durable storage artifacts, not ephemeral runtime state.
   The store persists cursor and checkpoint artifacts transactionally; cursor
   meaning, basis negotiation, and delivery semantics remain above the store.
8. The storage backend is pluggable. A backend may vary physical layout and
   performance characteristics; it may not vary canonical artifact meaning,
   recovery conclusions, or retention semantics.
9. Canonical commit envelopes are the semantic durability authority. Materialized
   snapshots are derived artifacts for bounded reads, replication acceleration,
   and recovery acceleration.
10. Storage-level diagnostics are a first-class contract, not debugging
    afterthoughts.
11. Write-ahead logging is the crash recovery mechanism; commit persistence is
    the durability mechanism. These are distinct.
12. Compaction must be semantically safe — it must not destroy information that
    retention policy says should survive.
13. Every authoritative durable artifact must be self-identifying and verifiable.
    Storage integrity must be provable through content hashes, digests, or
    check chains over authoritative artifacts.
14. Durable artifact identity must be stable, unique, and sufficient for
    replay, recovery, and replication references.

## Foundational Decisions

These are locked architectural decisions:

- commit envelopes are the atomic unit of durable persistence
- structural deltas are the physical storage model for branch-local state
- the version graph is stored explicitly, not reconstructed from commit metadata
- lineage events are stored as first-class persistent artifacts
- write-ahead log uses append-only sequential writes for crash recovery
- committed snapshots are immutable once persisted
- compaction operates on retention policy, not on storage pressure alone
- storage backends implement a trait-based abstraction
- the embedded backend is the default and must be production-grade
- recovery is always from canonical commit artifacts, never from WAL alone
- aspect-aware storage layout is a first-class physical design decision
- CDC cursor persistence uses the same transactional boundary as truth commits
- no hidden mutation during reads
- no storage-specific semantic behavior

## Capability Pillars

### Commit Persistence Architecture

#### Commit envelope storage

Technical role:
Every committed transaction in `forge-relational` produces a canonical commit
envelope. `forge-store` must persist these envelopes as the atomic unit of
durable truth.

What this enables:

- deterministic replay from stored canonical artifacts
- crash recovery that reconstructs exactly the committed state
- audit and certification from mechanically complete commit records
- replication by shipping commit envelopes to downstream stores

#### Structural delta storage

Technical role:
Branch state must be stored as deltas against a base, not as full copies of
the graph at every commit.

This is one of the hardest engineering surfaces in the store. The physical
delta model must resolve: delta granularity (entity, aspect, relation edge,
commit fragment), read amplification through long delta stacks, when
rebasing or compaction rewrites delta layers, how branch-local compaction
preserves cross-branch sharing, whether deltas are content-addressed or
commit-addressed or hybrid, and how schema-boundary deltas interact with
physical decoding. Delta layering is not just a storage optimization — it is
a core physical contract with read-amplification and compaction consequences.

What this enables:

- near-free branch creation (a branch is a pointer plus zero deltas)
- storage proportional to actual change, not total state size
- efficient version comparison by reading only the delta layers
- practical history retention even for large truth graphs

#### Aspect-aware columnar layout

Technical role:
Storage should organize entity and relation data by aspect, not as monolithic
row blobs, so aspect-scoped reads are physically efficient.

What this enables:

- aspect-scoped queries read only the aspects they need
- CDC narrowing can operate at the physical storage level
- bridge-grade aspect mapping benefits from storage-aligned reads
- partial entity reads for large graphs with many aspects

### Recovery Architecture

#### Write-ahead log

Technical role:
In-flight transactions must be protected against crashes by appending
operations to a sequential write-ahead log before modifying primary storage.

What this enables:

- crash recovery without data loss for committed transactions
- bounded recovery time proportional to WAL length since last checkpoint
- transactional guarantees that survive process failures
- clear separation between "will be committed" and "is committed"

#### Deterministic recovery

Technical role:
Recovery must reconstruct the exact committed truth state from stored canonical
artifacts. The recovered state must be observationally identical to the
pre-crash state for all committed truth.

The store must distinguish multiple recovery modes:

- **crash recovery**: replay WAL since last checkpoint, restore to last
  committed state. Must be fast and automatic on startup.
- **store rebuild**: reconstruct the entire store from authoritative commit
  envelopes. Used when derived artifacts are corrupted or lost.
- **integrity-audit rebuild**: recompute and verify all derived artifacts
  against authoritative artifacts without modifying the store. Used for
  certification and periodic integrity audits.
- **replication bootstrap**: materialize a new store from a shipped set of
  canonical artifacts. Used for edge deployment and disaster recovery.
- **snapshot-based fast restore**: restore from a materialized snapshot plus
  subsequent commit envelopes. Used when full rebuild from genesis is too
  expensive.

These are different modes with different artifact paths and different
performance contracts.

What this enables:

- trustworthy restart for long-lived systems
- certification that stored truth is correct after recovery
- replication targets that can verify recovery parity
- crash recovery as a testable, certifiable contract
- clear operational playbook for corruption, audit, and deployment scenarios

#### Checkpoint and compaction

Technical role:
The WAL must be periodically checkpointed to bound recovery time. Compaction
must merge historical commit layers into denser representations under
retention policy control.

What this enables:

- bounded recovery time at restart
- controlled storage growth
- explicit tradeoff between history depth and storage cost
- retention-policy-driven decisions about what history survives compaction

### Version Graph Architecture

#### Persistent version DAG

Technical role:
The version graph — branches, commits, parent relationships, merge points,
branch heads, and ancestry — must be stored explicitly as a navigable
persistent structure.

What this enables:

- branch creation, navigation, and comparison across restarts
- merge base computation from stored ancestry
- historical version navigation and time-travel reads
- branch lifecycle management (archive, delete, compact) under policy

#### Branch delta layering

Technical role:
Branch state is stored as a stack of delta layers over a shared base. Reading
branch state resolves through the delta stack to the base.

What this enables:

- branches that share most of their storage with the base branch
- independent branch compaction without affecting other branches
- efficient branch comparison by diffing delta stacks
- branch deletion that reclaims only the delta layers

#### Lineage graph persistence

Technical role:
Identity evolution events (replace, split, merge-like) must persist as
navigable graph structure, not loose event metadata.

What this enables:

- historical identity resolution across restarts
- correspondence and merge tooling that operates on stored lineage
- audit and certification of identity evolution
- branch-aware lineage queries against stored history

### Snapshot and History Architecture

#### Snapshot materialization

Technical role:
Immutable committed snapshots must be materializable as self-contained
stored artifacts that can be read without resolving through the full commit
history.

Snapshots are derived durable artifacts, not authoritative truth. Canonical
commit envelopes remain the semantic durability authority. Snapshots exist for
bounded reads, replication acceleration, and recovery acceleration. If a
snapshot is lost or corrupted, it must be reconstructible from commit
envelopes. Replication contracts may promote snapshots to a stronger delivery
guarantee, but the authority relationship does not change within the store.

What this enables:

- point-in-time reads with bounded cost
- parallel analysis against pinned snapshots
- replication acceleration by shipping materialized snapshots
- snapshot-backed signal evaluation from stored state

#### Retention-aware history

Technical role:
History retention must be an explicit policy rather than accidental storage
accumulation. The store must know what history to keep, what to compact,
and what to reclaim.

What this enables:

- controlled storage growth in long-lived systems
- explicit tradeoffs between auditability and cost
- retention windows for compliance (keep 90 days of full history)
- hot/warm/cold tiering of historical truth

#### CDC cursor persistence

Technical role:
Subscriber positions in the CDC stream must persist as durable artifacts
so subscribers can resume from exactly where they left off.

What this enables:

- subscriber recovery after crashes or disconnection
- durable integration pipelines that never lose their place
- protocol-level resume semantics for `forge-server`
- exactly-once delivery semantics through cursor-based acknowledgment

### Live Query Architecture

#### Read-to-live-view as a native primitive

Technical role:
"Read current state and stay synced" must be a first-class store primitive,
not an application-level pattern assembled from separate read and subscribe
operations. The store must be organized so that the most common query shapes
— current collection, entity detail, filtered view, aggregate — are
naturally subscribable with minimal overhead.

This means:

- a current-state read must be cheaply promotable to a durable live
  subscription backed by a CDC cursor, without requiring the consumer to
  re-query or re-fetch
- the store's physical layout must support efficient CDC narrowing for
  common read shapes: entity-scoped, aspect-scoped, collection-scoped,
  and subgraph-scoped change detection
- snapshot materialization and CDC cursor persistence together form the
  substrate for live queries: the snapshot provides the initial state,
  the cursor provides the advancement stream, and the store ensures that
  advancing from snapshot to HEAD is bounded and efficient
- ordinary collection and detail queries must be cheap to subscribe to —
  live subscriptions are not a premium feature for complex use cases, they
  are the default for how consumers interact with truth

The store does not own subscription evaluation or delivery (that is
`forge-server` and the signal graph). But the store must be physically
organized so that the narrowing, cursor advancement, and bounded-resume
operations that live queries require are efficient at the storage level.
If the store makes these operations expensive, no amount of protocol
cleverness above it can make live queries feel natural.

What this enables:

- CRUD applications where every read is a live view by default, with no
  additional code or infrastructure beyond the initial query
- collaborative surfaces where multiple users see the same collection
  update without manual invalidation or polling
- dashboard and monitoring views where current-state reads automatically
  receive incremental patches as truth changes
- the "read once, stay synced" developer experience that makes Forge feel
  simpler than traditional request/response architectures
- cheap ordinary subscriptions for common collection/detail queries that
  do not require the developer to think about CDC, cursors, or
  subscription lifecycle

This is the store's most important contribution to developer experience.
If reads are cheap but subscribing is expensive or awkward, the system
collapses back to traditional fetch-on-demand patterns and the entire
reactive architecture above the store loses its value.

### Schema Evolution Architecture

#### Schema boundary persistence

Technical role:
Every schema evolution boundary — where the declared schema changed between
commits — must be stored explicitly so recovery and reconciliation know
what schema was active at each point in history.

What this enables:

- recovery that applies the correct schema at each replay step
- reconciliation that can classify schema divergence across branches
- historical reads that interpret stored truth under the schema active at
  that commit
- migration tooling that knows the exact schema history

### Backend Architecture

#### Pluggable storage backends

Technical role:
The storage engine must define a trait-based backend abstraction so different
physical stores can implement the same durable contract.

What this enables:

- embedded storage for single-process deployments (default)
- SQLite backend for lightweight persistence
- RocksDB/sled backend for high-throughput persistent workloads
- custom backends for cloud-native or distributed deployments
- in-memory backend for testing

#### Embedded storage backend

Technical role:
The default backend must be a production-grade embedded storage engine
that requires no external dependencies, no separate process, and no
configuration beyond a file path.

What this enables:

- zero-dependency persistence for desktop, CLI, and embedded applications
- single-binary deployment for the full Forge stack
- development and testing without infrastructure setup
- the "just works" experience for persistence

### Replication Architecture

#### Immutable artifact publishing

Technical role:
Committed snapshots and commit envelopes must be publishable as immutable
artifacts that downstream stores or subscribers can consume.

"Replication-ready" means the artifacts are suitable building blocks for
replication. It does not mean `forge-store` alone solves replication
semantics. Consistency models, ordering guarantees, idempotence, conflict
semantics, backpressure, partial failure, and trust verification for
replication belong at the protocol/coordination layer above the store.

What this enables:

- store-to-store replication by shipping canonical artifacts
- edge deployment where local stores sync with a central store
- offline-first architectures where local persistence syncs on reconnect
- backup and disaster recovery from published artifacts

### Observability Architecture

#### Storage diagnostics

Technical role:
The storage engine must expose storage health, size, fragmentation,
compaction state, WAL size, retention usage, and backend-specific metrics
as first-class diagnostic surfaces.

What this enables:

- operational visibility into storage behavior
- capacity planning and growth monitoring
- compaction tuning and retention policy adjustment
- performance diagnosis when storage is the bottleneck

#### Integrity verification

Technical role:
The storage engine must support verification that stored truth matches
the canonical commit artifacts and that no silent corruption has occurred.

What this enables:

- post-recovery verification
- periodic integrity audits
- replication target verification
- certification-grade storage trust

### Multi-Resolution Materialization Architecture

#### Tiered materialization forms

Technical role:
Not all consumers need the entire truth graph equally. The store must support
multiple materialization resolutions as first-class bounded read artifacts:

- **full snapshot**: complete materialized state at a commit
- **aspect-local snapshot**: materialized state for a specific aspect subset
- **subgraph snapshot**: materialized state for a topological neighborhood,
  assembly region, dependency island, or other bounded subgraph
- **branch-local hot working set snapshot**: materialized current working
  surface of an active branch without full history
- **analysis-specific projection snapshot**: materialized view shaped for a
  specific analysis workflow (timing cone, connectivity neighborhood,
  structural classification region)

What this enables:

- lower read amplification for focused queries
- faster cold-start analysis against bounded regions
- cheaper branch switching when only the working set matters
- much better locality for large graphs where consumers rarely need the
  whole world equally
- store-managed hot materialization policies that maintain frequently demanded
  subgraph shapes under retention policy

For geometry kernels and chip simulators, this is the difference between
"load the entire topology to query one assembly" and "read exactly the
region you need with bounded cost."

### Content-Addressed Storage Architecture

#### Content-addressed structural blocks

Technical role:
Selected immutable storage layers should be represented as content-addressed
structural chunks: aspect blocks, relation adjacency segments, lineage
segments, snapshot fragments, and branch base fragments.

Content addressing is not applied universally — it is applied where immutable
reuse is naturally high: branch base sharing, snapshot fragment dedup,
cross-branch structural overlap, and repeated topology patterns.

What this enables:

- deduplication across branches that share most of their structure
- deduplication across snapshots that overlap heavily
- efficient replication where only novel content blocks ship
- integrity verification through content hashes as a natural property
- cached analysis artifacts that can reference stable storage identities
- dramatically better scalability for geometry and chip workloads where huge
  amounts of structure are repeated across branches

#### Cross-artifact digest graph

Technical role:
The store must maintain a durable graph of digest relationships among
canonical and derived artifacts: commit envelopes, snapshots, lineage
artifacts, schema boundary artifacts, derived caches, verification
artifacts, and published replication units.

What this enables:

- integrity verification through digest chain traversal
- corruption localization to specific artifact boundaries
- replay parity verification between stores
- partial replication trust checking
- audit tooling that traces integrity through the full artifact graph
- replication targets that can verify subset integrity without full rebuild

### Delta Stack Management Architecture

#### Read-amplification-aware delta management

Technical role:
Long delta stacks are a hidden performance tax on every read. The store must
have an explicit subsystem for managing delta depth and read cost:

- **delta depth budgeting**: configurable limits on delta stack depth before
  compaction is triggered
- **hot-path read amplification tracking**: counters and diagnostics for how
  many delta layers a read must resolve
- **selective branch-local rebasing**: rebase a single branch's delta stack
  against a fresher base without affecting other branches
- **merge-aware flattening**: when branches merge, flatten the delta stack at
  the merge point
- **aspect-local compaction**: compact only the aspects that have deep delta
  stacks, not the entire branch

What this enables:

- cheap branching preserved while preventing delta stacks from becoming a
  read tax over time
- adaptive behavior where short-lived experimental branches stay thin delta
  stacks and long-lived working branches get periodic rebasing
- aspect-scoped compaction that avoids rewriting the entire branch when only
  one aspect has deep history
- clear diagnostics for when read amplification is the performance bottleneck

### Derived Artifact Architecture

#### Basis-anchored cached analysis artifacts

Technical role:
The store must support persisting expensive derived analysis artifacts with
explicit basis identities. Examples include: generated mesh fragments,
topology classification results, spatial acceleration structures, timing
summaries, net reachability results, equivalence analyses, constraint
satisfaction summaries, and dependency slices.

These are stored as:
- **derived**: not authoritative truth
- **basis-pinned**: explicitly tied to a specific commit or snapshot
- **invalidation-aware**: invalidated when the basis advances beyond
  tolerance
- **reconstructible or discardable**: can always be rebuilt from the basis

What this enables:

- geometry and chip systems avoid recomputing expensive derived views over
  mostly stable bases
- the store becomes a serious accelerator substrate, not just a persistence
  layer
- downstream consumers can check whether a cached analysis is still valid
  for their current basis before rebuilding
- analysis sharing across users working from the same basis

#### Durable secondary-structure catalog

Technical role:
The store must maintain a catalog of non-authoritative derived structures:
mesh caches, B-rep acceleration structures, spatial indexes, reachability
indexes, adjacency summaries, timing indexes, structural fingerprints, and
correspondence maps.

Each catalog entry must declare:
- basis commit or snapshot
- validity rules and invalidation conditions
- space usage and rebuild cost estimate
- retention priority (hot, warm, cold, discardable)
- whether replicated or local-only

What this enables:

- a principled home for powerful accelerators without blurring authority
- retention-aware management of derived structures under explicit policy
- capacity planning visibility into derived artifact storage footprint
- clear rebuild cost estimation for operational decisions

#### Storage classes for exact vs approximate derived artifacts

Technical role:
Derived artifacts must carry an explicit accuracy classification:

- **exact**: provably correct relative to basis (e.g., exact topology
  correspondence, exact adjacency index)
- **conservative**: provably contains the truth but may be overapproximate
  (e.g., conservative timing bounds)
- **approximate**: numerically or structurally close but not exact (e.g.,
  approximate spatial cache, simplified mesh)
- **heuristic**: best-effort without formal guarantees (e.g., heuristic
  face-match candidates)
- **advisory**: informational only, not suitable for correctness-sensitive
  consumption

What this enables:

- downstream consumers know what they are consuming and can make trust
  decisions
- correctness-sensitive workflows can restrict themselves to exact and
  conservative artifacts
- heuristic results never impersonate truth
- the store supports both rigorous engineering and exploratory analysis
  without conflating them

#### Solver- and kernel-friendly pinned basis objects

Technical role:
The store must support first-class durable objects representing pinned
evaluation bases:

- pinned basis commit
- pinned snapshot
- pinned branch tip
- pinned schema boundary set
- pinned subgraph slice

Expensive consumers (geometry kernels, solvers, timing analyzers, AI agents)
bind their work to these pinned bases with strong reproducibility semantics.

What this enables:

- exact reproducibility of expensive analysis across sessions
- clear invalidation when the pinned basis advances
- solver state that can be resumed against the exact same basis
- auditable analysis results tied to verifiable bases

### Correspondence and Merge Assistance Architecture

#### Persistent correspondence indexes

Technical role:
Given the store's branch and lineage orientation, correspondence indexes
must be first-class durable accelerators. These answer:

- what in branch B corresponds to entity or subgraph X from branch A?
- what likely survived a rewrite or split?
- what structurally matches a prior region or pattern?
- what analysis results may be reusable under a changed basis?

Correspondence indexes build on structural fingerprint storage and extend
it into a real store subsystem with explicit basis, validity, and retention
semantics.

What this enables:

- geometry feature history and topology correspondence across design iterations
- chip netlist and module evolution tracking
- merge assistance through pre-computed correspondence candidates
- cross-branch diffing backed by durable index structures
- analysis reuse detection across branches

#### Merge-assistance durable artifacts

Technical role:
The store must support persisting merge-assistance data as durable,
basis-pinned, non-authoritative artifacts:

- structural correspondence candidates between branches
- conflict localization artifacts
- reusable reconciliation hints from prior merges
- prior merge decisions and their outcomes
- schema divergence classifications
- equivalence clusters across branches

What this enables:

- CAD branch merges backed by stored correspondence analysis
- chip branch convergence using prior reconciliation history
- collaborative systems that learn from past merge decisions
- AI agents performing branch reconciliation with durable context
- faster merge operations that reuse expensive correspondence work

### Time-Travel and Diff Architecture

#### Time-travel diff acceleration

Technical role:
The store must provide fast durable support for structured diffing operations:

- diff between arbitrary versions on the same branch
- branch-to-branch correspondence diff
- identity-aware diff that tracks entity evolution across versions
- schema-boundary-aware diff that respects schema changes at each boundary
- aspect-scoped diff that restricts comparison to specific aspects
- region-scoped diff that restricts comparison to subgraph boundaries

What this enables:

- CAD model investigation ("what changed in this assembly since last week?")
- timing or connectivity regression analysis for chip design
- certification and audit reports with structured change summaries
- AI agents comparing branches with precise structural diffs
- powerful diagnostic and explanation tooling for complex truth histories

When combined with persistent correspondence indexes and structural
fingerprints, this becomes a deeply defensible store capability.

### Verification and Trust Architecture

#### Incremental verification layers

Technical role:
The store must support persisting verification and proof artifacts attached
to commits and snapshots:

- integrity proof fragments
- structural consistency certificates
- replay parity digests
- lineage consistency checks
- schema-boundary verification artifacts
- analysis equivalence hashes

These are derived durable artifacts — they attest to verification results
without being authoritative truth themselves.

What this enables:

- certification-grade trust for chip and CAD workflows
- post-recovery verification that can reference stored proof artifacts
- periodic integrity audits backed by durable verification state
- replication targets that can verify incoming artifacts against stored
  digests
- analysis consumers that can trust cached results without re-verifying
  from scratch

### Branch Lifecycle Architecture

#### Hot/cold branch tiering

Technical role:
Branches must support lifecycle classification with physically distinct
storage treatment:

- **active design branches**: full caching, dense snapshots, retained
  derived artifacts, high priority
- **suspended experimental branches**: lightweight retention, compacted
  delta stacks, minimal derived artifacts
- **archival branches**: cold storage, sparse snapshots, no derived
  artifacts, compaction-aggressive
- **replication-only branches**: optimized for shipping artifacts to
  downstream stores
- **analysis-pinned branches**: snapshots promoted for read-only analysis
  without write overhead

Storage behavior varies physically per tier:
- caching and prefetch priority
- snapshot materialization density
- compaction aggressiveness
- retained derived artifact budget
- replication priority

What this enables:

- geometry and chip workflows that naturally create many branches without all
  branches paying equal storage cost
- explicit lifecycle management instead of treating all branches as equally
  important
- storage budget visibility per tier
- automated tiering policies based on branch activity

### Locality and Clustering Architecture

#### Range- and region-aware locality clustering

Technical role:
The store should preserve physical locality for data that is commonly
accessed together:

- topological neighborhoods in geometry graphs
- assembly and component subgraphs
- spatially nearby entities
- frequently co-read aspect groups
- module-local clustering for chip netlists
- cone-local and adjacency-local clustering
- timing-region clustering

What this enables:

- reduced I/O cost for common modeling operations that touch local
  neighborhoods
- better cache behavior for iterative solvers that walk local graph structure
- specialized storage behavior that is difficult for generic databases to
  replicate
- a defensible performance advantage for domain-specific workloads

This is one of the places where the store can become specialized in a way
that generic persistence layers cannot match.

### Analysis and Simulation Architecture

#### Simulation and analysis checkpoint lanes

Technical role:
The store must support separating truth commits from derived simulation and
analysis state:

- **truth lane**: authoritative committed truth (owned by `forge-relational`)
- **analysis checkpoint lane**: derived simulation or solver state pinned to
  exact truth bases
- **resumable progress lane**: solver iteration progress, partial analysis
  results, convergence state

Analysis checkpoint lanes are durable but derived. They are basis-pinned and
invalidation-aware. They must never be confused with authoritative truth
commits.

What this enables:

- users can resume expensive simulations and analyses safely after
  interruption
- solver state is separated from truth state architecturally
- analysis checkpoints are tied to exact bases with reproducibility
  guarantees
- multiple analysis tracks can coexist on the same branch without conflating
  their state
- chip timing analysis and geometry mesh generation can checkpoint durable
  intermediate results

### Bulk Operations Architecture

#### Deterministic bulk-ingest and bulk-transform paths

Technical role:
The store must provide first-class bulk operation paths for: importing large
structures, rewriting huge subgraphs, transforming schema or representation
layers, hydrating from external formats, and running massive refactors.

Bulk paths must preserve:
- deterministic commit chunking
- replayability from the resulting commit envelopes
- WAL safety throughout the operation
- progress checkpointing so interrupted bulk operations can resume
- bounded memory usage regardless of import size
- artifact parity with ordinary transactions (same canonical commit format)

What this enables:

- importing large geometry models or chip netlists without special-case code
- schema migrations that transform entire stores as replayable commit
  sequences
- large-scale refactors with progress visibility and resumability
- external format hydration that produces the same canonical artifacts as
  hand-built commits

Without first-class bulk paths, imports and transforms become ad-hoc,
memory-unbounded, non-resumable pain.

### Working Set Intelligence Architecture

#### Durable working set tracking

Technical role:
The store should maintain working-set intelligence based on observed access
patterns:

- recently hot subgraphs
- repeatedly materialized slices
- frequently resumed branches
- repeated analysis basis points
- hot lineage neighborhoods

The store can adapt behavior based on this intelligence:
- which regions to snapshot proactively
- which delta layers to prefetch
- which derived artifacts to retain under pressure
- where to compact aggressively vs preserve detail
- which structures to cluster together physically

What this enables:

- adaptive performance that improves with repeated use patterns
- proactive materialization of frequently demanded subgraph shapes
- intelligent retention decisions under storage pressure
- natural acceleration for domain workflows without manual tuning

This is not just performance optimization — for large technical graphs it
can materially change the feel and responsiveness of the system.

### Edge and Replication Architecture

#### Local-first and edge-first replication primitives

Technical role:
Beyond basic artifact publishing, the store must support replication
primitives designed for edge, workstation, and offline-first deployment:

- resumable artifact bundles
- partial branch replication (only specific branches, not the whole store)
- subgraph replication (only a bounded region, not the whole branch)
- branch pinsets (explicit declarations of which branches to replicate)
- snapshot-plus-tail replication (ship a snapshot plus subsequent commit
  envelopes)
- trust-verifiable pull ranges (recipient can verify integrity of received
  artifact ranges)

What this enables:

- geometry and design tools that sync workstation-local stores with a
  central server
- field-deployed systems that replicate only the branches and regions they
  need
- bandwidth-constrained environments that pull partial replications
- trust-verified sync where the recipient can prove artifact integrity
  without full store access

#### Deterministic import/export capsules

Technical role:
The store must support durable export and import capsules that package a
self-contained, verifiable subset of the store:

- canonical commit ranges
- required schema boundaries
- lineage fragments
- materialized snapshots if needed
- verification digests
- cursor positions if relevant
- derived caches optionally

What this enables:

- moving design work between environments (workstation to server, team to
  team)
- bug reproduction with exact truth state attached
- certification handoff with verifiable integrity
- analysis sharing with exact basis preservation
- partial replication bootstrap without shipping the entire store history

### Admission Control and Budget Architecture

#### Store-native admission control and budget contracts

Technical role:
The store must own explicit budgets and admission control for:

- branch depth and count
- retained history depth per branch
- snapshot materialization density
- derived artifact storage footprint
- compaction debt (how much pending compaction work exists)
- replay rebuild debt (how expensive a full rebuild would be)
- WAL growth rate and size
- correspondence index growth

What this enables:

- explicit budget visibility for engineering systems that must not silently
  degrade
- admission control that prevents unbounded growth before it becomes a
  crisis
- operational alerts when budgets are exceeded
- capacity planning based on real budget consumption metrics
- policy-driven decisions about when to archive, compact, or evict

For engineering systems, explicit budget surfaces prevent the silent
degradation that makes storage layers unpredictable under sustained use.

## Domain Fit

### Geometry and CAD

`forge-store` should support:

- persistent topology truth that survives modeling sessions
- branch-local design experiments stored as lightweight deltas
- rebuild history and lineage that survive process restarts
- certification-grade replay from stored commit artifacts
- multi-resolution materialization for assembly regions and topological
  neighborhoods without loading the entire model
- basis-anchored mesh caches and spatial acceleration structures that
  persist across sessions
- persistent correspondence indexes for topology evolution tracking across
  design iterations
- region-aware locality clustering for efficient neighborhood traversal
- hot/cold branch tiering for active design branches vs archived experiments
- simulation and analysis checkpoint lanes for mesh generation and FEA

Revolutionary use:
geometry kernels can maintain durable, branchable, replayable topology truth
across editing sessions with persistent cached analysis, region-level
materialization, and correspondence tracking — instead of treating persistence
as "save to file" with complete loss of internal history, identity semantics,
and expensive derived computation.

### AI Systems

`forge-store` should support:

- persistent branch exploration across agent sessions
- durable speculative workspaces that survive process boundaries
- retrievable decision history for explanation and audit
- snapshot-based evaluation against stored historical truth
- pinned basis objects for reproducible evaluation
- basis-anchored cached analysis artifacts for reusable computation
- merge-assistance artifacts for branch reconciliation context
- time-travel diff acceleration for comparing alternative approaches

Revolutionary use:
AI agents can maintain persistent, branchable world state across sessions
with full history, reproducibility, cached analysis reuse, and structured
branch comparison — instead of reconstructing context from logs or checkpoint
files.

### Web and Data Platforms

`forge-store` should support:

- durable truth with real-time CDC and crash recovery
- multi-tenant branch isolation with shared base storage
- point-in-time reads for historical reporting and audit
- cursor-based subscriber resume for downstream integrations
- content-addressed deduplication across tenant branches
- deterministic bulk-ingest for data migrations
- store-native admission control for capacity management
- import/export capsules for deployment and tenant migration

Revolutionary use:
web platforms get a versioned, branchable, auditable truth store with
built-in CDC and crash recovery, instead of PostgreSQL plus manual event
sourcing plus manual audit logging plus manual tenant isolation.

### Chip Design and Simulation

`forge-store` should support:

- large version graphs with deep analysis history
- certification-grade replay from stored canonical artifacts
- snapshot-safe concurrent analysis from materialized historical snapshots
- branch-parallel storage for independent analysis tracks
- basis-anchored timing summaries, net reachability results, and cone analyses
- multi-resolution materialization for timing cones and module neighborhoods
- simulation checkpoint lanes for resumable timing and DRC analysis
- region-aware locality clustering for module-local and cone-local I/O
- incremental verification layers for certification workflows
- cross-artifact digest graphs for integrity assurance

Revolutionary use:
chip design systems can persist their entire connectivity truth, version
history, analysis lineage, and cached analysis artifacts as a single durable
store with built-in replay, certification, and resumable analysis — instead of
tool-local file formats and ad hoc versioning.

### Collaborative and Offline-First Systems

`forge-store` should support:

- offline branch persistence that syncs on reconnect
- durable merge history with full conflict and reconciliation artifacts
- cursor-based resume for interrupted sync sessions
- branch-local edits that survive device boundaries
- edge-first replication primitives for workstation-local stores
- import/export capsules for moving work between environments
- merge-assistance artifacts for faster reconciliation
- partial branch and subgraph replication for bandwidth-constrained sync

Revolutionary use:
offline-first applications get real branch-native persistence with
structured merge, cursor-based resume, edge-first replication, and sync from
stored canonical artifacts — instead of conflict-resolution heuristics over
unstructured local storage.

## Roadmap Direction

This file is a vision document, not the execution roadmap. But the future work
should be derivable from it.

The highest-signal storage programs are:

- commit envelope persistence and deterministic recovery
- structural delta storage and branch delta layering
- write-ahead logging and checkpoint/compaction
- version graph and lineage graph persistence
- CDC cursor persistence
- embedded storage backend (production-grade default)
- multi-resolution snapshot materialization
- retention-policy-driven compaction
- content-addressed structural blocks and deduplication
- read-amplification-aware delta stack management
- basis-anchored cached analysis artifacts and secondary-structure catalog
- persistent correspondence indexes
- time-travel diff acceleration
- incremental verification layers and cross-artifact digest graph
- hot/cold branch lifecycle tiering
- region-aware locality clustering
- simulation and analysis checkpoint lanes
- deterministic bulk-ingest and bulk-transform paths
- durable working set intelligence
- edge-first replication primitives and import/export capsules
- merge-assistance durable artifacts
- store-native admission control and budget contracts
- replication-ready artifact publishing
- storage diagnostics and integrity verification
- schema evolution boundary persistence
- exact vs approximate derived artifact classification
- solver-friendly pinned basis objects

If a capability is named here and not yet built, it is roadmap work.

If a capability is built but not yet proven under crash, recovery, corruption,
and scale scenarios, it is certification work.

## Non-Goals

- turning the storage engine into a truth runtime (that is `forge-relational`)
- implementing query planning or optimization (that belongs above the store)
- owning signal evaluation or reactive scheduling
- defining sync protocol delivery or subscription management
- baking domain-specific meaning into stored artifacts
- requiring a specific storage backend for correctness
- treating the embedded backend as "just for testing"
- replacing the truth runtime's in-memory arena with direct storage I/O

## Companion Documents

- [forge_relational_vision.md](file:///c:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-relational/forge_relational_vision.md)
- [forge_signals2.md](file:///c:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge_signal/forge_signals2.md)
- [forge_runtime_bridge_vision.md](file:///c:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_runtime_bridge_vision.md)
- [forge_server_vision.md](file:///c:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-server/forge_server_vision.md)

The storage engine's structural delta model, branch delta layering, commit
envelope persistence, and retention-aware compaction are what make durable
truth practical instead of expensive. If these are weak, every layer above
the store — runtimes, bridge, protocol, applications — pays the cost of
either losing history or paying for full-state storage at every version.
