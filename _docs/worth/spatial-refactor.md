# Worth Spatial Refactor

## Goal

Rewrite `worth-spatial` into a crate that owns spatial semantics cleanly and
uses `forge-query` honestly for runtime-facing lifecycle.

This refactor is folder-driven. The work should proceed in dependency order
through the crate, not by scattering edits across unrelated subsystems.

The intended end state is:

- `worth-spatial` owns irreducible spatial meaning
- `forge-query` owns runtime-facing handle, binding, declaration, inspection,
  recovery, and workflow lifecycle
- numeric admission is explicit and typed
- public entry no longer starts in a Worth-local pseudo-platform
- each folder has one defensible authority story

## Recommendation

Rewrite this plan from scratch and treat the old one as historical context
only.

The previous document had the right diagnosis, but it mixed:

- migration history
- stale Query-era framing
- architectural thesis
- implementation notes
- acceptance rules

That shape is no longer useful. The crate now needs one fresh plan organized by
folder authority and current Query surfaces.

## Governing Rules

This refactor is constrained by:

- `MENTALITY.md`
  - solve the real structural problem first
- `arch_laws.md`
  - authority, lifecycle, and proof boundaries must be explicit
- `composition_laws.md`
  - semantic steps must stay named and narrow
- `domain_structure_laws.md`
  - folder layout must preserve meaning and authority
- `perf_laws.md`
  - do not repeatedly rediscover admitted meaning

The practical consequence is simple:

- keep spatial semantics local
- delete local Query-shaped lifecycle duplication
- add Query runtime capability when a real runtime gap exists
- do not protect old local glue just because code already exists

This plan assumes zero backward compatibility with the pre-refactor seam
shapes.

That means:

- do not preserve old local seam names just to avoid churn
- do not keep compatibility wrappers, aliases, or adapter facades once the new
  boundary exists
- do not keep legacy progression, materialization, or support seams alive in
  tests after production no longer owns them
- do not treat test-only survival of an old seam as an acceptable end state

## Query Alignment

The relevant Query docs for this plan are:

- `crates/forge-query/docs/domain-capabilities/platform-entry.md`
- `crates/forge-query/docs/domain-capabilities/canonical-domain-declarations.md`
- `crates/forge-query/docs/domain-capabilities/configured-domain-handles.md`
- `crates/forge-query/docs/domain-capabilities/typed-binding-pipeline.md`
- `crates/forge-query/docs/domain-capabilities/declaration-legality.md`
- `crates/forge-query/docs/domain-capabilities/declaration-progression.md`
- `crates/forge-query/docs/domain-capabilities/declaration-entry-orchestration.md`
- `crates/forge-query/docs/domain-capabilities/declaration-entry-inspection.md`
- `crates/forge-query/docs/domain-capabilities/declaration-entry-readiness.md`
- `crates/forge-query/docs/domain-capabilities/declaration-boundary-receipts.md`
- `crates/forge-query/docs/domain-capabilities/declaration-boundary-envelopes.md`
- `crates/forge-query/docs/domain-capabilities/ordinary-outcomes.md`
- `crates/forge-query/docs/domain-capabilities/family-helpers.md`
- `crates/forge-query/docs/domain-capabilities/grouped-authoring.md`
- `crates/forge-query/docs/domain-capabilities/grouped-contributions.md`
- `crates/forge-query/docs/domain-capabilities/grouped-products.md`
- `crates/forge-query/docs/domain-capabilities/continuation-pipeline.md`
- `crates/forge-query/docs/domain-capabilities/signal-compatibility-orchestration.md`
- `crates/forge-query/docs/domain-capabilities/recovery-boundary.md`
- `crates/forge-query/docs/domain-capabilities/aftermath/aftermath-review-support-eligibility-and-materialization.md`
- `crates/forge-query/docs/domain-capabilities/workflow/README.md`
- `crates/forge-query/docs/domain-capabilities/workflow/preview-inspection-and-mutation-planning.md`
- `crates/forge-query/docs/domain-capabilities/workflow/grouped-neighborhood-workflow.md`
- `crates/forge-query/docs/domain-capabilities/workflow/retained-artifact-to-next-step.md`
- `crates/forge-query/docs/domain-capabilities/workflow/stop-to-recovery.md`
- `crates/forge-query/docs/domain-capabilities/recipes/README.md`
- `crates/forge-query/docs/capabilities/existing-truth.md`
- `crates/forge-query/docs/capabilities/inspection.md`
- `crates/forge-query/docs/capabilities/lineage-and-correspondence.md`
- `crates/forge-query/docs/capabilities/projection-consumption.md`

