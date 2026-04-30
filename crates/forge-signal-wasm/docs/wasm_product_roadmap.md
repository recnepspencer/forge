# forge-signal-wasm Product Roadmap

## Purpose

This document defines the remaining product roadmap for `forge-signal-wasm`
after callback-computed closeout.

It is a future-only roadmap. It does not treat the wasm package as unfinished
runtime foundation, and it does not treat later package products as incidental
helpers. It exists to sequence the remaining work required to turn the closed
callback/runtime substrate into a broader application-facing product line
without splitting reactive truth across multiple convenience layers.

The governing rule remains:

- runtime truth stays canonical
- package surfaces stay productized
- later products consume semantics; they do not redefine them

`forge-signal-wasm` may author, type, route, explain, and package host-facing
surfaces, but it must not define temporal semantics that belong to core
`forge-signal`, authority semantics that belong to `forge-relational`, or
adapter-local truth engines that drift away from the runtime.

## Shipped Baseline

The roadmap no longer tracks the already-shipped foundation as future work.

The current shipped baseline includes:

- app-first `createSignals()` runtime product surface
- callback-first `computed(() => ...)` as a real runtime-owned derived lane
- coherent `input`, `computed`, and `output` product surfaces
- runtime-scoped callback lifecycle and generation-aware callback ownership
- React consumption as a disciplined consumer of runtime truth
- typed diagnostics, history, replay, merge, and package artifact surfaces
- release-proof packaging and temp-consumer verification
- explicit same-process exact restore vs callback-unavailable portable transport
  truth

The shipped closeout reference for the latest major wasm milestone is
[host_callback_computed_spec.md](./host_callback_computed_spec.md).

## Roadmap Rules

Rules for every remaining wasm product item:

- each milestone must describe a real product capability, not just a helper API
  family
- each milestone must preserve separate authority, execution, and package
  ownership
- each milestone must preserve replay, restore, diagnostics, and boundedness
  honesty at the public boundary it introduces
- each milestone must consume core runtime semantics rather than redefining
  async, temporal, or invalidation truth in package glue
- each milestone must define concrete acceptance evidence through runtime
  proofs, package proofs, diagnostics artifacts, or product-level certification
  scenarios
- no milestone is complete until both implementation and product-facing trust
  evidence exist
- forms and resources must never become the first place where host-capability
  or async lifecycle meaning is truly defined
- package APIs, diagnostics, and type surfaces must remain clean, explicit, and
  library-grade rather than becoming bags of host-specific flags

## Critical Path

This section is the first build priority.

These are the milestones that most directly keep the next wasm products from
recreating the exact convenience-era failure mode the callback-computed
closeout just removed:

- ambient browser facts turning back into reactive folklore
- forms inventing a second store
- API resources inventing a second async and freshness engine

Even here, the wasm package is still only productizing runtime truth.
Authority, derived execution, temporal legality, async lifecycle policy, and
canonical replay/restore semantics remain in the parent runtimes.

If this section is weak, the next wasm products will inherit the classic drift
pattern:

- browser facts become ambient closure reads again
- app-facing helpers redefine semantics that should have stayed runtime-owned
- diagnostics stop at the product boundary instead of explaining the real
  lifecycle
- one product surface quietly becomes the semantic owner for every unfinished
  concept underneath it

## Dependency Gate: Async-Node Substrate

This roadmap begins only after the core async-node substrate is available from
the `forge-signal` roadmap work currently being finished on another branch.

The intended dependency order is:

1. async nodes and resource lifecycle substrate exist in core runtime truth
2. wasm product work adds host capability
3. wasm product work adds forms
4. wasm product work adds API resources / query replacement

That order is normative for this roadmap.

Before Milestone 1 begins, the following must already be true:

- async/resource lifecycle is runtime-owned
- retry, timeout, cancellation, supersession, and revalidation semantics are
  real runtime policy families rather than adapter-local conventions
- replay, restore, branch, rollback, and diagnostics parity exist for async
  work
- wasm does not need to invent a second pending/fulfilled/rejected truth model

If those conditions are not true, this roadmap must pause rather than coding
against moving semantic targets.

## Milestone 1: Host Capability Product Lane

### Goal

Add a typed host-capability lane for non-signal host-derived facts so callback
code can consume approved browser/runtime inputs without pretending ambient
closure reads are reactive truth.

### Must Ship

- a frozen wasm-facing host-capability vocabulary
- typed handles or descriptors for approved host-capability families
- explicit ownership, invalidation, registration, and disposal semantics
- diagnostics and explanation artifacts that name host-capability reads and
  invalidation causes
- replay, restore, and import/export posture for each admitted capability
  family
