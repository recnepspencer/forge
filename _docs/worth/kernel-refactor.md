# Worth Kernel Refactor

## Goal

Rewrite `worth-kernel` into a crate that owns primitive-construction semantics
cleanly and uses `forge-query` honestly for runtime-facing lifecycle.

This refactor is folder-driven. The work should proceed in dependency order
through the crate rather than by cleaning up one subject family at a time
across unrelated folders.

The intended end state is:

- `worth-kernel` owns irreducible primitive-construction and kernel-local
  semantic meaning
- `forge-query` owns runtime-facing admission, declaration, inspection,
  workflow, receipt, projection-consumption, and recovery lifecycle
- `worth-kernel` no longer exports a second local runtime made of report,
  bundle, replay, and preview products
- each folder has one defensible authority story

## Recommendation

Do not reuse `kernel_reimagined.md` as the implementation plan.

That document is the right thesis, but it is not the right work spec. It is a
kernel philosophy and ambition document, not a folder-ordered cleanup plan.

This crate now needs one fresh plan organized around:

- real folder authority
- current Query surfaces
- zero-backward-compatibility seam deletion
- an implementation order that lets us remove local pseudo-runtime layers
  without losing kernel semantics

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
  - do not repeatedly rediscover admitted meaning or rebuild runtime posture
    locally

The practical consequence is:

- keep kernel-local construction semantics local
- delete local Query-shaped lifecycle duplication
- add Query runtime capability when a real runtime gap exists
- do not keep report or replay scaffolding alive just because the code already
  exists

This plan assumes zero backward compatibility with the pre-refactor seam
shapes.

That means:

- do not preserve old local seam names just to reduce churn
- do not keep compatibility wrappers, aliases, or adapter facades once the new
  boundary exists
- do not preserve legacy replay, preview, report, or bundle seams in tests
  after production no longer owns them
- do not treat test-only survival of a legacy seam as an acceptable closeout
  state

## Query Alignment

The most relevant Query docs for this plan are:

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
- `crates/forge-query/docs/domain-capabilities/contribution-composed-orchestration.md`
- `crates/forge-query/docs/domain-capabilities/orchestration-inventory.md`
- `crates/forge-query/docs/domain-capabilities/recipes/README.md`
- `crates/forge-query/docs/domain-capabilities/continuation-pipeline.md`
- `crates/forge-query/docs/domain-capabilities/signal-compatibility-orchestration.md`
- `crates/forge-query/docs/domain-capabilities/recovery-boundary.md`
- `crates/forge-query/docs/domain-capabilities/aftermath/aftermath-review-support-eligibility-and-materialization.md`
- `crates/forge-query/docs/domain-capabilities/workflow/README.md`
- `crates/forge-query/docs/domain-capabilities/workflow/preview-inspection-and-mutation-planning.md`
- `crates/forge-query/docs/domain-capabilities/workflow/grouped-neighborhood-workflow.md`
- `crates/forge-query/docs/domain-capabilities/workflow/retained-artifact-to-next-step.md`
- `crates/forge-query/docs/domain-capabilities/workflow/stop-to-recovery.md`
- `crates/forge-query/docs/capabilities/existing-truth.md`
- `crates/forge-query/docs/capabilities/inspection.md`
- `crates/forge-query/docs/capabilities/lineage-and-correspondence.md`
- `crates/forge-query/docs/capabilities/projection-consumption.md`

The boundary rule those docs imply is:

- `worth-kernel` owns primitive-construction semantics, family-specific
  construction planning, and kernel-specific interpretation of topology and
  spatial meaning
- `forge-query` owns admitted handle posture, binding, declaration entry,
  runtime progression, retained artifacts, inspection, recovery, grouped work,
  continuation, and workflow routing

If a `worth-kernel` folder owns `progression`, `runtime`, `preview parity`,
`report bundle`, `projection receipt`, or similar runtime-facing lifecycle
seams after public Query surfaces already exist, that folder should be treated
as suspect.

The working Query set by folder should be:

- `spatial_intent/create`
  - configured handles
  - family helpers
- `spatial_intent/relations` and `motion`
  - declaration legality
  - typed binding pipeline
  - ordinary outcomes
- `spatial_intent/lowering`
  - declaration entry orchestration
  - receipts
  - envelopes