The boundary rule those docs imply is:

- `worth-spatial` owns spatial identity, witness meaning, frame meaning,
  geometric legality, and family-specific semantics
- `forge-query` owns admitted handle posture, binding, declaration entry,
  runtime progression, retained artifacts, inspection, recovery, grouped work,
  continuation, and workflow routing

If a `worth-spatial` folder owns `progression`, `materialization`,
`runtime_declaration`, or a binding-like seam after runtime-facing semantics
already exist, that folder should be treated as suspect.

The docs above are not all equally important in every folder. The working set
by folder should be:

- `refs`
  - configured handles
  - canonical declarations
  - family helpers
- `resolution`
  - typed binding
  - declaration legality
  - declaration entry inspection
  - ordinary outcomes
  - recovery
- `lowering`
  - declaration entry orchestration
  - receipts
  - envelopes
  - retained-artifact-to-next-step
- `arbitration`
  - declaration entry readiness
  - inspection
  - recovery
  - stop-to-recovery
- `preview` / `constraints` / `continuity`
  - preview-inspection-and-mutation-planning
  - grouped-neighborhood-workflow
  - continuation pipeline
  - signal compatibility
  - lineage and correspondence
- `bindings`
  - aftermath review/support/eligibility/materialization
  - projection consumption
  - existing truth
- `certification` / `facade`
  - platform entry
  - public workflow README surfaces
  - inspection
  - recovery

## Adversarial Constraint

This refactor succeeds only if the same spatial ask preserves the same meaning
and stop posture regardless of entry mode:

- direct declaration input
- helper entry
- bound next-step entry
- grouped contribution
- continuation-facing entry
- inspection or recovery revisit

The crate fails this refactor if it:

- admits raw numeric values as if they were already semantic truth
- forces Query-facing code to rediscover witness or frame meaning
- rebuilds proof, diagnostics, workflow, or recovery as a local second system
- loses spatial family meaning across retained Query artifacts

## Folder Order

Implementation order:

1. `spatial_intent/refs`
2. `spatial_intent/resolution`
3. `spatial_intent/lowering`
4. `spatial_intent/arbitration`
5. `spatial_intent/preview`, `constraints`, `continuity`
6. `bindings`
7. `certification`, `facade`, README/doc alignment

This order follows dependency truth. `refs` defines the authored vocabulary
floor. `resolution` interprets that vocabulary. `lowering` projects admitted
meaning onto Query declaration lanes. `arbitration` depends on earlier semantic
shapes. The remaining folders should only be cleaned up after the core
spatial-intent chain is honest.

## Per-Folder Questions

Every folder rewrite must answer the same questions:

1. What irreducible spatial authority lives here?
2. What current contents are actually Query lifecycle duplication?
3. What must stay local?
4. What must collapse onto Query?
5. What test or certification evidence closes this folder?

## Folder 1: `spatial_intent/refs`

### Purpose

This folder should be the authored vocabulary floor for spatial asks.

### Local Authority That Must Stay

- anchor vocabulary
- frame vocabulary
- witness vocabulary
- witness catalog identity
- authored reference semantics

### What Must Not Live Here

- runtime progression
- support-report ecosystems
- public proof ladders
- materialized explanation layers
- fake binding or request staging

### Target Shape

`refs` should become a clean semantic substrate consumed by later folders. It
should not present itself as a runtime-facing subsystem.

### Acceptance

- the folder reads as authored vocabulary, not workflow
- public exports are semantic nouns, not progression nouns
- no local runtime-facing explanation surface competes with Query

### Current Status

- complete
- public `refs` and witness-catalog vocabulary now lives under explicit facade
  namespaces instead of flat top-level exports
- `spatial_intent` no longer blanket-reexports `refs::*`
- compile-fail proof now enforces that the old flat top-level `refs` and
  witness-catalog seam is actually gone

## Folder 2: `spatial_intent/resolution`

### Purpose

This folder should resolve witness, frame, and anchor meaning against admitted
spatial context.

### Local Authority That Must Stay

- witness-resolution semantics
- frame-resolution semantics
- numeric admission rules tied to resolution meaning
- catalog-backed interpretation logic
- ambiguity and denial classification that is genuinely spatial

### Suspect Contents

This folder currently appears to mix semantic resolution with:

- `frame_admission`
- `materialization`
- `progression`
- support-style explanation shaping

Those are likely the first major duplicate-lifecycle seams to remove.

### Target Shape

`resolution` should own semantic resolution only. Runtime-facing entry,
progression, inspection, and recovery should project through Query declaration
and inspection surfaces.

### Query Mapping

- configured domain handles
- typed binding pipeline
- declaration entry orchestration
- inspection
- ordinary outcomes
- recovery boundary

### Acceptance

- the same resolution meaning can enter through explicit declaration input or a
  helper projection
- denial and ambiguity posture are inspectable through Query-native surfaces
- local materialization/support glue is deleted, not preserved as long-lived
  compatibility scaffolding
- old progression/materialization/support seams are removed from tests once the
  replacement boundary exists

### Current Status

- complete
- witness materialization and progression compatibility seams are deleted rather
  than hidden
- witness helper entry is isolated as an explicit boundary instead of ambient
  kernel API
- the public facade now exposes witness helper entry only through
  `facade::witness_resolution::*`
- resolved witness truth and witness failure classes no longer survive as flat
  facade exports
- public contracts, compile-fail proof, and internal tests have all been
  rewritten onto the new boundary

## Folder 3: `spatial_intent/lowering`

### Purpose

This folder should lower admitted spatial meaning onto Query declaration
families.

### Local Authority That Must Stay

- anchor interpretation
- movement, placement, and directional semantics
- witness-fed geometric intent
- family-specific lowering rules
- local normalization required before Query declaration

### Suspect Contents

Any artifact shaped like:

- local runtime declaration wrappers
- local target-binding posture
- local support report materialization
- old adapter-era handoff types

should be treated as duplication unless it is the minimal semantic carrier
required before Query declaration entry.

### Target Shape

`lowering` should end at the first honest Query declaration boundary. It should
not continue into a local pseudo-platform that imitates Query progression.

### Query Mapping

- canonical domain declarations
- declaration entry orchestration
- boundary receipts
- boundary envelopes
- typed binding for next-step reuse

### Acceptance

- lowered spatial families flow through real Query declaration entry
- later workflow steps consume retained Query artifacts, not Worth-local handoff
  structs
- aspect-qualified spatial meaning survives into inspection and later binding

### Current Status

- complete
- public lowering entry now returns Query intent declarations directly instead
  of a local proof-artifact or lowered-intent handoff type
- the old top-level runtime-admission, runtime-declaration,
  target-binding-posture, and support-materialization seam is removed from the
  facade and enforced by compile-fail proof
- lowering support-report materialization is deleted rather than preserved as a
  test-only compatibility lane
- lowering now ends at the first honest public Query declaration boundary,
  while the remaining internal lowered-intent carrier is private support for
  local placement application only

## Folder 4: `spatial_intent/arbitration`

### Purpose

This folder should decide among competing spatial candidates and capability
postures.

### Local Authority That Must Stay

- candidate generation
- ranking and policy interaction
- capability-sensitive conflict classification
- escalation and clarification semantics

### Suspect Contents

Current names like:

- `progression`
- `materialization`
- `runtime_declaration`

strongly suggest the folder still mixes semantic arbitration with duplicated
runtime lifecycle.

### Current Status

Complete.

The fake proof ladder, runtime declaration wrapper, and support-materialization
sidecar are deleted. `SpatialIntentArbitrationDeclaration` now carries the one
surviving Query handoff directly, and the public facade no longer exposes a
second arbitration runtime system.

### Target Shape

`arbitration` should keep the semantic decision core and lose the local runtime
ladder. Public stop, readiness, inspection, and recovery posture should ride
Query surfaces.

### Query Mapping

- canonical domain declarations
- inspection
- readiness
- recovery
- helper projections where common arbitration jobs deserve them

### Acceptance

- arbitration enters through Query directly
- blocked, advisory, and clarification posture map onto Query stop and recovery
  seams
- the local semantic core remains visible without exporting a second runtime
  system
- old progression/materialization/runtime-declaration seams are deleted from
  production code and tests

## Folder 5: `spatial_intent/preview`, `constraints`, `continuity`

### Purpose

These folders should model downstream spatial workflow meaning, not fake a
parallel workflow engine.

### Local Authority That Must Stay

- preview-local geometric semantics
- constraint semantics
- continuity and identity-evolution semantics

### What Must Collapse

