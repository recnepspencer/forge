# Milestone 3.14.1 Phase 5 Implementation Plan

## Governing Sources and Precedence

This plan implements, but does not redefine:

1. [`milestone-3.14.1.md`](milestone-3.14.1.md), the parent milestone;
2. [`milestone-3.14.1-phase-5.md`](milestone-3.14.1-phase-5.md), the normative
   Phase 5 subordinate specification;
3. `workspaces/worth-ui/profiles/worth-ui-global-text-v2/manifest.toml`, the
   exact qualified data/policy record;
4. `milestone-3.14.1-proof-ledger.csv`, the single append-only proof ledger.

If implementation reveals a conflict or missing product decision, stop and
repair the specifications before coding. Do not invent a wrapper, duplicate
authority lane, second ledger, or private compatibility posture.

## Objective

Turn runtime-owned Phase 4 `UiQualifiedTextLayout` authority into exact alpha
and intrinsic-color glyph pixels through bounded native atlases and the
existing presentation lifecycle. Progress native atlas/upload work through one
bounded host-native physical Signal runtime, then translate its exact physical
completion through Runtime Bridge into Query-owned async result state and a
separate Query-semantic Signal invalidation graph. Preserve Phase 4 as the sole
Unicode, fallback, shaping, layout, measurement, and interaction-geometry
owner.

## Current Readiness Boundary

The source now contains substantial raster and Gate D atlas implementation.
Its retained `P5-ATLAS-01` evidence predates the required physical Signal
owner. The pivot changes the row's governed source, lifecycle authority,
completion/recovery semantics, shutdown, counters, and proof basis, so the next
governed candidate must reopen it. `P5-ATLAS-PINNING-01` and the other existing
Phase 5 rows remain OPEN. The Query async-presentation row also remains OPEN
until its real main, independent oracle, named hostile fault, counter, source
mapping, and authenticated execution receipt exist.

The active UI branch predates the completed Signal Milestone 13 source that is
present on `origin/master`. Batch D begins by integrating that exact upstream
substrate and revalidating its manifest/API surface. It must not copy M13 into
WORTH UI or implement a private approximation against the older local Signal
tree.

Before the first production batch:

- reexecute and validate the immutable Phase 1-4 portfolio on current source;
- confirm the twelve-row Phase 5 ledger inventory, reopen the stale atlas row,
  and keep atlas pinning, cost, and Query async presentation OPEN;
- freeze the staged transaction and typed denial/settlement API with compile
  twins and topology tests;
- freeze one host-native physical Signal runtime identity/construction owner,
  exact declarations/capacity, external effect port, performed observation,
  and shutdown contract before migrating manual lifecycle structures;
- ensure readiness tests emit no feature-row counter or mutation receipt.

## Destination Authority

| Responsibility | Owner | Boundary |
|---|---|---|
| Qualified raster demand, source selection, alpha/color pixels, digest, cost | `worth-ui-text` | concrete owned batches; no GPU/native dependencies |
| Durable qualified layout and mounted paint-span authority | `worth-ui-runtime` | exact `Arc<UiQualifiedTextLayout>`; private native text orchestration |
| Cross-domain transport | `worth-ui-host-contract` | borrowed inert demand, raster-batch, layout, and glyph-run views only |
| Atlas planning, reservation, pages, pins, eviction, upload, GPU draw, census | `worth-ui-host-native` | host-contract-only WUI dependency; no shaping/raster policy |
| Native async physical progression | one `worth-ui-host-native::native::physical_work_signal` owner per native host/device lifecycle | direct generic `worth-signal` substrate dependency; no Query; no raw WGPU handles in Signal |
| Installed presentation resource and retained application-visible async posture | Query, consumed only through `worth-ui-query-binding` | typed installed declaration/binding; no formatted-string identity or digest authority |
| Query meaning to operation correspondence | Runtime Bridge | binds the installed Query resource to the exact WUI presentation operation without owning native effects |
| WUI presentation audience edge | `worth-ui-query-binding` | sole WUI Query importer; installs/consumes the Runtime Bridge correspondence and translates native owner observations |
| Query-semantic invalidation | Query/Runtime Bridge Signal graph | installed semantic conditions, currentness, continuations, and precise downstream invalidation; never native effect authority |
| Physical eligibility/progression | host-native physical Signal graph | bounded request readiness, submit/poll completion reconciliation, retry/timeout/cancel/supersede/recovery, backpressure, wakes, shutdown; native owners remain physical truth |
| Headless inspection | `worth-ui-host-headless` | bounded semantic transcript over borrowed exact records |
| Application API | `worth-ui` facade | existing font/text/measurement surface; no atlas/raster controls |

