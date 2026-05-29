# Worth Spatial Refactor

## Goal

Define the refactor program that makes `worth-spatial` Query-native end to
end.

This document exists to drive a folder-by-folder rewrite of `worth-spatial`
until:

- `forge-query` is the front door for runtime-facing spatial work
- `worth-spatial` owns domain semantics instead of runtime-shaped ceremony
- numeric truth is admitted through `worth-math` instead of ambient `f64`
- spatial code stops rebuilding local proof, diagnostics, workflow, binding,
  and recovery subsystems
- the crate becomes a clean semantic partner to Query rather than a second
  platform pretending runtime has not started yet

This document was explored incrementally, but it should now be implemented in
clear dependency order rather than the order in which the sections were first
written.

## Why This Refactor Exists

`worth-spatial` has real domain value, but too much of the crate still carries
an older architecture:

- local proof ceremony
- local foundational materialization
- local workflow-like staging
- local runtime declaration adapters
- local support and explanation shaping
- local public seams that behave as if Query only comes later

That is no longer the honest model.

After `forge-query` `9.3.8`, serious spatial work does not need to begin in a
pre-runtime holding area and eventually get promoted into Query. Query can now
be the entrypoint.

That changes the role of `worth-spatial`.

`worth-spatial` should now:

- own authored spatial vocabulary
- own local geometric and numeric admission rules
- own family-specific semantic meaning
- define how spatial families lower onto Query declaration families, helpers,
  grouped surfaces, continuation seams, and recovery surfaces
- preserve the semantic facts Query must retain, narrow, inspect, bind, and
  recover from

`worth-spatial` should not:

- own a second proof pipeline once work is Query-facing
- own a second diagnostics or evidence pipeline once work is Query-facing
- own a second workflow story once work is Query-facing
- own adapter-era glue that translates finished local products into old Query
  request forms

This refactor exists because the crate currently does some of the right domain
work with too much local infrastructure wrapped around it.

## Governing Summaries

- `MENTALITY.md`
  - Protects: solving the hard structural problem first instead of nibbling at
    features.
  - Main constraint here: this refactor must attack the real architectural
    failure mode first, which is duplicate platform behavior around otherwise
    valuable domain semantics.

- `arch_laws.md`
  - Protects: authority separation, proof-bearing progression, declared
    effects, and explicit boundary artifacts.
  - Main constraint here: once a spatial family is Query-facing, Query should
    own the public artifact and lifecycle boundary rather than Worth faking one
    locally.

- `composition_laws.md`
  - Protects: named semantic steps instead of god files and helper swamps.
  - Main constraint here: local semantic kernels should stay local, but local
    proof/materialization/runtime adapter layers should collapse into Query
    surfaces rather than accumulating wrappers.

- `domain_structure_laws.md`
  - Protects: physical structure that preserves meaning, authority, and
    lifecycle distinctions.
  - Main constraint here: `worth-spatial` must separate authored semantics,
    local numeric admission, family-specific semantic kernels, Query-family
    lowering, and public helper seams instead of flattening them into one broad
    "lowering" or "runtime handoff" area.

- `perf_laws.md`
  - Protects: explicit cost surfaces, no repeated rediscovery, and no hidden
    breadth.
  - Main constraint here: spatial flows must not repeatedly re-admit anchors,
    re-resolve witness meaning, or re-explain support posture when that meaning
    could be retained once by Query artifacts.

- `worth_roadmap.md`
  - Protects: Worth as a manufacturing-grade geometry system that uses Forge
    runtimes honestly.
  - Main constraint here: geometry meaning should remain a Worth strength, but
    runtime-facing lifecycle and composition should enter through Query and stay
    there.

- `worth/test-requirements.md`
  - Protects: typed failure, replay honesty, diagnostics sufficiency, and
    workflow-class closure.
  - Main constraint here: the refactor must improve typed denial, inspection,
    and recovery rather than merely shuffling code.

