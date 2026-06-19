# Forge Query Test Requirements

## Scope

This document defines the certification-grade query test requirements for:

- Milestone 1
- Milestone 2
- Milestone 3
- Milestone 4
- Milestone 5
- Milestone 5.1
- Milestone 5.2
- Milestone 5.3
- Milestone 5.4
- Milestone 5.5
- Milestone 5.6
- Milestone 6
- Milestone 7
- Milestone 8
- Milestone 9
- Milestone 9.1
- Milestone 9.2
- Milestone 9.3
- Milestone 9.3.1
- Milestone 9.3.2
- Milestone 9.3.3
- Milestone 9.3.4
- Milestone 9.3.5
- Milestone 9.3.6
- Milestone 9.3.7
- Milestone 9.3.8
- Runtime API Public Stabilization Gate
- Runtime Authoritative Mutation Evidence Gate
- Milestone 9.4
- Milestone 9.5
- Milestone 9.6
- Milestone 9.7
- Milestone 10
- Milestone 11
- Milestone 12
- Milestone 13

Unlike the bridge roadmap, the query roadmap still builds major foundational
surface area in Milestone 1 onward. The certification rules therefore start at
Milestone 1 rather than only appearing late in the roadmap.

## Purpose

`forge-query` cannot be considered shipped merely because a typed builder
exists, a read returns rows, or a live subscription "looks right" in a direct
test.

The query layer makes claims about:

- canonical query meaning independent of construction path
- schema-aware legality before execution
- proof-carrying planning and snapshot-backed execution
- collection, pagination, traversal, aggregation, and CDC-shaped result truth
- live promotion and incremental result maintenance
- region-scoped invalidation and change-stream-backed delivery contracts
- preview-session basis identity and branch workflow parity
- frontier-aware planning and deterministic parallel admission
- branch/history/diff parity
- lineage/correspondence query meaning
- query-authored mutation, merge, and writeback lowering
- unified facade/configuration honesty
- scopes, templates, saved queries, and view-shape semantics
- policy masking, tenant schema variation, and relationship-proof denial
- query-owned subscription declaration, bridge lowering, and admission
- subscription family diagnostics, bridge parity, and runtime certification
- cross-runtime causal explanation across relational authority, bridge
  routing/evaluation, signal invalidation/evaluation, lineage, provenance, and
  Query inspection
- basis capability lifecycles, authority-scoped effect pipelines, typed
  projection fact materialization, structured admission decision lattices, and
  lower-runtime capability routing before the public runtime API freezes
- temporal query basis, time-aware subscription lowering, and time-only
  delivery
- public runtime API stabilization, golden DX transcripts, async-safe state
  gates, and inspection-as-contract
- async/resource query families, completion causality, and supersession
- mixed truth/time/async delivery ordering, coalescing, and replay
- temporal/async support metadata, diagnostics, and certification closure
- store-backed durability, pushdown, and artifact portability
- blob-backed delivery and upload-associated query semantics

Those are adversarial surfaces. They need certification tests, not just feature
checks.

## Global Adversarial Constraint

The query test suite must prove the following:

> Under alternate builder paths, schema variation, branch divergence,
> historical replay, live-update churn, temporal wakes, async completion
> races, policy masking, tenant-scoped schema drift, lineage ambiguity,
> store/runtime path variation, and restart/resume pressure, the same
> canonical query intent must produce the same query meaning, the same typed
> result/delivery contract, and the same machine-checkable explanation of why
> results changed, unless the scenario is intentionally semantically different
> or intentionally rejected.

If a query surface works only under one builder path, one execution path, one
schema state, one policy context, or one happy-path subscription shape, it is
not certified.

## Meta-Rules

These tests are all certification tests. They must:

- emit canonical machine-checkable artifacts, not "the response looked right"
- compare canonical digests across independently produced runs
- prove typed rejection for illegal or unsupported query forms
- prove replay/resume parity whenever the milestone claims restart, history, or
  durable continuation behavior
- verify exact counter contracts whenever the milestone claims boundedness,
  narrowing, or fallback honesty
- prove that runtime-backed and store-backed paths agree whenever both are
  admitted for the same capability
- prove that live-maintained results converge to the same truth as fresh query
  re-execution for the same basis
