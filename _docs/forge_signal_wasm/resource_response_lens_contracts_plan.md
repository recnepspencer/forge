# Branch-Native Resource Effects And Response Lenses Plan

> **Status:** Planned engineering spec, reset from the earlier response-lens
> plan after reviewing the native `forge-signal` branch and merge substrate.
>
> **Roadmap parent:** [wasm_product_roadmap.md](./wasm_product_roadmap.md)
>
> **Prerequisite milestone:** [api_surface_dx_plan.md](./api_surface_dx_plan.md)
>
> **Predecessor feature closeout:**
> [resource_response_auto_patching_remaining.md](./resource_response_auto_patching_remaining.md)
>
> **Resource certification parent:** [test-requirements.md](./test-requirements.md)

## Goal

Make resource response patching, delivery, optimistic reflection, rollback, and
advanced response topology automatic by treating every admitted resource change
as a branch-native signal effect.

Response lenses are still required, but they are no longer the foundation.
They are one lowering family that proves how a response-shaped resource effect
maps into signal branch, snapshot, aspect, merge, replay, and diagnostics
truth.

The target outcome is:

- resource authors declare response topology and mutation effect posture once
- Forge lowers declarations into branchable resource effect plans
- ordinary local patches, delivered patches, optimistic writes, rollback,
  rebase, and confirmation all consume the same effect envelope
- response topologies such as GraphQL connections, normalized entity bags,
  grouped lists, tuple envelopes, sparse pages, detail responses, summaries,
  and JSON-bearing items compile into effect-local loci instead of route-local
  patch folklore
- every speculative or committed effect either applies through a declared
  branch/aspect/lens proof or denies before visible truth changes

This milestone is not a response-shape helper milestone.
It is the resource product layer that makes the existing `forge-signal`
branch/merge substrate the default model for resource effects.

## Why This Milestone Exists

The closed response-contract slice proved that collection response contracts can
derive item and item-aspect patching without manual route plumbing.

That work remains valid as a subset, but it is not the right foundation for the
next product step.

The deeper substrate already exists in `forge-signal`:

- branch creation, switching, ancestry, branch-local replay, and branch-head
  snapshots
- exact branch snapshot capture and restore with reconstructability proof
- branch merge planning and execution
- merge strategy, merge base, conflict policy, conflict isolation, identity
  matcher, source-only policy, deletion policy, and aspect policy selection
- per-aspect and host-declared-region conflict isolation
- branch-state proof, merge-plan proof, merge-result proof, replay parity proof,
  and replay artifact proof surfaces

The wasm/package surface already exposes much of that through `history()`, but
the resource plan was treating branch/merge power as a later optimistic
consumer. That is backwards.

The correct foundation is:

- resource patching is a committed branch effect
- delivered patching is the same branch effect with external provenance
- optimistic patching is the same branch effect applied to a speculative branch
- rollback is exact branch restore or inverse effect execution
- rebase is branch merge planning plus resource effect conflict evidence
- response lenses lower response topology into branch/aspect/resource effect
  loci

If this milestone does not start there, it will build weaker JS-side machinery
beside a stronger native substrate.

## Governing Source Summaries

- `MENTALITY.md`
  Protects hostile-proof foundation design. This plan must solve branch-native
  speculative authority first rather than add optimistic behavior after a
  response helper layer exists.
- `arch_laws.md`
  Protects boundary honesty and proof-bearing phase progression. Resource
  effects must progress from declaration to lowered plan to branch execution
  through distinct proof types.
- `composition_laws.md`
  Protects semantic compilation units. Implementation files and tests must be
  named for effect planning, branch admission, response-lens lowering, or merge
  certification, not generic helpers or phase names.
- `domain_structure_laws.md`
  Protects true responsibility boundaries. Authoritative resource line truth,
  speculative branch state, response-lens derivation, delivery provenance, and
  diagnostics explanation must not share structural space.
- `perf_laws.md`
  Protects bounded semantic breadth. Branch-native resource effects must carry
  locus, aspect, region, and topology cost proof so cheap-looking APIs do not
  hide broad scans or full response reconstruction.