- `spatial_intent/arbitration` and `preview`
  - declaration entry readiness
  - inspection
  - workflow preview planning
  - recovery
- `construction/phase_chain`
  - canonical declarations
  - declaration progression
  - retained-artifact-to-next-step
  - contribution-composed orchestration
- `construction/authoring`
  - platform entry
  - configured handles
  - grouped authoring
  - grouped contributions
  - contribution-composed orchestration
  - recipes
- `construction/result_surface`
  - ordinary outcomes
  - declaration entry inspection
  - continuation pipeline
  - contribution-composed orchestration
  - aftermath support/materialization
  - projection consumption
- `construction/runtime_proof` and `proof`
  - inspection
  - existing truth
  - lineage and correspondence
  - continuation pipeline
  - orchestration inventory
  - projection consumption
- `construction/certification`
  - workflow
  - inspection
  - grouped products
  - orchestration inventory
  - recovery
- `facade` and root `certification`
  - platform entry
  - public workflow surfaces
  - inspection
  - orchestration inventory
  - recovery

## Adversarial Constraint

This refactor succeeds only if the same primitive-construction ask preserves
the same semantic meaning and stop posture regardless of entry mode:

- direct authored request
- authored request with local spatial intent shaping
- explicit Query-backed authoring session
- preview or branch-local revisit
- replay or inspection revisit
- retained-artifact aftermath consumption

The crate fails this refactor if it:

- turns kernel-local semantic preparation into a second runtime platform
- reopens raw truth to rediscover aftermath or inspection facts that Query
  should already carry
- exports report, bundle, or replay scaffolding as if those were the true
  public operating surface
- forces consumers to choose between Query-native lifecycle and a parallel
  local kernel lifecycle
- loses primitive-family meaning across receipts, outcomes, retained artifacts,
  or inspection surfaces

## Folder Order

Implementation order:

1. `spatial_intent/create`
2. `spatial_intent/relations` and `spatial_intent/motion`
3. `spatial_intent/lowering`
4. `spatial_intent/arbitration` and `spatial_intent/preview`
5. `construction/phase_chain`
6. `construction/authoring`
7. `construction/result_surface`
8. `construction/runtime_proof` and `construction/proof`
9. residual `construction/certification` corpus closeout
10. `facade`, root certification harness, and docs

This order follows dependency truth.

`spatial_intent` defines the authored primitive-construction grammar and the
first honest Query handoff. `construction/phase_chain` then becomes the first
place where family-specific build semantics should survive without runtime
duplication. Authoring, result, and proof surfaces should only be cleaned up
after the phase chain is honest. Public facade and certification closeout come
last so they can certify the final architecture instead of a moving target.

Certification is not a final afterthought phase.

The actual working rule is:

- each production folder cleanup must pull its nearest certification and
  public-contract proof along with it
- compile-fail, public-api, parity, replay, and closeout evidence should be
  narrowed at the same time the production seam is deleted
- the later certification phases exist only for residual corpus-wide proof,
  remaining bundle/report collapse, and final facade/docs closeout

The practical certification split for this crate should be:

- `spatial_intent/*` batches
  - pull matching root `certification/public_facade_contracts` proof in the
    same batch
- `construction/phase_chain`, `authoring`, and `result_surface`
  - pull the closest `construction/certification` proof and closeout rows in
    the same batch
- `construction/runtime_proof` and `construction/proof`
  - treat these as production-adjacent proof folders, not "later
    certification"
- residual `construction/certification`
  - only the cross-folder or corpus-wide evidence that truly cannot collapse
    earlier
- final `facade` and root certification pass
  - only public-surface and corpus-wide closeout, not first discovery of local
    architectural drift

In practice, that means:

- `spatial_intent/*` phases should rewrite the matching public contract and
  helper/export proof immediately
- `construction/phase_chain`, `authoring`, and `result_surface` should each
  collapse their nearest proof/report vocabulary in the same batch as the
  production seam
- `construction/runtime_proof` and `construction/proof` should be treated as
  production-adjacent proof folders, not deferred certification garnish
- the residual `construction/certification` phase should only contain what
  truly cannot be closed until the earlier folders are already honest

## Per-Folder Questions

Every folder rewrite must answer the same questions:

1. What irreducible kernel authority lives here?
2. What current contents are actually Query lifecycle duplication?
3. What must stay local?
4. What must collapse onto Query?
5. What test or certification evidence closes this folder now, instead of
   being deferred to the end?