- prove that view-shape, policy, tenant, and lineage variations change only
  the semantics they are supposed to change
- prove that temporal execution basis never collapses into historical truth
  basis, ambient clocks, or host-local timers
- prove that async completions, retries, cancellations, and supersession
  cannot update a stale or policy/tenant-invalid query basis
- prove that mixed truth/time/async delivery order is canonical and replayable
  rather than dependent on host event arrival order

These requirements are mandatory, not advisory.

### Global Certification Shape

Every named certification suite must define at least these lanes unless the
suite explicitly states a narrower reason:

- `control_lane` - canonical admitted baseline
- `hostile_lane` - adversarial variation being certified
- `parity_lane` or `replay_lane` - an independently produced equivalent or
  restart/replay path

If the suite is about explicit rejection, the hostile lane may terminate in a
typed failure, but it still needs a successful or equivalent comparison basis.

### Mandatory Assertion Classes

Every named certification suite must include all applicable assertion classes:

- equality assertions for semantically equivalent lanes
- inequality assertions for intentionally different semantic lanes
- typed-failure assertions for rejected lanes
- zero-or-absence assertions for forbidden residue, forbidden widening, or
  forbidden fallback

### Canonical Query Certification Bundle

At minimum, certification bundles should emit the canonical fields applicable
to the suite scope:

- `query_digest`
- `plan_digest`
- `result_digest`
- `result_shape_digest`
- `basis_digest`
- `policy_digest`
- `lineage_digest`
- `delivery_digest`
- `temporal_basis_digest`
- `async_resource_digest`
- `cause_ordering_digest`
- `replay_digest`
- `failure_digest`
- `counter_snapshot`

Not every suite uses every field, but every suite should emit the stable,
scope-appropriate canonical bundle rather than free-form debug logs.

### Mutation-Sensitivity Rule

Every named certification suite must include at least one perturbation from
each applicable class:

- a perturbation that changes pacing, construction path, diagnostics richness,
  or execution path without changing canonical query meaning
- a perturbation that changes canonical query meaning and must therefore change
  at least one declared digest
- a perturbation that must fail explicitly before semantic drift occurs

### Anti-Fake-Test Rule

The following do not count as certification:

- asserting only that a query compiled or returned non-empty output
- asserting only that a digest is present
- comparing a value only to itself from the same run
- validating only a happy path without an adversarial lane
- validating only one execution path when the milestone claims path parity
- inspecting logs as the primary proof artifact

## Milestone 9.7 Phase 17 Required Suite

`Public-Bridge Reader-Lane Honesty Closure Test` is required for Phase 17. It
must prove that the public-bridge hostile certification path consumes published
derived artifacts only through typed projection-consumption receipts, that the
common and builder public-bootstrap paths produce equivalent certification
artifacts, that the public-bridge certification inventory has exact-zero direct
materialization row reads, and that sabotage restoring row-spelunking is
localized and rejected.

The detailed assertion matrix lives in
[Milestones 9.4-9.7](./test-requirements-milestones-9_4-9_7.md).

## Milestone 9.7 Phase 18 Required Suite

`Milestone 9.7 Derived Closure Posture Test` is required for Phase 18. It must
prove that milestone `Closed` posture is derived from the Phase 13 shared-read
pinning closure, Phase 15 journal/replay closure, Phase 16 concurrent hostile
matrix, and Phase 17 public-bridge reader-lane honesty artifact. The suite must
show that reopening any required phase-local proof, or removing its evidence
digest, prevents Milestone 9.7 from reporting `Closed`.

The closeout parity requirement is also part of this suite: support/profile
publication, this requirements matrix, and
[milestone-9.7-closeout.md](./milestone-9.7-closeout.md) must agree on the
derived posture boundary and must name defended exclusions instead of silently
expanding Milestone 9.7 ownership. The support-profile contract must not
hard-code `Closed`; `Closed` is produced only by the derived closure artifact
after it receives all required phase-local proofs.

## Milestone 9.8 Phase 1 Required Suite

`Consumer Evidence Report Kit Parity Test` is required for Phase 1. It must
prove that the Query-owned report kit lets consumers declare report fields,
semantic scope, and digest participation once, then receive sealed
canonical-scheme report identity, field-inventory identity, digest-participation
identity, and read-only field access without caller-owned digest construction.