The lawful transaction is:

```text
runtime layout/damage
  -> Query-installed pending presentation resource
  -> Runtime Bridge correspondence and Query-semantic Signal frontier
  -> binding-issued exact presentation basis
  -> text-owned demand
  -> host-native physical Signal request
  -> native effect-free atlas plan and miss set
  -> text-owned raster batches for admitted misses
  -> native external submit/poll observation
  -> physical Signal completion reconciliation
  -> native staged atlas settlement and later draw/present
  -> Runtime Bridge basis/currentness validation
  -> Query current/stale/failed/cancelled/superseded/unresolved posture
  -> Query-semantic Signal invalidation of exact downstream consumers
```

No consumer may reshape, refallback, rebreak, consult a system font, infer
paint from visual order, retain framework text authority, or treat an atlas
entry as semantic truth.

### Physical Signal owner placement

The manifest, authority, and thread-affinity audit selects
`worth-ui-host-native` as the physical Signal owner:

- it is the strongest owner of device/queue/surface, native resource registry,
  atlas storage, staging owners, submission indexes, completion polling,
  recovery, and terminal close;
- adding `worth-signal.workspace = true` creates a lawful substrate dependency
  without adding another WORTH UI crate edge; host-native's only WORTH UI
  dependency remains host-contract;
- it can own one pure Signal worker and bounded typed mailboxes while keeping
  all raw WGPU handles and effect calls on the native event/device thread;
- runtime cannot own the graph without duplicating physical currentness and
  recovery authority, and host-contract cannot own it because that crate is
  inert transport.

The destination owner is
`worth-ui-host-native/src/native/physical_work_signal/`, with named children
for construction/identity, declarations, routing, completion reconciliation,
wake delivery, shutdown, performed observations/counters, and the bounded
worker. The Winit readiness registry is the wake transport from ready physical
work to the native event thread. No public native facade exposes the Signal
runtime or atlas-control authority.

The Query-semantic graph is distinct. Query/Runtime Bridge owns its runtime and
`worth-ui-query-binding::presentation_async` installs the exact WORTH UI
correspondence. Runtime imports neither Query nor Signal. No runtime identity,
aspect slot, request handle, completion envelope, capacity, or shutdown receipt
crosses between the two graphs.

## Implementation Batches

### Batch A: protocol and authority freeze

1. Add borrowed raster-demand, raster-batch, and glyph-run views to
   host-contract.
2. Add the move-only native atlas plan/reservation and typed before-effect,
   presented, effects-indeterminate, reconstruction, and cleanup outcomes.
3. Add the private runtime native-text transaction owner.
4. Enforce dependency direction, non-forgeability, no `Clone`/serialization,
   no raw string identity, and consumer non-authority with compile twins.
5. Complete only readiness evidence; keep feature rows OPEN.

### Batch B: exact demand and alpha outline raster

1. Derive paragraph-local glyph demand from the exact layout, paint spans,
   logical damage, and raster scale.
2. Implement the qualified atlas key, origins, extents, variation-aware ink,
   alpha outline coverage, bounded bytes, digests, and actual counters.
3. Preserve layout attribution separately from safe raster equivalence.
4. Prove first/middle/last local work at 1, 32, 2,048, and 4,096 retained
   paragraphs and reject consumer reshape/system-font mutants.
5. Close `P5-GLYPH-RASTER-01` only after its causal mapping exists.

### Batch C: intrinsic color and exhaustive RGI

1. Implement qualified COLRv0/CPAL, COLRv1/CPAL, CBDT/CBLC, and sbix PNG/
   one-hop-dupe semantics.
2. Reject SVG, sbix jpg/tiff, malformed graphs/enums/images, missing targets,
   cycles, invalid compositing, and unqualified sources before layout/raster
   effects.