Every folder phase should therefore name its immediate proof partner:

- nearest `public_facade_contracts` contract or compile-fail row for public
  seam changes
- nearest `construction/certification` bundle, suite, closeout, or boundary
  row for construction/runtime seam changes
- any remaining proof not pulled with the production batch must be treated as
  suspect until justified explicitly

## Folder 1: `spatial_intent/create`

### Purpose

This folder should be the authored create-grammar floor for primitive
construction.

### Local Authority That Must Stay

- create-intent subject composition
- placement-authoring grammar over primitive construction requests
- kernel-local authored vocabulary for "created here, then placed here"

### What Must Not Live Here

- runtime admission posture
- query-family negotiation
- preview or replay shaping
- receipt or report ecosystems

### Target Shape

`create` should read like a tiny authored grammar layer. It should not present
itself as a public runtime or workflow surface.

### Acceptance

- the folder reads as authored vocabulary, not orchestration
- public exports are semantic authoring helpers, not runtime nouns
- no query-family or report posture leaks into the folder
- the matching public-contract proof changes in the same batch as the
  production seam

### Current Status

- complete
- create-placement now survives as an explicit authoring namespace instead of a
  flat generic intent export
- matching public-contract and compile-fail proof should close the old flat
  seam at the same time as the production boundary

## Folder 2: `spatial_intent/relations` and `spatial_intent/motion`

### Purpose

These folders should express kernel-authored spatial intent families over
primitive construction requests.

### Local Authority That Must Stay

- primitive-construction motion grammar
- relation and constraint grammar over primitive construction asks
- subject-local authored chaining before Query handoff
- local semantic classification of motion versus constraint intent

### Suspect Contents

These folders currently appear to mix authored intent with:

- direct admission calls into `worth-spatial`
- local `admit*` and `finish*` posture on public intent wrappers
- catalog-aware helper branching that may really be a Query-facing lifecycle
  concern rather than authoring meaning

### Target Shape

These folders should own authored intent grammar and the minimum local semantic
admission needed before Query declaration. They should not become a public
parallel lifecycle where callers choose between "local finish" and the real
Query front door.

### Query Mapping

- typed binding pipeline
- declaration legality
- ordinary outcomes
- existing-truth binding where catalog-backed lookup stops being purely local

### Acceptance

- subject-local intent grammar remains expressive
- runtime-facing admission, support, and workflow posture do not stay trapped
  in local intent wrappers
- catalog-aware variants do not quietly become a second target-binding
  subsystem
- public facade and prelude no longer ambiently flatten motion and relation
  intent types as if they were root-level operating surfaces
- certification and compile-fail proof reject direct public `admit*` posture
  and reject the old flat export lanes at the same time

### Current Status

- complete
- public motion and relation grammar survives, but direct public `admit` and
  `admit_with_catalog` helper posture is gone
- lowering still uses the internal admission seam where needed, but the
  public story is now authored grammar plus Query-facing completion rather
  than a parallel local admission shell
- root and prelude flat exports for these intent types are gone; callers now
  cross the explicit authoring namespace instead

## Folder 3: `spatial_intent/lowering`

### Purpose

This folder should lower admitted primitive-construction intent into the first
honest Query declaration boundary.

### Local Authority That Must Stay

- kernel-specific intent-to-placement lowering rules
- local error taxonomy when primitive-construction semantics fail before Query
  truth exists
- family-specific lowering that only `worth-kernel` can define honestly

### Suspect Contents

Anything shaped like:

- local execution helpers that continue past declaration entry
- catalog-aware alternate runtime lanes
- local lifecycle packaging around already-admitted spatial semantics

should be treated as duplication unless it is the minimal semantic carrier
required before Query declaration.

### Target Shape

`lowering` should end at the first honest Query declaration boundary. It should
not continue into a local execution shell that imitates Query progression.

### Query Mapping

- canonical domain declarations
- declaration entry orchestration
- declaration-boundary receipts
- declaration-boundary envelopes

### Acceptance

- lowered primitive-construction intent flows through real Query declaration
  entry
- later workflow steps consume retained Query artifacts instead of local
  handoff wrappers
- local `finish`-style execution helpers disappear once the replacement
  boundary exists
