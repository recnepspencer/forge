# forge-signal-wasm Product Roadmap

> **Status:** Active 2026-04-29
>
> **Core runtime prerequisite:** [_docs/forge_signal/forge_signal_temporal_async_roadmap.md](../../../_docs/forge_signal/forge_signal_temporal_async_roadmap.md)
>
> **Web runtime parent:** [web_runtime_spec.md](./web_runtime_spec.md)
>
> **React adapter parent:** [react_adapter_spec.md](./react_adapter_spec.md)
>
> **Callback-computed closeout parent:** [host_callback_computed_spec.md](./host_callback_computed_spec.md)

## Goal

Define the next wasm-side product sequence now that callback-computed closeout
is complete and after the upcoming async-node core work is available, so
`forge-signal-wasm`
keeps one honest application-facing trajectory instead of growing by
feature-by-feature convenience pressure.

This roadmap exists to answer:

- what comes after callback-computed maturity
- what must wait for async nodes
- what order host capability, forms, and API resources should land in
- why that order is architecturally required rather than preference

## Scope

This is a wasm and product-consumer roadmap, not a core `forge-signal` runtime
roadmap.

It plans follow-on wasm/product work that consumes the core runtime honestly:

- host-capability productization on top of the signal and async substrate
- forms surfaces on top of callback-computed, observation, and async truth
- API resource / query-replacement surfaces on top of temporal, async, and
  host-capability truth

It does not redefine:

- temporal runtime semantics
- async node lifecycle semantics
- retry / timeout / cancellation / supersession legality
- replay / restore / branch truth

Those remain core runtime responsibilities.

## Adversarial Constraint

`forge-signal-wasm` must survive the following hostile condition:

> A long-lived web runtime that mixes callback-computed nodes, host-derived
> facts, async nodes, form draft/validation/submit flows, and resource-backed
> application data must converge to the same committed truth, the same
> lifecycle classifications, and the same diagnostics explanations regardless
> of whether work was driven by signal invalidation, host-capability change,
> async completion, retry/cancellation, snapshot restore, or branch replay.

If any follow-on wasm product:

- invents host-derived semantics outside a typed runtime-owned capability lane
- treats forms as local UI state instead of runtime-owned derived and async
  truth
- treats resource/query caching as a separate truth engine parallel to the
  runtime
- smuggles retry/freshness/visibility semantics into adapter-local glue
- hides cost posture behind attractive APIs with no counters or proof artifacts

then the wasm product roadmap has failed.

## Governing Rules

- wasm/product layers consume core runtime semantics; they do not redefine
  them
- any new product surface must preserve the authority split:
  `forge-relational` owns truth, `forge-signal` owns derived execution truth,
  `forge-signal-wasm` owns host-facing authoring and typed product surfaces
- every phase must declare:
  - the runtime substrate it depends on
  - the product truth it owns
  - the diagnostics and boundedness proof it requires
- if a product need exposes a missing runtime semantic family, that family must
  be elevated explicitly rather than hidden in package glue

## Dependency Lock

This roadmap begins only after the async-node substrate is available from the
core runtime work currently being finished on another branch.

The intended dependency order is:

1. async nodes and resource lifecycle substrate exist in core runtime truth
2. wasm product work adds host capability
3. wasm product work adds forms
4. wasm product work adds API resources / query replacement

That order is normative for this roadmap.

## Why This Order

### Async Nodes First

Host capability, forms, and resources all need the async-node substrate to be
real first.

Without async nodes:

- form submission and validation lifecycles degrade into ad hoc package state
- resource loading becomes a second async engine
- timeout, retry, cancellation, freshness, and supersession semantics drift out
  of the runtime

So this roadmap does not treat those products as independent of the core async
branch. They are consumers of it.

### Host Capability Before Forms And Resources

Callback-computed closeout correctly deferred general host capability beyond
captured signal reads. That deferral should end here, before broader product
surfaces are allowed to lean on ambient browser facts.

Host capability comes first because:

- it is the missing typed lane between pure signal reads and real web-app
  products that need browser-derived facts
- forms will want honest access to focus, visibility, connectivity,
  browser-local persistence, and similar host-derived facts
- resource/query products will want honest access to online/offline state,
  visibility, timers, and other host-derived revalidation inputs
