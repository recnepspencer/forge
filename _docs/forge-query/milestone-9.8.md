# Milestone 9.8 Engineering Spec: Downstream Consumer Product Kit For Evidence Reports, Boundary Audits, And Support Pinning

> **Status:** Draft
>
> **Roadmap parent:** [forge_query_roadmap.md](./forge_query_roadmap.md)
>
> **Primary predecessors:** [milestone-9.7.md](./milestone-9.7.md), [milestone-9.6.md](./milestone-9.6.md), [milestone-9.5.md](./milestone-9.5.md)
>
> **Purpose:** ship the runtime-owned kit that eliminates consumer-side
> folklore around Query's product contracts — declarative evidence-report
> scaffolding, a shipped boundary-bypass audit, exportable, pinnable support
> snapshots, and a shipped in-memory consumer test backend — and prove
> closure by reference-consumer adoption rather than by API presence.

## Goal

Make the cost of consuming Query correctly lower than the cost of consuming
it wrong. Today every downstream evidence report costs hundreds of lines of
hand-rolled struct, getter, error, and digest plumbing; the no-bypass rule is
enforced by consumer-written source greps; and support posture is re-derived
per consumer as hand-built gap rows. Each is a runtime-owned contract
materialized by consumer folklore. This milestone ships the kit that owns
them, and closes only when the reference consumer (`worth-kernel`) has
deleted its hand-rolled equivalents.

## Why This Milestone Exists

The first serious consumer pays a linear ceremony tax that masks how steep
the adoption curve is for every consumer after it:

- each `worth-kernel` evidence report repeats the same ~250-line pattern —
  report struct, getter wall, three-variant error enum with identical
  `Display` impls, digest plumbing — across `runtime_basis`,
  `graph_composition_parity`, `projection_consumption_receipt`, and siblings
- the Query hard prohibitions are enforced downstream by a consumer-owned
  audit that `include_str!`s 27 source files and greps for forbidden
  patterns like `.write(` — charming, unshippable, and reinvented (or
  skipped) by every future consumer
- support posture reaches consumers as hand-assembled gap rows with no
  serialized, versioned, diffable snapshot a downstream CI can pin, so a
  posture regression surfaces at runtime admission instead of at build time

Milestones `9.6` and `9.7` harden what Query *says*; this milestone hardens
what Query *gives consumers to build with*. Per `MENTALITY.md`, foundations
are slow and features are fast — this kit is what makes downstream features
fast.

## Governing Summaries

- `MENTALITY.md`: enforce mechanically, not by convention — a prohibition
  list enforced by consumer greps is category-3 enforcement owned by the
  wrong party; the kit moves enforcement to the runtime that owns the rule.
- `arch_laws.md`: Law 6 (domain code returns what changed; the framework
  derives the ceremony), Law 26 (explicit equivalence contracts), Law 34
  (the framework owns resource lifecycle — including the lifecycle of its
  own contract enforcement), Law 41 (sealed proof-carrying types).
- `composition_laws.md`: the kit must produce named, predictable consumer
  files — a derive that hides meaning would trade boilerplate for fog; the
  kit names responsibilities, it does not bury them.
- `domain_structure_laws.md`: shared code must earn its shared location by
  shared authority — these three surfaces qualify precisely because every
  consumer depends on the same contracts for the same semantic reasons.
- `perf_laws.md`: structural waste dominates constant waste; the per-consumer
  ceremony tax is structural waste at the platform boundary, and the kit
  amortizes it across the largest semantically honest boundary — the runtime
  itself.
- `forge_query_roadmap.md`: the platform framework stance says ordinary
  developers stay inside `forge-query` for the majority of their work; that
  claim is dishonest while correct consumption requires folklore.

## Adversarial Constraint

A downstream domain crate must be able to author a digest-bearing evidence
report, enforce the no-bypass contract, pin its support-posture
dependencies, and obtain a valid honestly-postured test runtime using only
Query-shipped kit surfaces — and every divergence class must fail
mechanically in the consumer's build: a report field that escapes digest
participation, a bypass of a prohibited runtime seam, a support-posture
regression against a pinned row, a test backend faking a lane it cannot
honor, or a resurrection of hand-rolled scaffolding in covered consumer
surfaces.