- `forge_query_vision.md`
  - Protects: Query as the typed, aspect-aware, live-promotable public layer.
  - Main constraint here: Query should carry spatial meaning as retained,
    aspect-qualified declaration truth rather than as a late wrapper over a
    Worth-local pseudo-platform.

- `forge_query_roadmap.md`
  - Protects: Query as the finished runtime-facing composition layer.
  - Main constraint here: this refactor should target the shipped Query surface
    categories that the docs now teach: platform entry, configured handles,
    binding, declaration pipeline, helpers, workflows, recipes, inspection,
    recovery, grouped work, continuation, and certification.

- `crates/forge-query/docs/README.md`
  - Protects: Query docs are organized by job, not by implementation order.
  - Main constraint here: this refactor should describe spatial integration in
    terms of the public Query doc roots a user would actually reach for, not in
    terms of old internal adapter history.

- `crates/forge-query/docs/domain-capabilities/README.md`
  - Protects: domain work should start from admitted handles, binding,
    ordinary outcomes, recovery, helpers, declaration pipeline, and
    continuation seams.
  - Main constraint here: `worth-spatial` should align its public story to that
    TOC instead of teaching private proof/materialization subsystems first.

## Adversarial Constraint

`worth-spatial` must survive this hostile condition:

> A long-lived branch-bearing geometry system with ambiguous anchors,
> feature-owned and tag-owned references, preview and replay posture,
> hostile numeric edge cases, identity evolution, grouped authoring, and
> AI-authored spatial requests must preserve the same semantic admission
> result, the same numeric legality result, the same retained aspect posture,
> the same workflow classification, and the same repair guidance regardless of
> whether the ask entered through a helper, a bound context request, a direct
> declaration input, a grouped surface, or a continuation-facing next-step
> surface.

If `worth-spatial`:

- lets raw `f64` values masquerade as admitted semantic truth
- lets witness or anchor meaning remain disposable local glue
- rebuilds proof/materialization/workflow/binding/recovery surfaces locally
- loses semantic slice meaning across Query handoff boundaries
- or forces Query-facing layers to rediscover spatial meaning from old local
  adapter artifacts

then the refactor has failed.

## Query-First Mental Model

Relational is the truth-bearing graph and state authority. Signal is the
derived DAG and invalidation engine. Bridge keeps truth and derivation coherent
across runtime boundaries. Query is the ergonomic public operating layer that
turns all of that lower machinery into:

- platform entry
- configured handles
- declarations
- progression
- foundational evidence
- routes
- receipts
- envelopes
- binding
- grouped work
- helper ergonomics
- continuation
- inspection
- readiness
- recovery

This matters here because spatial work no longer needs a "before Query" public
life cycle.

The honest model is:

1. Query is the front door for runtime-facing spatial work.
2. `worth-spatial` contributes domain semantics to that door.
3. Query retains and carries the public lifecycle.
4. Worth stays responsible for domain meaning, not duplicate platform glue.

## End-To-End Workflow

This refactor should now be judged against one explicit end-to-end spatial
workflow.

### Ordinary Path

1. enter Query through platform entry
2. admit a configured geometry domain handle
3. optionally bind the next declaration input from current context or retained
   artifacts
4. declare one spatial family through Query
5. run legality and progression through Query
6. materialize route, receipt, and envelope through Query when that family is
   runtime-facing
7. move to grouped work, signal compatibility, continuation, inspection, or
   recovery through Query-owned seams

### Helper Path

1. enter Query through the admitted geometry handle
2. call a family-native helper
3. lower onto the same canonical Query declaration, binding, and orchestration
   seams
4. receive the same ordinary, checked, or proof-visible posture as the generic
   Query surfaces

### Recovery Path

1. a spatial family stops on denial, ambiguity, unsupported posture,
   stale/rebind posture, wrong-world posture, or authority mismatch
