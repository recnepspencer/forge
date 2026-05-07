# Worker-First Runtime Placement And Main-Thread Host Bridge Plan

> **Status:** Planned engineering spec
>
> **Roadmap parent:** [wasm_product_roadmap.md](./wasm_product_roadmap.md)
>
> **Web runtime parent:** [web_runtime_spec.md](./web_runtime_spec.md)
>
> **Product prerequisites:**
>
> - [host_callback_computed_spec.md](./host_callback_computed_spec.md)
> - [host_capability_closeout.md](./host_capability_closeout.md)
> - [api_surface_closeout.md](./api_surface_closeout.md)
> - [router_navigation_projection_plan.md](./router_navigation_projection_plan.md)
>
> **Core lineage:**
>
> - [_docs/forge_signal/forge_signal_vision.md](../../../_docs/forge_signal/forge_signal_vision.md)
> - [_docs/forge_signal/forge_signal_temporal_async_roadmap.md](../../../_docs/forge_signal/forge_signal_temporal_async_roadmap.md)
> - [_docs/forge_signal/milestone-b-plan.md](../../../_docs/forge_signal/milestone-b-plan.md)
> - [_docs/forge_signal/milestone-d-closeout.md](../../../_docs/forge_signal/milestone-d-closeout.md)
> - [_docs/forge_signal/milestone-11-closeout.md](../../../_docs/forge_signal/milestone-11-closeout.md)
>
> **Test requirements:** [worker_runtime_test_requirements.md](./worker_runtime_test_requirements.md)
>
> **Primary architectural driver:** make worker-first deployment the canonical
> `forge-signal-wasm` execution posture so runtime-owned derived work leaves the
> UI thread without creating a second main-thread truth engine or pretending
> that browser-only host capabilities and live JavaScript closures are portable
> runtime data.

## Summary

Milestone 9 makes worker-first execution a first-class `forge-signal-wasm`
product capability.

This milestone is not "spawn a worker and proxy some calls."

It is:

- a worker-owned runtime authority for graph state, invalidation,
  recomputation, async/resource lifecycle, routing continuity, forms/resource
  continuity, history, replay coordination, and diagnostics production
- a typed main-thread host bridge for browser-owned host facts, host-side
  effects, committed public delivery, and lifecycle/disposal coordination
- a proof-bearing placement taxonomy that distinguishes worker-executable work,
  main-thread-hosted work, and typed unavailable work
- bounded transaction-, capability-, and observation-level bridge envelopes
  instead of per-node chatter
- replay-, restore-, import-, and export-honest capability artifacts
- counters and certification that prove worker-first deployment reduces
  main-thread operational breadth instead of merely relocating compute while
  leaving hidden serialization and coordination tax behind

The governing rule is:

`place runtime truth once, bridge host boundaries once, deny dishonest placement explicitly`

If worker support leaves the main thread as a second lifecycle/cache/router/
resource authority, the milestone is incomplete.

## 1. Goal

Make dedicated worker deployment the canonical `forge-signal-wasm` execution
posture so that:

- most runtime-owned derived work no longer runs on the UI thread
- browser-only host facts remain explicit typed inputs instead of ambient reads
- host-side effects remain explicit main-thread execution boundaries
- main-thread compatibility mode and worker-first mode preserve one semantic
  truth story
- callback-first ergonomics and worker honesty can coexist without pretending
  that arbitrary live JavaScript closures are portable runtime data
- bridge cost stays bounded by semantic delta and committed public delivery
  rather than by total graph breadth

## 2. Why This Milestone Exists

The earlier wasm milestones closed the product semantics needed for serious
application use:

- host capability closed ambient browser-read folklore
- controller and graph ownership closed composition and lifecycle drift
- opaque identity and authoring cleanup made app-facing graphs humane
- forms and API surfaces now consume runtime-owned async and lifecycle truth
- router work closes navigation, redirect, and route continuity as graph truth

That success creates the next real product problem:

- the package can now represent large long-lived application graphs honestly
- those graphs can include expensive derived outputs, resource churn, route
  churn, diagnostics, and history
- leaving the bulk of that runtime work on the main thread turns semantic
  success into UI-freeze failure

The naive answer is "move the heavy parts to a worker."

That is not good enough.

The actual risk is deeper:

- a worker can become a cosmetic sidecar while the main thread keeps local
  lifecycle truth
- callback-first computed can be lied about as worker-portable when the closure
  is really a process-local host capability
- browser history, visibility, online/offline, viewport, timers, and DOM
  effects can leak back into ambient reads or imperative side channels
- output delivery and diagnostics can become the new hidden hot path even if
  recompute moved off-thread

This milestone exists to solve worker placement as a truth-boundary problem,
not as an optimization garnish.

## 3. Hard Part

The hard part is not posting messages to a worker.

The hard part is freezing one exact truth-preserving relationship among:

- worker-owned runtime state
- typed transaction admission
- host-capability ingress from the main thread
- callback/computation placement eligibility
- host-side effect execution
- committed output and observation delivery
- diagnostics/history/read surfaces
- branch, restore, replay, export, and import capability honesty
- worker-first and main-thread compatibility semantic parity

The design fails if:

- the main thread keeps a second lifecycle, router, resource, or freshness
  authority for convenience
- a worker-ineligible callback silently pins unrelated graph breadth to the
  main thread
- browser-only host facts turn back into ambient closure reads
- output or diagnostics delivery reintroduces broad UI-thread work behind
  cheap-looking APIs