This milestone fails if any covered path:

- ships scaffolding that produces digests outside the Milestone `9.6`
  canonical evidence-identity scheme
- enforces the bypass rules by string search where visibility or type
  boundaries can enforce them structurally
- exports a support snapshot that can disagree with the live support matrix
  for the same runtime version
- claims closure while the reference consumer still carries hand-rolled
  report plumbing, its grep audit, or hand-built gap rows in covered
  surfaces

## Product Decision Lock

- The kit is Query product surface, not test convenience; it ships with
  support rows, docs, and certification like every other family.
- Report scaffolding is declarative per arch law 6: consumers declare fields
  and semantics; the kit derives digest participation, accessors, and
  error/display plumbing.
- All kit-produced identity lowers through the `9.6` canonical
  evidence-identity primitive; the kit cannot mint a second digest scheme.
- Bypass enforcement prefers visibility and sealed types; the shipped audit
  artifact covers only what structure cannot reach, and it is runtime-owned.
- Support snapshots are serialized, versioned, schema-stable projections of
  the same support matrix the runtime answers from — one truth, derived
  views, per arch law 33.
- Closure is adoption-proven: the reference consumer deletes its folklore in
  the same change program, per the `MENTALITY.md` scope-expansion rule.

## Phase Plan

### Phase 1: Evidence Report Composition Kit Boundary

Ship the declarative evidence-report scaffolding: a consumer declares a
report's fields, semantic scope, and digest participation once; the kit
derives canonical digest construction, accessors, and error/display plumbing
over the `9.6` primitive.

**Relevant subsystems**
- consumer kit (new boundary home inside `forge-query`)
- canonical evidence identity (from Milestone `9.6`)

**Relevant Query source surfaces**
- the Milestone `9.6` evidence-identity primitive surface
- [runtime/support_matrix.rs](../../crates/forge-query/src/runtime/support_matrix.rs) as the
  house example of the report shape the kit must be able to express

**Relevant downstream evidence (reference consumer)**
- [worth-kernel runtime_basis.rs](../../crates/worth-kernel/src/construction/runtime_proof/runtime_basis.rs)
- [worth-kernel graph_composition_parity.rs](../../crates/worth-kernel/src/construction/runtime_proof/query/graph_composition_parity.rs)

**Target shape (illustrative, not frozen API)**

The reference consumer's reports today are ~250–350 lines each: a report
struct, a getter wall, hand-rolled digest plumbing, and a three-variant error
enum with a hand-written `Display` — repeated nearly verbatim per report
file. The target shape collapses each to its semantic core:

```rust
// AFTER: declare fields and semantics once; digest participation, accessors,
// sealed construction, and error/display plumbing are derived — and the
// digest is canonical-scheme (9.6) by construction
#[derive(ForgeQueryEvidenceReport)]
#[evidence(scope = "worth-kernel.basis-lane")]
pub struct PrimitiveConstructionRuntimeBasisLaneReport {
    label: ForgeQuerySessionLabel,
    effect_policy: ForgeQueryEffectPolicy,
    authority_lane: ForgeQueryAuthorityLane,
    #[evidence(sequence)]
    evidence: Vec<String>,
}
// a field missing its digest-participation posture does not silently skip
// the digest — it fails to compile
```

**Warnings**
- Do not generate fog: derived accessors and digest participation must remain
  predictable from the declaration site per composition law 13; a macro that
  hides what enters the digest is worse than the boilerplate it replaces.
- Do not let the kit accept pre-rendered strings as digest contributions;
  field contributions are typed, tagged values per the `9.6` contract.
- Do not couple the kit to `worth-kernel` shapes; it must express Query's own
  report families first.