3. Share outline/color/bitmap semantics between ink and raster results.
4. Execute every exact Unicode 17 RGI mapping internally and representative
   native source/class pixels externally.
5. Convict split, selector, layer, alpha, palette, and tint mutants before
   closing `P5-COLOR-EMOJI-01`.

### Batch D: native atlas transaction

1. Integrate the completed Signal Milestone 13 source from `origin/master` and
   verify the exact facade, resource progression, performed observation, and
   counter contracts before WORTH UI production edits.
2. Add `worth-signal.workspace = true` only to `worth-ui-host-native`. Construct
   one `UiNativePhysicalSignalOwner` inside `UiNativeHostState` per native
   host/device lifecycle, with a non-forgeable runtime identity, exact bounded
   capacity, one pure worker, and bounded route mailboxes.
3. Declare closed physical operation families for atlas upload and later native
   presentation work. Install distinct producer-local aspects/partitions for
   host lineage, transaction/presentation attempt, demand/raster-key set,
   target binding, submission, and recovery. Do not reuse Query-semantic slots
   or persist Signal aspects.
4. Retain separate alpha and RGBA atlas/resource owners with the exact profile
   caps. Keep effect-free lookup, placement, capacity, peak overlap, live pins,
   deterministic unpinned eviction, and move-only reservation as native
   physical truth.
5. Route bounded upload eligibility, ready work, submit attempt, completion
   polling, retry, timeout, cancellation, supersession, recovery scheduling,
   backpressure, wake delivery, and shutdown through the physical Signal owner.
   The replaceable native external port performs WGPU work and returns typed
   observations; it cannot mint Signal or atlas verdicts.
6. Commit atlas entries, use epochs, and pins only after Signal admits the
   current physical completion. Reject stale, duplicate, foreign, out-of-order,
   cancelled, superseded, timed-out, or post-close envelopes. Effects-
   indeterminate work remains quarantined and carries bounded owner-issued
   recovery until reconciled.
7. Join physical Signal pending work, native staging/submission owners, atlas
   plans/entries/pins/pages, recovery authorities, WGPU resources, wakes, and
   worker admission in one compiler-total census and quiescent close.
8. Migrate the current manual lifecycle inventory as specified below. Keep only
   physical registries and effect adapters; delete or demote bespoke
   progression authorities.
9. Prove the optimized implementation against a genuinely independent
   physical-progression/atlas model, deterministic execution, a real qualified
   DX12 boundary, before/partial-effect injected ports, saturation, pin
   advance/release, recovery, and close.
10. Reopen and freshly prove `P5-ATLAS-01` because this batch changes its exact
    owner/source/evidence. Close `P5-ATLAS-PINNING-01` only from lawful product
    construction and causal live-unpin controls.

### Batch E: paint, scale, and presentation-specific Query binding

1. Carry original-range paint-span identity and logical straight foreground
   through headless and native glyph-run records.
2. Prove bidi visual order does not change paint ownership and intrinsic-color
   glyphs ignore adjacent foreground.
3. Prove color-value-only edits reuse exact layout/atlas entries and update
   only affected commands/damage.
4. Prove cluster-safe paint-boundary changes are local.
5. Prove pure DPI preserves layout/geometry and replaces raster/atlas only;
   text-scale/width changes replace layout first.
6. Add a dedicated `presentation_async` responsibility under
   `worth-ui-query-binding`; install the Query async declaration and bind it to
   exact mounted presentation, target, layout, paint, DPI, scale, attempt, and
   predecessor/currentness types.
7. Extend Query's audience declaration facade because its current identity part
   is string-only: add a closed typed async request-identity-part contract under
   `worth-query/src/application/declaration/async_resource/request_identity.rs`
   and export it through the existing declaration facade. Do not encode the
   WORTH UI basis as `WorthQueryAsyncRequestIdentityPart::text` strings.
8. Add Query's retained `Unresolved` async result state in
   `worth-query/src/runtime/async_result_state.rs`, and extend
   `worth-runtime-bridge/src/source/async_declaration/completion/` with the
   portable effects-indeterminate completion class that alone maps to it.