- the matching phase-boundary compile-fail and public contract rows are
  rewritten in the same batch

### Current Status

- complete
- `finish` and `finish_with_catalog` survive only as internal lowering seams
  under `spatial_intent/lowering`
- public lowering now enters through
  `PrimitiveConstructionAuthoringSession::{prepare_result,prepare_outcome,...}`
  instead of direct finish helpers on authored intent wrappers
- the accepted query-entry input set is explicit and sealed; downstream code
  cannot reopen lowering by implementing blanket conversion traits
- broad query/report surfaces that previously accepted arbitrary
  `Into<PrimitiveConstructionIntent>` inputs now either accept sealed authored
  lowering inputs or require a fully lowered `PrimitiveConstructionIntent`

## Folder 4: `spatial_intent/arbitration` and `spatial_intent/preview`

### Purpose

These folders should express primitive-intent conflict and preview meaning, not
build a parallel preview and conflict engine.

### Local Authority That Must Stay

- primitive-specific conflict classification
- clarification meaning and candidate semantics
- primitive-specific preview interpretation
- continuity interpretation that is genuinely kernel-local

### Suspect Contents

Current names and exports suggest these folders may still duplicate:

- runtime preview posture
- readiness and policy resolution
- conflict resolution workflow that Query already knows how to carry for
  runtime-facing operations

### Target Shape

These folders should keep the kernel-specific conflict and preview semantics
but lose any local pseudo-workflow shell. Public preview, continuity, and
resolution posture should ride Query inspection and workflow surfaces.

### Query Mapping

- declaration entry readiness
- inspection
- preview-inspection-and-mutation-planning
- recovery boundary
- stop-to-recovery

### Acceptance

- arbitration and preview enter Query-native runtime posture directly
- clarification and preview meaning remain visible without exporting a second
  workflow subsystem
- no local preview parity or conflict-resolution mini-runtime survives as a
  public contract
- the nearest preview/arbitration report and bundle proof is narrowed at the
  same time as the production seam

### Current Status

- complete
- `preview` no longer survives as an independent semantic folder; its live
  primitive preview assessment now lives under `spatial_intent/arbitration`
  and the old helper-only preview shell is deleted
- free helper functions for conflict analysis, preview construction, continuity
  preview, and explicit resolution no longer survive as the main Folder 4
  operating surface
- the surviving public semantic lane is explicit: conflict and clarification
  live on `PrimitiveIntentConflict`, while preview and continuity meaning live
  on `PrimitiveIntentPreviewAssessment`
- preview and continuity facade buckets now read as construction-report
  surfaces, not sibling semantic engines

## Folder 5: `construction/phase_chain`

### Purpose

This folder should own the primitive-construction phase chain itself: request,
intent, admitted scaffold, family birth input, and topology-ready birth
semantics.

### Local Authority That Must Stay

- primitive family request meaning
- primitive family birth input normalization
- kernel-local geometry-to-topology handoff semantics
- pre-runtime geometric legality and conditioning meaning
- topology-ready birth preparation before runtime-facing mutation entry

### Suspect Contents

This folder is the first major likely sinkhole. It appears to mix real kernel
authority with:

- explicit phase ladders exported as public proof products
- admitted-handoff helper layering
- local runtime-facing post-handoff packaging
- family-specific wiring that may have grown around old runtime gaps

### Target Shape

`phase_chain` should remain the load-bearing kernel core, but only for
pre-runtime semantic preparation. Once it has produced the first honest
topology-ready or Query-ready declaration artifact, later lifecycle should not
be re-owned here.

### Query Mapping

- canonical domain declarations
- declaration progression
- retained-artifact-to-next-step
- contribution-composed orchestration where declaration-bound companion
  semantics would otherwise be restitched locally
- grouped authoring where family declarations are structurally shared

### Acceptance

- the phase chain is still explicit and proof-bearing
- it does not continue into a parallel runtime system after the first honest
  Query handoff
- family-specific birth logic stays local while generic lifecycle leaves the
  folder
- the nearest phase-boundary closeout proof moves with the batch instead of
  becoming a later certification cleanup

### Current Status

- complete
- the old broad admitted post-handoff bag is gone; `phase_chain` now hands
  off one named admitted artifact into later construction surfaces
- the nearest boundary and closeout proof moved with the production change
  instead of waiting for a later certification pass

## Folder 6: `construction/authoring`