**Test requirements**
- Add an `Evidence Report Kit Parity Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: re-express a representative existing report
  (one Query-owned, one `worth-kernel`-shaped) through the kit and prove the
  kit-built report carries identical semantic content and canonical-scheme
  digests for identical inputs.
- Adversarial rejection: prove a field that is declared but excluded from
  digest participation, or mutated after construction, cannot silently alter
  the report digest — the misuse fails at compile time or constructs a typed
  rejection, never a divergent digest.

**Engineering decisions**
- The kit's declaration surface is data-shaped (declaration structs or a
  derive over them), not closure-shaped, so reports stay inspectable.
- Kit-built reports are sealed after construction per arch law 41.

**Open questions**
- None.

### Phase 2: Prohibition Registry And Seam Visibility Boundary

Freeze the single source of truth for the hard prohibitions: one runtime-
owned prohibition registry, with every coverable seam sealed by visibility or
typed admission so the strongest enforcement tier carries as much of the rule
set as structure allows.

**Relevant subsystems**
- prohibition registry (new boundary home inside `forge-query`)
- runtime seam visibility

**Relevant Query source surfaces**
- [runtime/workspace.rs](../../crates/forge-query/src/runtime/workspace.rs) (covered seams:
  `write`, `batch`, existing-truth binding and probe surfaces named by the
  Milestone `9.5` Phase 4 seam-retirement list)

**Relevant downstream evidence (reference consumer)**
- [worth-kernel no_local_runtime_workaround_audit.rs](../../crates/worth-kernel/src/construction/runtime_proof/query/no_local_runtime_workaround_audit.rs)

**Warnings**
- Do not solve this entirely with the Phase 3 audit; every seam that can be
  sealed by visibility or typed admission must be, and the audit covers only
  the remainder per the `MENTALITY.md` enforcement hierarchy.
- Do not let the registry and the hard-prohibitions documentation drift; the
  registry is the single source of truth both derive from.

**Test requirements**
- Add a `Prohibition Registry And Seam Visibility Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial localization: a compile-fail contract proving a consumer crate
  cannot reach a sealed covered seam at all — the strongest enforcement tier
  doing its job before any audit exists.
- Adversarial agreement: prove the hard-prohibitions documentation and the
  registry name exactly the same seam set, so adding a prohibition in prose
  without a registry entry fails a Query-side completeness assertion.

**Engineering decisions**
- The prohibition registry is the single source of truth; docs, visibility
  decisions, and the Phase 3 audit all derive from it.
- Seam sealing is preferred over audit coverage wherever the type or
  visibility system can express the rule.

**Open questions**
- None.

### Phase 3: Shipped Bypass Audit Artifact Boundary

Ship the audit artifact for the residue structure cannot reach: derived from
the Phase 2 registry, structurally resolving (path-resolved usage, not text),
and consumable from any downstream crate's test suite as one call.

**Relevant subsystems**
- shipped audit artifact (new boundary home inside the consumer kit)
- prohibition registry (consumed from Phase 2)

**Relevant Query source surfaces**
- the Phase 2 prohibition registry surface

**Relevant downstream evidence (reference consumer)**
- [worth-kernel no_local_runtime_workaround_audit.rs](../../crates/worth-kernel/src/construction/runtime_proof/query/no_local_runtime_workaround_audit.rs)

**Target shape (illustrative, not frozen API)**

The consumer-owned enforcement this phase replaces, as it exists today in
`worth-kernel`:

```rust
// BEFORE: 27 include_str!'d source files string-grepped for forbidden
// patterns — trips on comments, misses aliased calls, reinvented per consumer
let violation_count = AUDITED_FILES
    .iter()
    .flat_map(|(_, source)| {
        FORBIDDEN_RUNTIME_PATTERNS
            .iter()
            .map(|pattern| source.contains(pattern))
    })
    .filter(|found| *found)
    .count();
```

The target shape after this phase:

```rust
// AFTER: one runtime-shipped, structurally-resolving audit; the consumer's
// entire enforcement file becomes one test
#[test]
fn worth_kernel_stays_on_the_sanctioned_query_path() {
    forge_query::kit::boundary_audit()
        .covering_crate("worth-kernel")
        .assert_clean();
    // typed findings name the seam and call site; comments, doc examples,
    // and string literals cannot trip it
}
```

**Warnings**
- Do not ship an audit that trips on comments, doc examples, or string
  literals; classification must be structural (path-resolved usage), not
  textual.
- Do not let the audit grow its own pattern set beside the registry; coverage
  completeness is derived, never hand-curated.