9. Add the Query-free WORTH UI runtime correspondence types and bind the new
   substrate meaning to pending/current/stale/failed/cancelled/superseded/
   unresolved posture without importing WORTH UI vocabulary into Query or
   Runtime Bridge.
10. Install the distinct Query/Runtime Bridge semantic Signal graph for the
    presentation operation. Declare exact producer-local aspects and
    mounted-instance/layout/original-range/paint-span/raster-key-set/
    presentation-attempt/target/host-lineage partitions. Signal evidence
    authorizes no native effect.
11. Use Milestone 13 immediate reverse indexing, pre-enqueue aspect/scope
    filtering, comparator stops, cause-preserving deduplication, and performed
    observations. Prove content, width, paint value, paint boundary, DPI,
    upload completion, and pin-release locality through the Query lifecycle.
12. Add compile/topology twins proving the semantic and physical Signal graphs
    cannot share identity, aspects, handles, capacities, envelopes, shutdown,
    or an ambient runtime.
13. Close `P5-TEXT-DPI-01` and `P5-TEXT-SPAN-PAINT-01`; keep
    `P5-TEXT-ASYNC-PRESENTATION-01` OPEN until the real external boundary in F.

### Batch F: pixels, reconstruction, and cost

1. Integrate the exact transaction with the retained native draw list,
   transparent target, readback, compositor capture, and terminal lifecycle.
2. Feed host-native's typed external observations through physical Signal
   completion reconciliation, then Runtime Bridge, then the installed Query
   presentation resource and semantic Signal frontier.
3. Share one real Windows/WGPU courtroom across the applicable rows.
4. Destroy layout, raster, atlas pages/index, pins, draw commands, target, and
   presentation state independently; reconstruct from mounted authority and
   prove the next delta is local.
5. Prove representative source/class pixels, clipping, damage, attribution,
   unchanged zero, ordinary/reconstructive slope, physical amplification, and
   exact resource cleanup.
6. Execute one shared 1/32/2,048/4,096 locality matrix for content-only,
   width-only, paint-value-only, paint-boundary, DPI, one miss/many hits, one
   completion/many mounted presentations, and shared/exclusive pin release.
   Record realized M13 counters from the exact graph plus analyzed bytes,
   shaping, raster, atlas, upload, damage, presentation, pixel, and census
   counters. Predicted counters are forbidden.
7. In the same `HP-03` world, independently adjudicate the exact ten-event
   `presentation-transitions` trace from the specification while observing
   physical Signal progression, Query posture, semantic invalidation, native
   pixels, retained affinity, and terminal census; run cancellation as a
   separate before-effects/after-effects hostile control.
8. Close `P5-TEXT-PIXELS-01`, `P5-TEXT-RECONSTRUCTION-01`,
   `P5-TEXT-COST-01`, and `P5-TEXT-ASYNC-PRESENTATION-01` only from their
   exact causal controls.

### Batch G: documentation and closure

1. Update `workspaces/worth-ui/docs/text-platform.md` for Phase 5 raster,
   paint, DPI/text-scale, atlas, saturation, reconstruction, and authority.
2. Keep the public example free of atlas knobs and compile it through the
   shared governed example portfolio.
3. Bind all feature rows to exact real production, independent oracles, named
   hostile faults, sources, counters, and authenticated receipts.
4. Run each unique corpus/model/native command once, retain content-addressed
   evidence, stream progress, and resume unchanged staged successes.
5. Validate the just-produced portfolio without rerunning it.
6. Close `P5-CLOSE-01` only after every predecessor and feature row is final.

## Ledger Inventory

