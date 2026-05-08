# Worth Read Composition Side Quest

> **Status:** Closed - Phase 1 through Phase 3 complete; Phase 4 remains future
> domain-adoption guidance
>
> **Roadmap parent:** [worth_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/worth_roadmap.md)
>
> **Primary adjacent milestone:** [milestone-3.md](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/milestone-3.md)
>
> **Primary runtime dependency:** [forge-query-runtime-kernel-hard-break.md](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/forge-query-runtime-kernel-hard-break.md)
>
> **Test requirements:**
> - [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/test-requirements.md)
> - [topo-test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/topo-test-requirements.md)

## Goal

Define one read-composition product in `forge-query` that makes bounded
neighborhood reads feel as seamless and mechanically safe as graph composition,
then narrow Worth onto that product through domain-owned read families so
topology, later spatial, and later kernel code no longer rebuild row joins,
neighborhood indexes, or ad hoc traversal logic in user space.

## Closeout Status

This side quest is closed for the Milestone 3 return gate.

Closed phases:

- **Phase 1:** `forge-query` now exposes the public read-composition product:
  `compose_read(...)`, `compose_read_with_invariant_pack(...)`,
  `define_read_family(...)`, `execute_read_family(...)`,
  `ForgeQueryReadGraph`, `ForgeQueryReadReceipt`, and typed read denials.
- **Phase 2:** `worth-topo` now exposes the topology-domain read facade through
  `TopologyDomainQuery`, typed request families, decoded topology views,
  proof reports, aggregate reports, and closeout reports.
- **Phase 3:** the no-N+1 proof surface is machine-checkable through
  `topology_read_lowering_breadth`,
  `topology_read_fallback_posture`, `topology_read_view_parity`, and
  `topology_read_relationship_proof_posture`; Milestone 3 closeout is
  enforced by `certify_milestone_three_closeout()`.

Phase 4 is intentionally not part of this closeout. It remains the adoption
pattern for later Worth domains after the topology-first gate.

Developer-facing docs shipped with the implementation:

- `crates/forge-query/docs/read-composition.md`
- `crates/worth-topo/docs/domain-reads.md`
- `crates/worth-topo/docs/runtime-support.md`

Final verification:

- `cargo fmt --check`
- `cargo test -p worth-topo`

## Why This Side Quest Exists

Worth is already better than the old mirror-runtime shape, but it still has a
structural read-product gap.

At kickoff:

- `forge-query` already ships authored query families, bounded traversal
  selectors, canonicalized composition, live-family promotion, relationship
  proof admission, and bridge lowering for relation-scoped subscriptions
- `worth-topo` still performed repeated row-decoding and neighborhood discovery
  locally in certification helpers, witness builders, and runtime test support
- the interim snapshot row/index helpers fixed the immediate `O(E * R)` and
  repeated table scan smell, but they were still post-read helpers rather than
  a first-class execution product

That was honest tactical progress and still the wrong final product shape.

The bad product shape is:

- rows are the ergonomic product
- neighborhoods are caller reconstruction
- traversal is declared centrally but execution is rediscovered locally
- N+1 avoidance is a discipline problem instead of a product property

The target product shape is the same kind of feeling we already expect from
graph composition:

- compose one read request against admitted read operators
- let `forge-query` own admission, canonicalization, execution, and receipts
- let Worth own domain families, domain invariants, and domain decoders
- receive one deterministic derived result plus one proof-bearing execution
  receipt
- make fallback or unsupported posture explicit debt instead of silent local
  reconstruction

That is the product shape that can actually scale from topology neighborhoods
to rebinding neighborhoods, trim neighborhoods, and fillet support
neighborhoods without reopening the same hole.

If Worth keeps solving query-shaped neighborhood problems as:

- raw `workspace.read(...)` rows
- local domain-specific indexes
- per-call witness helpers
- repeated caller-owned loop and adjacency walks

then the same architectural failure will repeat at every future domain:

- topology cert lanes
- topology/geometry rebinding
- NURBS trim neighborhoods
- fillet support neighborhoods
- branch/history continuity inspections

This side quest exists to stop that repetition permanently by making
bounded read composition a real generic Query product and then making Worth
consume that product through domain-owned vocabulary rather than through
repeated local reinvention.

## Governing Summaries

- `MENTALITY.md`
  The most important thing it protects here is adversarial breadth-first
  design. The spec must target the failure mode "rows are the product, so
  every domain eventually rebuilds neighborhoods locally" before adding more
  helper affordances on top.
- `arch_laws.md`
  The most important thing it protects here is that query semantics must be
  contractual and proof-bearing rather than helper folklore. The read
  composition kernel, Worth domain read families, and decoded Worth views must
  be separate, typed boundaries rather than one bag of row helpers.
- `perf_laws.md`
  The most important thing it protects here is boundary-honest traversal cost.
  A read-composition surface must not look like a local neighbor read while
  secretly performing whole-row scans, repeated rediscovery, or caller-owned
  loops that externalize bulk work.