The suite must include:

- one Query-owned report shape expressed through the kit
- one `BranchPreviewBasisReport`-shaped fixture expressed through the kit,
  including admitted and rejected variants
- perturbations proving participating field changes alter report identity
- perturbations proving diagnostic nonparticipating value changes do not alter
  report identity but do alter field inventory
- field add/remove/reorder/retype pressure against field-inventory identity
- participation-posture changes against digest-participation identity
- compile-fail coverage proving sealed reports cannot be externally
  constructed, fields cannot be mutated after sealing, report identity cannot
  be forged, pre-rendered digest contribution APIs are unavailable, and fields
  cannot be added without explicit participation posture
- proof that kit identities lower through `ForgeQueryEvidenceIdentity` scopes,
  not worth-kernel digest helpers, rendered strings, `Debug`, or delimiter
  joining

## Milestone 9.8 Phase 2 Required Suite

`Prohibition Registry And Seam Visibility Test` is required for Phase 2. It
must prove that Query owns one typed hard-prohibition registry and that every
covered direct workspace seam which can be enforced by Rust visibility is
sealed before the Phase 3 audit exists.

The suite must include:

- a typed registry row for each covered direct workspace write, batch, and
  existing-truth bypass seam
- exact agreement between the registry row keys and the documented hard
  prohibition seam-key projection
- proof that all Phase 2 rows use `sealed-by-visibility`, not Phase 3 audit
  residue, for seams the type system can close
- compile-fail coverage from an external consumer-shaped crate proving direct
  `ForgeQueryWorkspace` write, batch, existing-truth bind, probe, update,
  assert, verify, and delete seams are not reachable
- compile-fail coverage proving external crates cannot fabricate registry rows
  or downgrade a sealed seam into audit-only residue
- proof that the admitted replacement lane remains typed and explicit through
  the existing submission or Query-authored intent path rather than a string
  pattern list

## Milestone 9.8 Phase 3 Required Suite

`Shipped Bypass Audit Artifact Boundary Test` is required for Phase 3. It must
prove that the audit derives from the Phase 2 prohibition registry, walks
Rust syntax rather than raw source text, and emits a durable evidence report
for every reviewed source set.

The suite must include:

- seeded executable bypass detection for both method-call and associated-path
  forms derived from registry public symbols
- public facade coverage over a real downstream source inventory, using
  path-bearing worth-kernel Rust source files rather than only inline snippets
- a seeded dirty downstream source entry evaluated through the same public
  source-inventory path, proving typed failure findings name seam, mechanism,
  source label, source path, line, and column
- negative fixtures proving comments, doc attributes, and string literals do
  not produce findings
- registry/audit drift coverage proving every Phase 2 prohibited seam has an
  derived audit coverage row and an explicit mechanism label
- report identity perturbation proving the finding set participates in the
  sealed audit identity
- public facade DX coverage for `hard_prohibition_boundary_audit()
  .covering_sources(...).assert_clean()` and typed `try_assert_clean()`
  failure inspection
- source-set validation coverage for invalid crate names, blank source paths,
  and duplicate source labels
- mechanism honesty proving the first shipped audit is `ast-method-name-
  resolved`, including tests that pin associated-path suffix resolution and
  method-call name-only overmatch as an explicitly named limitation rather
  than compiler-backed type resolution

## Milestone 9.8 Phase 4 Required Suite

`Support Snapshot Projection Test` is required for Phase 4. It must prove that
the consumer-kit support snapshot is a serialized, schema-versioned,
digest-bound projection of the live runtime public support matrix, not a
second support authority.

The suite must include:

- projection equivalence against a real runtime support matrix derived through
  `ForgeQueryRuntimePublicApiContract` and
  `ForgeQueryRuntimePublicSupportMatrix`
- row-for-row equality for surface, facade family, support status, teaching
  posture, owner milestone, extension rule, fail-closed posture,
  parallel-API prohibition, optional contract digest, and live row digest
- deterministic export coverage proving repeated projection of the same live
  matrix emits the same snapshot digest and stable JSON document