| Requirement | Primary batch | Closure boundary |
|---|---|---|
| `P5-PREDECESSOR-01` | A | authenticated current-source through-Phase-4 handoff |
| `P5-GLYPH-RASTER-01` | B | exact attributable alpha/color raster production |
| `P5-COLOR-EMOJI-01` | C | exhaustive RGI plus every admitted color source |
| `P5-ATLAS-01` | D | separate bounded native atlas owners plus one bounded physical Signal lifecycle; reopened and freshly source-bound |
| `P5-ATLAS-PINNING-01` | D | live pins and deterministic unpinned eviction |
| `P5-TEXT-DPI-01` | E | layout-preserving pure-DPI raster replacement |
| `P5-TEXT-SPAN-PAINT-01` | E | original-range paint and color-only reuse |
| `P5-TEXT-PIXELS-01` | F | headless/native attribution and external pixels |
| `P5-TEXT-RECONSTRUCTION-01` | F | full derived-state reconstruction from mounted authority |
| `P5-TEXT-COST-01` | E/F | realized M13 UI frontier slopes plus exact ordinary/reconstructive/unchanged/resource cost |
| `P5-TEXT-ASYNC-PRESENTATION-01` | E/F | Query-installed presentation meaning in E; real native completion and retained transition proof in F |
| `P5-CLOSE-01` | G | complete retained portfolio and closure laws |

The exact guarantees and hostile families live in the subordinate
specification and ledger. This plan does not redefine them.

## Proof Economy and Resume

- Key unique execution by source-state, claim, command, oracle, and dependency
  identity.
- Execute exhaustive Unicode/RGI, atlas model, compiled examples, and real
  Windows/WGPU worlds once per key.
- Let rows validate distinct observations from shared authenticated receipts.
- Keep row-owned outputs separate from dependency inputs; a consumer cache may
  never rewind a producer artifact.
- Stage ledger and evidence atomically. A late failure leaves the published
  ledger/artifacts unchanged.
- Resume authenticated staged successes only when every binding is unchanged.
- Stream requirement, execute/reuse posture, result, and duration.
- The final verifier validates the retained portfolio and closure laws; it does
  not reexecute the portfolio.

### Shared Signal/locality proof matrix

One parameterized execution covers retained sizes 1, 32, 2,048, and 4,096 for
each of content-only, width-only, paint-value-only, paint-boundary, pure-DPI,
one miss/many hits, one completion/many presentations, and layout removal with
shared/exclusive pins. The exact performed M13 observation is joined to WORTH
UI analysis/shaping/raster/atlas/upload/damage/presentation/pixel/census
counters. An independent dependency model adjudicates the exact immediate
subscribers and scope rejections.

The hostile matrix injects, at production owners: complete subscriber-closure
walking; aspect filtering after enqueue; partition/detail/range filtering after
enqueue; every-mounted-instance invalidation; paint-to-layout widening;
DPI-to-layout widening; dropped immediate-dependency cause during deduplication;
global-union scope; hidden complete-document scan; physical completion outside
physical Signal; Query current publication outside Query admission;
Signal-as-effect authority; stale/duplicate/out-of-order completion; and
terminal Signal/native resource retention. A printed label or a second lawful
run is not a control.

## Query Boundary

The broad Query audience migration remains outside Phase 5. The minimal
presentation-specific integration is required and lives in the existing WORTH
UI Query audience/binding surface:

- `worth-ui-query-binding` is the sole production Query importer;
- runtime and host-native import no Query facade in any target;
- host-native alone imports Signal for its private physical-work runtime;
  runtime imports neither Signal nor Query;
- host-native decides physical completion and emits inert typed observations;
- the host-native physical Signal graph owns progression around those
  observations, while the Query/Runtime Bridge graph owns semantic
  invalidation; neither graph accepts the other's local identities;
- Runtime Bridge admits an observation only against the exact Query-installed
  presentation basis and currentness lineage;
- Query owns application-visible pending/current/stale/failed/cancelled/
  superseded/unresolved posture;
- semantic Signal provides application invalidation evidence, physical Signal
  provides bounded native work progression, and neither grants WGPU effect
  authority;
- Foundational canonicalization derives evidence only after typed admission;
- terminal projections and JSON are reporting/materialization only;
- live recovery authority remains owner-local and nonserializable;
- no compatibility wrapper, raw workspace authority, formatted identity,
  digest authority, or parallel Query lane is introduced.

The destination `worth-ui-query-binding/src/presentation_async/` responsibility
is separate from scalar `projection_invalidation/`: it reuses the Query
authority model and currentness discipline, not scalar-source semantics that
do not fit external presentation completion.

## Current Implementation Disposition

Retain:

- host-native ownership of WGPU submission, polling, physical settlement,
  recovery authority, and resource census;