- replay and restore preserve values while losing the worker/main-thread
  capability story that produced them
- the same graph means one thing in worker-first mode and another thing in
  compatibility mode
- bridge traffic scales with graph size rather than with changed host or public
  delivery surface
- "worker support" still allows runtime-owned work to freeze the UI thread
  because serialization, effect churn, or delivery breadth stayed broad

## 4. Explicit Assumptions

- `forge-signal` remains the owner of derived computation, async lifecycle,
  temporal legality, observation truth, diagnostics truth, rollback, branch,
  and replay semantics.
- `forge-signal-wasm` remains the owner of web-facing authoring, host callback
  marshalling, host capability integration, browser-history integration, and
  product-facing output/effect surfaces.
- browser APIs such as DOM, `window`, history mutation, visibility, viewport,
  and other browser-owned host facts remain main-thread authority boundaries
  unless a browser API is actually worker-available and admitted explicitly.
- async-node runtime substrate, policy families, and async capability on
  ordinary nodes are already closed in core runtime truth and may be consumed
  here.
- callback-first authoring remains a product requirement; worker-first
  placement may not solve portability by forcing users back to raw low-level
  recipe authoring alone.
- live JavaScript closures remain process-local host capabilities unless they
  are lowered into a separately admitted worker-executable representation.
- main-thread compatibility mode remains a supported posture for environments
  where worker execution is unavailable or intentionally disabled, but it is no
  longer the preferred heavy-application posture.
- the milestone is package/runtime only; service-worker sync, cross-tab
  distributed coordination, and general closure source serialization remain out
  of scope unless explicitly admitted.

## 5. Governing Summaries

- `MENTALITY.md`
  The most important thing it protects here is adversarial-constraint-first
  infrastructure design. Worker placement must start from UI-freeze, truth
  split, replay dishonesty, and bridge-breadth failure rather than from "web
  workers seem faster."
- `arch_laws.md`
  The most important laws here are 7, 9, 20, 21, 24, 27, 30, 33, 34, 35, 37,
  and 41. Cross-thread boundaries must use self-describing envelopes,
  lifecycle propagation must be mechanically enforced, orchestration boundaries
  must stay explicit, diagnostics must stay separate from operational truth,
  eligibility must precede construction, execution must consume lowered plans,
  authority and derivation must stay separate, framework-owned resources must
  remain lifecycle-managed, and proof-bearing placement categories must be
  typed rather than conventional.
- `perf_laws.md`
  The most important thing it protects is breadth honesty across the bridge.
  Worker support must not hide broad serialization, broad public delivery,
  broad host-capability churn, or per-node RPC behind cheap-looking APIs.
- `domain_laws.md`
  The most important thing it protects is subsystem shape. Worker runtime
  placement, host-capability ingress, host-effect egress, placement
  classification, diagnostics/history boundary reads, and compatibility mode
  must each have distinct responsibility homes instead of one broad worker
  helper layer.
- `forge_signal_vision.md`
  The most important thing it protects is that `forge-signal` remains derived
  execution substrate, never truth storage. Worker placement must move derived
  runtime authority, not create a second web-local truth engine.
- `wasm_product_roadmap.md`
  The most important thing it protects is sequencing and boundary honesty.
  Worker-first placement belongs after host capability, graph lifecycle, API,
  and router semantics close so one stable product truth can move wholesale
  instead of each feature lane inventing its own background engine.
- `test-requirements.md`
  The most important thing it protects is proof discipline at the package
  boundary. Worker support is not closed when demos feel smooth; it is closed
  when main-thread compatibility mode and worker-first mode converge under
  hostile replay/restore/branch/host-churn workloads with named artifacts and
  cost proof.
- `web_runtime_spec.md`
  The most important thing it protects is the framework-agnostic web-runtime
  thesis. Workers must become part of the real wasm product surface without
  turning React or another framework into the hidden semantic owner.
- `host_callback_computed_spec.md`
  The most important thing it protects is callback honesty. Live closures are
  process-local host capabilities; worker placement must classify and lower
  them honestly rather than pretending they are portable runtime data.
- `router_navigation_projection_plan.md`
  The most important thing it protects is explicit host-boundary routing.
  Browser history, direct URL mutation, speculative navigation, and route
  continuity must remain typed graph-consumed truth even when runtime execution
  moves off-thread.
- `forge_signal_temporal_async_roadmap.md`
  The most important thing it protects is that async lifecycle, retry, timeout,
  supersession, revalidation, and temporal meaning are already runtime-owned.
  This milestone must move those semantics into worker placement, not
  reinterpret them in package glue.
- `milestone-b-plan.md`
  The most important thing it protects is the exact style of lifecycle-owned
  substrate this spec should model. Host work may exist, but admission,
  legality, denial, diagnostics, replay, and boundedness must remain
  runtime-owned with proof-bearing types and hostile certification.
- `milestone-d-closeout.md`
  The most important thing it protects is that async capability now composes
  with ordinary nodes. That means a worker-owned runtime can move not just
  resource lanes but arbitrary async-capable derived work off the UI thread.
- `milestone-11-closeout.md`
  The most important thing it protects is committed observation truth.
  Worker-first delivery must reduce to the existing commit-bounded observation
  contract instead of inventing a second UI-thread observer engine.

## 6. Adversarial Constraint

Milestone 9 must survive the following hostile condition:

> A long-lived web application with large callback-authored and graph-published
> derived state, async-capable nodes, route churn, form activity, resource
> refresh and delivery churn, browser-history events, visibility/online/timer/
> viewport host updates, branch restore and replay activity, mixed worker-
> executable and main-thread-hosted authored work, and high observation/effect
> churn must converge to the same committed runtime truth, the same lifecycle
> truth, the same visible output truth, and the same diagnostics/history
> explanation whether it runs in main-thread compatibility mode or worker-first
> mode, while keeping main-thread work bounded to typed host-boundary
> admission, explicit main-thread-only effect execution, and committed public
> delivery.

Concretely, the design must remain correct when all of the following are true:

- one graph contains both worker-executable nodes and callback-hosted or
  main-thread-only work
- invalidation storms and async completions overlap with host-capability churn
- browser-history events race app-issued navigation and speculative branch
  navigation
- a branch restore occurs before and after resource completion, host updates,
  and output delivery
- equivalent runs use worker-first mode and main-thread compatibility mode
- some callbacks are worker-ineligible, duplicated, stale, missing, or no
  longer available at restore/import time
- diagnostics richness changes between equivalent runs
- output graphs are large and structured enough that serialization and delivery
  cost matter materially

If any supported path:

- creates a second lifecycle/cache/router/resource authority on the main thread
- silently broadens worker-ineligible work into whole-graph main-thread
  placement
- hides broad bridge or serialization work behind cheap-looking APIs
- loses replay/restore/import capability honesty
- changes committed semantics between worker-first and compatibility mode
- or still allows runtime-owned work to freeze the UI thread

then the milestone has failed.

## 7. Product Decision Lock

- worker-first deployment is the preferred serious-application posture
- main-thread compatibility mode remains explicit and supported, but secondary
- worker placement is a runtime-authority choice, not an optional optimization
  helper
- browser-only host facts remain typed host-capability inputs and may not
  return to ambient closure reads
- host effects that touch DOM, browser APIs, framework-owned mutable state, or
  other main-thread-only platform objects remain explicit main-thread
  execution boundaries
- runtime-owned invalidation, recomputation, async lifecycle, route/resource/
  form continuity, history, replay coordination, and diagnostics production
  belong in the worker whenever the graph admits that placement honestly
- authored work must be classified before execution as:
  - worker-executable
  - main-thread-hosted
  - unavailable or denied
- worker-ineligible work must not silently pin unrelated graph breadth to the
  main thread
- callback-first authoring remains supported, but live closures are not treated
  as portable worker data by default
- any worker-executable callback lane must lower through an admitted portable
  representation rather than relying on folklore about source serialization
- main-thread-hosted execution is a narrow host-boundary lane, not a general
  derived-computation escape hatch:
  - it is allowed only for explicit host-boundary execution shapes that consume
    closed worker-issued requests and produce typed results or denial artifacts
  - it may not become the normal placement for arbitrary interior derived graph
    nodes
- denial is the default response when requested worker posture cannot be proven
  honestly
- fallback is allowed only where the product surface declares it explicitly and
  emits a first-class fallback artifact; hidden compatibility fallback is out of
  spec
- committed output delivery, observation delivery, diagnostics/history reads,
  and lifecycle/disposal updates must cross the bridge through typed envelopes
- bridge batching/coalescing is part of the semantic contract, not an optional
  optimization pass
- canonical identity for parity and history is based on:
  - declaration identity
  - placement identity
  - lowered-plan identity
  - capability attachment, denial, detach, and unavailability artifact identity
- parity, suppression, replay, restore, import, and export may not compare
  friendly names, pointer identity, closure object identity, or diagnostic text
- worker-host causality is transaction- and generation-ordered, with each
  boundary envelope carrying enough causal identity to place it within the
  canonical runtime history
- host-effect acknowledgements and main-thread-hosted execution results are not
  independent truth authorities:
  - they may report typed completion, failure, denial, detach, or unavailable
    results
  - any runtime state transition caused by them must occur only after the worker
    admits the typed result envelope into canonical runtime authority
- worker topology for this milestone is one dedicated worker-owned runtime
  authority per runtime instance, not a pool of helper workers and not a
  main-thread runtime with optional background helpers
- replay, restore, import, and export must preserve capability posture
  explicitly:
  - worker-executable
  - main-thread-hosted
  - unavailable

Normative consequence:

- any implementation that keeps route/resource/forms continuity truth on the
  main thread for convenience is out of spec
- any implementation that evaluates browser-only host facts inside worker code
  by ambient access is out of spec
- any implementation that proxies per-read or per-node host traffic across the
  boundary as the normal hot path is out of spec
- any implementation that quietly reuses stale callback-derived values when the
  required capability is missing is out of spec
- any implementation that defaults to main-thread fallback without an explicit
  product-declared fallback artifact is out of spec
- any implementation that treats host acknowledgements or host-execution results
  as direct truth mutation outside worker admission is out of spec
- any implementation that compares names or object identity instead of canonical
  declared identities for parity or history is out of spec
- any implementation that turns main-thread-hosted execution into a general
  interior derivation engine is out of spec
- any implementation that claims worker support while still forcing broad
  serialization or observer work on the main thread is out of spec

## 8. Scope

### 8.1 In Scope

- worker-owned runtime lifecycle for graph state, invalidation, recomputation,
  async/resource lifecycle, route/resource/forms continuity, replay
  coordination, and diagnostics production
- worker bootstrap, runtime ownership, and graph publication posture
- typed bridge envelopes for transaction submission, host-capability ingress,
  output/observation delivery, host-effect egress, diagnostics/history reads,
  and lifecycle/disposal updates