- load-and-compare coverage proving the serialized document reloads through
  the public consumer-kit loader and still compares to the live matrix
- schema-boundary denial coverage proving mismatched schema version or schema
  identity fails with a typed support-snapshot error
- digest-drift denial coverage proving row or document mutation cannot be
  silently coerced into an accepted snapshot
- facade DX coverage proving downstream consumers can project, export, load,
  and compare without private constructors or caller-owned digest assembly

## Milestone 9.8 Phase 5 Required Suite

`Worth-Kernel Support Pinning Drift Test` is required for Phase 5. It must
prove that the Query-owned support pinning contract lets a consumer declare
the runtime support rows it requires, bind those requirements to the Phase 4
snapshot row identities, and fail with typed localized findings when a required
row regresses.

The suite must include:

- a satisfied `worth-kernel`-shaped contract requiring the `Write` and
  `Inspect` facade families from a real support snapshot projected from
  `ForgeQueryRuntimePublicSupportMatrix`
- adversarial drift coverage proving a `Write` posture regression fails only
  consumers pinned to `Write`, with findings naming the row, expected posture,
  actual posture, and live row digest mismatch
- proof that consumers not pinned to the regressed row do not fail merely
  because the source matrix digest changed; that digest change is reported as
  nonblocking evidence unless a pinned row also changes
- rejection coverage proving incomplete pin declarations fail typed before
  sealing and blocking findings fail through `assert_satisfied`
- facade DX coverage proving downstream consumers can declare, seal, evaluate,
  and assert support pins without private constructors or caller-owned digest
  assembly
- external compile-fail coverage proving contracts, requirements, reports, and
  findings cannot be fabricated by consumers
- reference-consumer adoption coverage proving the covered `worth-kernel`
  construction authoring path no longer carries `REQUIRED_QUERY_FAMILIES`,
  `REPORTED_QUERY_FAMILIES`, or `PrimitiveConstructionQueryGapRow` for the
  Query rows now owned by the support pinning contract

## Milestone 9.8 Phase 6 Required Suite

`Shipped Test Backend Honesty Test` is required for Phase 6. It must prove
that the consumer-kit in-memory backend gives downstream crates a real
`ForgeQueryWorkspace` over Query's runtime admission, support, receipt, and
read/write paths, not a caller-owned adapter pile or fabricated receipt seam.

The suite must include:

- facade DX coverage proving downstream consumers can import
  `in_memory_test_runtime` and `ForgeQueryTestBackendSchema` through
  `forge_query::facade::consumer_kit`
- public workspace coverage proving a schema-backed test workspace can declare
  a live view, write through `workspace.insert`, and read through the ordinary
  live-read path
- support-matrix honesty coverage proving supported rows are limited to
  implemented lanes and unsupported or deferred families remain fail-closed
- adversarial collection denial coverage proving writes outside the configured
  single-collection schema fail instead of silently widening the backend
- schema denial coverage proving blank or duplicate schema declarations fail
  before workspace construction
- preview and inspection evidence coverage proving the backend admits preview
  basis and write-receipt inspection through runtime-owned evidence artifacts
  rather than consumer-fabricated receipts
- residue coverage proving the shipped backend avoids public runtime adapter
  trait implementation and hand-fabricated mutation receipt requirements in
  downstream consumer tests

## Milestone 9.8 Phase 7 Required Suite

`Reference Consumer Report Adoption Test` is required for Phase 7. It must prove
that the covered `worth-kernel` construction evidence report surfaces adopt the
Phase 1 evidence report kit and canonical Query evidence identity scheme instead
of preserving local report/digest folklore.

The suite must include:

- semantic preservation coverage proving migrated reports still expose the same
  worth-kernel facts and drift assertions after report identity moves to the kit
- canonical identity coverage proving identical report inputs produce stable
  `forge.query.evidence-identity.v1` report identities through
  `EvidenceReportDeclaration::seal`
- covered residue coverage proving the covered report support files no longer
  import or call worth-kernel digest assembly for Query evidence identity
- defended residue coverage proving remaining construction digest helpers are
  classified as worth-domain artifact identity, not Query evidence identity

## Milestone 9.8 Phase 8 Required Suite