### Purpose

This folder should be the explicit kernel authoring front door for primitive
construction.

### Local Authority That Must Stay

- construction-authoring session semantics
- kernel-specific authority-chain meaning
- primitive-construction entry decisions that are genuinely local before Query
  takes over

### Suspect Contents

This folder currently appears to own:

- public query-family admission narration
- local query-gap rows and authority-chain reporting
- result and outcome entry helpers that may be standing in for broader
  Query-native entry and aftermath surfaces

### Target Shape

`authoring` should be the one explicit authoring front door for the crate. It
may still need to talk about Query family support, but it should not become a
parallel runtime manifesto or support-report ecosystem.

### Query Mapping

- platform entry
- configured domain handles
- grouped authoring
- grouped contributions
- contribution-composed orchestration
- recipes

### Acceptance

- there is one explicit authoring front door
- Query is clearly the runtime front door after authoring
- query-gap and authority-chain explanation do not metastasize into the main
  public operating surface
- the nearest authoring/session proof rows collapse in the same batch as the
  production narrowing

### Current Status

- complete
- public authoring now enters through one explicit authoring-entry seam rather
  than a session-level `prepare_*` helper menu
- the old direct session helper lane is gone and compile-fail proof now locks
  that deletion in place

## Folder 7: `construction/result_surface`

### Purpose

This folder should define the one honest primitive-construction outcome and
artifact story.

### Local Authority That Must Stay

- primitive-construction accepted versus rejected semantic outcome meaning
- canonical primitive-construction artifact meaning
- kernel-specific interpretation of topology birth, realization strategy, and
  family aftermath

### What Must Collapse

- parallel happy-path result ecologies
- local report families that try to be the final public aftermath surface
- duplicate recovery or inspection packaging that Query should already own

### Target Shape

`result_surface` should expose one coherent kernel-owned outcome and artifact
boundary. Query should carry the runtime-facing receipt, inspection, and
retained-artifact lifecycle around that boundary.

### Query Mapping

- ordinary outcomes
- declaration entry inspection
- continuation pipeline
- contribution-composed orchestration
- declaration-boundary receipts
- aftermath review/support/eligibility/materialization
- projection consumption

### Acceptance

- callers do not have to choose among multiple result, outcome, bundle, and
  report families to understand primitive construction
- kernel-specific artifact meaning stays local
- runtime-facing lifecycle around that artifact is Query-native
- report/evidence certification that used to justify the old result ecology is
  narrowed in the same batch

### Current Status

- complete
- `PreparedPrimitiveConstructionResult` is now the one public prepared-result
  lane
- the public canonical artifact export is gone
- direct prepared-result truth replaced the old nested public
  artifact/evidence split
- matching compile-fail proof moved in the same batch as the production
  collapse

## Folder 8: `construction/runtime_proof` and `construction/proof`

### Purpose

These folders should prove the kernel against runtime-facing truth. They should
not become a second public runtime API.

### Local Authority That Must Stay

- kernel-specific replay and parity interpretation
- kernel-specific proof substrate and digest protocol meaning
- kernel-specific hostile gap registration when a real Query runtime gap exists

### Suspect Contents

These folders currently appear to own a large amount of:

- branch-preview runtime reports
- replay parity reports
- query inspection parity reports
- projection-consumption receipt reports
- no-local-runtime-workaround audits

Some of that is valid proof. Some of it is very likely public pseudo-runtime
surface area that survived because the crate needed evidence before the runtime
story was fully honest.

### Target Shape

`runtime_proof` and `proof` should remain certification-grade and hostile, but
they should stop teaching a parallel product surface. They must prove the real
Query-native lifecycle rather than replacing it with kernel-local report
objects.

### Query Mapping

- inspection
- projection consumption
- existing truth
- lineage and correspondence
- continuation pipeline
- orchestration inventory
- retained-artifact-to-next-step
- workflow preview / continuation where parity is being certified

### Acceptance

- proof surfaces certify the real runtime path
- no query-parity helper or report object becomes the preferred public way to
  consume runtime meaning
- real runtime gaps are documented as Query work, not normalized into permanent
  kernel-local APIs
- certification changes here are part of the production batch, not deferred
  because the folder has "proof" in its name

### Current Status

- major active concern
- this is likely the largest remaining source of API inflation in the crate