- `web_runtime_spec.md`
  Protects runtime-owned rollback, branch, restore, observation, and
  diagnostics truth. The resource product surface must consume those semantics
  instead of recreating them in package glue.
- `wasm_product_roadmap.md`
  Protects roadmap sequencing and runtime-truth ownership. This milestone
  belongs after API/resource and worker placement because the product surface is
  now broad enough that speculative resource effects must be branch-native.
- `test-requirements.md`
  Protects hostile certification for resource/API behavior. This milestone must
  extend those proof obligations with branch-native effect convergence,
  rollback, rebase, and response-topology certification.
- `api_surface_closeout.md`
  Protects the already-closed line model. The new plan must preserve resource
  family identity, lifecycle, delivery, branch restore, replay, and diagnostics
  truth rather than reopening them under a second effect engine.

## Adversarial Constraint

A long-lived application with collection, paged, detail, grouped, normalized,
tuple-discriminated, sparse-window, GraphQL-connection, recursive, and
JSON-bearing resource responses must be able to apply local patches, delivered
patches, optimistic writes, server confirmations, server failures, rollbacks,
branch restores, and rebases through one branch-native resource effect model.

If two semantically equivalent histories can produce:

- different committed resource line truth
- different speculative visible truth
- different branch-state, replay, restore, merge, diagnostics, or history truth
- a route-local inverse function where the declared effect had enough topology
  to derive rollback
- a broad response replacement where an aspect-local or lens-local branch
  effect was provable
- a hidden JS-side optimistic state engine beside signal branches
- a response-lens write that cannot participate in merge planning
- a rollback that needs to reinterpret live response shape instead of consuming
  recorded effect proof
- or a merge/rebase conflict that cannot name the response locus, aspect, or
  topology family that caused it

then this milestone has failed.

## Product Decision Lock

- `forge-signal` branches are the default substrate for speculative resource
  truth, not an optional future integration.
- A resource effect is the canonical artifact. Patches, delivery packets,
  optimistic writes, confirmations, rollbacks, and rebases derive from it.
- Response lenses are lowering strategies from declared response topology into
  branch-native resource effect loci.
- Every admitted effect must name its branch, basis, resource family, line,
  semantic locus, affected aspects or regions, provenance, inverse posture,
  merge posture, and cost envelope.
- Effects that cannot participate in branch restore, replay, diagnostics, and
  merge explanation must deny before changing visible truth.
- Optimistic resource updates must run on speculative branches by default unless
  the declaration proves direct committed application is the intended posture.
- Rollback must be exact branch restore, inverse effect application, or a typed
  optimistic-unavailable denial. Route-local inverse folklore is out of spec.
- Rebase must be merge-plan-driven. A resource effect cannot invent its own
  conflict model beside the signal merge subsystem.
- Aspect-local and host-declared-region effects must preserve their narrowness
  through the wasm boundary; the package must not collapse them into broad
  response replacement because it lacks a product spelling.
- Broad replacement remains legal only as an explicit branch effect with broad
  scope, broad cost, and broad diagnostics.
- UI reactions such as toasts, banners, modals, logging, and analytics are
  consumers of typed optimistic lifecycle events. The resource runtime does not
  execute UI policy.

Normative consequence:

- any implementation that stores optimistic resource state outside the signal
  branch model is out of spec
- any implementation that treats response lenses as the authority instead of
  branch-native effect lowering is out of spec
- any implementation that cannot plan, deny, rollback, rebase, or explain an
  effect without re-reading live response topology is out of spec
- any implementation that exposes only narrow local patch helpers but cannot
  express the same operation as delivery, speculation, rollback, and merge is
  below the product bar

## Architectural Model

### Authority Split

1. **Signal runtime branch authority**
   - owns branch identity, snapshots, restore, replay, merge, branch-state
     proofs, and merge proofs
   - remains the source of truth for speculative and committed runtime state