- placement taxonomy and proof-bearing placement eligibility forms
- callback/computation placement classification and honest denial/fallback
  artifacts
- browser-history and host-capability boundaries as main-thread ingress to a
  worker-owned runtime
- replay/restore/import/export capability artifacts for worker-first posture
- bridge batching/coalescing rules and performance counters
- certification families proving parity between worker-first and compatibility
  mode

### 8.2 Explicitly Out Of Scope

- general serialization of live JavaScript callback source or closure capture
- ambient worker access to DOM, `window`, or framework-owned mutable objects
- service-worker persistence, cross-tab distributed worker coordination, or
  offline sync products
- arbitrary main-thread local caches to "improve rendering"
- productizing a compile transform unless a later phase explicitly admits one
- changing core async, temporal, router, form, or resource semantics that are
  already owned by parent runtime or prior wasm milestones

## 9. Current-State Assessment

The package is structurally ready for this milestone in several important ways:

- host capability is now a typed product lane rather than ambient browser-read
  folklore
- callback computed, graph publication, and graph-owned lifecycle are real
  product surfaces
- async-node lifecycle, policy truth, and async capability on arbitrary nodes
  now exist in core runtime truth
- API/resource and router work have already closed one coherent lifecycle and
  continuity story at the package boundary
- committed observation, rollback suppression, diagnostics, and branch/restore
  truth are already real substrate, not aspirations

The missing worker category is still real:

- the package does not yet freeze one canonical worker-owned runtime posture
- browser host ingress and host-effect egress are not yet closed as a typed
  worker bridge
- callback placement eligibility is not yet a proof-bearing product boundary
- replay/import/export do not yet tell one frozen worker/main-thread capability
  story
- no package-level certification currently proves that worker-first mode and
  compatibility mode converge under hostile churn
- no public performance envelope currently proves that moving work off-thread
  actually narrows main-thread cost instead of relocating hidden coordination

That means the product now has enough semantic substrate to move into a worker
honestly, but not yet the placement architecture that makes that move truthful.

## 10. Architecture Rules For This Milestone

### 10.1 Worker Placement Is Runtime Authority Placement, Not Generic Offload

Worker placement must mean the worker owns the canonical runtime state and
execution path for the admitted graph, not that a main-thread runtime asks a
worker to do occasional compute jobs.

Acceptable:

- one worker-owned runtime instance is the execution authority
- transactions, invalidation, recomputation, async lifecycle, route/resource/
  forms continuity, and diagnostics production happen inside that authority
- the main thread submits typed requests and consumes typed committed results

Not acceptable:

- main thread keeps the authoritative graph and worker computes side values
- route/resource/freshness truth lives on the main thread because rendering is
  nearby
- worker work is advisory while main-thread state decides the real lifecycle

Required consequence:

- worker-first mode has one canonical runtime authority
- compatibility mode and worker-first mode differ in placement, not in meaning
- the bridge is a boundary around authority, not a convenience callback layer

### 10.2 Main Thread And Worker Must Exchange Typed Boundary Envelopes

Cross-thread traffic must use typed, self-describing envelopes rather than ad
hoc RPC argument bags.

At minimum, the milestone must freeze distinct boundary families for:

- transaction submission
- transaction result and committed-effect summary
- host-capability update batches
- browser-history and host-navigation events
- host-effect execution requests and acknowledgements
- output and observation delivery packets
- diagnostics/history/export/import requests and responses
- lifecycle/disposal/detach events
- capability denial, fallback, and unavailability artifacts

Required consequence:

- each envelope is sufficient to reconstruct what happened without querying
  hidden shared state
- boundary categories with different failure or cost semantics remain distinct
- a cheap-looking call cannot hide a broad orchestration boundary
- every envelope family carries the canonical causal identity needed to place it
  within transaction and generation ordering
- result or acknowledgement envelopes from the host are admissible evidence only
  after worker-side canonical admission; receiving a message does not itself
  mutate truth

### 10.3 Placement Eligibility Must Be Proven Before Execution

Worker placement must never be discovered late in the hot path.

Authored work must lower through a placement-classification boundary that proves
whether the work is:

- worker-executable through an admitted portable representation
- main-thread-hosted through an explicit host-execution capability
- denied or unavailable for the requested deployment posture

Required consequence:

- worker placement legality is decided before runtime execution begins
- worker-first mode does not guess, then silently fall back after partial work
- later phases consume placement-bearing handles or descriptors rather than raw
  callbacks or loosely typed declarations

Normative proof chain:

- raw authored declarations may exist only at the authoring boundary
- raw declarations must lower into a sealed placement-classified form before any
  worker-first publication or execution path may consume them
- placement-classified forms must lower into exactly one sealed execution-plan
  family:
  - worker execution plan
  - host execution plan
  - denial or unavailable artifact
- only lowered execution-plan families may cross into worker runtime admission,
  replay or restore reconstruction, or host-execution routing
- constructors for placement-classified forms, lowered execution plans, and
  boundary-envelope proofs must be private to the owning proving modules so
  external code cannot forge them structurally
- any compatibility or worker-first API that still accepts raw callbacks past
  the proving boundary is out of spec

### 10.4 Worker-Ineligible Work Must Not Collapse Unrelated Graph Breadth

One main-thread-only node or effect may not silently pin an otherwise
worker-admissible graph to the main thread.

Required consequence:

- placement is tracked at the smallest structurally honest boundary the runtime
  can maintain
- worker-ineligible work is isolated through typed host-execution lanes or
  explicit denial
- unrelated worker-admissible invalidation and recomputation remain in the
  worker
- any aggregate fallback that broadens placement must be explicit, measured,
  and treated as debt or denial rather than hidden behavior

### 10.5 Host Capability Remains Main-Thread Authority Input

Browser-owned facts remain main-thread authority boundaries.

That includes at least:

- visibility
- viewport
- online/offline
- clock when sourced from browser timers or browser event loops
- persistence-backed browser-local facts
- browser history and raw location changes

Required consequence:

- host capability updates are admitted on the main thread, typed there, and
  bridged into the worker runtime
- worker code does not read `window`, DOM state, or other browser-owned facts
  ambiently
- host-capability ingress remains one runtime truth lane, not a second local
  store or framework-specific side channel

### 10.6 Host Effects Remain Main-Thread Execution Boundaries

Host effects are not worker work.

If an effect mutates:

- DOM state
- browser APIs
- framework-local mutable objects
- imperative platform instances

then it remains an explicit main-thread execution boundary.

Required consequence:

- worker runtime may request host-effect execution, but does not perform it
  directly
- host-effect execution is represented as a typed request/acknowledgement or
  typed failure/unavailability artifact
- effect results do not create a second hidden lifecycle authority on the main
  thread
- effect batching and delivery cost are measured explicitly
- acknowledgements are observational results from the host boundary; any
  lifecycle or graph meaning they induce exists only after worker-side typed
  admission

### 10.7 Observation, Output, And Diagnostics Must Reuse One Runtime Story

Worker-first mode must not invent a second observer or diagnostics engine.

Required consequence:

- committed observation still derives from the existing runtime observation
  substrate
- output delivery packets are derived from committed runtime truth
- diagnostics/history reads remain consumers of the same committed and retained
  runtime artifacts
- richer diagnostics may cost more, but cannot change committed truth or
  placement legality

### 10.8 Replay, Restore, Export, And Import Must Preserve Capability Honesty

Worker placement is not honest unless historical operations preserve the
capability story, not just the value story.

Required consequence:

- restore and replay preserve whether work was worker-executable,
  main-thread-hosted, or unavailable
- export/import never pretend that live closures or host-only capabilities were
  portable runtime data
- same-runtime exact restore may admit richer capability reattachment than
  portable transport, but the distinction must be explicit and typed
- branch restore must not resurrect obsolete placement capability or stale
  host-execution authority

Normative identity and equivalence contract:

- every authored declaration that participates in placement classification must
  have one canonical declaration identity stable across equivalent forward runs
- every placement-classified form must derive one canonical placement identity
  from declaration identity plus the admitted capability posture used to prove
  it
- every lowered worker or host execution plan must derive one canonical plan
  identity from placement identity plus the exact lowered execution boundary it
  authorizes
- every host capability attachment, reattachment, denial, fallback, detach, or
  unavailable artifact must carry canonical identity fields sufficient to match
  equivalent histories exactly
- replay, restore, import, and export artifacts must compare canonical identity
  forms, not friendly names, pointer identity, closure object identity, or
  diagnostic text
- any reuse, suppression, or parity proof that relies on looser equality than
  the declared identity contract is out of spec

### 10.9 Bridge Traffic Must Stay Breadth-Bounded

The bridge must be treated as a hot path with named cost contracts.

Required consequence:

- transaction submission is batched at the largest semantically honest boundary
- host-capability ingress may coalesce equivalent or superseded updates where
  semantics allow
- output and observation delivery must scale with changed public-delivery
  surface rather than total graph breadth
- diagnostics/history summary reads must not trigger hidden rich reconstruction
  on the operational path
- per-read, per-node, and per-effect chatter must be treated as a failure mode,
  not an implementation detail

### 10.10 Worker Topology Is One Runtime Authority Per Runtime Instance

This milestone does not admit helper-worker topology as its primary execution
model.

Required consequence:

- a runtime instance has one dedicated worker-owned authority when deployed in
  worker-first mode
- auxiliary host boundaries may exist, but they serve that one authority rather
  than becoming peer execution engines
- worker-first certification applies to this topology directly, not to a looser
  family of possible worker arrangements

### 10.11 Fallback And Denial Policy Must Be Product-Declared

Worker-first mode must fail honestly when placement cannot be proven.

Required consequence:

- denial is the default when requested worker posture cannot be proven honestly
- fallback is legal only on product surfaces that declare it explicitly
- every fallback path emits a first-class artifact naming what failed, what was
  widened, and why the fallback was admitted
- hidden compatibility fallback, convenience retries on the main thread, or
  silent broadening are out of spec

## 11. Phases

### Phase 1: Placement Taxonomy And Bridge Artifact Lock

Purpose:

- freeze the typed placement and envelope model before implementation fans out

This phase must ship:

- frozen placement categories for worker-executable, main-thread-hosted, and
  unavailable work
- frozen boundary-envelope vocabulary for transaction, capability ingress,
  output/observation delivery, host effect, diagnostics/history, and lifecycle
  control
- frozen causality model for transaction and generation ordering across boundary
  envelopes
- frozen product fallback policy naming where fallback is legal and where denial
  is mandatory
- compile-time and runtime boundary shapes that prevent ad hoc cross-thread
  bags from becoming the real contract
- one explicit worker-first versus compatibility deployment taxonomy

