# Worth Bootstrap Plan

## Goal

Define the clean-start bootstrap for Worth as a new geometry-domain stack built
on the existing Forge runtimes instead of on the transitional `forge-spec` /
`forge-core` / `forge-kernel` authority model.

This document is not a milestone spec.

It is the architecture bootstrap note that answers:

- what new crates we should create first
- what each crate should own
- what old crates are reference sources rather than foundations
- what the first minimal vertical slice should prove

## Naming

- `forge` remains the runtime/platform brand
- the geometry kernel domain becomes `worth`
- runtime crate names stay as-is
- new domain crates should use the `worth-*` prefix

## Adversarial Constraint

Worth must survive a naive-architecture failure mode that is already visible in
the old stack:

- truth authority duplicated across multiple layers
- derived state mixed with authority
- topology, orchestration, tracing, and compute invalidation encoded in
  separate overlapping systems
- branch/replay/merge/inspection semantics drifting because there is no single
  authoritative commit story

A Worth bootstrap is only valid if it makes the following mechanically true:

- one authoritative truth runtime owns committed Worth state
- one derived runtime owns recomputation and diagnostics for Worth
- one bridge boundary owns causal routing between them
- every other Worth crate is domain logic on top of those boundaries, not a
  second runtime

## Governing Runtime Thesis

Worth should be built around the existing runtime stack:

- `forge-relational` owns authoritative truth, identity, history, diffs,
  lineage, validation, commit strategies, and merge strategies
- `forge-signal` owns derived computation, invalidation, recomputation,
  transactions, diagnostics, and history for derived work
- `forge-runtime-bridge` owns patch-to-invalidation routing, explicit truth-view
  evaluation, speculative coordination, and causal explanation across the
  boundary

The resulting design rule is simple:

- Worth does not build a new truth runtime
- Worth does not build a new reactive runtime
- Worth defines domain schema, domain invariants, domain materializations, and
  domain algorithms on top of the runtimes

## Keep vs Replace

### Keep

- `worth-math`
- `worth-geom`
- `forge-relational`
- `forge-signal`
- `forge-runtime-bridge`

### Treat as Reference, Not Foundation

- `forge-topo`
- `forge-kernel`
- `forge-core`
- `forge-spec`

### Likely Replace

- the old bespoke spec-graph truth model
- kernel-local signal orchestration as the architectural center
- core-local tracing/policy/envelope systems where runtime-native artifacts now
  provide the stronger authority story

## New Crate Set

The initial bootstrap should create only the minimum crates needed to express
the new architecture honestly.

### `worth-schema`

Owns:

- Worth truth kinds and relation kinds
- Worth aspect vocabulary
- Worth invariant declarations
- schema registration and lowering glue needed by `forge-relational`
- domain naming for topology, geometry binding, feature intent, lineage, and
  diagnostics surfaces

Does not own:

- mutation execution
- topology materialization
- geometry numerics
- signal scheduling

Why it comes first:

- the truth schema is the authority boundary
- every later crate depends on this vocabulary

### `worth-topo`

Owns:

- topology-domain semantics
- topology materialization/projection from relational truth
- topology-specific invariant lowering and validation helpers
- topology algorithms and query helpers that are still valuable in a
  runtime-first world
- topology fingerprints and structural interpretation for Worth

Does not own:

- authoritative truth mutation lifecycle
- a bespoke transaction runtime
- a parallel graph/spec runtime

Reference sources:

- `forge-topo` algorithms
- `forge-topo` validator taxonomy
- `forge-topo` domain vocabulary

### `worth-core`

Create only if the design proves it is needed.

If created, it should be thin and own only:

- small shared Worth-specific domain types
- small shared error/value vocabulary that is still domain-local after runtime
  adoption

It must not become:

- a second runtime substrate
- a tracing platform competing with runtime artifacts
- a policy/transaction/envelope architecture that duplicates runtime machinery

### Deferred Crates

Do not create these until there is a concrete need:

- `worth-kernel`
- `worth-bridge`
- `worth-proof`

Those may eventually exist, but the bootstrap should prove the substrate first
instead of minting orchestration crates prematurely.

## Ownership Boundaries

### Authoritative

Authoritative Worth state lives in `forge-relational`.

That includes:

- topology truth
- feature truth
- geometry-binding truth
- lineage/correspondence truth
- commit/publication/replay artifacts for Worth truth

### Derived

Derived Worth state lives in `forge-signal`.

That includes:

- topology materializations
- geometry-ready materializations
- query/index/cache-like read models
- fingerprints and diagnostics derived from truth
- future view/projection/runtime caches

### Bridge-Owned

Truth-to-compute coordination lives in `forge-runtime-bridge`.

That includes:

- truth patch to invalidation mapping
- explicit truth-view evaluation
- speculative branch coordination
- discard/promote boundaries
- causal explanations across truth and compute

### Worth-Owned

Worth owns the domain semantics layered over those boundaries:

- schema meaning
- aspect meaning
- invariant meaning
- materialization meaning
- geometry/topology algorithm meaning

## Initial Worth Truth Model

The bootstrap truth model should be intentionally small.