**Test requirements**
- Add a `Shipped Bypass Audit Honesty Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial detection: seed a covered bypass (a prohibited seam usage in a
  fixture consumer crate) and prove the shipped audit fails the build with a
  typed finding naming the seam and site.
- Adversarial false-positive honesty: prove a comment mention, doc example,
  and string literal containing the forbidden pattern do not trip the audit.
- Adversarial drift localization: prove that adding a seam to the Phase 2
  prohibition registry without audit coverage fails a Query-side
  completeness test.

**Engineering decisions**
- The audit artifact is versioned with the runtime so consumers cannot pin a
  stale rule set silently.
- Audit findings are typed artifacts naming seam and call site, suitable for
  the same evidence handling as every other kit output.

**Open questions**
- None.

### Phase 4: Support Snapshot Projection Boundary

Ship the serialized, versioned, schema-stable support snapshot: a derived,
digest-bound projection of the live support matrix that downstream tooling
and CI can store, diff, and reason about outside a running workspace.

**Relevant subsystems**
- support matrix projection (new boundary home inside the consumer kit)

**Relevant Query source surfaces**
- [runtime/support_matrix.rs](../../crates/forge-query/src/runtime/support_matrix.rs)
- [runtime/support/profile.rs](../../crates/forge-query/src/runtime/support/profile.rs)
- [application/support/report.rs](../../crates/forge-query/src/application/support/report.rs)

**Warnings**
- Do not create a second support truth; the snapshot is a derived projection
  of the same matrix the runtime answers admission from, digest-bound to it.
- Do not freeze the snapshot schema informally; schema identity is versioned
  and participates in the snapshot digest.

**Test requirements**
- Add a `Support Snapshot Projection Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: prove snapshot content equals live matrix content
  for the same runtime version — row for row, posture for posture, digest
  for digest — and that re-export is deterministic.
- Adversarial denial: prove comparing or loading a snapshot across a schema
  version boundary fails typed rather than silently coercing rows.

**Engineering decisions**
- Snapshot identity lowers through the `9.6` evidence primitive.
- The snapshot is a projection per arch law 33: destroyable and rebuildable
  from the matrix alone.

**Open questions**
- None.

### Phase 5: Consumer Pinning Contract Boundary

Ship the typed consumer pinning contract over the Phase 4 snapshot: a
downstream crate declares the rows and postures it depends on, and its own
build fails with a localized typed finding when posture regresses.

**Relevant subsystems**
- consumer pinning contract (new boundary home inside the consumer kit)
- support snapshot projection (consumed from Phase 4)

**Relevant Query source surfaces**
- the Phase 4 snapshot projection surface

**Relevant downstream evidence (reference consumer)**
- [worth-kernel authoring.rs](../../crates/worth-kernel/src/construction/authoring.rs)
  (hand-built required-family admission and gap-row assembly)

**Target shape (illustrative, not frozen API)**

The hand-rolled posture bookkeeping this phase replaces, as it exists today
in `worth-kernel` (`REQUIRED_QUERY_FAMILIES` / `REPORTED_QUERY_FAMILIES`
constants plus ~40 lines of per-session admission-and-filter logic building
`PrimitiveConstructionQueryGapRow`s by hand):

```rust
// BEFORE
const REQUIRED_QUERY_FAMILIES: [ForgeQueryRuntimeFacadeFamily; 2] = [
    ForgeQueryRuntimeFacadeFamily::Write,
    ForgeQueryRuntimeFacadeFamily::Inspect,
];
// ...followed by hand-built gap-row assembly over the public api contract
```

The target shape after this phase:

```rust
// AFTER: a typed pin artifact; a posture regression in forge-query fails the
// consumer's build with a finding naming the row — not a runtime admission
// surprise
forge_query::kit::support_pins! {
    workspace = "worth-kernel",
    require = [Write: Supported, Inspect: Supported],
    observe = [BranchPreview], // surfaces as gap rows automatically when unsupported
}
```

**Warnings**
- Do not make pinning advisory; a pinned row whose posture regresses must
  fail the consumer build, not log a warning.
- Do not let pins reference rows by free-form strings; pin declarations bind
  to typed row identity from the snapshot schema.