- if this lane is not typed first, both forms and resources will recreate
  ambient closure-read folklore under prettier APIs

### Forms Before Resources

Forms come before resources because forms are the smaller and more local
application product that still exercises the hard substrate honestly:

- source state vs derived draft state
- validation and submit-readiness derivation
- async submission lifecycle
- rollback-safe observation
- host-derived UX state where appropriate

Forms are therefore the first serious app-facing proof that the callback,
observation, async, and host-capability story composes into a humane product
surface.

### Resources Last

API resources and TanStack-style query replacement come last because they are
the broadest and riskiest consumer of the whole stack.

That product will need to consume, not redefine:

- async node lifecycle truth
- retry / timeout / cancellation / supersession policy families
- temporal freshness and stale-window semantics
- host capability where browser/runtime-local facts affect revalidation
- branch / replay / restore honesty
- diagnostics and boundedness artifacts

If resources land first, they are likely to become the accidental owner of:

- host-derived fetch facts
- freshness semantics
- visibility semantics
- submission semantics that forms should inherit differently

This roadmap rejects that inversion.

## Phase 0: Async-Node Prerequisite Check

### Goal

Refuse to begin wasm follow-on product work until the core async-node substrate
is actually available and honest enough to inherit.

### Must Be True

- async/resource lifecycle is runtime-owned
- retry, timeout, cancellation, supersession, and revalidation semantics are
  real runtime policy families rather than adapter-local conventions
- replay, restore, branch, rollback, and diagnostics parity exist for async
  work
- wasm does not need to invent parallel pending/fulfilled/rejected truth

### Exit Condition

The async-node branch is merged or otherwise frozen enough that the wasm
product work can name one canonical substrate instead of coding against moving
semantic targets.

## Phase 1: Host Capability

### Goal

Add a typed host-capability lane for non-signal host-derived facts so callback
code can consume approved browser/runtime inputs without pretending ambient
closure reads are reactive truth.

### Examples

- viewport/media facts
- online/offline state
- visibility/focus state
- browser-local persistence-backed facts
- clock/timer-facing host facts that belong above pure signal reads

### Must Ship

- a frozen wasm-facing host-capability vocabulary
- typed handles or descriptors for approved host-capability families
- explicit ownership, invalidation, and disposal semantics
- diagnostics and explanation artifacts that name host-capability reads
- replay/restore/import-export posture for each admitted capability family
- counters and boundedness proof for capability registration, invalidation, and
  delivery

### Must Preserve

- host capability remains a typed lane, not ambient closure permission
- capability semantics remain runtime-truth consumers, not React-local or
  browser-local shadow state
- unsupported host reads remain explicitly non-reactive by contract

## Phase 2: Forms

### Goal

Build a first-class forms surface on top of callback-computed, observation,
async, and host-capability truth.

### Must Ship

- source/draft/effective/dirty/readiness vocabulary that feels native in app
  code
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

## Phase 3: API Resources / Query Replacement

### Goal

Build the API resource / TanStack-replacement layer as a consumer of the now
completed callback, async, host-capability, and forms-adjacent semantics.

### Must Ship

- a resource authoring surface that feels first-class in TypeScript
- explicit cache/freshness/revalidation semantics backed by the runtime
- diagnostics, replay, and restore truth for resource-backed application state
- ergonomics strong enough to replace query-library-shaped usage without
  inventing a second async truth model

### Must Preserve

- resource identity, freshness, retry, timeout, cancellation, and supersession
  remain runtime-owned semantics
- forms and resources share substrate truth rather than carrying separate async
  worlds
- host-derived revalidation facts flow through host capability, not ad hoc
  browser checks inside resource callbacks

## Completion Standard

This roadmap is complete only when:

- async-node core truth is present and inherited honestly
- host capability exists as a typed wasm/product lane
- forms are built as real runtime consumers rather than local UI sugar
- API resources/query replacement consume the completed semantics above them
- no phase creates a second reactive or async truth engine beside the runtime

## Companion Documents

- [web_runtime_spec.md](./web_runtime_spec.md)
- [react_adapter_spec.md](./react_adapter_spec.md)
- [host_callback_computed_spec.md](./host_callback_computed_spec.md)
- [_docs/forge_signal/forge_signal_temporal_async_roadmap.md](../../../_docs/forge_signal/forge_signal_temporal_async_roadmap.md)