- `domain_laws.md`
  The most important thing it protects here is domain-owned vocabulary and
  facade discipline. Generic read-composition mechanics belong in
  `forge-query`; topology, trim, rebinding, and fillet neighborhood names
  belong in Worth-owned facades, not in generic helper buckets.
- `VISION.md`
  The most important thing it protects here is that the spec graph remains the
  product and all derived interpretations stay auditable. Read results
  therefore need to be stable, inspectable, and replay-safe derived products
  over authoritative truth, not convenience caches that become shadow
  authority.
- `worth_roadmap.md`
  The most important thing it protects here is that Worth must enter runtime
  and query work through Forge Query rather than inventing local substitutes.
  This side quest belongs before more topology hostility widening and before
  spatial/kernel query growth because it prevents repeated reinvention at each
  later milestone.
- `test-requirements.md`
  The most important thing it protects here is proof-bearing closure over
  workflow classes rather than demos. The side quest must close read-shaped
  workflow classes like neighborhood lookup, branch-local parity reads, and
  replay-safe witness extraction with machine-checkable breadth evidence.
- `topo-test-requirements.md`
  The most important thing it protects here is query and traversal brutality.
  Worth must not claim topology hostility readiness while its witness and
  locality reads still depend on caller-owned row joins or hidden broad scans.
- `milestone-3.md`
  The most important thing it protects here is the Milestone 3 no-hidden-broad
  scan rule. The side quest must give Milestone 3 a durable product answer,
  not just a better helper answer, or later hostile lanes will keep
  reintroducing manual neighborhood logic.
- `milestone-2-closeout.md`
  The most important thing it protects here is the destroyable/rebuildable
  derived read pipeline. Read composition results and Worth domain views may be
  retained as derived products, but they may not become a second derived
  authority or a cache that forgets it is derived.
- `forge-query-runtime-kernel-hard-break.md`
  The most important thing it protects here is that Worth consumes real Query
  runtime capability instead of rebuilding a local runtime substitute. This
  side quest must consume shipped Query traversal/composition/lowering
  capability rather than inventing a second Worth-local query engine.
- `forge-query-runtime-rewrite-plan-phases.md`
  The most important thing it protects here is that Worth read vocabulary and
  runtime consumption were always intended to become first-class rather than
  stringly helpers. This side quest is the missing read-composition
  continuation of that direction, focused on read neighborhoods and traversal
  cost honesty.

## Adversarial Constraint

This side quest must survive this hostile condition:

> Arbitrary admitted Worth neighborhood reads, witness extractions, and
> traversal-shaped certification workflows across topology, later
> topology-to-geometry binding, later NURBS trim neighborhoods, later fillet
> support neighborhoods, branch-local reads, and replayed reads must compile to
> one canonical read graph, execute through one Query-owned bounded read
> composition path, return one deterministic derived result with one
> proof-bearing execution receipt, and never rely on repeated caller-owned row
> scans, repeated rediscovery of relationships, or hidden N+1 loops masquerading
> as local reads.

The design fails if:

- raw rows remain the ergonomic recurring product for graph-shaped reads
- a neighborhood read that looks local actually executes as repeated
  per-node/per-edge follow-up work without explicit receipts
- topology, spatial, and kernel domains each build separate local read helper
  stacks over the same underlying Query runtime
- the same authored read request lowers or executes differently depending on
  which caller assembled it
- read results or decoded Worth views become authority-shaped caches rather
  than disposable derived views over Query-backed truth
- formulas, validators, or invariant packs live only in Worth helper code
  instead of narrowing the admitted read graph itself
- the architecture only works for small local topology neighborhoods and
  collapses once later domains need anchored expansion or explicit broad search
  over trim, carrier, boolean, or ambiguity-frontier workloads

## Product Decision Lock

- This is a read-composition side quest, not a second generic query engine.
- `forge-query` remains the owner of generic read-composition operators,
  admission, canonicalization, execution, receipts, and runtime delivery.
- Worth owns:
  - domain read family vocabulary
  - domain read-family lowering adapters
  - domain-decoded view families
  - domain-specific invariant packs
  - domain read certification meaning
- snapshot row/index helpers were allowed as tactical fallback and migration
  debt during the migration, not as the permanent primary execution engine.
- Raw `workspace.read(...)` row sets are not the desired external authoring or
  recurring execution model for graph-shaped Worth reads.
- Repeated caller-owned row joins in active Worth runtime, certification, or
  witness paths are not an allowed end state.
- No domain may bypass the shared read-composition boundary and reintroduce its
  own local traversal helper stack once the shared substrate exists.
- Execution breadth, fallback, relationship proof posture, and parity
  eligibility must be observable at the same boundary where Worth presents the
  neighborhood read.

## Existing Forge Query Capability We Must Consume

This side quest is not allowed to pretend Forge Query lacks composition,
traversal, or proof machinery when it already ships it.

The current shipped capability we must consume includes:

- graph-composition product lessons we should mirror for reads:
  - one public composition entrypoint
  - one canonical lowered program artifact
  - one typed receipt / denial surface
  - one hard-to-bypass happy path
- authored query families:
  - `QueryFamily::Detail`
  - `QueryFamily::Collection`
