# Milestone D Engineering Spec: Async Capability On Arbitrary Nodes

> **Status:** Planned
>
> **Roadmap parent:** [forge_signal_temporal_async_roadmap.md](./forge_signal_temporal_async_roadmap.md)
>
> **Vision parents:**
> - [forge_signals2.md](./forge_signals2.md)
> - [forge_signal_vision.md](./forge_signal_vision.md)
>
> **Architecture parent:** [signal_architecture2.md](./signal_architecture2.md)
>
> **Test requirements:** [test-requirements.md](./test-requirements.md)
>
> **Prerequisite milestone:**
> - [milestone-c-plan.md](./milestone-c-plan.md)
>
> **Primary architectural driver:** make async/resource lifecycle a
> first-class capability attachable to ordinary nodes so async semantics stop
> reading like a parallel node species while preserving the runtime law that
> graph dirtiness and async lifecycle are distinct axes of truth.

## Summary

Milestone D is not "replace resource nodes with a new abstraction."

It is:

- capability-first async semantics on ordinary node identities
- explicit separation between node evaluation state and async lifecycle state
- support for async-capable nodes at graph leaves, graph middles, and
  family/query boundaries
- support for hierarchical async composition rather than only edge-of-graph
  fetch-like nodes
- condition-, aspect-, partition-, and previous-value-aware async admission
- one descriptor-backed async policy substrate that can attach to computed,
  keyed, query-style, and resource-like nodes
- replay-, restore-, branch-, and certification-honest capability attachment
- public API and vocabulary that teach "async is a node capability" instead of
  "async lives in a different runtime species"

The governing rule is:

`a node may gain async lifecycle capability without losing its normal graph semantics`

If the runtime still forces product authors to think in terms of "plain nodes
here, special async resource nodes over there," the milestone is incomplete.

## 1. Goal

Make async capability attachable to arbitrary nodes in `forge-signal` so that:

- any node can opt into runtime-owned async lifecycle
- the core graph state machine remains `Clean | MaybeStale | Dirty`
- async lifecycle remains orthogonal runtime truth rather than becoming a new
  catch-all node-state enum
- node conditions, temporal policies, aspects, partitions, and previous-value
  semantics can shape async admission and refresh honestly
- async-capable nodes can appear anywhere ordinary nodes can appear, including
  as interior graph gates with both upstream dependencies and downstream
  dependents
- future route/query/form/background APIs can feel like node capabilities
  rather than wrappers around a second subsystem

## 2. Why This Milestone Exists

Milestones B and C deliberately closed the async substrate in resource-shaped
terms because request identity, completion denial, retry, timeout,
revalidation, observation, retention, diagnostics, and replay needed a hard
runtime owner before the abstraction could broaden safely.

That was the right sequencing choice, but it leaves one real design debt:

- the substrate already attaches to ordinary `NodeId` values
- the lifecycle and policy truth are already node-local runtime artifacts
- but the milestone language, API framing, and mental model still read like
  async belongs to a parallel node species called "resource nodes"

That gap matters because the long-term product vision is broader than
"resource-like nodes":

- query-style nodes should be able to own async refresh without leaving the
  node model
- condition-gated nodes should be able to admit async work under temporal and
  previous-value policies
- aspect-aware nodes should be able to scope async invalidation and refresh
  meaningfully
- async-capable nodes should be able to depend on other async-capable nodes so
  long as request identity, cancellation, replay, and lifecycle truth remain
  honest through the hierarchy
- higher-level products should not need to explain why some nodes are "real
  nodes" while others are "resource nodes"

Milestone D exists to make the capability model first-class without collapsing
graph execution semantics and async lifecycle semantics into one muddy state
machine.

## 3. Hard Part

The hard part is not letting more things launch async work.

The hard part is preserving one exact truth-preserving relationship among:

- ordinary node identity
- node dirtiness and validation state
- optional async capability attachment
- async request identity, generation, attempt, and branch epoch
- condition- and policy-based async eligibility
- async-capable interior node behavior where lifecycle and output can gate
  downstream work without inventing a second dependency model
- aspect- and partition-aware async refresh scope
- output continuity and observation
- replay, restore, branch, and diagnostics explanation
- public API vocabulary

The design fails if:

- `Pending` becomes a fourth generic node-state instead of lifecycle truth
- conditions and temporal gates get reinterpreted as half-lifecycle state
- async capability attachment changes ordinary invalidation law
- lifecycle state and committed output continuity collapse into one result-like
  value
- only special "resource nodes" can use the runtime-owned async substrate
- async capability is treated as valid only for graph leaves and not for
  interior or hierarchical nodes