Phase 1 gate:

- no later phase begins until placement classification and bridge-envelope
  categories are precise enough that the worker boundary can be implemented
  without semantic guesswork

### Phase 2: Worker-Owned Runtime Shell And Graph Lifecycle

Purpose:

- move canonical runtime authority into the worker without changing runtime
  meaning

This phase must ship:

- worker bootstrap and lifecycle ownership for the runtime instance
- worker-owned graph state, invalidation, recomputation, async lifecycle, and
  diagnostics production
- graph publication and graph-owned lifecycle behavior for already lowered or
  placement-classified non-callback workloads that remains honest when the
  runtime authority is worker-owned
- main-thread compatibility parity harnesses for equivalent graphs

Phase 2 restriction:

- Phase 2 may not invent a provisional callback transport, temporary main-thread
  execution escape hatch, or hidden compatibility fallback to make general
  callback-authored publication appear to work early
- if authored work has not yet passed the placement proving boundary, it may not
  be admitted to worker-first graph publication in this phase

Phase 2 gate:

- no later phase begins until worker-first mode and compatibility mode can
  prove the same committed graph truth for equivalent non-host workloads
- general callback-authored worker-first publication remains blocked until Phase
  4 closes placement classification and lowering honestly

### Phase 3: Main-Thread Host Capability And Host Effect Bridges

Purpose:

- close the browser-owned host boundaries as typed ingress/egress lanes

This phase must ship:

- typed host-capability ingress from the main thread into the worker runtime
- typed browser-history and raw location ingress for router-facing host events
- typed host-effect egress from the worker into the main thread
- explicit stale, denied, detached, and unavailable host-boundary artifacts
- bounded coalescing and attribution for host-boundary traffic
- host-boundary admission that preserves transaction and generation ordering

Phase 3 gate:

- no later phase begins until browser-only host facts and main-thread-only host
  effects are integrated without ambient reads or imperative side channels

### Phase 4: Computation Placement, Callback Eligibility, And Honest Fallback

Purpose:

- solve the hardest truth problem: callback-first authoring under worker-first
  placement

This phase must ship:

- placement classification for authored computed, output, and effect work
- a worker-executable lowering lane for admitted portable work
- an explicit main-thread-hosted execution lane for work that remains process-
  local host capability, limited to typed host-boundary execution against
  worker-supplied closed inputs
- typed denial/fallback/unavailability artifacts when requested worker posture
  cannot be satisfied honestly
- isolation rules that keep one worker-ineligible node/effect from collapsing
  unrelated graph breadth
- explicit product-declared matrix of which authored categories:
  - are worker-executable
  - are admissible through the narrow main-thread-hosted lane
  - are denied rather than widened

Main-thread-hosted lane contract:

- worker-owned runtime remains the only graph and lifecycle authority
- main-thread-hosted execution may consume only a typed closed request emitted by
  the worker and may return only a typed result, typed failure, or typed
  denial/unavailability artifact
- main-thread-hosted execution may not perform ambient graph reads, ambient
  runtime writes, or local shadow lifecycle tracking
- if a callback-authored shape cannot be expressed honestly through this closed
  request and result contract, it must be denied or explicitly restricted rather
  than silently widened into arbitrary main-thread derivation

Phase 4 gate:

- no later phase begins until callback placement, denial, and fallback are
  explicit enough that worker-first mode no longer relies on folklore about
  closure portability

### Phase 5: Observation, Output, Diagnostics, And History Boundary

Purpose:

- close the public-facing bridge so off-thread runtime truth does not become
  on-thread delivery folklore

This phase must ship:

- committed observation delivery packets from worker to main thread
- committed output delivery packets for structured public outputs
- diagnostics/history summary and rich-read boundaries with explicit cold-work
  attribution
- lifecycle/disposal/detach updates that preserve resource and observer truth
- public cost envelopes for delivery and diagnostics boundaries

Phase 5 gate:

- no later phase begins until observation, output, and diagnostics reads can be
  shown to preserve one runtime story with bounded delivery breadth

### Phase 6: Replay, Restore, Import/Export, And Capability Parity

Purpose:

- make worker-first historical behavior as honest as ordinary forward execution

This phase must ship:

- worker-first replay and restore parity
- same-runtime exact restore posture for worker-hosted capabilities where
  admissible
- portable import/export posture with explicit callback or capability
  unavailability artifacts where needed
- branch restore behavior that preserves placement and capability truth
- compatibility artifacts for environments without worker support

Phase 6 gate:

- no later phase begins until historical operations preserve capability posture
  explicitly instead of only reconstructing output values

### Phase 7: Certification, Performance Closeout, And Product Guidance

Purpose:

- close the milestone with hostile proof and explicit product guidance

This phase must ship:

- full certification proving worker-first and compatibility parity under
  hostile workloads
- named counters, complexity contracts, and boundary performance envelopes
- docs and examples that teach worker-first as the recommended posture for
  heavy applications
- compatibility guidance that states exactly when main-thread deployment,
  denial, or fallback is expected

Phase 7 gate:

- the milestone is not closed until worker-first mode can be recommended
  without a giant footnote that "real semantics still live on the main thread"

## 12. Must Ship

Milestone 9 is not done because a demo app becomes smoother in Chrome.

It is done only when `forge-signal-wasm` ships:

- a worker-owned runtime posture for graph state, invalidation,
  recomputation, async/resource lifecycle, route/resource/forms continuity,
  replay coordination, and diagnostics production