Initial entity families:

- model
- body
- lump
- region
- shell
- face
- loop
- halfedge
- edge
- vertex
- surface_binding
- curve_binding
- coedge_binding
- vertex_geometry_binding

Initial relation families:

- ownership hierarchy
- face-loop relations
- loop-halfedge entry
- halfedge next
- halfedge radial_next
- halfedge origin
- halfedge uses edge
- halfedge bounds face
- face uses surface binding
- edge uses curve binding
- halfedge uses coedge binding
- vertex uses geometry binding

Initial aspect families:

- `topology.structure`
- `topology.ownership`
- `topology.boundary`
- `topology.radial`
- `geometry.binding`
- `lineage`
- `diagnostics`

This is enough to prove the runtime shape without prematurely encoding the full
future kernel.

## Initial Invariant Classes

The bootstrap should start with a narrow but load-bearing invariant set:

- ownership consistency
- required-single relation presence where the schema demands it
- loop entry coherence
- halfedge next coherence
- halfedge radial coherence
- edge incidence legality
- vertex origin legality

Important rule:

- start with invariants that protect truth-legality first
- do not begin with rich geometry semantics
- use relational invariant machinery as the authority boundary

## First Vertical Slice

The first slice should be tiny but complete.

### Slice Goal

Prove that Worth can commit topology truth in `forge-relational`, route the
result through `forge-runtime-bridge`, materialize a topology-derived view in
`forge-signal`, and explain the result through runtime diagnostics.

### Slice Scope

The first slice should support:

- one minimal seeded topology
- one authored topology mutation path
- one commit through relational
- one bridge route
- one signal recompute
- one derived topology read
- one diagnostics/explanation path

### Recommended Concrete Scenario

Use a minimal topological seed and one structural edit:

- seed one body / region / shell / face / loop / halfedge cycle / edge / vertex
  arrangement
- perform one `split_edge`-style truth mutation or an equivalent small topology
  edit
- publish the commit
- route the resulting truth delta into one topology-materialization target in
  signal
- read the materialized output
- capture:
  - relational commit/publication artifacts
  - bridge routing/evaluation diagnostics
  - signal diagnostics/history

### Why This Slice Comes First

It proves:

- truth authority
- invariant enforcement
- patch semantics
- bridge mapping
- derived recompute
- end-to-end explanation

without requiring the rest of the old kernel to be ported.

## Bootstrapping Sequence

### Phase 1: Name and Declare

- create `worth-schema`
- define initial entity kinds, relation kinds, aspects, and invariant groups
- write the schema registration and lowering surfaces

Coherent state after Phase 1:

- Worth truth vocabulary exists
- no domain algorithms yet
- no derived runtime integration yet

### Phase 2: Materialize Minimal Topology

- create `worth-topo`
- implement minimal topology materialization from relational truth
- implement the smallest invariant and projection helpers needed for the first
  topology slice

Coherent state after Phase 2:

- topology meaning exists on top of relational truth
- Worth still has no broad kernel orchestration

### Phase 3: Prove Runtime Flow

- add bridge mapping for the first topology slice
- add signal materialization/read path
- add end-to-end certification-style tests for commit, route, recompute, and
  diagnostics

Coherent state after Phase 3:

- Worth has one truthful end-to-end runtime story
- all later work can extend the substrate instead of inventing a new one

## Migration Matrix

### Mine from `forge-topo`

- topology vocabulary
- operator semantics
- validator decomposition
- adjacency/query patterns
- useful structural algorithms

### Mine from `forge-spec`

- only schema vocabulary or useful mutation-shape ideas

Do not preserve:

- bespoke truth-runtime role
- bespoke graph authority model

### Mine from `forge-core`

- only thin shared domain types if they still make sense after runtime adoption

Do not preserve automatically:

- envelope patterns
- tracing substrate
- policy substrate

### Mine from `forge-kernel`

- high-value domain workflows
- scenario/test ideas
- geometry/topology operation semantics

Do not preserve automatically:

- orchestration structure
- transitional `SpecEnvelope` architecture
- signal-as-host-cache center of gravity

## What Not To Do

- do not start by renaming old crates in place
- do not create a new bespoke Worth truth runtime
- do not recreate `forge-spec` under a new name
- do not make `worth-core` a dumping ground
- do not port old tracing/envelope machinery before proving the runtime-native
  artifact story is insufficient
- do not begin with boolean pipelines or broad kernel orchestration

## Success Criteria for the Bootstrap

The bootstrap is successful when:

- a new Worth crate set exists and is architecturally clean
- Worth truth is authored in `forge-relational`
- Worth derived views are recomputed in `forge-signal`
- Worth truth-to-compute routing goes through `forge-runtime-bridge`
- one topology vertical slice works end to end
- the old stack remains available only as a reference source

## Immediate Next Actions

1. Create `worth-schema`.
2. Register the minimal initial Worth truth vocabulary.
3. Create `worth-topo`.
4. Implement the smallest topology materialization path.
5. Add the first bridge mapping and signal target.
6. Prove the first vertical slice with diagnostics and replay-aware artifacts.