2. **Resource line authority**
   - owns family identity, line identity, lifecycle, freshness, request posture,
     local value truth, delivery basis, and resource diagnostics/history
   - admits resource effects only through proof-bearing branch/resource plans
3. **Resource effect planner**
   - converts authored local patch, delivery packet, optimistic mutation, or
     confirmation/failure input into a lowered branch-native resource effect
     plan
   - resolves branch posture, basis posture, merge posture, inverse posture,
     locus, aspect/region scope, and cost before execution
4. **Response topology lowering**
   - compiles declared response lenses into effect loci and effect-local
     replacement/inverse strategies
   - never owns branch truth, lifecycle truth, delivery truth, or mutation
     intent
5. **Optimistic lifecycle projection**
   - derives UI-observable events from branch-native effect outcomes:
     speculative applied, committed, rolled back, denied, rebased, conflicted,
     superseded, and unavailable
   - does not own UI behavior

The resource effect planner is therefore not:

- a second signal branch engine
- a response-shape guessing engine
- a transport protocol
- a UI callback runner
- a cache beside line truth

It is the wasm/product bridge that makes resource operations consume native
branch, merge, aspect, replay, restore, and proof capabilities.

### Canonical Effect Envelope

Every admitted resource effect must lower into one canonical envelope family.

The envelope must carry:

- resource family identity and line identity
- current branch id and intended target branch id
- basis snapshot id, delivery basis id, or branch-state proof where applicable
- effect provenance:
  - local patch
  - delivered patch
  - optimistic mutation
  - server confirmation
  - server failure
  - branch restore
  - merge/rebase
- semantic locus:
  - item
  - item aspect
  - JSON item aspect
  - membership
  - entity store
  - summary
  - detail field
  - broad response
- response topology family when the effect touches response-shaped value
- affected aspects and host-declared regions where available
- preimage, inverse descriptor, or optimistic-unavailable reason
- merge policy posture and conflict-isolation posture
- diagnostics/history compact facts
- performance counters for lookup, traversal, reconstruction, branch work, and
  diagnostics materialization

This envelope is the artifact from which diagnostics, history, delivery
acknowledgment, optimistic lifecycle events, replay evidence, and verification
packages derive.

### Response Lens Role

Response lenses remain important, but their role is narrower and sharper.

A response lens declaration proves:

- where resource-visible data lives inside a response topology
- which effect loci can be safely written
- which aspects, JSON paths, summaries, detail fields, membership entries, or
  entity records exist
- which topology-preserving replacement or inverse operation is legal
- which writes must deny because the topology does not support safe effect
  lowering

A compiled response lens does not execute product truth directly.
It feeds the resource effect planner with topology-specific proof.

### Optimistic Default

Optimistic resource effects are not a later product add-on.

They are the default stress test for whether the effect model is honest:

1. Capture committed branch/state proof.
2. Create or select a speculative branch.
3. Apply the lowered resource effect to the speculative branch.
4. Expose speculative visible truth through the existing resource line facade.
5. On server success, confirm or merge the speculative effect into the committed
   branch.
6. On server failure, restore the prior snapshot or apply the inverse effect.
7. On committed-branch drift, rebase through branch merge planning and emit
   conflict or rebase evidence.

If a response topology cannot support this flow, it must emit explicit
optimistic-unavailable evidence instead of pretending optimism can be added by
route code.

## Required Wasm Boundary Work

The native runtime already has more branch and merge capability than the
resource product surface currently uses.

This milestone must first close the wasm/product exposure gap:

- expose merge strategy selection where product code needs to choose or preview
  `AdoptSourceHead`, `AdoptSourceSubset`, `ReplaySourceDeltaOntoTarget`, or
  `RebaseSourceOntoTarget`
- expose merge-base policy selection where product code needs to preview rebase
  against a specific basis
- expose source-only policy selection for normalized, sparse, grouped, and
  delivery-created entities
- expose aspect policy bindings so resource effects can request
  `RequireConflict`, `PreferSource`, or `PreferTarget` for declared aspects
- preserve conflict isolation names and make per-aspect or host-declared-region
  isolation visible in product summaries