- a typed placement taxonomy for worker-executable, main-thread-hosted, and
  unavailable work
- typed boundary envelopes for transaction admission, host-capability ingress,
  browser-history ingress, host-effect egress, committed output delivery,
  committed observation delivery, diagnostics/history/export/import requests,
  and lifecycle/disposal coordination
- explicit capability denial, fallback, detachment, and unavailability
  artifacts
- main-thread compatibility mode with semantic parity proof against
  worker-first mode
- callback/computation placement classification and honest fallback behavior
- a sealed proof-bearing progression from raw declaration to placement-
  classified form to lowered worker plan, lowered host plan, or denial artifact
- replay/restore/import/export capability posture that never lies about closure
  or host portability
- a canonical identity and equivalence contract for declaration identity,
  placement identity, lowered-plan identity, and capability attachment or denial
  artifacts
- named counters and public or cert-visible performance envelopes for worker
  and main-thread boundaries
- compile-time or type-surface boundaries that prevent ordinary code from
  bypassing the placement and bridge contract
- certification suites and hostile artifacts proving the milestone under
  worker churn, host churn, async churn, branch/restore, and large public
  delivery breadth

## 13. Must Preserve

- `forge-signal` remains the owner of derived computation, async lifecycle,
  temporal legality, observation truth, diagnostics truth, rollback, branch,
  and replay semantics
- browser APIs remain host authority boundaries rather than ambient worker
  state
- worker placement does not create a second cache, lifecycle engine, router
  engine, or resource engine on the main thread
- callback-first ergonomics remain available without pretending that arbitrary
  live closures are portable runtime data
- committed observation remains commit-bounded and rollback-safe
- output, diagnostics, and history remain derived views over one canonical
  runtime truth artifact
- compatibility mode remains honest and explicit rather than a hidden fallback
  that changes meaning

## 14. Performance Contracts

The milestone must expose named counters for at least:

- worker transaction submission count
- worker transaction batch width
- worker invalidation breadth
- worker recomputation breadth
- host-capability ingress count
- host-capability ingress coalesced width
- browser-history ingress count
- host-effect request count
- host-effect completion count
- host-effect denial or unavailable count
- output delivery packet count
- output delivery breadth
- observation delivery packet count
- observation delivery breadth
- diagnostics summary read count
- diagnostics rich read count
- diagnostics cold reconstruction count
- worker/main-thread round-trip count
- worker placement denial count
- worker fallback count
- main-thread-hosted callback execution count
- replay capability-unavailable count
- restore capability-reattach count
- compatibility-mode parity check count
- main-thread broad-work denial count
- bridge serialization allocation count
- bridge deserialization allocation count

The milestone must also declare named complexity contracts for:

- transaction submission bridging
- host-capability ingress admission
- browser-history ingress admission
- host-effect request routing
- committed output delivery
- committed observation delivery
- diagnostics summary reads
- diagnostics rich reads
- callback placement classification
- worker-executable declaration lowering
- main-thread-hosted callback execution routing
- replay/restore capability reconstruction
- import/export capability classification
- fallback and denial classification

Each contract must name its real cost bases explicitly. At minimum:

- transaction submission cost must be stated in terms of committed mutation
  batch width and bridged payload breadth, not total graph size
- output and observation delivery cost must be stated in terms of committed
  public delivery breadth, not total node count
- host-capability ingress cost must be stated in terms of changed capability
  frontier and coalesced update width
- callback placement classification cost must be stated in terms of declaration
  classification work, not runtime graph breadth
- diagnostics summary read cost must remain summary lookup only, with zero rich
  reconstruction
- replay/restore capability reconstruction cost must be stated in terms of
  retained capability artifacts and historical span, not total live graph size

### 14.1 Named Worker Placement Performance Failure Modes

Milestone 9 should name the failure modes it intends to prohibit so later work
cannot reintroduce them under nicer names.

At minimum:

- `BridgeChatterStorm`
  Operational paths emit per-read, per-node, or per-effect chatter across the
  worker boundary instead of semantically batched envelopes.
- `CompatibilityTruthLeak`
  Main-thread compatibility mode becomes the place where semantics are actually
  defined, with worker-first mode merely approximating it.
- `MainThreadProjectionInflation`
  Output, diagnostics, or lifecycle views are maintained on the main thread as
  though they were authority instead of derived public projections.
- `PlacementCollapse`
  One worker-ineligible node or effect silently pins unrelated graph breadth to
  the main thread.
- `CallbackPortabilityLie`
  Live closures are treated as worker-portable runtime data without an admitted
  lowering path or explicit unavailability artifact.
- `HistoryCapabilityAmnesia`
  Replay, restore, export, or import preserves values while losing the
  capability posture that made those values possible.
- `UIFreezeBySerialization`
  Runtime work moves off-thread, but broad public delivery or serialization
  still blocks the UI thread.
- `AmbientHostReadRelapse`
  Browser facts regain semantic meaning through ambient reads instead of typed
  host-capability ingress.

## 15. Required Named Proof Families

- `The Worker Compatibility Truth Equivalence Test`
- `The Mixed Placement Graph Isolation Test`
- `The Host Capability Worker Bridge Parity Test`
- `The Browser History Worker Admission Parity Test`
- `The Main-Thread Host Effect Boundary Test`
- `The Callback Placement Eligibility And Denial Test`
- `The Worker Ineligible Node Does Not Collapse Graph Breadth Test`
- `The Observation And Output Delivery Boundary Test`
- `The Diagnostics Summary Cost Honesty Test`
- `The Worker Replay Restore Capability Honesty Test`
- `The Import Export Callback Unavailability Test`
- `The Worker Bridge Boundedness Test`
- `The UI Freeze Surface Denial Test`