- authored query builders:
  - `QueryBuilder<F>::project(...)`
  - `QueryBuilder<F>::traverse(...)`
  - predicate and ordering authoring
- typed query builders:
  - `TypedQueryBuilder<S, F>::project(...)`
  - `TypedQueryBuilder<S, F>::traverse(...)`
  - typed traversal relations over schema-owned constants
- bounded traversal selectors:
  - `TraversalSelector::bounded(relation, depth)`
- canonicalized composition and template expansion that preserve traversal
  parity
- live-family promotion that already treats collection-plus-traversal
  differently:
  - traversal-bearing collections promote to
    `LiveQueryFamily::BoundedMaterialization`
  - non-traversal ordered collections remain
    `LiveQueryFamily::OrderedCollection`
- bridge lowering that already emits relation-scoped slices for traversal-shaped
  live subscriptions
- live subscription planning that already changes query family and bridge slice
  posture based on declared traversal instead of forcing callers to rediscover
  relationship breadth manually
- relationship proof admission that already denies unbounded recursive walks
  and carries topology-class evidence before truth touch
- runtime program installation and operation execution that can declare read,
  write, and patch-delivery effects over installed query operations

The practical implication is:

`Worth does not need to invent traversal. We need to promote bounded read composition into the same product tier that graph composition already occupies.`

This side quest must therefore consume and narrow these generic surfaces rather
than wrapping them in another row-first helper layer.

## Target Product Shape

The hard problem here is not "name better topology helpers."

The hard problem is:

`make bounded read graphs a first-class Query runtime product so Worth cannot accidentally slide back into caller-owned row joins`

The target end state should feel to Worth authors like graph composition feels
today:

- one public `compose_read(...)`-style Worth-facing domain read surface
- one canonical Query-owned read-graph lowering and execution path
- one receipt-bearing result boundary
- one explicit denial lane for unsupported or dishonest locality claims

The intended naming and layering posture is:

- public happy path:
  - `compose_read(...)`
- canonical internal artifact:
  - `ReadGraph`
- reusable higher-level packaging when repetition is real:
  - `ReadFamily` or an equivalently named installed read family surface

This ordering is intentional.

The side quest should optimize first for the beautiful one-shot and
domain-constructor experience, not for a prematurely heavy installed-family
framework.

Concretely, the intended stack is:

1. A caller composes one read through `compose_read(...)`.
2. Worth domain layers may wrap that with typed domain read-family
   constructors, but they still lower to the same canonical artifact.
3. The request lowers into one canonical `ReadGraph` composed from admitted
   read operators.
4. `forge-query` admits and executes that `ReadGraph` through one first-class
   bounded read-composition runtime surface rather than forcing a raw row
   drain.
5. Worth decodes the result into a disposable domain view.
6. The returned result carries proof-bearing execution posture:
   - execution engine
   - traversal breadth
   - read-operator family coverage
   - relationship-proof posture
   - fallback class
   - fallback count
   - parity eligibility

The built-in read operators should cover the common graph shapes directly:

- bounded successor / cycle walk
- bounded fan-out and fan-in adjacency
- shared-endpoint or shared-attachment neighborhoods
- bounded ancestor / descendant walks
- radial or membership neighborhoods where the generic graph form is stable
- anchored frontier expansion from explicit seed sets
- explicit broad-search operator families whose receipts cannot claim local
  posture

Extension must not mean arbitrary row callbacks.

Extension must mean:

- composing admitted read operators
- attaching domain-specific invariant packs
- attaching domain-specific decoders
- attaching domain-specific result certification

Raw row drains may remain as escape hatches, but they must become the
visibly-lower-level path rather than the default implementation story for
recurring Worth neighborhoods.

## Product Shape Examples

The side quest should stay grounded in concrete product feel, not only in
phase language.

The intended public experience is:

### Local example: loop cycle

```rust
let outcome = workspace.compose_read(|read| {
    let seed = read.anchor_entity("half-edge", half_edge_id)?;
    let cycle = read.walk_successor_cycle(&seed, max_depth)?;
    read.decode(LoopCycleView::decoder(cycle))
})?;
```

Expected result posture:

- one decoded loop-cycle result
- one receipt attached to the result
- scope class reported as `LocalNeighborhood`

### Anchored expansion example: local rewire support

```rust
let outcome = workspace.compose_read(|read| {
    let seeds = read.anchor_entities("candidate-half-edges", candidate_half_edges)?;
    let expanded = read.expand_from(&seeds, |graph| {
        graph
            .walk_outgoing("HalfEdgeUsesVertex", 1)?
            .walk_incoming("HalfEdgeUsesVertex", 1)?
            .walk_outgoing("HalfEdgeUsesEdge", 1)?
            .walk_outgoing("HalfEdgeNext", 2)?
    })?;
    read.require_invariant(topology_rewire_support_pack())?;
    read.decode(LocalRewireSupportView::decoder(expanded))
})?;
```

Expected result posture:

- one decoded support neighborhood result
- one receipt attached to the result
- scope class reported as `AnchoredExpansion`

### Explicit broad-search example: grazing trim / carrier candidate sweep