- expose branch/merge proof fields needed by resource diagnostics without
  forcing callers to parse raw native artifacts
- keep raw native artifacts available for proof lanes, but provide product
  summaries whose names match resource effect vocabulary

This work belongs before advanced response topology. If the product surface
cannot steer native branch merge semantics, advanced topology code will recreate
weaker policy logic in TypeScript.

## Implementation Topology Requirements

Production structure must make the branch-first model visible.

Expected homes:

- branch-native resource effect declaration
- branch-native resource effect planning
- branch effect admission
- speculative branch lifecycle
- server confirmation and failure resolution
- branch merge/rebase projection
- response topology lowering
- response lens capability matrix
- JSON item aspect lowering
- topology-specific effect loci
- effect inverse descriptors
- optimistic lifecycle event projection
- resource effect diagnostics/history envelope construction
- resource effect performance accounting

The tree must not collapse these into `helpers`, `utils`, `actions`,
`response_logic`, `optimistic_manager`, or a single broad response-lens file.

Every implementation file must answer one question. Examples:

- `resource_effect_branch_plan.ts`
  May own the lowered branch plan shape.
- `speculative_branch_lifecycle.ts`
  May own creation, selection, restore, and disposal posture for speculative
  branches.
- `response_lens_effect_locus.ts`
  May own how compiled response lenses describe effect-local loci.
- `json_aspect_effect_locus.ts`
  May own typed JSON path effect loci and hostile path denial.

A file named for response lenses must not own optimistic branch lifecycle.
A file named for branch effects must not parse JSON paths.
A file named for delivery must not decide optimistic rollback policy.

## Phase Deliverable Standard

Every phase must ship tangible artifacts:

- at least one type encoding the phase proof
- at least one lowering/admission function consuming the previous proof and
  producing the next proof
- at least one runtime-visible capability, denial, diagnostic, history, proof,
  or lifecycle artifact
- at least one hostile runtime test named after the invariant being certified
- type-smoke or compile-denial evidence when public TypeScript capability is
  exposed
- one cost or breadth proof whenever a public operation can look cheap

Docs may support the phase, but docs never close the phase.

## Phases

### Phase 1: Branch Capability Exposure And Product Summaries

Purpose:

- make the native branch/merge substrate fully usable from the product resource
  layer before response-specific work grows around a weaker abstraction

This phase must ship:

- product-facing merge preview request support for strategy, merge base,
  source-only policy, deletion policy, conflict policy, conflict isolation
  policy, identity matcher, and aspect policy bindings
- product summaries for selected merge semantics, aspect decision plans,
  conflict isolation plans, branch-state proofs, merge-plan proofs, and
  merge-result proofs
- type surfaces that prevent callers from forging branch proof summaries as
  admission authority
- runtime denials for unknown policy names, unsupported aspect bindings, and
  merge requests whose branch or snapshot basis is unavailable
- proof that `history()` exposes enough branch and merge truth for resource
  effect planning without falling back to raw internal imports

Phase 1 gate:

- no resource effect phase begins until product code can plan and prove a
  branch merge or rebase with the same policy dimensions the native runtime
  supports.

### Phase 2: Canonical Branch-Native Resource Effect Envelope

Purpose:

- create the canonical artifact that every patch, delivery, optimistic write,
  confirmation, failure, rollback, and rebase will derive from

This phase must ship:

- sealed or branded resource effect declaration and lowered effect plan types
- effect provenance taxonomy
- semantic locus taxonomy for item, item aspect, JSON item aspect, membership,
  entity store, summary, detail field, and broad response
- branch posture taxonomy:
  - committed branch effect
  - speculative branch effect
  - branch restore effect
  - branch merge/rebase effect
  - unavailable branch effect
- basis posture taxonomy covering line basis, delivery basis, branch-state
  proof, snapshot proof, and optimistic preimage
- one admission path that consumes the lowered effect plan before line mutation
- compact diagnostics/history facts derived from the effect envelope
- performance counters for effect planning breadth and effect execution breadth

Phase 2 gate:

- local patch and delivery paths must be expressible as the same resource
  effect envelope even before advanced response topology is added.

### Phase 3: Speculative Branch Lifecycle And Optimistic Events

Purpose:

- make optimistic resource application branch-native by default

This phase must ship:

- speculative branch creation, reuse, restore, disposal, and leak-denial posture
- optimistic application through lowered resource effect plans
- optimistic lifecycle events:
  - applied
  - committed
  - rolled back
  - denied
  - rebased
  - conflicted
  - superseded
  - unavailable
- rollback through exact branch restore where possible
- rollback through inverse effect where branch restore is unavailable but the
  effect proof is sufficient
- explicit optimistic-unavailable artifacts when neither branch restore nor
  inverse effect is safe
- no UI execution hooks; events are typed facts for UI/framework consumers

Phase 3 gate:

- an optimistic item/aspect effect must be able to apply speculatively, confirm,
  rollback, and explain itself without route-local inverse code or a parallel
  optimistic cache.

### Phase 4: Response Topology Lowering Into Effect Loci

Purpose:

- reintroduce response lenses as topology compilers that feed the branch-native
  effect model

This phase must ship:

- response lens declaration vocabulary for collection, paged, detail, summary,
  membership, entity-store, JSON item aspect, and broad response loci
- compiled response lens proof artifacts that cannot be forged by callers
- capability matrix rows that map directly to semantic effect loci
- lowering from compiled response lens proof into resource effect locus proof
- no-side-effect denial before effect construction when topology cannot justify
  the requested locus
- parity with the closed direct-array, object-items, custom collection, object
  aspects, list, and paged response-contract slice

Phase 4 gate:

- response lenses are closed only when they produce branch-native effect loci,
  not when they merely patch response values directly.

### Phase 5: JSON Item Aspect Effects

Purpose:

- make nested JSON item writes automatic without identity corruption,
  prototype pollution, stale array index folklore, or non-rollbackable mutation

This phase must ship:

- typed JSON path declarations as item-local aspect effect loci
- hostile segment denial for `__proto__`, `constructor`, and `prototype`
- required, optional, absent, null, present, object, and array-crossing rules
- frozen, sealed, cyclic, non-object, getter/setter, and non-JSON denial or
  explicit policy posture
- identity preservation proof for JSON writes
- inverse descriptors or optimistic-unavailable artifacts for each JSON write
  class
- path traversal and clone/reconstruction cost counters

Phase 5 gate:

- JSON writes may not ship as direct object mutation helpers. They must be
  branch-native item-aspect effects with rollback, denial, diagnostics, and
  cost proof.

### Phase 6: Advanced Response Topology Effect Families

Purpose:

- support serious backend response shapes through the same effect model

This phase must ship effect-locus lowering for:

- GraphQL-style connections
- normalized entity bags
- grouped collections
- tuple or heterogeneous envelopes selected by discriminator
- sparse page chunks
- map-backed collections
- multiple named collections in one response
- recursive trees with declared descendant boundaries
- detail responses
- summary responses

Each topology must prove:

- one admitted local effect
- one admitted delivery effect
- one optimistic speculative effect when reversible
- one equivalent broad replacement branch history
- one illegal topology-corrupting effect denied before side effects
- one merge/rebase or explicit merge-unavailable artifact
- cost counters naming lookup, traversal, and reconstruction breadth

Phase 6 gate:

- no topology is considered supported until local, delivery, speculative, broad
  replacement, denial, diagnostics/history, and merge/rebase posture are all
  certified.

### Phase 7: Branch Merge Rebase And Conflict Certification

Purpose:

- make resource effect conflicts explainable through native branch merge plans
  rather than resource-local conflict folklore

This phase must ship:

- resource effect rebase planning through `history().plan_merge...` proof lanes
- mapping from native node/aspect conflict evidence to resource semantic locus
  evidence
- aspect policy binding from declared resource aspects into merge preview and
  execution
- conflict isolation evidence for per-node, per-aspect, and host-declared-region
  resource effects