- async capability on keyed/query/computed nodes silently bypasses request
  identity, denial law, or replay parity
- public APIs teach product authors to create a second async abstraction layer
  instead of reusing the node capability substrate

## 4. Explicit Assumptions

- Milestone A remains the only accepted temporal substrate.
- Milestone B remains the only accepted async/resource lifecycle substrate.
- Milestone C remains the only accepted async/resource policy substrate.
- async capability attaches to node identity; it does not replace graph node
  identity.
- async lifecycle is runtime-owned derived truth; host truth remains outside
  `forge-signal`.
- node evaluation semantics and async lifecycle semantics remain orthogonal.
- this milestone is still core-only; wasm, query, route-resource, form, and UI
  ergonomics remain later consumers.

## 5. Governing Summaries

- `MENTALITY.md`
  The most important thing it protects here is not confusing a conceptual
  improvement with a real one. Milestone D must not re-skin the existing
  resource substrate while leaving the mental model and proof obligations
  unchanged.
- `arch_laws.md`
  The most important laws here are 7, 8, 17, 20, 21, 24, 30, 36, 40, and 41.
  Categories must stay distinct, planning must precede execution, resource-like
  async truth must stay framework-owned, replay and restore must remain honest,
  and proof-bearing types must say what has actually been established.
- `perf_laws.md`
  The most important thing it protects is that broader capability attachment
  must not widen admission, completion, or replay work into graph-wide scans or
  hidden adapter glue.
- `domain_laws.md`
  The most important thing it protects is subsystem shape. Capability
  attachment, lifecycle truth, async eligibility, async-capable node families,
  and migration/API vocabulary need explicit homes instead of one vague
  "resource generalization" helper.
- `forge_signal_vision.md`
  The most important thing it protects is the long-term runtime thesis: derived
  computation is one graph with capabilities, not a pile of neighboring special
  abstractions.
- `forge_signals2.md`
  The most important thing it protects is that conditional execution,
  observability, replay, planning, and future query-style runtime behavior are
  all meant to compose in one substrate.
- `signal_architecture2.md`
  The most important thing it protects is proof-bearing pipeline structure and
  contract-first planning. Capability attachment must lower before execution.
- `forge_signal_temporal_async_roadmap.md`
  The most important thing it protects is sequencing. This milestone belongs
  after policy-family closeout because it broadens substrate expression rather
  than inventing lifecycle or policy truth.
- `test-requirements.md`
  The most important thing it protects is that arbitrary-node async capability
  still has to satisfy the hostile temporal and async grammars rather than
  creating a softer second path around them.

## 6. Adversarial Constraint

Milestone D must survive the following hostile condition:

> A branchable, replayable runtime with deterministic execution,
> condition-gated nodes, aspect-aware invalidation, previous-value-sensitive
> nodes, and runtime-owned async lifecycle must converge to the same committed
> truth and the same denial/explanation artifacts regardless of whether async
> capability is attached to a classic resource-style node, a computed node, a
> keyed/query-style node, or a condition-gated node.

Concretely, the design must remain correct when all of the following are true:

- a node with async capability is also `OnDemand`, `Debounce`, or
  `StaleAfter`-gated
- async admission depends on previous committed value plus temporal window
- async-capable keyed/query families share policy descriptors but not request
  identity
- aspect-local changes should refresh only the async-capable partitions that
  actually subscribed
- a branch restore replays the same async-capable node story under different
  API entrypoints
- the same node shape is accessed through both capability-first and legacy
  resource-shaped public APIs
- replay-compatible policy history exists but capability attachment changed
  public shape

If any supported path makes async legality depend on which public API shape was
used, or on whether the node was mentally classified as "resource" versus
"ordinary," the milestone has failed.

## 7. Product Decision Lock

- async is a capability attachable to arbitrary nodes
- node dirtiness remains `Clean | MaybeStale | Dirty`
- async lifecycle remains orthogonal to node dirtiness
- conditions govern async admission or refresh eligibility; they do not become
  lifecycle states
- async-capable interior nodes may act as graph gates, but they do so through
  ordinary dependency semantics plus lifecycle/output truth, not through a
  special hidden execution model
- aspects and partitions may scope async refresh and observation without
  changing request identity law
- previous-value and temporal semantics may shape async admission but must still
  replay identically
- legacy resource-shaped APIs may remain as compatibility vocabulary, but they
  must lower through the same capability substrate
- hierarchical async composition is allowed, including async-capable nodes
  depending on other async-capable nodes, so long as cancellation, retry,
  revalidation, replay, and observation remain one coherent runtime story
- no product layer may need a second async truth model once this milestone is
  complete

Normative consequence:

- any implementation that adds generic `Pending` to the ordinary node-state
  machine is out of spec
- any implementation that keeps async capability exclusive to a mentally
  separate node species is out of spec
- any implementation that lets capability attachment bypass Milestone B/C
  request identity, denial, retention, observation, or replay law is out of
  spec
- any implementation that makes condition resolution part of lifecycle truth
  instead of admission truth is out of spec

## 8. Scope

### 8.1 In Scope

- explicit async-capability attachment on ordinary nodes
- lowered descriptors for async-capable nodes independent of public API shape
- capability-aware node declaration and builder vocabulary
- condition-, temporal-, and previous-value-aware async admission on ordinary
  nodes
- aspect-/partition-aware async-capable node behavior
- async-capable keyed/query/computed node families
- async-capable interior nodes that can gate downstream work through lifecycle,
  output continuity, or both
- hierarchical async-capable node composition, including dependent cancellation
  and replay/restore parity through the hierarchy
- migration/compatibility surface between legacy resource-shaped APIs and the
  capability-first API story
- diagnostics, replay, restore, and explanation updates required to make the
  capability model visible and honest

### 8.2 Explicitly Out Of Scope

- wasm bindings
- route-resource APIs
- form/action APIs
- browser query ergonomics
- network transport products
- app-level loading UI design
- domain-specific cache products above the core async-capability substrate

## 9. Current-State Assessment

The current substrate is closer to this milestone than the milestone docs
currently admit:

- async/resource identity is already attached to ordinary `NodeId` values
- the runtime already owns lifecycle truth, denial law, replay, restore,
  diagnostics, and performance envelopes
- policy registries already describe replay-critical async behavior

The current gap is mostly architectural framing and capability composition:

- milestone language still teaches "resource nodes" as a separate conceptual
  species
- builder/declaration language does not yet put async capability on equal
  footing with aspects, conditions, comparators, keyed families, and partitions
- arbitrary-node async composition with conditions/aspects/previous-value is
  not yet a first-class spec target
- public API vocabulary is still more subsystem-first than capability-first

This means the substrate is not wrong. It is incomplete relative to the
intended product model.

## 10. Architecture Rules For This Milestone

### 10.1 Capability Attachment Must Be Explicit

Async capability must be declared on node construction or node-family
construction through typed capability declarations.

The capability declaration must lower into the same frozen descriptor and
policy pipeline already used by the resource substrate.

### 10.2 Node Dirtiness And Async Lifecycle Must Stay Separate

The runtime must preserve two distinct axes:

- graph evaluation state
- async lifecycle state

They may influence each other, but they may not be collapsed into one enum or
one façade value that hides which truth actually changed.

### 10.3 Conditions Govern Admission, Not Lifecycle

`OnDemand`, `Debounce`, `Throttle`, `StaleAfter`, delta-threshold, and
previous-value-sensitive gates may shape:

- whether async work is admitted
- whether revalidation happens
- whether refresh is deferred

They may not themselves become lifecycle classifications.

### 10.4 Aspects And Partitions Must Compose With Async Capability

An async-capable node may subscribe to aspect-local or partition-local change
and admit or refresh work on that narrower basis.

This composition must still preserve request identity, generation, denial law,
and replay parity.

### 10.5 Interior Async Nodes Must Stay In The Graph Model

An async-capable node may appear:

- at a graph leaf
- in the middle of the graph
- at a keyed/query family boundary

When it appears in the middle of the graph, it may act as a gate for
downstream work through:

- lifecycle truth
- committed output truth
- output continuity posture

But it must still obey ordinary dependency semantics. The runtime must not
invent a second hidden graph model for "async gate nodes."

### 10.6 Async-Capable Families Must Be First-Class

Keyed families, query-style families, and computed families must be able to
attach async capability without escaping the node model.

Family-local policy lowering is acceptable; family-local truth models are not.

### 10.7 Hierarchical Async Composition Must Remain Honest

Async-capable nodes may depend on other async-capable nodes.

The runtime must define and later certify honest behavior for at least:

- parent/child cancellation propagation
- replay and restore of multi-level async hierarchies
- upstream async lifecycle change gating downstream async admission
- observation and output continuity across async-capable dependency chains

Hierarchical composition is a first-class target, not an accidental byproduct.

### 10.8 Legacy Resource Vocabulary Must Reduce To Capability Vocabulary

The milestone may preserve compatibility aliases such as
`ResourceNodeId::from_node(...)`, but the architectural truth must be:

- node identity first
- capability attachment second
- lifecycle semantics third

Public docs and APIs should stop teaching the opposite order.

## 11. Phase Breakdown

### Phase 1: Capability Model And Descriptor Unification