2. Query inspection explains the retained stop truth
3. Query recovery provides the next-step class
4. Worth-specific semantics remain visible as family meaning, not as a second
   local recovery engine

## Folder-By-Folder Program

This refactor will proceed folder by folder through `worth-spatial`.

For each folder we must answer:

1. what is irreducible local spatial meaning?
2. what is actually local numeric admission?
3. what Query-facing lifecycle is being rebuilt locally?
4. what should be deleted, collapsed, or moved into `worth-math`?
5. what should become a declaration family, helper surface, binding seam,
   grouped surface, inspection surface, or recovery surface in Query?

The implementation order should be:

1. `spatial_intent/refs` + `spatial_intent/resolution`
   - turn witness and frame resolution from a local proof/materialization
     subsystem into a Query-declared witness family with helper entrypoints
2. `spatial_intent/lowering`
   - rebuild the runtime-facing lowering story around real Query declaration
     pipeline artifacts instead of old intent/effect-era adapter thinking
3. `spatial_intent/arbitration`
   - keep the semantic arbitration core, delete the fake proof ladder and
     runtime/materialization glue, and move the public story onto Query
     declaration/inspection/recovery seams
4. `spatial_intent/preview` + `spatial_intent/constraints` +
   `spatial_intent/continuity`
   - rebuild these as downstream Query workflow and continuation families over
     retained spatial meaning instead of local workflow-shaped summaries
5. `bindings`
   - center primitive birth as domain semantics and consequence truth, then
     project its runtime-facing lifecycle through Query
6. `certification` + `facade`
   - narrow the public Worth surface to local semantics plus explicit Query
     helper/integration seams
7. README and doc alignment
   - update `worth-spatial` README and related Worth docs so they teach the
     Query-first story and stop centering adapter-era local lifecycles

## Current Status

The refactor is not starting from zero.

### What Is Already Valuable

- `refs` contains real authored vocabulary
- `resolution` contains real witness semantics
- `lowering` already exposed that local proof/materialization/runtime adapter
  layers were separable concerns
- `arbitration` already isolated real conflict semantics from policy and
  capability posture

### What Is Now Stale

- "pre-runtime" as the primary public framing
- local `forge-proof` progression as a public-facing transition story
- local `forge-foundational` materialization as the normal public explanation
  path
- old `ForgeQueryIntent*` and effect-era adapter thinking as the target Query
  seam
- Worth-local runtime declaration wrappers that exist mainly to translate a
  finished local product into legacy Query nouns

### Immediate Consequence

The old Query mapping sections should not be used as implementation authority
without reinterpretation. Their broad architectural instinct was useful, but
their Query-facing noun set and staging model no longer match the current
public Query product.

## Query Surface Map For This Refactor

This section replaces the old "Query API mapping" mindset.

The right question is no longer:

> which old Query type name do we translate this local product into?

The right questions are:

- what Query surface category should this family enter through?
- does this job need binding, declaration entry, grouped work, continuation, or
  recovery?
- should the ergonomic story be generic, helper-driven, or both?

### Start Here

The most important Query doc roots for this refactor are:

- `crates/forge-query/docs/README.md`
- `crates/forge-query/docs/domain-capabilities/README.md`
- `crates/forge-query/docs/domain-capabilities/choosing/README.md`
- `crates/forge-query/docs/domain-capabilities/workflow/README.md`
- `crates/forge-query/docs/domain-capabilities/recipes/README.md`

### Core Surface Categories

For runtime-facing spatial work, prefer:

- platform entry
- configured domain handles
- typed binding pipeline
- ordinary outcomes
- recovery boundary
- family helpers
- canonical domain declarations
- declaration progression
- declaration foundational evidence
- declaration route plans, receipts, and envelopes
- declaration entry orchestration
- declaration entry inspection and readiness
- grouped authoring, grouped products, and grouped contributions
- signal compatibility orchestration
- continuation pipeline
- support and traceability docs