`Reference Consumer Enforcement Adoption Test` is required for Phase 8. It
must prove that the covered `worth-kernel` construction enforcement and support
posture surfaces consume Query-owned kit artifacts instead of preserving local
Query folklore.

The suite must include:

- reference-consumer coverage proving `worth-kernel` evaluates real source
  files through `hard_prohibition_boundary_audit().covering_sources(...)`,
  and that the resulting report is clean
- seeded violation coverage proving a prohibited Query seam introduced through
  the same Worth source-inventory path fails with a typed finding naming seam,
  source label, source path, line, and column
- support-pinning adoption coverage proving the durable
  `query_support_pins.json` contract evaluates against a live workspace support
  snapshot and fails through typed support-pin findings rather than a local
  required-family loop
- residue coverage proving covered construction surfaces contain zero
  `FORBIDDEN_RUNTIME_PATTERNS`, `REQUIRED_QUERY_FAMILIES`,
  `REPORTED_QUERY_FAMILIES`, `PrimitiveConstructionQueryGapRow`, or local
  `query_runtime_violation_count` Query-prohibition enforcement
- classification coverage proving any remaining source-string audits in
  `worth-kernel` are Worth-domain topology or legacy-deletion hygiene, not
  Query-prohibition rows, and that the classification is published as a
  canonical evidence report
- backend-applicability coverage proving `worth-kernel` either adopts the
  shipped in-memory test backend for a real hand-assembled adapter residue
  surface, or publishes an explicit not-applicable classification with zero
  hand-implemented Query runtime adapter traits and zero hand-fabricated
  mutation receipts in covered construction surfaces

## Milestone 9.8 Phase 9 Required Suite

`Milestone 9.8 Consumer Kit Hostile Certification Matrix` is required for
Phase 9. It must prove that the consumer kit closes as a support/profile
artifact only when docs, support rows, hostile certification, and
reference-consumer adoption residue agree.

The Consumer Kit is the ordinary downstream path for reference consumers that
need Query evidence, prohibition, snapshot, pinning, and test-backend proof.

The suite must include:

- support-profile closure coverage proving `support_report()
  .consumer_kit_closure()` publishes a sealed canonical closure artifact for
  every required kit family:
  `evidence-report-kit`, `hard-prohibition-registry`, `boundary-audit`,
  `support-snapshot`, `support-pinning`, `in-memory-test-backend`, and
  `reference-consumer-adoption`
- hostile matrix aggregation proving report misuse, seeded bypasses, support
  snapshot/pin agreement, test backend residue, and reference-consumer residue
  participate in one canonical certification digest
- sabotage coverage proving removing any required family evidence digest,
  reopening any family posture, breaking docs agreement, or publishing
  Query-owned reference-consumer residue prevents Milestone 9.8 from reporting
  `Closed`
- docs agreement coverage proving public docs and support/profile rows name
  the same consumer-kit family set and teach the kit as the ordinary downstream
  path, not optional ergonomics
- reference-consumer residue coverage proving current `worth-kernel` adoption
  counts publish zero Query-owned report/digest residue, zero Query-prohibition
  audit residue, zero support-pinning residue, and defended worth-domain
  residue as explicit evidence
- DX coverage proving downstream callers can inspect closure through the
  support report without private constructors, filesystem scanning, or
  caller-owned digest assembly

## Milestone 9.9 Phase 20 Required Suite

`Milestone 9.9 Graph Touch Obligation Authority Hostile Certification Matrix`
is required for Phase 20. It must prove that graph touch obligation authority
closes only when Query-owned selection, dispatch, execution status, reduction,
budget, residue, consumer adoption, reference-consumer adoption, docs, and
support rows agree.

The closure record for this suite is
[milestone-9.9-closeout.md](./milestone-9.9-closeout.md). That closeout is the
source of truth for defended exclusions and accepted residue; this matrix is
the required machine-checkable proof surface.

The suite must include:

- kind x lane x representative touch coverage proving every
  `ForgeQueryGraphObligationKind` and every covered
  `ForgeQueryGraphObligationSupportLane` participates in the certified matrix
  with no fake no-op executor rows
- support-status honesty proving `Supported`, `Unsupported`,
  `NotApplicable`, `DiagnosticOnly`, and `DeferredToBackstop` all appear as
  explicit support postures with budget and artifact-policy evidence