- define async-capability declarations for ordinary nodes
- lower them through the existing resource/policy substrate
- make legacy resource-shaped declarations compatibility shims over the same
  descriptor truth

### Phase 2: Eligibility And Condition Composition

- define how conditions, temporal policies, and previous-value policies govern
  async admission on ordinary nodes
- ensure replay/restore parity for those combined gates

### Phase 3: Aspect, Partition, And Family Composition

- make async-capable keyed/query/computed families first-class
- define aspect-local and partition-local async refresh semantics
- define interior async-node semantics for lifecycle/output gating in the
  middle of dependency chains
- certify that family and partition breadth remain bounded

### Phase 4: Hierarchical Composition And Graph-Gate Closeout

- certify multi-level async-capable dependency chains
- define cancellation/retry/revalidation/observation behavior across those
  chains
- prove replay/restore parity for async-capable interior nodes and hierarchies

### Phase 5: Public API And Vocabulary Sweep

- update builders and facades so async capability reads like a node capability
- preserve compatibility aliases where useful
- remove wording that implies a second node species

### Phase 6: Diagnostics, Replay, And Historical Closeout

- certify that explanation, replay, restore, branch, and retained history all
  tell the same story for capability-attached async nodes as for legacy
  resource-shaped paths

## 12. Must Preserve

- deterministic execution remains a product contract
- commit-bounded observation remains unchanged
- rollback remains hard rewind
- authority stays outside `forge-signal`
- Milestone A temporal law remains the only time substrate
- Milestone B lifecycle law remains the only lifecycle substrate
- Milestone C policy law remains the only policy substrate
- request identity, generation, attempt, branch epoch, and denial categories
  remain distinct
- lifecycle truth remains distinct from output continuity truth
- diagnostics richness remains distinct from committed runtime truth

## 13. Performance Contracts

The milestone must expose named counters for at least:

- async capability attachment count
- async-capable node declaration count
- async-capable family declaration count
- async capability descriptor lowering count
- condition-governed async admission count
- previous-value-governed async admission count
- aspect-local async refresh count
- partition-local async refresh count
- interior async gate admission count
- hierarchical async propagation count
- async capability compatibility alias lowering count
- async capability broad-scan denial count

The milestone must also declare named complexity contracts for:

- capability attachment lowering
- condition-aware async admission
- aspect-local async refresh selection
- interior async gate evaluation/admission coordination
- family-local async admission
- hierarchical async replay/restore reconstruction
- replay/restore reconstruction for capability-attached async nodes
- compatibility alias lowering and validation

## 14. Acceptance Evidence

Milestone D is complete only when `forge-signal` can certify all of the
following with canonical machine-checkable artifacts:

- `Async Capability Attachment Equivalence Test`
- `Condition-Gated Async Admission Parity Test`
- `Aspect-Scoped Async Capability Test`
- `Previous-Value And Temporal Async Capability Parity Test`
- `Interior Async Node Gate Equivalence Test`
- `Hierarchical Async Capability Replay And Cancellation Test`
- `Legacy Resource Alias Compatibility Test`

The final closeout must prove:

- capability-first and legacy resource-shaped APIs lower to the same canonical
  descriptor truth when they mean the same thing
- node dirtiness digests remain independent from async lifecycle digests
- condition/temporal/previous-value gating changes admission truth without
  mutating lifecycle categories dishonestly
- interior async-capable nodes preserve one dependency model and do not invent
  hidden gate-only semantics
- hierarchical async-capable nodes preserve replay, restore, cancellation, and
  observation parity through dependency chains
- family- and aspect-scoped async breadth remains bounded and attributable

## 15. Sequencing Notes

This milestone belongs after Milestone C because:

- lifecycle truth had to close before broadening the capability model
- policy truth had to close before capability attachment could become general
- adversarial temporal and async certification had to exist before the public
  abstraction widened

Milestone D is the first follow-on that should feel explicitly product-shaping
again. It does not reopen B/C law. It expresses that law through a better
capability model.

## 16. Milestone Done When

Milestone D is done only when `forge-signal` can support async capability on
arbitrary nodes through a frozen, typed, replay-honest substrate that:

- preserves one node model
- preserves separate dirtiness and lifecycle truth
- composes honestly with conditions, aspects, partitions, keyed families, and
  previous-value semantics
- supports async-capable interior nodes and hierarchical async dependency
  chains without inventing a second graph model
- keeps legacy resource-shaped APIs as compatibility vocabulary rather than as
  architectural truth
- lets future product layers say "this node is async-capable" without having
  to explain a second semantic species

At that point, the runtime story matches the intended product vision:

> async is a property a node can have, not a separate universe living beside
> the graph.