```rust
let outcome = workspace.compose_read(|read| {
    let seeds = read.anchor_entities("candidate-face-pairs", candidate_face_pairs)?;
    let search = read.explicit_broad_search("grazing-candidates", |graph| {
        graph
            .walk_outgoing("FaceUsesCarrier", 1)?
            .walk_outgoing("CarrierUsesTrimLoop", 2)?
            .walk_outgoing("TrimLoopUsesTrimEdge", 3)?
            .expand_ambiguity_frontier(max_frontier)?
    })?;
    read.require_invariant(grazing_boolean_candidate_pack())?;
    read.decode(GrazingBooleanCandidateView::decoder(search))
})?;
```

Expected result posture:

- one decoded candidate result
- one receipt attached to the result
- scope class reported as `ExplicitBroadSearch`

These examples are intentionally hypothetical API sketches, not frozen final
Rust signatures. Their purpose is to keep the implementation honest about the
product feel we are trying to preserve.

## Receipt Shape Expectations

The receipt should not feel like optional bureaucracy or a second manual
inspection step.

By default, the composed read result should carry or expose one attached
receipt containing at minimum:

- read-graph identity or digest
- scope class
- execution engine
- traversal breadth
- read-operator family coverage
- relationship-proof posture
- fallback class
- fallback count
- parity eligibility
- typed denial posture when execution fails

The product goal is:

`the default success path already tells the caller what kind of read actually ran`

Callers may inspect more deeply, but they should not need a second API just to
learn whether a supposedly local neighborhood actually escalated.

## Read Scope Classes

This side quest must not pretend that every hard read is a local neighborhood.

The read-composition kernel must support at least three honest scope classes:

### 1. Local Neighborhood

These are truly local, bounded graph reads.

Examples:

- loop cycle walks
- local successor / predecessor neighborhoods
- radial fan neighborhoods
- shared-endpoint or shared-attachment neighborhoods

The receipt may honestly classify these as local only when the executed read
graph stayed inside the declared bounded local operator surface.

### 2. Anchored Expansion

These begin from one or more explicit seeds and expand through admitted
operators with explicit breadth, depth, or frontier policy.

Examples:

- gather topology and binding neighborhoods reachable from a candidate
  intersection anchor set
- gather trim and carrier neighborhoods around a rebinding seed set
- gather naming and continuity neighborhoods around an ambiguity seed set

These are not the same thing as a tiny local read. They are still bounded and
composed, but their product contract must expose that they are expansion reads
rather than local adjacency reads.

### 3. Explicit Broad Search

These are honest search-class or whole-scope reads whose cost model is broad.

Examples:

- candidate intersection sweeps across grazing trimmed carriers
- ambiguity frontier discovery where no honest small local bound exists yet
- whole-scope carrier or trim searches used to seed later local work

These reads are allowed only when they declare themselves as broad. They must
never masquerade as local neighborhoods or anchored expansions.

The core honesty rule is:

`if a read broadens from local to anchored expansion to broad search, the receipt must say so exactly`

This classification is required so the side quest scales honestly into later
boolean, trim, rebinding, and fillet workloads instead of freezing around only
topology-local examples.

## Concrete First Ship

The first ship must be concrete enough that implementation cannot hide behind
"we have a direction."

The first implementation target is deliberately two-layered:

- generic product ship in `forge-query`
- first domain adoption ship in `worth-topo`

The generic product ship must introduce:

- one public `compose_read(...)` entrypoint
- one canonical `ReadGraph` artifact
- one typed read admission trace
- one typed read execution receipt
- one explicit fallback taxonomy

Reusable installed read families are allowed in the first ship only if they
layer cleanly on top of the same `compose_read(...)` / `ReadGraph` kernel.
They are not required to make the first ship honest.

The first domain adoption ship must target:

- crate:
  - `worth-topo`
- boundary family:
  - `query/domain/` or an equivalently responsibility-shaped subtree under
    `crates/worth-topo/src/query/`
- first admitted request family:
  - `LoopCycleNeighborhood`
- first follow-on request families:
  - half-edge shared-vertex neighborhood
  - half-edge radial neighborhood
  - local rewire witness neighborhood
- first migrated callers:
  - topology runtime relation-update support under
    `crates/worth-topo/src/projection/runtime_boundary/query_runtime/tests/relation_update/`
  - topology-domain query support under
    `crates/worth-topo/src/query/tests/domain_query/`
  - any current loop-cycle or local-successor callers that can consume the
    first real Query-executed family without semantic drift
  - then:
    - `crates/worth-topo/src/certification/milestone_three/bowtie_adjacent.rs`
    - `crates/worth-topo/src/certification/milestone_three/ambiguous_local_rewire.rs`
    - `crates/worth-topo/src/certification/milestone_three/broken_radial_localization.rs`
    - `crates/worth-topo/src/certification/milestone_three/split_collapse_churn.rs`

The intended first-ship API shape is:

- one generic read-composition kernel centered on `compose_read(...)`
- one canonical `ReadGraph` artifact beneath it
- one topology-domain read family
- one topology-domain lowering facade
- one topology-domain view family
- one explicit execution / fallback / proof report family