## Folder 9: residual `construction/certification` corpus closeout

### Purpose

This phase exists for the certification residue that truly depends on earlier
folder completion: kernel-wide corpus proof, cross-folder hostile evidence,
and final report/bundle collapse that cannot honestly happen sooner.

### Local Authority That Must Stay

- primitive corpus proof
- kernel-specific hostility suites
- milestone and family boundary interpretation
- kernel-specific closeout evidence

### What Must Collapse

- public-facing report ecologies that duplicate runtime receipts or inspection
- broad bundle trees whose main job is to compensate for an overgrown facade
- proof packaging that survives as product surface instead of remaining
  certification evidence

### Target Shape

`construction/certification` should stay broad internally because it is a proof
subsystem, but most of its local boundary cleanup should have already happened
during the earlier folder phases. By the time work reaches this phase, what
remains should be corpus-wide or genuinely cross-folder proof, not the first
time we question one local production seam.

### Query Mapping

- inspection
- workflow
- grouped products
- orchestration inventory
- recovery
- projection consumption

### Acceptance

- certification remains adversarial and machine-checkable
- public evidence surfaces are narrow and intentional
- proof artifacts no longer masquerade as the public operating API of the
  crate
- no folder-local public report cleanup of consequence is postponed here just
  because certification was treated as a separate campaign
- by the time work reaches this phase, most consequential proof narrowing
  should already have happened in the earlier folder batches

### Current Status

- materially advanced, not closed
- the first large public certification bundle collapse is done:
  `facade::certification::{motion,policy}` no longer survive as public
  operating buckets, and the surviving `preview`, `continuity`, and
  `arbitration` certification lanes now expose hostility-suite truth instead
  of public bundle constructors
- public contract and compile-fail proof now certify that direct diagnostics
  and query-proof lanes are the public surface instead of
  preview/continuity/policy/motion/arbitration certification bundles
- the milestone-four closeout path no longer treats preview, continuity, and
  policy-profile bundle artifacts as canonical internal proof currency; those
  bundles are deleted and replaced by direct representative evidence built from
  surface rows, hostility suites, replay parity, query inspection/projection,
  and branch-runtime truth
- the remaining work is now narrower and deeper in the certification corpus:
  motion, arbitration, and policy-pressure verified bundle internals still
  survive under `construction/certification`, and Folder 9 is not honest to
  mark complete until those residual proof packages are either consumed by
  corpus-only closeout or deleted

## Folder 10: `facade`, root certification harness, and docs

### Purpose

This phase narrows the public surface and proves the crate boundary is honest.

### Public Surface Goal

`worth-kernel` should export:

- primitive-construction authored vocabulary
- irreducible primitive-construction semantic kernels
- explicit Query-facing authoring, outcome, and inspection seams

It should not present itself as a report warehouse or a second runtime-facing
operating surface.

### Certification Goal

Certification must prove:

1. kernel-local semantics stay local
2. Query-facing entry is the runtime front door
3. helper ergonomics are projections over canonical Query lanes
4. hidden implementation folders do not leak back into the facade
5. report and bundle ecosystems do not masquerade as the main public API

The root certification harness in this phase should mostly be composing and
auditing the folder-by-folder proof that was already narrowed earlier. It
should not be the first place where we discover a major local runtime seam or
public report ecology.

### Documentation Goal

The README and nearby docs must teach:

- Query-first runtime entry
- what `worth-kernel` owns
- what Query owns
- how primitive-construction authoring, outcome, and inspection actually flow

They must stop teaching:

- flat root happy-path exports
- queryless helper culture
- local proof/report bundles as the main product surface

### Acceptance

- facade exports are intentionally narrow
- compile-fail and public API tests enforce that narrowness
- certification targets real boundary rules, not just file names
- docs match the code and the plan

### Current Status

- not yet rewritten in this style
- current facade shape already suggests a subject-first export history rather
  than a folder-first authority story

## Rewrite Rules

When rewriting any folder:

1. preserve kernel-local semantics
2. delete fake lifecycle layers with no independent authority
3. do not keep local report or support ecosystems when Query inspection,
   receipt, or recovery should own the public story
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
- local kernel semantics remain strong
- local duplicate lifecycle glue is systematically removed
- old seam names, compatibility wrappers, and test-only legacy ladders are
  gone
- certification and docs teach the same architecture