- selection replay equivalence proving equivalent touch descriptor, operating
  world descriptor, and obligation index inputs produce stable selection
  digests, and intentionally unrelated touches produce exact no-match results
  with lookup counters rather than fake candidates
- false-fire and false-miss pressure proving representative collection,
  relation, aspect, mutation-family, read-shape, and operating-world
  perturbations do not silently select the wrong obligations
- reduction algebra certification proving canonical ordering, duplicate rule
  observation accounting, severity reduction, denial projection, and digest
  stability under equivalent multi-obligation observations
- execution budget certification proving broad state-load attempts produce
  `BudgetExceeded` or a declared dense execution path before unbounded graph
  walks, with exact state-load counters and artifact-policy-gated diagnostics
- residue matrix certification proving every residue row has `introduced_in`,
  `must_not_exceed_count`, and `removal_trigger`, and that residue count never
  grows after introduction
- Consumer Kit certification proving a downstream reference consumer can adopt
  a covered graph obligation through Query-owned registration, selector
  coverage, support pinning, in-memory proof, bypass audit, adoption manifest,
  and residue manifest surfaces without local ceremony
- worth-topo and worth-kernel reference-consumer agreement proving their
  adoption manifests and residue manifests consume the same Query-owned proof
  vocabulary rather than local validator or phase-chain folklore
- docs agreement coverage proving AI guidance, support rows, and this
  requirements matrix name the same certification surface and do not teach
  manual invariant packs or consumer-local legality as the ordinary covered
  path

## Section Index

- [Milestones 1-4](./test-requirements-milestones-1-4.md)
- [Milestones 5-5.6](./test-requirements-milestones-5-5_6.md)
- [Milestones 6-8](./test-requirements-milestones-6-8.md)
- [Milestones 9-9.2](./test-requirements-milestones-9-9_2.md)
- [Milestones 9.3-9.3.3 And Runtime Gates](./test-requirements-milestone-9_3-and-runtime-gates.md)
- [Milestones 9.4-9.7](./test-requirements-milestones-9_4-9_7.md)
- [Milestones 10-13](./test-requirements-milestones-10-13.md)
- [Cross-Milestone Support And Honesty Suites](./test-requirements-cross-milestone.md)

## What These Tests Collectively Prove

Together, these tests prove that `forge-query` is:

- canonical about query meaning rather than builder-path dependent
- schema-aware before execution rather than repaired by runtime fallback
- snapshot- and basis-honest across runtime-backed and store-backed paths
- query-shaped across collection, live, diff, and delivery surfaces
- bridge-honest across query-owned subscription declaration and admission
  surfaces
- able to explain cross-runtime causality through Query inspection without
  domain consumers spelunking runtime bridge, relational, or signal internals
- explicit about temporal query basis, time-only delivery, async resource
  causality, and mixed truth/time/async cause ordering
- incapable of accepting stale async completions, ambient clocks, raw timer
  events, or host-arrival-order delivery semantics as certified query behavior
- explicit about lineage, correspondence, policy, and tenant-boundary meaning
- durable and portable where it claims durable or portable artifact support
- explicit about admitted versus non-admitted query-family combinations
- incapable of silently widening, degrading, or advertising unsupported beta
  surfaces as certified support
- certifiable through canonical artifacts rather than by visual inspection

## Milestone Certification Rule

No query milestone should be considered closed until its named certification
suite emits canonical machine-checkable outputs and passes across:

- original execution
- an adversarial or hostile variation lane
- an independently produced equivalent or replay/resume lane where applicable

Without that, the query surface may still be promising, but it is not yet
trust-grade.

## Beta Support Rule

No beta query surface should be considered supported until:

- its milestone-local named suite passes
- the `Admitted Query Family Boundary Test` passes for its admitted combination
  class
- the `Fallback Non-Leakage / No Silent Widening Test` proves unsupported
  neighbors fail closed
- the `Cross-Feature Composition Matrix Test` covers the relevant composition
  class if the surface is composed
- the `Beta Support Matrix Enforcement Test` shows support metadata,
  capability advertisement, and certification coverage are in sync

Without that, a query surface may exist experimentally, but it is not honest to
present it as beta-supported.