- workflow summary objects that act like local public runtime products
- local continuation ladders
- local grouped-work stand-ins
- local recovery ecosystems that restate Query-owned stop posture

### Current Status

Complete.

The separate `preview/` and `continuity/` helper pipelines are deleted. Their
surviving semantic contributions now live directly on arbitration declaration
and resolution artifacts, while `constraints/` remains the honest semantic
admission surface for placement-style constraints.

### Target Shape

These folders should become semantic contributors to Query workflow,
continuation, grouped-work, and recovery surfaces.

### Query Mapping

- workflow families
- continuation pipeline
- grouped authoring and grouped products
- signal compatibility where relevant
- recovery boundary

### Acceptance

- preview and continuity are explainable as Query workflows
- next-step classification uses Query inspection/recovery
- grouped and continuation steps reuse retained Query artifacts instead of
  Worth-local pseudo-artifacts
- old preview/continuity helper entrypoints, wrapper objects, and tests are
  deleted rather than hidden behind compatibility seams

## Folder 6: `bindings`

### Purpose

This folder should own primitive-birth semantics and consequence meaning.

### Local Authority That Must Stay

- primitive family semantics
- topology-birth class semantics
- geometric and topology admission
- primitive-birth planning
- primitive-birth validation and rejection meaning

### What Must Collapse

- thick local report ecologies treated as the final public surface
- local consequence packaging that makes Query rediscover primitive-birth truth
- local runtime-lifecycle wrappers around already-admitted primitive meaning

### Target Shape

`bindings` should produce one honest primitive-birth semantic artifact family
and one consequence model that Query can carry forward for runtime-facing
aftermath.

### Query Mapping

- canonical domain declarations
- aftermath review/support/materialization surfaces
- recovery
- helper projections where common primitive-birth flows justify them

### Acceptance

- primitive-birth consequence posture is not scattered across parallel local
  report families
- Query carries the runtime-facing consequence story without archaeology over
  adapter structs

### Current Status

- complete
- primitive birth now exposes one public plan surface plus one consequence
  model instead of separate public authority, completeness-report,
  mapping-report, rejection-row, and contract-count ecologies
- legacy bindings aftermath/report products are removed from the facade and
  enforced dead by boundary and compile-fail proof

## Folder 7: `certification`, `facade`, docs

### Purpose

This phase narrows the public surface and proves the crate boundary is honest.

### Public Surface Goal

`worth-spatial` should export:

- authored spatial vocabulary
- irreducible spatial semantic kernels
- explicit Query-facing helper and integration seams

It should not present itself as the final runtime-facing public operating
surface.

### Certification Goal

Certification must prove:

1. local semantics stay local
2. Query-facing entry is the runtime front door
3. helper ergonomics are projections over canonical Query lanes
4. hidden implementation folders do not leak back into the facade

### Documentation Goal

The README and nearby docs must teach:

- Query-first runtime entry
- what `worth-spatial` owns
- what Query owns
- one or two end-to-end modern examples

They must stop teaching:

- pre-runtime-first framing
- local proof-first framing
- adapter-era runtime declaration wrappers

### Acceptance

- facade exports are intentionally narrow
- compile-fail and public API tests enforce that narrowness
- certification targets real boundary rules, not just file names
- docs match the code and the plan

### Current Status

- complete
- the public facade is now namespaced instead of flat, so semantic entry
  points live under explicit modules rather than one top-level operating
  surface
- compile-fail and boundary proof now enforce that the old flat semantic
  facade is actually dead
- crate docs now teach the same Query-first namespaced surface that the code
  exposes

## Rewrite Rules

When rewriting any folder:

1. preserve domain semantics
2. delete fake lifecycle layers with no independent authority
3. do not keep local materialization or support ecosystems when Query
   inspection/recovery should own the public story
4. add Query runtime capability when there is a real gap
5. document new Query runtime capability in Query docs when added
6. close each folder with hostile QA on the actual boundary
7. remove backward-compatibility shims once the replacement boundary lands
8. rewrite or delete tests that hold old seam shapes in place
9. treat "internal only" or "`#[cfg(test)]` only" legacy seam survival as
   temporary at most, not a valid closeout state

## Done Condition

This refactor is complete only if:

- folder order was followed
- each folder has one coherent authority story
- Query is the runtime-facing front door
- local spatial semantics remain strong
- local duplicate lifecycle glue is systematically removed
- old seam names, compatibility wrappers, and test-only legacy ladders are
  gone
- certification and docs teach the same architecture