### Rules For Using The Map

When refactoring any spatial slice:

1. identify the irreducible spatial semantics
2. identify the first honest Query-facing job
3. choose the Query surface category for that job
4. only then choose the local Worth helper or declaration family shape

The burden of proof is now on local lifecycle reinvention, not on Query use.

## `spatial_intent/refs` + `spatial_intent/resolution`

This slice should become the clearest early example of the new architecture.

### What Must Stay Local

- authored witness vocabulary
- frame vocabulary
- anchor vocabulary
- carrier and parameter-space semantics
- numeric admission rules
- catalog-backed resolution semantics

### What Is Currently Stale

This slice still behaves like a local mini-platform:

- local request/admit/resolve proof ladders
- local public resolution entrypoints
- local foundational support/explanation/provenance materialization

Those are no longer the right public seams.

### Rewrite Direction

This slice should become:

- one or more spatial witness declaration families
- optional typed binding from current context or retained artifacts
- helper-driven geometry entrypoints for common witness jobs
- Query-owned progression, inspection, ordinary outcomes, and recovery

The old local proof and materialization code may still exist briefly as
internal transition machinery, but it should stop being the public story and
should be deleted once the Query-family seam lands cleanly.

### Query Surface Target

- configured domain handles
- typed binding pipeline
- canonical domain declarations
- declaration progression
- declaration entry inspection
- recovery boundary
- family helpers

### Acceptance Evidence

- witness resolution can begin through Query, not only through local helper
  chains
- the same witness meaning can enter through explicit declaration input or
  helper ergonomics
- support, ambiguity, and denial posture are visible through Query inspection
  and recovery rather than only through local foundational report builders

## `spatial_intent/lowering`

This slice remains important, but the target Query seam has changed.

### What Must Stay Local

- local movement, rotation, and placement semantics
- anchor interpretation
- witness-fed geometric meaning
- numeric and directional normalization rules

### What Must Change

Lowering should stop aiming at old Query adapter nouns and instead target:

- declaration families that preserve aspect-qualified spatial meaning
- route/receipt/envelope posture retained by Query
- binding-ready retained artifacts for later workflow or continuation steps
- Query inspection and recovery instead of Worth-local explanation surfaces

### Query Surface Target

- canonical domain declarations
- declaration progression
- declaration foundational evidence
- declaration route plans
- declaration boundary receipts
- declaration boundary envelopes
- declaration entry orchestration
- declaration entry inspection
- typed binding pipeline

### Acceptance Evidence

- a lowered spatial family can flow end to end through declaration to envelope
- later binding and continuation work consume retained Query artifacts rather
  than Worth-local handoff structs
- aspect-qualified spatial meaning survives into inspection and later binding

## `spatial_intent/arbitration`

Arbitration has one of the cleanest semantic cores and one of the stalest glue
layers.

### What Must Stay Local

- candidate generation
- candidate ranking
- policy profile interaction
- capability-sensitive conflict classification
- escalation choice

### What Should Be Deleted

- infallible local proof ladders that only wrap the same payload
- local runtime declaration adapters that translate finished local products
  into old Query request forms
- local foundational support/explanation materialization as the normal public
  story

### Rewrite Direction

Arbitration should become:

- one Query declaration family over Worth-owned semantic analysis
- one helper surface for common arbitration jobs
- one Query inspection and recovery story
- one typed support/readiness story where blocked capability posture matters

### Query Surface Target

- canonical domain declarations
- declaration progression
- declaration entry inspection
- declaration entry readiness
- recovery boundary
- family helpers
- support and traceability

### Acceptance Evidence

- arbitration can enter through Query directly
- blocked, advisory, and clarification posture project onto Query-native stop
  and recovery surfaces
- the local semantic core remains visible without exposing the old adapter
  layers as durable public seams

## `spatial_intent/preview` + `spatial_intent/constraints` + `spatial_intent/continuity`