## 16. Acceptance Evidence

Milestone 9 is complete only when `forge-signal-wasm` can certify all of the
following with canonical machine-checkable artifacts:

- the `Worker Compatibility Truth Equivalence Test`
- the `Mixed Placement Graph Isolation Test`
- the `Host Capability Worker Bridge Parity Test`
- the `Browser History Worker Admission Parity Test`
- the `Main-Thread Host Effect Boundary Test`
- the `Callback Placement Eligibility And Denial Test`
- the `Worker Ineligible Node Does Not Collapse Graph Breadth Test`
- the `Observation And Output Delivery Boundary Test`
- the `Diagnostics Summary Cost Honesty Test`
- the `Worker Replay Restore Capability Honesty Test`
- the `Import Export Callback Unavailability Test`
- the `Worker Bridge Boundedness Test`
- the `UI Freeze Surface Denial Test`

The certification bundle must include canonical digests or equivalent artifacts
for:

- placement classification
- worker runtime identity
- transaction envelopes
- host-capability update envelopes
- browser-history event envelopes
- host-effect request and acknowledgement envelopes
- committed output delivery packets
- committed observation delivery packets
- diagnostics/history read envelopes
- fallback and denial classifications
- capability availability and reattachment posture
- replay/restore/import/export capability artifacts
- compatibility-mode and worker-first committed truth digests
- boundary performance envelopes
- bridge allocation posture
- main-thread broad-work denial artifacts

## 17. Architectural Notes

- The strongest shape is one worker-owned runtime plus typed host bridges, not
  a main-thread runtime with a worker compute helper.
- Callback-first ergonomics and worker-first placement are compatible only if
  placement classification is explicit. The spec should prefer honest dual
  lanes over a fake universal closure story.
- Browser-history integration should look like host capability for navigation:
  raw browser events on the main thread, typed route meaning in the runtime.
- Output delivery should prefer one committed projection packet per boundary
  rather than many tiny reactive messages.
- Diagnostics summary APIs should remain explicitly cheap; rich explanation is a
  separate boundary with separate attribution.

## 18. Explicit Deferrals

- general closure-source serialization and restoration across hosts
- arbitrary compiler transforms that infer worker-executable representations
  from normal callback source
- service-worker and shared-worker product surfaces
- cross-tab distributed runtime authority
- general DOM-capability virtualization for worker code
- making every callback-authored computed automatically worker-executable
  without an admitted lowering path

Those remain later work. They may not block this milestone from making
worker-first deployment honest and valuable for the majority of runtime-owned
work.

## 19. Sequencing Notes

This milestone belongs after router work because:

- route continuity, browser-history integration, and speculative navigation
  must already be defined as runtime consumers before they can move off-thread
- resource/API and forms continuity must already reduce to closed runtime truth
  rather than being redefined while workerizing

This milestone belongs after host capability and callback-computed closeout
because:

- browser-only facts must already be typed main-thread inputs
- callback lifecycle and capability honesty must already be explicit before
  worker placement can classify them correctly

This milestone belongs before roadmap completion because:

- keeping most runtime-owned work off the UI thread is a real product boundary
  for serious web applications, not a cosmetic optimization pass

## 20. Required Self-Check

- Does this milestone solve a real structural problem or just package work
  cosmetically?
  Yes. It creates the missing placement architecture that turns closed wasm
  semantics into a worker-first runtime product rather than a main-thread-only
  engine with optional offload garnish.
- Is the adversarial constraint precise and load-bearing?
  Yes. Mixed placement, host churn, async churn, replay/restore honesty,
  delivery breadth, and UI-freeze denial all directly shape the architecture.
- Does the milestone preserve crate authority boundaries?
  Yes. Runtime truth stays with `forge-signal`; browser facts and host effects
  remain explicit host boundaries; wasm owns the product bridge.
- Does the milestone define proof obligations, not just implementation tasks?
  Yes. Placement categories, bridge envelopes, fallback artifacts, historical
  capability artifacts, counters, and hostile proof families are all required.
- Could a competent engineer map this spec into honest types, modules, and
  tests?
  Yes. The spec names boundary families, phases, failure modes, proof lanes,
  and performance contracts clearly enough to do that.
- Does the milestone belong in this roadmap sequence, or is it out of order?
  Yes. It intentionally follows the closure of host capability, graph
  lifecycle, API/resource, and router semantics so one stable product truth can
  move off-thread instead of being re-solved piecemeal.

## 21. Milestone Done When

Milestone 9 is done only when `forge-signal-wasm` can run as a worker-first web
runtime product where:

- one worker-owned runtime remains the canonical authority for derived
  execution truth
- browser-only host facts and host effects remain explicit typed main-thread
  boundaries
- worker-first mode and compatibility mode converge to the same committed
  semantics
- callback and capability portability limits are surfaced honestly through
  typed placement and historical artifacts
- bridge traffic is semantically batched and breadth-bounded
- runtime-owned work no longer freezes the UI thread through hidden main-thread
  execution, serialization, or delivery breadth

At that point, worker-first deployment becomes what it should be:

- not an optimization rumor
- not a compatibility footnote
- not a second runtime

but the honest default execution posture for serious `forge-signal-wasm`
applications.