- typed conflict artifacts for UI/framework consumers
- denial when a resource topology cannot map native merge evidence back to a
  stable resource locus

Phase 7 gate:

- conflicting optimistic resource effects must either rebase through native
  merge proof or emit a typed conflict/unavailable artifact that names the
  resource locus and native branch evidence.

### Phase 8: Full Resource Effect Closeout

Purpose:

- prove that the milestone is one coherent branch-native product model

This phase must ship:

- suite-0-style hostile convergence for local patch, delivery patch,
  optimistic write, confirmation, failure, rollback, branch restore, merge,
  rebase, broad replacement, and diagnostics/history reads
- type-denial coverage for forged effect plans, forged lens proofs, illegal
  capability rows, and capability unions where legality is only maybe present
- documentation examples for branch-native optimistic resource effects,
  response-lens topology declarations, JSON effects, advanced topology effects,
  and UI lifecycle event consumption
- closeout matrix tying every effect family to runtime tests, type denials,
  diagnostics/history proof, branch/merge proof, and performance evidence

Phase 8 gate:

- the milestone is not closed until equivalent committed and speculative
  histories converge across branch restore, replay, and merge proof surfaces.

## Certification Requirements

Every proof family must follow the resource certification standard:

- define the hostile workload
- verify runtime behavior, public facade behavior, diagnostics/history behavior,
  branch/restore behavior, and type-surface behavior where relevant
- emit a canonical verification package rather than only assertion pass/fail
- include explicit denial artifacts for ineligible or unsupported work
- prove no side effects on denial before value, diagnostics, lifecycle, branch,
  or history truth changes
- name the performance boundary when a public API looks cheap

### The Branch Capability Exposure Test

Purpose:

- prove the product history surface exposes the native branch/merge dimensions
  resource effects need

What to stress:

- strategy selection
- merge-base selection
- source-only policy selection
- aspect policy bindings
- conflict isolation policy selection
- unknown policy names
- missing branch and missing snapshot basis

What to verify:

- product summaries preserve selected native semantics
- proof envelopes remain available
- callers cannot forge branch proof authority
- unsupported policy dimensions deny before merge execution

Pass condition:

- emit branch catalog digest, merge preview digest, selected-semantics digest,
  proof-envelope digest, policy-denial digest, and type-boundary digest

### The Canonical Resource Effect Envelope Test

Purpose:

- prove local patches and delivery packets lower into one effect artifact

What to stress:

- item replacement
- item-aspect replacement
- summary replacement
- broad replacement
- equivalent local and delivered paths
- malformed basis and stale delivery basis

What to verify:

- both paths emit the same semantic effect envelope where equivalent
- denials happen before line mutation
- diagnostics/history derive from the effect envelope, not from separate local
  and delivery code paths

Pass condition:

- emit effect-declaration digest, lowered-effect digest, basis digest,
  diagnostics/history digest, no-side-effect digest, and execution-breadth
  envelope

### The Speculative Branch Lifecycle Test

Purpose:

- prove optimistic resource state lives on signal branches rather than a
  package-local cache

What to stress:

- optimistic apply
- server success
- server failure
- branch restore rollback
- inverse-effect rollback
- unavailable rollback
- speculative branch disposal
- repeated optimistic writes on one line

What to verify:

- speculative visible truth is branch-local
- committed truth remains unchanged until confirmation or merge
- rollback restores exact previous truth
- no orphan speculative branch state survives disposal
- UI lifecycle events are typed facts and do not execute UI callbacks

Pass condition:

- emit committed-branch digest, speculative-branch digest, optimistic-event
  digest, rollback digest, disposal digest, and branch-state proof digest

### The Response Lens Effect Locus Test

Purpose:

- prove response lenses lower topology into branch-native effect loci

What to stress:

- direct arrays
- object item fields
- explicit custom collections
- list and paged parity
- detail responses
- summary responses
- unsupported topology

What to verify:

- capability matrix rows map to resource effect locus variants
- unsupported rows deny before effect construction
- closed response-contract behavior remains a subset
- compiled lens proofs cannot be forged

Pass condition:

- emit response-declaration digest, compiled-lens digest, effect-locus digest,
  parity digest, denial digest, and compile-boundary digest

### The JSON Branch Effect Test

Purpose:

- prove JSON item writes are safe branch-native aspect effects

What to stress:

- missing required paths
- optional absent, null, and present paths
- array crossing
- stale positional identity attempts
- `__proto__`, `constructor`, and `prototype`
- identity-field writes
- frozen, sealed, cyclic, getter/setter, and non-object receivers

What to verify:

- hostile paths deny before mutation
- identity-changing writes deny
- admitted writes emit inverse or optimistic-unavailable evidence
- branch rollback restores the exact preimage
- path cost counters name traversal and reconstruction breadth

Pass condition:

- emit parsed-path digest, protected-identity digest, denial digest,
  inverse-posture digest, rollback digest, and path-cost envelope

### Advanced Topology Branch Effect Tests

Required topology proof families:

- `The GraphQL Connection Branch Effect Test`
- `The Normalized Entity Bag Branch Effect Test`
- `The Grouped Collection Branch Effect Test`
- `The Tuple Envelope Branch Effect Test`
- `The Sparse Page Chunk Branch Effect Test`
- `The Map Backed Collection Branch Effect Test`
- `The Multiple Collection Branch Effect Test`
- `The Recursive Tree Branch Effect Test`
- `The Detail Response Branch Effect Test`
- `The Summary Response Branch Effect Test`

Each topology proof must stress:

- one admitted local effect
- one equivalent delivered effect
- one optimistic effect when reversible
- one broad replacement branch history
- one illegal topology-corrupting effect
- one branch restore after the effect
- one merge/rebase case or explicit merge-unavailable denial

Each topology proof must verify:

- equivalent local, delivery, optimistic-confirmed, and broad-replacement
  histories converge when they mean the same thing
- illegal effects deny before value, diagnostics, lifecycle, branch, or history
  mutation
- diagnostics/history name the topology and effect locus
- performance counters name lookup, traversal, and reconstruction breadth

Each topology proof must emit:

- topology declaration digest
- effect-locus digest
- local-effect digest
- delivery-effect digest
- optimistic-posture digest
- branch-restore digest
- merge/rebase digest or merge-unavailable artifact
- denial digest
- topology performance envelope

### The Rebase Conflict Explanation Test

Purpose:

- prove optimistic resource conflicts are explained through native branch merge
  evidence and resource loci together

What to stress:

- committed branch drift after optimistic apply
- two optimistic effects touching disjoint aspects
- two optimistic effects touching the same aspect
- normalized entity and membership conflicts
- grouped movement conflicts
- sparse unloaded item conflicts
- topology that cannot map native conflict evidence back to a stable resource
  locus

What to verify:

- disjoint aspect effects can rebase where native merge permits it
- conflicting effects emit typed conflict artifacts
- conflict artifacts name both native branch evidence and resource semantic
  locus
- unsupported topology mapping denies explicitly

Pass condition:

- emit native-merge-plan digest, resource-locus digest, aspect-policy digest,
  conflict-isolation digest, rebase digest, conflict digest, and unavailable
  artifact where relevant

### The Full Branch-Native Resource Effect Convergence Test

Purpose:

- prove the whole milestone is one system

What to stress:

- local patch
- delivered patch
- optimistic write
- server success
- server failure
- rollback
- branch restore
- merge/rebase
- broad replacement
- diagnostics summary
- rich history
- retained replay

What to verify:

- equivalent histories converge to the same committed line truth
- speculative histories remain branch-local until admitted
- diagnostics summary stays compact
- rich history reconstructs from canonical effect envelopes
- replay/restore/merge proofs stay aligned with resource effect proof

Pass condition:

- emit family identity digest, line identity digest, branch-state digest,
  effect-envelope digest, optimistic-lifecycle digest, rollback digest,
  merge/rebase digest, diagnostics/history digest, replay/restore digest, and
  boundary performance envelope

## Acceptance Evidence

This milestone is complete only when the wasm product surface can prove:

- resource effects are canonical artifacts for local patch, delivery,
  optimistic write, confirmation, failure, rollback, branch restore, and rebase
- product history exposes the native branch/merge controls resource effects
  need, including aspect policy bindings
- response lenses lower into branch-native effect loci rather than executing a
  parallel response patch engine
- optimistic resource truth lives on signal branches by default
- rollback is exact branch restore, inverse effect, or explicit unavailable
  artifact
- rebase and conflict explanation use native branch merge plans plus
  resource-locus mapping
- JSON and advanced response topology effects preserve identity, topology,
  denial atomicity, diagnostics/history, and cost honesty
- UI lifecycle behavior is exposed as typed events and never executed by the
  resource runtime
- type denials and runtime denials agree on every public capability boundary

## Performance And Cost Contracts

Required cost posture:

- effect planning happens before execution
- execution consumes lowered branch/resource/lens proof, not raw declarations
- branch work, response lookup, response reconstruction, inverse capture,
  diagnostics materialization, and merge planning each expose separate counters
- direct entity-store and map-backed loci must prove O(1) lookup where declared
- array, grouped, connection, sparse, recursive, and multiple-collection loci
  must name visible-item, group, page, loaded-range, descendant, or collection
  breadth explicitly
- speculative branch application must name branch creation/reuse/restoration
  cost rather than hiding it inside patch execution
- diagnostics summary must consume compact effect facts and not materialize rich
  replay or merge artifacts

Any helper that hides a broad scan, branch snapshot, merge plan, inverse capture,
or rich diagnostics reconstruction behind a cheap-looking method name is out of
spec unless the cost is named and certified.

## Out Of Scope

- service-worker synchronization
- network transport ownership
- UI toast/banner/modal execution
- arbitrary response topology inference without declaration
- arbitrary item identity inference without declaration
- identity migration after patch unless a later milestone designs it
- core `forge-signal` branch or merge semantics that do not already exist or
  are not intentionally added to the native crate first
- generic aspect-capacity redesign beyond using the current aspect surface
  honestly

## Sequencing Notes

This milestone replaces the earlier response-lens-centered Milestone 10 shape.

It still belongs after API Surface DX Hardening because:

- the API/resource line model had to be closed before resource effects could be
  made branch-native
- response-contract auto patching remains a valid subset and migration target
- the DX milestone exposes the ergonomic route lane that this milestone now
  strengthens with branch-native effects

It still belongs after Worker-First Runtime Placement because:

- the worker milestone is already closed as Milestone 9 and should not be
  renumbered retroactively
- branch-native resource effects benefit from worker placement because branch,
  replay, restore, merge planning, and diagnostics are exactly the kind of
  runtime work that should not be trapped on the UI thread

It belongs before roadmap completion because:

- resource mutation, delivery, router continuity, forms submission, and external
  integration should not normalize route-local optimistic caches while the
  signal runtime already has real branch machinery
- advanced response topology should not grow as TypeScript response-patching
  folklore beside native merge and branch proofs

## Self-Check

- Does this milestone solve a real structural problem?
  Yes. It replaces response-helper-centered patching with branch-native resource
  effects.
- Is the adversarial constraint precise and load-bearing?
  Yes. It forces local, delivery, optimistic, rollback, branch restore, merge,
  and response topology through one proof model.
- Does the milestone preserve crate authority boundaries?
  Yes. Native signal branches own branch/merge truth; resource lines own line
  truth; response lenses own topology lowering only.
- Does the milestone define proof obligations?
  Yes. It names branch exposure, effect envelope, speculative lifecycle,
  response-locus lowering, JSON, topology, rebase conflict, and full convergence
  proof families.
- Could a competent engineer map this spec into honest types, modules, and
  tests?
  Yes. The phases name the proof types, effect envelope, locus variants,
  lifecycle surfaces, and certification lanes.
- Does the milestone belong in this roadmap sequence?
  Yes. It follows the closed resource/API and worker milestones and prevents
  the next resource product layer from creating a weaker optimistic engine.