**Test requirements**
- Add a `Support Snapshot Pinning Drift Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial drift localization: regress one covered row's posture in a
  fixture runtime and prove exactly the consumers pinned to that row fail,
  with a typed finding naming the row, the pinned posture, and the actual
  posture — and no unpinned consumer fails.
- Adversarial rejection: prove a pin against a nonexistent row, or a pin
  whose declared posture vocabulary does not match the snapshot schema
  version, fails typed at pin evaluation rather than passing vacuously.

**Engineering decisions**
- Pin declarations are typed artifacts in the consumer crate, not strings in
  CI configuration.
- Pin identity lowers through the `9.6` evidence primitive.

**Open questions**
- None.

### Phase 6: Shipped In-Memory Consumer Test Backend Boundary

Ship the in-memory test runtime as kit surface: one public, honestly-postured
backend a downstream crate can build a valid `ForgeQueryWorkspace` from for
reads, writes, previews, and invariant registration — without implementing
runtime-bridge adapter traits or fabricating receipts by hand.

**Relevant subsystems**
- consumer test backend (new boundary home inside the consumer kit)
- the internal `memory_workspace` machinery (currently private)
- runtime builder and backend parts intake
- the Milestone `9.5` raw runtime read bootstrap (the read-lane neighbor this
  phase generalizes for consumers)

**Relevant Query source surfaces**
- [memory_workspace/mod.rs](../../crates/forge-query/src/memory_workspace/mod.rs)
  (private today; the implementation seed for the shipped backend)
- [runtime/builder.rs](../../crates/forge-query/src/runtime/builder.rs)
- [runtime/backend/parts.rs](../../crates/forge-query/src/runtime/backend/parts.rs)

**Relevant downstream evidence (external consumers)**
- `workflow-editor` (`workflow_query_forge/src/workspace_reads/tests_support.rs`
  and its certification harness) hand-implements seven-plus
  `ForgeQueryRuntime*Adapter` traits plus a `RuntimeBridge` assembly in
  dev-dependencies just to test workspace reads
- `hadwiger-research` test suites implement
  `ForgeQueryRuntimeWriteAuthorityAdapter` and fabricate
  `WriteAuthorityExecutionReceipt` / `ForgeQueryMutationReceipt` values by
  hand to exercise invariant denial materialization

**Target shape (illustrative, not frozen API)**

```rust
// BEFORE (workflow-editor today): seven hand-written adapter impls plus a
// RuntimeBridge assembly in dev-deps, per consumer, just to test reads

// AFTER: one kit entry returning a valid, honestly-postured workspace
let mut workspace = forge_query::kit::in_memory_test_runtime()
    .with_schema(test_schema)
    .workspace("workflow-editor.tests")?;
// reads, writes, previews, and invariant registration are real lanes;
// anything the backend cannot honor fails closed through the ordinary
// support matrix, never by silently faking receipts
```

**Warnings**
- Do not ship the backend with optimistic support rows; everything it cannot
  honor must fail closed through the same admission discipline as production
  backends, or consumers will certify against fiction.
- Do not fork a second workspace semantics for testing; the backend is a
  backend, and the workspace above it is the ordinary workspace.
- Do not leave receipt construction public-by-necessity; if a consumer test
  still needs to fabricate a receipt by hand, the backend is incomplete.

**Test requirements**
- Add a `Shipped Test Backend Honesty Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: a representative covered workload (reads, writes,
  preview admission, invariant registration and denial) produces the same
  canonical artifacts, receipts, and support postures through the shipped
  backend as through the equivalent hand-assembled bridge-backed harness it
  replaces.
- Adversarial denial: every family the backend cannot honor fails closed with
  the same typed admission stops as a production backend — proven by
  exhaustively walking the support matrix rather than spot-checking.
- Adversarial residue: an exact assertion that the covered consumer test
  suites contain zero hand-implemented runtime adapter traits and zero
  hand-fabricated receipt values after adoption.

**Engineering decisions**
- The private `memory_workspace` machinery is promoted through the kit rather
  than duplicated; one in-memory truth implementation, one public posture.
- The backend ships with honest support rows from birth — it is a first-class
  backend posture, not a mock.

**Open questions**
- None.

### Phase 7: Reference Consumer Evidence Report Adoption Boundary

Prove the report kit by adoption: migrate `worth-kernel`'s covered evidence
reports onto the Phase 1 kit and the canonical digest scheme, deleting the
hand-rolled report and digest plumbing in the same change program.