- counters and boundedness proof for capability registration, invalidation,
  delivery, and callback reevaluation

### Must Preserve

- host capability remains a typed lane, not ambient closure permission
- capability semantics remain runtime-truth consumers rather than React-local
  or browser-local shadow state
- unsupported host reads remain explicitly non-reactive by contract
- callback-computed purity for ordinary signal-only code remains simple and
  does not become entangled with optional host-capability machinery

### Explicit Boundary

Milestone 1 includes viewport, visibility, online/offline, timers, local
persistence-backed facts, and similar host-facing runtime inputs where the
package needs an explicit typed lane above pure signal reads.

Milestone 1 does not include letting arbitrary closure reads become reactive by
declaration, nor does it include burying host invalidation inside forms or
resource APIs. Unsupported host reads must remain visibly non-reactive rather
than being grandfathered in as convenience behavior.

### Acceptance Evidence

This milestone is complete only when the wasm product surface can prove:

- host-derived callback code does not rely on ambient closure folklore
- supported host capabilities are typed, lifecycle-owned, and diagnostics-
  visible
- restore, replay, and import behavior is explicit for each admitted capability
  family
- host-capability invalidation stays bounded and explainable at the public
  product boundary

## Milestone 2: Forms Product Surface

### Goal

Build a first-class forms surface on top of callback-computed, observation,
async, and host-capability truth.

### Must Ship

- source, draft, effective, dirty, readiness, and submission vocabulary that
  feels native in app code
- validation and submission state derived through runtime-owned signals and
  async nodes
- rollback-safe observation and diagnostics for form activity
- explicit host-capability integration where browser-local facts matter
- package-facing types and examples that teach one obvious forms story

### Must Preserve

- forms do not become a second local store
- submit lifecycle consumes async runtime truth rather than inventing local
  pending/success/error grammar
- validation and readiness remain diagnosable runtime-derived state
- the forms surface remains a consumer of callback, async, and host-capability
  truth rather than redefining any of them

### Explicit Boundary

Milestone 2 includes humane forms authoring, validation, readiness, draft
state, and submission lifecycle on top of the completed runtime substrate.

Milestone 2 does not include treating local component state as the authority
for form lifecycle, inventing a separate async submission state machine, or
letting browser-local conditions bypass the typed host-capability lane.

### Acceptance Evidence

This milestone is complete only when the wasm product surface can prove:

- forms are built as real runtime consumers rather than local UI sugar
- async submission truth, validation truth, and host-derived form facts do not
  split into separate semantic engines
- diagnostics explain form readiness, validation, submission, and rollback
  coherently
- the package examples teach a forms story that does not require tribal
  knowledge

## Milestone 3: API Resource And Query-Replacement Surface

### Goal

Build the API resource / query-replacement layer as a consumer of the completed
callback, temporal, async, and host-capability semantics.

### Must Ship

- a resource authoring surface that feels first-class in TypeScript
- explicit cache, freshness, and revalidation semantics backed by runtime
  truth
- diagnostics, replay, and restore truth for resource-backed application state
- ergonomics strong enough to replace query-library-shaped usage without
  inventing a second async truth model

### Must Preserve

- resource identity, freshness, retry, timeout, cancellation, and supersession
  remain runtime-owned semantics
- forms and resources share substrate truth rather than carrying separate async
  worlds
- host-derived revalidation facts flow through host capability rather than ad
  hoc browser checks inside resource callbacks
- resource ergonomics do not hide broad scans, broad invalidation, or broad
  refetch orchestration behind cheap-looking helper calls

### Explicit Boundary

Milestone 3 includes query-replacement-grade resource authoring, freshness,
retry/revalidation semantics, and diagnostics-rich resource state on top of the
completed substrate.

Milestone 3 does not include turning resources into the semantic owner of
host-derived facts, async policy, or freshness meaning for the rest of the
package. Resources must inherit those semantics rather than define them.

### Acceptance Evidence

This milestone is complete only when the wasm product surface can prove:

- API resources/query replacement consume the completed semantics above them
  rather than redefining them
- host-derived revalidation and async lifecycle truth flow through named typed
  product lanes
- resource diagnostics explain freshness, retry, invalidation, and visibility-
  or host-driven revalidation honestly
- the package can recommend the resource surface without a giant footnote about
  "real semantics living somewhere else"

## Roadmap Done When

This roadmap is complete only when:

- async-node core truth is present and inherited honestly
- host capability exists as a typed wasm/product lane
- forms are built as real runtime consumers rather than local UI sugar
- API resources/query replacement consume the completed semantics above them
- no milestone creates a second reactive or async truth engine beside the
  runtime