The first implementation does not need to solve:

- NURBS neighborhoods
- fillet support neighborhoods
- branch-history query families beyond what topology already needs

But it must solve the read-composition kernel strongly enough that later
domains reuse the pattern instead of bypassing it.

## What This Side Quest Must Replace

The first replacement target is not every query in Worth. It is every recurring
Worth workflow that currently hides neighborhood meaning in caller-owned row
joins or post-read indexes.

The current replacement set includes at minimum:

- `worth-topo` hostile certification witness builders such as:
  - bowtie-adjacent neighborhood selection
  - ambiguous local rewire neighborhood selection
  - broken radial witness extraction
- `worth-topo` runtime test support that repeatedly discovers:
  - entity identity by row scans
  - relation targets by row scans
  - half-edge neighbor sets by ad hoc index construction
- any future Worth spatial or kernel workflow that would otherwise start from:
  - `workspace.read(...)`
  - raw entity / relation rows
  - repeated local traversal helpers
  - domain-specific N+1 row joins

The side quest does not need to replace:

- one-off debug helpers
- certification-only tactical fallback paths that are explicitly marked as debt
- readouts whose honest cost model is already whole-view and declared as such

But it must replace recurring workflow families that would otherwise teach the
rest of Worth to assemble graph neighborhoods manually.

## Raw Read Boundary Rule

This side quest does not ban `workspace.read(...)`.

It bans using raw row reads as the recurring *domain read API*.

Allowed:

- framework/runtime internals
- one-off debug inspection
- whole-view reads whose honest contract is already whole-view
- internal fallback implementation inside the Worth read-composition substrate

Forbidden as the lasting public or recurring workflow shape:

- certification callers pulling raw entity/relation rows and rediscovering
  adjacency themselves
- runtime helpers that expose row sets and expect callers to reconstruct graph
  neighborhoods manually
- domain read APIs that merely rename "read rows and scan them yourself"

The enforcement target is:

`raw rows may remain an implementation substrate, but not the recurring domain contract`

## Phases

The phases below are a strict dependency chain, not a buffet.

The implementation order is mandatory:

1. finish the generic `forge-query` read-composition kernel
2. move active Worth topology read families onto that finished kernel
3. close the side quest with aggregate proof and explicit remaining debt rows
4. only then resume broader Milestone 3 work

That ordering has now been satisfied for the topology-first Milestone 3 gate.
Future adoption work must still preserve the same ordering inside each later
domain rather than widening domain read families faster than the generic
execution product can support honestly.

### Phase 1: Finish The Query-Native Read Composition Kernel - Closed

Solve the real hard problem first in `forge-query` and keep working there until
the generic read product is complete enough for honest Worth adoption.

This phase must introduce and finish one first-class runtime surface for
bounded neighborhood execution so recurring callers are no longer forced into:

- raw live-view row drains
- local relation joins
- post-read neighborhood reconstruction
- hidden caller-owned loops

The product must be shaped in the same way graph composition is shaped:

- one public `compose_read(...)` entrypoint
- one canonical `ReadGraph` artifact
- one typed execution receipt
- one typed denial lane

This phase must freeze:

- the `ReadGraph` artifact produced by a bounded read request
- the execution receipt produced by running that artifact
- the denial path for unsupported depth, unsupported family, or dishonest
  locality claims
- the scope-class posture emitted by the runtime:
  - `LocalNeighborhood`
  - `AnchoredExpansion`
  - `ExplicitBroadSearch`
- the runtime distinction between:
  - query-runtime current execution
  - query-runtime historical execution
  - explicit fallback / debt classes
- the generic read-operator surface required not only for active Worth
  topology migration, but also for later trim, carrier, NURBS, fillet, and
  branch-history domains, so later widening does not force those domains back
  into local reinvention
- the read-side relationship-proof surface strongly enough that Worth can
  consume it as a real runtime contract rather than a placeholder
- the reusable `ReadFamily` layer as a required part of kernel completeness;
  Worth migration must not begin until installed/reusable read families are a
  first-class part of the same read-composition product

This phase is complete only when `forge-query` can do more than declare and
admit traversal. It must be able to execute the bounded read families Worth
actually needs through a first-class runtime product without forcing active
Worth callers back into tactical snapshot-row execution, and it must expose a
generic operator set broad enough that later domains can compose their reads
without reopening the row-product failure mode.

### Phase 2: Move Active Worth Topology Read Families Onto The Finished Kernel - Closed

Only after Phase 1 is complete may Worth freeze its domain boundary and migrate
active topology reads onto the kernel.

This phase intentionally bundles what the older draft split into "boundary",
"views", and "migration", because those surfaces are not independently honest.
Worth does not really have a domain boundary until active callers are using it.

This phase must introduce one shared Worth shape for:

- domain read-family vocabulary
- domain read-family lowering
- domain read result decoding
- breadth, fallback, and proof evidence

The boundary must be domain-owned and facade-shaped.

For topology, that means surfaces analogous in responsibility to:

- `topology_query_vocabulary/`
- `topology_query_lowering/`
- `topology_query_views/`
- `topology_query_certification/`

The exact folder names may vary, but the responsibilities may not collapse.

This phase must make the following things explicit:

- which requests are domain read families
- which requests lower to canonical Query read-graph execution
- which requests still require explicit fallback
- which requests are forbidden because they would require unbounded traversal
  or hidden broad scans

The topology-first facade ownership rule must freeze here:

- external topology certification/runtime callers depend on one `worth-topo`
  read facade
- internal helper modules stay `pub(crate)`
- no second root query story appears beside the `worth-topo` facade
- the first developer-facing feature doc draft for the generic
  `compose_read(...)` / `ReadGraph` boundary must be created or revised in
  parallel using the `feature-doc-writer` skill
- the topology-domain read families and receipts must gain matching
  developer-facing docs in parallel using the `feature-doc-writer` skill so
  engineers can use the right surface without reading certification harnesses

This phase must also define decoded, disposable, derived Worth views for the
first admitted topology read families.

The resulting view types must:

- expose domain meaning directly
- hide raw row joins and relation-string mechanics
- preserve determinism and replay-safe identity semantics
- remain clearly derived rather than authority-shaped
- expose breadth, fallback, and proof posture at the same boundary

Examples of the intended shape:

- `TopologyHalfEdgeSharedVertexNeighborhoodView`
- `TopologyHalfEdgeRadialNeighborhoodView`
- `TopologyLoopCycleView`
- `TopologyLocalRewireNeighborhoodView`

Equivalent names are acceptable if the responsibilities remain distinct.

The first mandatory migration set is:

- `BowtieAdjacentRewire`
- `AmbiguousLocalRewireContinuity`
- `BrokenRadialLocalization`
- current topology runtime relation-update support that duplicates the same
  neighbor discovery

The migration rule is:

- if the neighborhood can execute honestly through the new Query-native
  read-composition runtime surface, do that
- if any future family still needs tactical snapshot rows, consume them only
  through the topology-domain read facade with explicit fallback evidence
- no caller is allowed to reach around the new facade and assemble the old row
  joins again

This phase is complete only when Worth has one named topology read story
instead of a mixture of:

- raw row reads
- local indexes
- local witness helpers
- local traversal loops
- ad hoc per-call caches

and when the recurring topology hostility and witness lanes no longer teach
local row-join patterns to the rest of Worth.

At the end of this phase, snapshot row/index helpers should no longer appear
as the primary language in active callers. If tactical helpers still exist,
they should be visible only behind the new read facade or explicit fallback
evidence.

The derived-view rule is strict:

- no decoded Worth domain view may be storable as authority
- no decoded Worth domain view may be manually patched by callers
- if a caller needs a different neighborhood, it must issue a new request
  rather than mutating the returned view

### Phase 3: Close The Side Quest With No-N-plus-1 Proof And Debt Closure - Closed

Turn the performance claim into a closeout obligation only after active Worth
callers are genuinely on the new kernel.

This phase must introduce direct, machine-checkable query-side and Worth-side
proof surfaces for:

- scope class
- execution engine class
- lookup breadth
- traversal breadth
- fallback class
- fallback count
- locality claim versus actual breadth
- relationship proof posture
- decoded view determinism
- branch-local and replay parity for domain read views where applicable

At minimum, the side quest must name and expose counters or aggregate rows for:

- domain read request count
- local neighborhood execution count
- anchored expansion execution count
- explicit broad-search execution count
- query-native execution count
- lowered traversal count
- relationship proof admission count
- row-scan fallback count
- whole-view fallback count
- repeated rediscovery denial count
- domain read parity count

This phase must also freeze one hard rule:

`a recurring Worth read path is not allowed to hide N+1 behavior behind a domain-looking helper`

If the breadth is broad, the result surface must say so.

This phase is complete only when all of the following are true:

- the no-N+1 claim is a certification claim with explicit counters and exact
  tests rather than a code-review aspiration
- the aggregate proof surface can say which read families are truly
  Query-executed versus still falling back
- any remaining fallback consumers are explicit debt rows rather than hidden
  implementation facts
- the side quest can honestly block and then unblock Milestone 3

Named closeout contracts should include surfaces analogous to:

- `topology_read_lowering_breadth`
- `topology_read_fallback_posture`
- `topology_read_view_parity`
- `topology_read_relationship_proof_posture`

### Phase 4: Widen Domain Adoption Beyond Topology - Future Guidance

This phase is future-facing and is not allowed to distract from the
closed Phase 1-3 gate that unblocks Milestone 3 return.

Define how the already-complete generic kernel and side-quest proof surfaces
scale into later Worth domains without copying topology-specific assumptions.

This phase is about later domain-family adoption, not about delaying generic
kernel capability. The generic operator and proof widening required to survive
later trim, carrier, NURBS, fillet, and branch-history workloads belongs in
Phase 1. What waits until this phase is the domain-owned adoption of that
already-widened kernel into later Worth crates and later domain-family
vocabularies.

This phase must freeze the extension pattern for later domains such as:

- topology-to-geometry rebinding neighborhoods
- trim and carrier neighborhoods
- NURBS support neighborhoods
- fillet support and junction neighborhoods
- branch/history identity-evolution neighborhoods

The key rule is:

- shared lifecycle and proof structure may be common
- domain vocabulary and decoded views remain domain-owned

This phase is complete only when the spec can explain how later Worth crates
reuse the architectural pattern without inheriting topology-specific type names
or falling back to raw row reads again.

## Concrete Implementation Order

The implementation sequence is gated and must be followed in order:

1. `forge-query`: finish the generic read-composition kernel.
   - Introduce the public bounded read-composition execution artifact and
     receipt family.
   - Execute one admitted bounded traversal family end to end.
   - Keep expanding the generic operator, proof, and denial surface until
     active Worth topology migration no longer depends on kernel TODOs.
   - Do the generic widening here, not later:
     - shared-vertex / shared-attachment style graph operators
     - stronger frontier and anchored-expansion operators
     - broad-search operators that stay graph-native and receipt-honest
     - any other domain-agnostic operators needed so later trim, carrier,
       NURBS, fillet, and branch-history domains do not need to rebuild the
       same capability locally
   - The first ship should prefer a simple bounded loop-cycle or
     local-successor neighborhood over a broad witness family.
2. `worth-topo`: freeze the topology-domain boundary only after Step 1 is
   complete enough for honest adoption.
   - Retarget one topology-domain request family to execute through the kernel.
   - `LoopCycleNeighborhood` is the best first candidate because it is common,
     bounded, topology-pure, and easier to certify honestly than the more
     coupled witness requests.
   - Add the topology-domain facade, decoded views, and topology-domain docs in
     the same gate, because those surfaces are only honest once active callers
     can use them.
3. `worth-topo`: migrate the active hostile and runtime support callers.
   - preserve mixed-path honesty while migrating
   - one request family should be genuinely Query-executed before broader
     migrations continue
   - other families were allowed to remain explicit fallback debt temporarily,
     but the returned proof surface had to distinguish them exactly
4. `worth-topo` plus `forge-query`: close the side quest with aggregate proof.
   - certify which families are Query-executed
   - certify which families still fall back
   - certify breadth, fallback, and proof posture aggregates
   - only after this step may Milestone 3 resume
5. Only after the side quest is closed may Worth domain adoption widen to more
   coupled topology families and later-domain families:
   - shared-vertex half-edge neighborhoods
   - local rewire witness neighborhoods
   - radial witness neighborhoods
   - later trim, carrier, NURBS, and fillet neighborhoods

The hard sequencing rule is:

`do the generic widening first, do not widen Worth domain read families faster than the generic Query execution product can support honestly, and do not resume Milestone 3 until the side quest closeout proof exists`

This side quest fails if we add many more Worth read families while their
real execution story is still "lower canonically, then rebuild locally."

## Mechanical Enforcement Requirements

- the topology-domain read facade must be the only public recurring read
  boundary for topology neighborhoods
- the generic `compose_read(...)` surface must be the only public recurring
  graph-shaped read composition entrypoint at the `forge-query` layer
- lowering modules, row-decoding helpers, and fallback helpers must be
  `pub(crate)` unless the facade explicitly promotes them
- decoded domain read views must not expose public mutable fields
- unsupported domain read families must deny through typed constructors or
  typed lowering errors, not ad hoc runtime strings
- typed traversal relations should be preferred over raw relation-name strings
  wherever schema-owned constants already exist
- compile-fail or public API tests should exist for:
  - constructing a topology-domain read view directly without the proving
    builder
  - bypassing the facade to call forbidden root modules if such exposure would
    create a second read story
  - mutating proof-bearing request or view artifacts after admission when the
    contract claims the meaning is frozen
  - constructing a bounded read graph with unsupported operator shapes or
    unsupported depth without typed denial at the shared `forge-query`
    boundary

## Naive Trap Denials

The implementation must explicitly avoid these traps:

- `SnapshotIndexForever`
  - the fallback index becomes the real long-term query API in nicer clothes
- `StringlyTraversalWorth`
  - every caller reassembles relation names manually instead of consuming typed
    schema/traversal constants
- `RowProductForever`
  - row drains remain the ergonomic public product so every domain eventually
    reconstructs neighborhoods locally again
- `DomainViewAsAuthority`
  - decoded query views are retained, patched, or compared as if they were
    authoritative truth
- `HelperLoopLeak`
  - callers still receive raw rows or partial helper outputs and finish the
    neighborhood walk themselves
- `SubscriptionAmnesia`
  - traversal-bearing queries use authored traversal for one-shot reads but
    ignore the corresponding live-family and bridge-lowering posture, so later
    subscription semantics drift from read semantics
- `BroadFallbackSilence`
  - fallback to snapshot indexing or whole-view scans occurs without explicit
    breadth evidence
- `ValidatorAfterthought`
  - domain invariants stay only in Worth helper code instead of narrowing the
  admitted read graph or admission path itself
- `LocalityLie`
  - an anchored expansion or broad search is reported or wrapped as a "local
    neighborhood" because that API shape feels nicer