These folders should no longer be described as local workflow stand-ins.

### What Must Stay Local

- preview-local geometric semantics
- constraint semantics
- continuity and identity evolution semantics that are genuinely spatial

### Rewrite Direction

These slices should compile onto:

- workflow guides
- signal compatibility orchestration
- continuation pipeline
- grouped surfaces where neighborhood meaning matters
- recovery boundary for stop classification

The public story should be:

- Query workflow family
- Query continuation family
- Query grouped family

not:

- Worth-local workflow summary objects that later happen to hand off into Query

### Acceptance Evidence

- preview and continuity can be described as end-to-end Query workflows
- the next-step classification after preview/continuity work uses Query
  inspection and recovery
- grouped and continuation seams reuse retained Query artifacts instead of
  local pseudo-artifacts

## `bindings`

This slice is really about primitive birth and consequence posture.

### What Must Stay Local

- primitive family semantics
- topology birth class semantics
- geometric/topology admission
- primitive birth planning

### What Must Change

This slice should stop producing a thick local report ecology as if that were
the final product surface.

It should instead center:

- one admitted primitive birth artifact family
- one clear consequence model
- one Query-facing consequence and aftermath story for runtime-facing cases

### Query Surface Target

- canonical domain declarations
- declaration progression
- aftermath review/support/eligibility/materialization
- support and traceability
- recovery boundary
- family helpers where common birth workflows deserve them

### Acceptance Evidence

- primitive birth consequence posture no longer depends on many parallel local
  report families
- Query can carry the runtime-facing consequence story without rediscovering
  primitive birth meaning from scattered adapters

## `certification` + `facade`

This slice must now certify the new public honesty.

### Public Surface Goal

`worth-spatial` should expose:

- authored spatial vocabulary
- irreducible local semantic kernels
- explicit Query-facing helper and integration seams

It should stop behaving like the final runtime-facing operating surface.

### Certification Goal

Certification should prove:

1. the local semantic surface is honest
2. the Query-facing seam is honest
3. the public surface is intentionally narrow
4. helper ergonomics are projections over canonical Query lanes, not second
   engines

### README Goal

The README must teach:

- Query-first entry
- what local semantics Worth owns
- what public lifecycle Query owns
- one or two small end-to-end code paths through the modern Query surface

It must stop centering:

- local `forge-proof` artifacts
- local `forge-foundational` materialization helpers
- adapter-era runtime declaration wrappers

### Acceptance Evidence

- facade exports are narrower and more intentional
- compile-fail tests grow where local implementation details should stay hidden
- public API tests are grouped by real surface family
- README and spec teach the same Query-first story

## Concrete Rewrite Rules

When touching a spatial slice during this refactor:

1. keep the domain semantics
2. delete fake proof ladders with no real semantic authority
3. delete local foundational materialization layers when Query inspection or
   recovery should own the public story
4. replace old Query adapter nouns with current Query surface categories
5. add helper ergonomics only as projections over canonical Query lanes
6. update the README or nearby docs when the public mental model changes

## What This Refactor Is Not

This refactor is not:

- a demand that every local spatial helper disappear
- a demand that every domain type become generic Query vocabulary
- a claim that Query owns spatial semantics
- a claim that all old internal code must be deleted before any new seam lands

It is a demand that:

- runtime-facing entry begins honestly in Query
- Worth owns semantic truth, not duplicate platform glue
- public spatial workflows align to the Query product that actually exists now

## Acceptance Standard

This document is only fulfilled if:

- Query is the clear runtime-facing front door for spatial work
- the old "pre-runtime first, Query later" framing is gone
- the public story is organized around current Query surfaces
- local proof/materialization/runtime adapter glue is systematically collapsed
- the folder-by-folder plan preserves strong spatial semantics while deleting
  stale platform duplication
- the README, spec, and public API all teach the same architecture