- native atlas alpha/color stores, placement/index truth, WGPU page/texture/
  buffer owners, staging owners, submission indexes, move-only plans, pin
  identities, and external effect adapters as physical registries/mechanics;
- runtime ownership of mounted presentation attempts and exact layout/pin
  preparation;
- the Winit readiness registry as coalescing wake transport only;
- scalar `projection_invalidation/` as the upstream Query authority pattern,
  not as presentation-specific types.

Supersede in D:

- `UiNativeHostState::text_atlas_in_flight` as an independently progressing
  transaction scheduler; retain its plan/uploads only as a bounded physical
  obligation keyed by the Signal request;
- manual `drain_text_atlas_physical`/`poll_text_atlas_transaction` scheduling
  loops as readiness/completion authority; polling occurs only when physical
  Signal emits exact ready work and returns through typed reconciliation;
- `text_atlas_recovery` as a standalone recovery coordinator; retain the
  nonserializable recovery capability in the physical registry, but Signal
  owns retry/recovery scheduling and currentness;
- callback-settlement routes that map upload return values directly into atlas
  commit or framework outcome; the external port emits inert observations and
  physical Signal performs lifecycle admission first;
- any local retry queue, timeout loop, pending-work scheduler, unbounded
  command collection, or shutdown ordering parallel to the physical Signal
  owner;
- readiness-registry logic beyond level-triggered wake transport.
- `UiNativeHostState::pending_presentations` plus its `try_settle` close-time
  polling as a presentation-work scheduler; retain the concrete readback,
  submission, and presentation obligations in the physical registry, but
  route their readiness/completion/retry/timeout/cancel/shutdown progression
  through the same physical Signal owner;
- presentation/readback transaction timeout and recovery loops that can
  progress independently of the physical graph. Gate F may add new operation
  declarations to the existing runtime, never a second presentation worker or
  callback-settlement graph.

Change in D/E/F:

- add `worth-signal` to host-native and no other WORTH UI production crate for
  native physical progression;
- reopen and reprove `P5-ATLAS-01`, then prove `P5-ATLAS-PINNING-01` from the
  corrected physical lifecycle;
- bind every native settlement observation to exact host, transaction,
  presentation basis, and currentness before it can cross the bridge;
- add the typed Query async request/binding and retained posture owner under
  `worth-ui-query-binding/presentation_async`;
- replace the runtime crate's direct test-only `worth-query-host` installation
  dependency with Query-binding-owned test support, so runtime has no direct
  Query or Signal dependency in any target;
- connect the real native observation through Runtime Bridge and Query in
  `HP-03`, including cancellation, supersession, indeterminate recovery, and
  terminal release;
- record UI-specific performed M13 frontier observations and combined domain
  counters in `P5-TEXT-COST-01`.

Explicitly defer:

- broad Query audience migration unrelated to text presentation;
- Query resources for glyphs, atlas entries, pages, uploads, submissions, or
  readiness wakes;
- Signal-owned WGPU effects or raw resources, serialized recovery authority,
  and operational terminal projections;
- Phase 6 input/focus/IME and presentation-affinity expansion.

## Verification Per Batch

Run in proportion to the batch, including:

- focused owner unit tests and mutation controls;
- physical Signal exact-constructor/count/capacity/declaration/currentness/
  retry/cancel/timeout/recovery/wake/shutdown tests and a second-runtime
  compile/topology denial;
- semantic-versus-physical graph identity/aspect/envelope separation and
  absence of parallel scheduler/retry/timer/callback/recovery authorities;
- affected public application/runtime/headless/native integration tests;
- exhaustive corpus/model or the shared real native world when its boundary
  changes;
- proof-ledger mapping/source/counter/artifact validation;
- the shared 1/32/2,048/4,096 performed-frontier matrix and its independent
  dependency model when Gate E/F locality changes;
- compiled public example and documentation checks where relevant;
- `cargo fmt --all -- --check`;
- dirty Rust line-cap and code-composition review;
- `cargo run --manifest-path tools/boundary-check/Cargo.toml -- --root .`;
- `cargo run --manifest-path tools/agent-context/Cargo.toml -- check`.

Do not run the atomic closer until the batch's production, causal controls,
source mappings, proof economy, and final source state are stable.