**Relevant subsystems**
- `worth-kernel` construction runtime-proof report surfaces
- evidence report composition kit (from Phase 1)

**Relevant downstream surfaces**
- [worth-kernel runtime_basis.rs](../../crates/worth-kernel/src/construction/runtime_proof/runtime_basis.rs)
- [worth-kernel graph_composition_parity.rs](../../crates/worth-kernel/src/construction/runtime_proof/query/graph_composition_parity.rs)
- [worth-kernel projection_consumption_receipt.rs](../../crates/worth-kernel/src/construction/runtime_proof/query/projection_consumption_receipt.rs)

**Warnings**
- Do not preserve the old report scaffolding beside the kit "for safety";
  surviving folklore is the failure mode this milestone exists to kill.
- Do not let adoption weaken `worth-kernel`'s evidence semantics; every
  migrated report keeps its semantic fields and its assertions, re-expressed
  rather than reduced.

**Test requirements**
- Add a `Reference Consumer Report Adoption Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: migrated `worth-kernel` reports preserve their
  semantic assertions and produce canonical-scheme digests for identical
  inputs.
- Adversarial residue: an exact structural assertion that covered
  `worth-kernel` report surfaces contain zero remaining hand-rolled digest
  construction and zero hand-written report/getter/error plumbing the kit
  now derives.

**Engineering decisions**
- `worth-kernel` is the named reference consumer; its deletion diff is part
  of this milestone's acceptance evidence.
- Kit gaps discovered during report adoption are fixed in Phase 1, not worked
  around consumer-side — adoption is the kit's hostile review.

**Open questions**
- None.

### Phase 8: Reference Consumer Audit, Pinning, And Test-Backend Adoption Boundary

Prove the enforcement and posture surfaces by adoption: replace
`worth-kernel`'s grep audit with the shipped audit artifact, its hand-built
required-family and gap-row bookkeeping with support pins, and any
hand-assembled test harness surfaces with the shipped backend where it is the
honest fit.

**Relevant subsystems**
- `worth-kernel` audit and authoring authority surfaces
- shipped audit artifact (Phase 3), pinning contract (Phase 5), test backend
  (Phase 6)

**Relevant downstream surfaces**
- [worth-kernel no_local_runtime_workaround_audit.rs](../../crates/worth-kernel/src/construction/runtime_proof/query/no_local_runtime_workaround_audit.rs)
- [worth-kernel authoring.rs](../../crates/worth-kernel/src/construction/authoring.rs)

**Warnings**
- Do not treat this phase as optional polish; per `MENTALITY.md`, cross-crate
  scope expansion to prove the foundation is the norm, and a kit no consumer
  has adopted is plausibility, not proof.
- Do not force the shipped backend where `worth-kernel`'s topology-backed
  runtime entry is the honest production-shaped harness; backend adoption
  covers the surfaces where hand assembly was the only reason for the
  current shape.

**Test requirements**
- Add a `Reference Consumer Enforcement Adoption Test` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Adversarial equivalence: the shipped audit detects the same seeded
  violation classes the grep audit detected, through the shipped artifact,
  from `worth-kernel`'s own test suite.
- Adversarial residue: an exact structural assertion that covered
  `worth-kernel` surfaces contain zero `include_str!` audit machinery and
  zero hand-built required-family or gap-row assembly where pinning now
  answers.

**Engineering decisions**
- Adoption order follows kit dependency order: audit, then pins, then
  backend.
- Kit gaps discovered during enforcement adoption are fixed in Phases 2–6,
  not worked around consumer-side.

**Open questions**
- None.

### Phase 9: Support, Docs, And Hostile Certification Closure Boundary

Close the milestone with support/profile honesty for the kit families,
documentation follow-through, and one hostile certification program across
every kit surface plus the adoption evidence.

**Relevant subsystems**
- `application` support/profile reporting
- public documentation coverage
- milestone certification

**Relevant Query source surfaces**
- [application/support/report.rs](../../crates/forge-query/src/application/support/report.rs)
- [application/tests.rs](../../crates/forge-query/src/application/tests.rs)
- [public_doc_coverage/tests/support.rs](../../crates/forge-query/src/public_doc_coverage/tests/support.rs)

**Documentation follow-through**
- The kit surfaces enter the public docs as the ordinary consumer path, and
  every doc passage that still teaches hand-rolled reports, consumer greps,
  or hand-built gap rows is removed in this phase.

**Warnings**
- Do not close on kit API presence; closure is the certification matrix plus
  the adoption residue assertions passing together.
- Do not let docs teach the kit as optional ergonomics; it is the ordinary
  path, and the folklore patterns it replaces are named anti-patterns.

**Test requirements**
- Add a `Milestone 9.8 Consumer Kit Hostile Certification Matrix` to
  [test-requirements.md](./test-requirements.md) and close it in this phase.
- Combined adversarial matrix: drive report misuse, seeded bypasses, posture
  regressions, and folklore-resurrection probes in one program and require
  narrow canonical artifacts for the kit-report digest scheme, the audit
  finding set, the snapshot/pin agreement, and the adoption residue counts.
- Adversarial agreement: prove docs, support/profile rows, and certification
  output agree exactly on the kit families' posture.

**Engineering decisions**
- Support/profile output is authoritative for kit family closure.
- The certification matrix runs against the Milestone `9.5` raw runtime
  bootstrap so kit certification never grows private harness assembly.

**Open questions**
- None.

## Must Ship

- the declarative evidence-report composition kit over the `9.6` canonical
  evidence-identity primitive
- runtime-owned bypass enforcement: sealed/visibility-tightened seams plus
  one shipped audit artifact derived from a single prohibition registry
- the serialized, versioned support snapshot and the typed consumer pinning
  contract with build-failing drift detection
- the shipped in-memory consumer test backend with honest fail-closed support
  posture, replacing hand-implemented adapter assemblies and hand-fabricated
  receipts in consumer test suites
- reference-consumer adoption with deletion of `worth-kernel`'s hand-rolled
  report plumbing, grep audit, and gap-row assembly in covered surfaces
- support/profile, docs, and hostile certification closure for the kit
  families

## Must Preserve

- the `9.6` canonical evidence-identity scheme as the only digest authority
  the kit can express
- the hard prohibitions' meaning — relocated into enforceable structure, not
  reworded
- one support truth: the snapshot remains a digest-bound derived projection
  of the live matrix
- `worth-kernel`'s evidence semantics through migration — re-expressed, never
  reduced
- the Query facade as the only consumer surface; the kit adds no second
  entry path into runtime internals

## Acceptance Evidence

This milestone is complete only when `forge-query` can prove:

- the Milestone `9.8` certification suites added to
  [test-requirements.md](./test-requirements.md) pass with narrow
  machine-checkable artifacts
- a kit-authored report reproduces a hand-rolled report's semantics with
  canonical-scheme digests, and misuse fails typed or fails to compile
- the shipped audit detects seeded bypasses structurally, with zero textual
  false positives, from a downstream crate's own test suite
- a pinned posture regression fails exactly the pinned consumers' builds with
  typed findings
- a downstream-shaped test suite obtains a valid workspace from the shipped
  in-memory backend and exercises reads, writes, previews, and invariant
  registration with zero hand-implemented adapter traits and zero
  hand-fabricated receipts, and every unbacked family fails closed
- covered `worth-kernel` surfaces carry zero remaining hand-rolled digest,
  audit, or gap-row folklore, with the deletion diff recorded as part of the
  milestone evidence

## Sequencing Notes

- This milestone belongs after [milestone-9.6.md](./milestone-9.6.md), which
  supplies the evidence-identity primitive the kit is built on, and after
  [milestone-9.7.md](./milestone-9.7.md), so the kit's report, audit, and
  pinning surfaces cover the concurrency-era facade families rather than
  shipping against a surface about to be re-expressed.
- It sits before Milestone `10` in the critical path so the kit
  pressure-tests the frozen runtime-backed product surface through real
  consumer adoption before store-backed work builds on it; where staffing
  allows, its phases may overlap early Milestone `10` work, since store
  execution does not consume kit surfaces.
- Durable artifacts are out of scope: persisted snapshots, durable audit
  archives, and store-backed kit artifacts remain Milestone `10`/`11` scope.