## Shipped Closeout Evidence

- `forge-query` read-composition kernel document and implementation plan:
  `crates/forge-query/docs/read-composition.md`
- first-class Query-native bounded read-composition execution surface:
  `workspace.compose_read(...)`,
  `workspace.compose_read_with_invariant_pack(...)`,
  `workspace.define_read_family(...)`, and
  `workspace.execute_read_family(...)`
- first-class read artifact, receipt, and typed denial lane:
  `ForgeQueryReadGraph`, `ForgeQueryReadResult`, `ForgeQueryReadReceipt`, and
  `ForgeQueryReadDenial`
- topology-first domain read-family vocabulary:
  `TopologyDomainQueryRequestFamily`
- topology-first lowering path onto the read-composition kernel plus
  relationship-proof capability:
  the migrated domain query families lower through Query-owned read families
  and expose relationship-proof posture in their request reports
- topology-first decoded read-view family:
  shared-vertex, radial, loop-cycle, and local-rewire topology-domain views
- explicit execution / fallback taxonomy:
  query-runtime current execution, query-runtime historical execution, any
  remaining fallback/debt rows, and phase-three blocker rows are exposed at the
  domain closeout boundary
- direct performance and breadth proof surface:
  no-N+1 contract rows expose lowering breadth, fallback posture, view parity,
  and relationship-proof posture
- migrated topology hostility/witness cluster:
  Milestone 3 hostile scenarios now consume the side-quest closeout and return
  gate through `certify_milestone_three_closeout()`
- matching developer-facing feature docs:
  `crates/forge-query/docs/read-composition.md`,
  `crates/worth-topo/docs/domain-reads.md`, and
  `crates/worth-topo/docs/runtime-support.md`

## Must Preserve

- `forge-query` remains the generic read-composition / traversal / runtime
  substrate
- Worth remains the owner of domain semantics, domain invariant meaning, and
  decoded domain views
- authority remains separate from derivation
- tactical snapshot row helpers may remain only as explicit debt and must not
  become shadow authority
- unsupported traversal shapes fail typed and early
- hidden broad scans remain forbidden in recurring local-claim workflows

## Acceptance Evidence

- a direct topology read-family support surface listing the first admitted
  neighborhood request families
- a canonical parity proof that equivalent Worth read requests lower to
  identical Query-backed read graphs
- admitted topology read families whose neighborhood results execute through
  the Query-native read-composition runtime surface, including snapshot
  read-only execution through the historical basis-aware path rather than
  snapshot fallback
- a topology hostility proof showing the migrated witness lanes no longer use
  caller-owned row joins
- exact breadth counters proving when a request executed natively versus fell
  back
- exact scope-class evidence proving when a request was truly local versus
  anchored expansion versus explicit broad search
- explicit denial proof for unsupported or unbounded traversal requests
- branch-local and replay parity evidence for any domain read views claimed as
  deterministic across those contexts
- aggregate proof that recurring Worth topology read workflows no longer carry
  hidden N+1 joins in their active certification/runtime paths
- explicit debt rows for any remaining tactical fallback consumers
- public developer docs that explain:
  - what `compose_read(...)` is for
  - how `ReadGraph` relates to the public API
  - how to interpret local vs anchored-expansion vs broad-search receipts
  - when not to use raw row drains

## Architectural Notes

- The intended end state is not "every Worth caller learns Forge Query
  internals." The end state is "every Worth caller composes one domain read and
  gets one domain view backed by one real read-composition substrate."
- tactical snapshot row helpers remain useful as migration and honesty tools.
  The problem is not their existence. The problem would be letting them become
  the final public or recurring query story.
- This side quest should prefer Query composition, traversal authoring,
  relationship proof admission, read-graph admission, and bridge-lowered
  subscription family selection wherever those capabilities already exist, so
  relationship storage, traversal breadth, and signal invalidation stay
  derived from declared query structure rather than from local rediscovery
  after the fact.
- The first implementation can legitimately be topology-only, but the spec must
  keep the extension boundary honest for later spatial and kernel domains.

## Sequencing Notes

- This side quest belongs after the runtime hard break because it assumes Worth
  is already consuming the real Query runtime rather than a mirror.
- It belongs before more Milestone 3 hostility widening because otherwise each
  new hostile lane will keep inventing its own neighborhood logic.
- Milestone 3 was blocked until Phase 3 of this side quest was complete.
  Phase 1 kernel work and Phase 2 Worth migration were not sufficient on their
  own; the unblock required the side quest closeout proof and explicit debt
  rows. That closeout proof now exists and is enforced by
  `certify_milestone_three_closeout()`.
- It also belongs before later spatial, NURBS, fillet, and branch-history
  query-heavy milestones because the generic kernel must be widened early
  enough to survive those domains without forcing them back into local
  reinvention, even though domain-family adoption itself still waits until
  after the side quest closeout gate.
- This is a side quest, not a replacement milestone. Milestone 3 remains the
  active topology-edit milestone, and its remaining expansion should consume
  this closed substrate rather than bypass it.
