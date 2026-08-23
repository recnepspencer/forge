# WORTH Query Test Requirements

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

`worth-query` cannot be considered shipped merely because a typed builder
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
  be WORTHd, pre-rendered digest contribution APIs are unavailable, and fields
  cannot be added without explicit participation posture
- proof that kit identities lower through `WORTHQueryEvidenceIdentity` scopes,
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
  `WORTHQueryWorkspace` write, batch, existing-truth bind, probe, update,
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
  `WORTHQueryRuntimePublicApiContract` and
  `WORTHQueryRuntimePublicSupportMatrix`
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
  `WORTHQueryRuntimePublicSupportMatrix`
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
`WORTHQueryWorkspace` over Query's runtime admission, support, receipt, and
read/write paths, not a caller-owned adapter pile or fabricated receipt seam.

The suite must include:

- facade DX coverage proving downstream consumers can import
  `in_memory_test_runtime` and `WORTHQueryTestBackendSchema` through
  `worth_query::facade::consumer_kit`
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
  `WORTH.query.evidence-identity.v1` report identities through
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

`Typed Consumer Residue Audit For Query Proof Folklore` is required for Phase
9. It must prove that `query_consumer_residue_audit(...)` is the Query-owned
authority for downstream Query-proof folklore residue, not a local grep list,
local class registry, or compatibility-only test-backend scanner.

The suite must include:

- one independent hostile fixture per `WORTHQueryConsumerResidueClass`, proving
  typed finding class, detection key, replacement lane, line/column, finding
  identity scope, and report identity scope through the public facade
- AST role coverage proving local Query report/proof structs, raw support row
  type paths, support-matrix row searches, proof-like local bindings,
  assignments, returns, and struct field values are detected by syntax role
  rather than incidental source substrings
- false-positive certification proving comments, doc comments, doc attributes,
  string literals, raw strings, char literals, ordinary debug formatting,
  ordinary delimiter joining, ordinary delimiter formatting, and unrelated
  `row_for_family(...)` calls do not produce residue
- exact-text multiplicity coverage proving repeated runtime/test-backend
  residue in one source file emits repeated typed findings with distinct
  coordinates rather than collapsing to the first site
- registry agreement coverage proving every residue class has exactly one row,
  every row has nonempty detection key, explanation, and replacement lane, AST
  detection is reserved for proof-folklore classes, and replacement lanes point
  to shipped Consumer Kit families
- identity perturbation coverage proving identical inputs produce identical
  report identities while consumer name, audited root set, finding set, and
  source coordinates participate in canonical identity
- compatibility coverage proving `query_test_backend_residue_audit(...)`
  delegates to the generic registry, filters out proof-folklore classes, and
  preserves legacy test-backend evidence scopes
- reference-consumer adoption coverage proving real downstream roots can run
  the generic audit without local residue classes, local scanners, or local
  replacement matrices
- sealed Query-owned root authority coverage proving downstream public callers
  cannot suppress residue while Query-owned implementation certification can
  classify Query roots through an unWORTHable authority token

## Milestone 9.8 Phase 10 Required Suite

`Milestone 9.8 Consumer Kit Hostile Certification Matrix` is required for
Phase 10. It must prove that the consumer kit closes as a support/profile
artifact only when docs, support rows, hostile certification, and
reference-consumer adoption residue agree.

The Consumer Kit is the ordinary downstream path for reference consumers that
need Query evidence, prohibition, snapshot, pinning, and test-backend proof.

The suite must include:

- support-profile closure coverage proving `support_report()
  .consumer_kit_closure()` publishes a sealed canonical closure artifact for
  every required kit family:
  `evidence-report-kit`, `hard-prohibition-registry`, `boundary-audit`,
  `support-snapshot`, `support-pinning`, `in-memory-test-backend`,
  `consumer-residue-audit`, and `reference-consumer-adoption`
- hostile matrix aggregation proving report misuse, seeded bypasses, support
  snapshot/pin agreement, test backend residue, generic consumer proof-folklore
  residue, and reference-consumer residue participate in one canonical
  certification digest
- typed consumer-residue certification evidence proving
  `consumer-residue-audit` closure comes from Query-owned detector execution,
  not source marker or test-name string checks
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
- reference-consumer source-inventory coverage proving adoption tests assert the
  generic audit report's audited source paths, skipped non-Rust source count,
  source-inventory digest, report identity, and finding identities
- compile-fail coverage proving downstream callers cannot WORTH Query-owned
  implementation-root authority to suppress consumer-residue findings
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
  `WORTHQueryGraphObligationKind` and every covered
  `WORTHQueryGraphObligationSupportLane` participates in the certified matrix
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

## Milestone 9.13 Phases 13-20 Required Suite

`Milestone 9.13 Runtime-Installed Domain Authority Certification` is required
for the add-on Phase 13-20 boundary. It must prove that one typed package and
one runtime installation are the only semantic authority path from domain
setup through execution, contributions, recovery, inspection, and consumer
adoption.

The suite must include:

- a source-backed authority inventory that classifies package input,
  installation, installed-handle capabilities, derived indexes, diagnostics,
  physical adapters, and prohibited competing authority by defining and
  exporting path
- package permutation, collision, callback-denial, and structured operating-
  context identity coverage proving Query canonicalizes and seals all semantic
  identity while failed validation produces no successor
- atomic installation and runtime-affinity coverage proving equivalent
  packages in distinct runtimes do not make their handles interchangeable and
  a failed late substrate compilation publishes no partial registry state
- substrate equivalence and rebuild coverage proving ordinary read,
  explanation, live, and internal-oracle paths consume the same installed
  operation definition without a caller-supplied registry, with bounded indexed
  lookup under unrelated package and operation growth
- installed contribution coverage proving evaluation, admission, preparation,
  materialization, targets, and mixed outcomes retain one domain, package,
  runtime, generation, world, and target authority chain
- end-to-end execution coverage proving read, workflow, projection,
  inspection, live continuation, stale-generation denial, and rebind preserve
  their distinct proofs and perform zero later-phase work after invalid
  authority
- external-consumer DX coverage proving a downstream extension trait lowers
  through `WorthQueryInstalledDomainHandle` without owning identity,
  registration, planning, execution, receipts, or diagnostics
- compile-fail coverage for all installed-domain sealed constructors and
  transitions, including package admission, runtime generation, receipts,
  contribution transitions, target restamping, raw domain authority, raw
  operation ownership, caller-authored context digests, manual registries,
  low-level materializers, and closed-live-handle revival
- typed sabotage coverage for raw domain strings, caller-authored context
  digests, application-facade executable authority, independent and caller-
  supplied operation registries, Query phase-materializer imports, and
  consumer-owned semantic domain adapters, while physical adapters remain
  accepted
- source-backed reference-consumer coverage across Hadwiger and Worth UI that
  records the audited source inventory, proves zero competing-authority
  findings, and executes representative installed read, workflow,
  contribution, invariant, projection, and inspection journeys
- documentation agreement coverage proving the current feature guide and
  `AI_README.md` teach only package declaration, builder installation,
  workspace lookup, installed handles, Query-sealed context identity, and
  downstream extension vocabulary; every relative product-doc link must
  resolve
- one canonical certification bundle that composes the authority inventory,
  compile-fail manifest, residue registry, domain-capability certification,
  real reference-consumer journey, and audited source-inventory digests, and
  reopens if any required evidence family is absent

Phases 13-20 may report closed only when the named source-backed evidence map
contains every phase exactly, the real consumer residue audit is clean, and the
certification bundle reports zero authority findings, zero missing compile-fail
boundaries, and zero missing installed-domain residue classes.

## Milestone 9.13 Phases 21-30 Required Suite

`Milestone 9.13 Foundational-Native Aspect Authority Certification` is required
for the add-on Phase 21-30 boundary. It must prove that exact Foundational
aspect value, patch, and state meaning survives every trust boundary and that
Foundational admission is the only source of native aspect authority across
Relational, Query, durability, replay, and consumers.

The suite must include:

- a source-backed authority inventory that classifies raw authored values,
  portable patch/state candidates, contract validation, authoritative patches
  and states, Relational intents and plans, durable payloads, Query wrappers,
  result carriers, canonical encoders, and prohibited competing authority
- Foundational portable-boundary coverage for whole set, whole clear, field
  set, field clear, scalar and nested struct values, entity references, null,
  absence, contract revision, mutation mask, canonical export, and fresh
  readmission
- hostile portable readmission coverage for missing and stale contracts,
  malformed canonical wrappers, illegal masks, undeclared fields, duplicate or
  contradictory operations, forged proof-shaped bytes, and atomic state
  snapshot denial
- Relational entity/relation and creation/update parity through the real
  transaction boundary, proving native and compatible field authoring converge
  only where Foundational operation meaning is identical
- transaction permutation, batching, merge, conflict, precondition, invariant,
  uniqueness, commit-strategy, touched-scope, receipt, publication, and exact
  bounded-work coverage over native patches
- checkpoint and replay restart coverage proving serialized payloads remain
  non-authoritative, current contracts are consulted, full native state is
  re-admitted before publication, and log/checkpoint rebuilds converge
- public Query mutation coverage for every Foundational scalar family and
  representative structs through ordinary runtime-backed execution with exact
  patch identity and no field-map, JSON, display-text, or raw-id authority
- contract-derived schema and predicate matrices covering exact native operand
  families, legal operator capabilities, canonicalization, early denial, and
  runtime/internal-oracle parity
- result, live, retained-row, projection, consumed-fact, and typed-refinement
  coverage proving scalar and complete struct meaning is never flattened or
  reconstructed by consumers
- canonical identity parity proving one Foundational value/patch basis is
  composed with explicit artifact domain separation across transactions,
  durability, Query mutation, results, projection, replay, and certification
- compile-fail, visibility, facade, prohibition, residue, and sabotage coverage
  rejecting raw-to-proof promotion, proof-bearing deserialization, direct
  restored-state insertion, set-only native projection, Query-owned value
  algebras, duplicate semantic encoders, scalar-only bridges, and consumer-local
  native reconstruction
- source-backed Hadwiger and Worth UI adoption coverage that records deleted
  adapters, projections, local value models, and authority reconstruction and
  executes representative scalar, struct, reference, clear, predicate, result,
  and replay journeys through the ordinary facade
- documentation agreement coverage proving feature docs and `AI_README.md`
  teach only the present native authoring, installed-domain, result, and
  refinement grammar without historical or competing methods
- one canonical certification bundle composing phase evidence, native-family
  and operation matrices, transaction and restart parity, compile boundaries,
  residue and sabotage registries, consumer adoption, canonical identity, and
  exact work counters

Phases 21-30 may report closed only when every patch operation, native scalar
family, and required struct journey has one authoritative owner and complete
evidence; all portable, Relational, durable, Query, and consumer boundaries
agree; and the certification bundle reports zero competing authority, semantic
projection, proof-forging, replay-readmission, or consumer-residue findings.

## Milestone 9.16.2 Required Suite

`Portable Package And PostgreSQL Runtime Durability Certification` is required
for Milestone 9.16.2. It begins at the production package and persistent
`worth-query-host` facades, uses production `worth-runtime-postgres` adapters
against a real PostgreSQL server, and crosses fresh process boundaries.

This suite is the aggregate Milestone 9.16.2 court, not a substitute for phase
closure. The fifteen-phase plan in
[milestone-9.16.2.md](./milestone-9.16.2.md) must retain phase-local evidence
for identity/provenance, typed export, reconstruction, archive trust,
Relational backend conformance, PostgreSQL migration/lifecycle, release
coexistence, Relational commit/replay, owner-first readiness, Query outbox
admission, claim/fencing, dispatch/reconciliation, disaster recovery,
capacity/isolation, and the NCR cutover. An aggregate green result cannot close
an earlier boundary whose own authority, crash, mutation, or cost evidence is
missing.

The suite must run these named scenarios:

- `portable_release_round_trips_by_exact_identity`: deterministic archive,
  independently expected semantic identity, hostile records, coexisting
  releases, and fresh Query validation agree without serialized authority
- `acknowledged_relational_commit_survives_restart`: kill before commit, after
  durable commit before response, and during checkpoint/tail recovery; a fresh
  host observes either the complete mutation plus outbox or neither
- `existing_outbox_restarts_without_shadow_payload`: destroy every receipt,
  runtime id, and handle; reconciliation rediscovers the canonical outbox fact,
  competing fenced workers preserve one current claim, and retries retain one
  idempotency identity
- `persistent_host_readiness_is_owner_driven`: package validation, Relational
  recovery, provider rebinding, projection reconciliation, and claim admission
  must all close before readiness; adapter rows cannot mint owner authority
- `postgres_topology_admits_committed_owners`: dependency and destination-tree
  checks prove Signal and Runtime Bridge enter as owner siblings in 9.17 without
  moving the PostgreSQL adapter or persistent Query-host facades
- `ncr_state_and_notification_recover`: the real NCR journey commits workflow
  state and the existing notification outbox, restarts, dispatches safely, and
  serves the exact resulting state through the ordinary host path

Milestone 9.16.2 may report closed only when package portability, acknowledged
state durability, existing-outbox restart, host readiness, owner/adapter
dependency direction, scale/work counters, backup/restore, executable docs, and
residue evidence agree.

## Milestone 9.17.1 Required Suite

`Owner Component Basis And Relational Branch-Local MVCC Certification` is
required for Milestone 9.17.1. It must use real Relational transaction/history
facades and the real Signal branch-basis facade rather than directly writing
heads, versions, snapshots, generations, or retention state. Its durability
lane must use the production `worth-runtime-postgres` owner adapters against a
real PostgreSQL server and a fresh process.

The suite must build one causally complete component world and run these named
scenarios:

- `blocked_branch_a_does_not_stop_branch_b`: branch A blocks immediately before
  its lawful publication boundary while branch B commits; branch-B wait and
  unrelated-state-touch counters remain exactly zero
- `same_relational_head_has_one_winner`: two branch-A transactions share one
  expected head; exactly one publishes and the other receives the precise
  stale/conflict outcome without head movement
- `equal_ordinals_do_not_substitute_authority`: equal local versions on
  different branches and runtimes are swapped one axis at a time and deny
  before owner effects
- `boundary_crossing_requires_owner_readmission`: serialized, restored, and
  checkpoint-derived component descriptors cannot regain operational authority
  without current owner validation
- `signal_basis_reuse_is_exact_and_immutable`: several consumers retain one
  exact Signal basis with zero graph copy/evaluation/cache duplication, while a
  mutation requires an owner-issued fork or advance
- `retention_follows_live_obligations`: branch, snapshot, transaction,
  candidate, and external composition pins independently prevent reclamation
  and release exactly
- `cancellation_cleans_every_owner_phase`: cancellation before reservation,
  after reservation, after validation, after candidate creation, and before
  publication leaves no unauthorized head movement or unbounded residue
- `branch_local_work_has_branch_local_slopes`: unrelated branch count, history,
  writers, and diagnostics do not increase selected-branch validation or
  publication counters
- `component_owners_recover_exact_branches`: kill after acknowledged Relational
  and Signal owner publications, destroy process-local authority, and recover
  exact branches/bases through owner-first PostgreSQL reload and readmission
- `durable_component_artifacts_do_not_substitute`: cross-branch Relational
  checkpoint/tails and cross-definition/runtime Signal artifacts fail before
  effects; SQL rows and restored descriptors mint no owner authority

The suite must also include consolidated public compile-pass/compile-fail
evidence for raw basis minting, cross-branch transaction/head pairing, phase
skipping, prepared-candidate publication, generic authority substitution, and
consumed-witness reuse. Compiler evidence is limited to those current public
authority guarantees; it is not a general Rust API census.

Milestone 9.17.1 may report closed only when the current ordinary Relational
path contains no global commit coordinator or broad mutable entry that
serializes independent branches, every owner basis is private-minted and
readmission-bound, both owners recover exactly from real PostgreSQL, and no
composite product authority is claimed.

## Milestone 9.17.2 Required Suite

`Composite Runtime-World History And Coordinated Publication Certification` is
required for Milestone 9.17.2. It must invoke the frozen 9.17.1 owner ports and
the real Runtime Bridge runtime-world facade. Expected history must be produced
by an independent action interpreter, not by Bridge classifiers or branch-head
queries. Publication and recovery must use the production Runtime Bridge
PostgreSQL adapter against a real server.

The suite must run these named scenarios:

- `shared_signal_basis_divergent_relational_worlds`: two product branches reuse
  one exact immutable Signal basis while their Relational branches diverge
- `component_specific_advancement_is_exact`: Relational-only, Signal-only, and
  combined operations advance exactly the named components and retain every
  unchanged basis byte-for-byte/canonically
- `foreign_or_equal_component_basis_is_rejected`: each runtime, branch,
  version, generation, definition/schema, correspondence, product-head,
  operation, and attempt axis drifts independently and denies before
  publication
- `same_product_head_has_one_winner`: two compatible prepared programs race one
  expected product head; one CAS wins and one receives stale-head posture
- `partial_preparation_never_becomes_current`: every owner success/rejection
  ordering, timeout, and cancellation point leaves the product head unchanged
  unless the final Bridge CAS succeeds
- `response_loss_recovers_performed_publication`: loss after CAS but before
  response returns a recovery handle that observes the performed commit without
  duplicating it
- `single_parent_history_and_retention_are_exact`: arbitrary retained-commit
  branch creation, parentage, archive, pins, reclamation, and derived-index
  destruction/rebuild agree with the independent oracle
- `independent_product_branches_progress`: a blocked publication on one product
  branch contributes exactly zero waits to another branch
- `ordinary_publication_has_bounded_structural_cost`: component, branch,
  history, and diagnostic population slopes match the fixed-component contract
- `durable_product_head_recovers_owner_first`: kill before composite commit,
  after durable commit/head CAS but before response, and after component reopen
  but before Bridge readmission; a fresh process recovers the exact performed
  result only after both component owners readmit every referenced basis
- `owner_local_outbox_is_not_product_current`: a Relational owner candidate may
  contain the existing Query outbox fact, but failed Signal preparation, stale
  product head, or failed composite CAS produces no performed publication and
  no product-notification eligibility

The suite must include targeted compile-fail evidence for raw component tuple
admission, candidate-to-current promotion, publication phase skipping, weaker
Proof substitution, direct product-head mutation, and reused performed
evidence. Mutation controls must remove or bypass component correspondence,
unchanged-basis retention, compatibility, final CAS, performed minting, and
half-publication isolation and turn the named scenario red.

Milestone 9.17.2 may report closed only when one canonical Bridge artifact owns
each composite commit/head transition, owner failures cannot expose a half
world, exact PostgreSQL lifecycle/recovery evidence is bounded, owner-local
outboxes remain product-ineligible, and no final Query facade completion is
claimed.

## Milestone 9.17.3 And Umbrella Required Suite

`Query Composite Product-Branch End-To-End Certification` is required for
Milestone 9.17.3 and final Milestone 9.17 closure. It must begin at the real
public Query composition root in a causally complete installed application
world and observe real owner and Bridge outcomes. Query-only fixtures or direct
lower-runtime assembly are supplemental and cannot close the suite. The restart
and dispatch lanes must use `WorthQueryHost::open_persistent`, production
`worth-runtime-postgres` adapters, a real PostgreSQL server, separate processes,
and the real external-effect transport contract.

The suite must run these named scenarios:

- `public_shared_signal_basis_workflow`: public branch creation expresses an
  explicit Relational fork and exact Signal-basis reuse, then reads, mutates,
  publishes, inspects, and observes the correct composite world
- `public_component_divergence_matrix`: Relational-only, Signal-only, and
  combined public operations preserve exact component posture through plans,
  sessions, read sets, proposals, invariants, effects, terminals, receipts,
  history, live delivery, inspection, recovery, and aftermath
- `stale_between_every_query_phase`: the product head advances between each
  adjacent public/private Query transition; stale/rebind posture appears before
  the next effect and no phase silently retargets
- `one_axis_query_binding_drift_denies`: product branch, composite head,
  component bases, correspondence, provider session, operation, attempt, and
  freshness axes drift one at a time and open no door
- `partial_preparation_is_unobservable`: one-shot, live, history, preview,
  inspection, and aftermath observers never see component candidates or a half
  product world
- `query_boundary_round_trip_readmits`: serialized/restored Query artifacts
  lose currentness and regain it only through Query and Bridge owner readmission
- `diagnostics_tiers_preserve_operational_truth`: operational, development, and
  forensic executions produce identical product truth and operational receipts
  while only lawful sidecars differ
- `default_and_parallel_lanes_converge`: admitted serial and parallel lanes
  produce identical component history, composite history, outcomes, receipts,
  and global publication order
- `response_loss_and_cancellation_preserve_exact_lifecycle`: every Query-to-
  Bridge/owner transfer has typed cancellation/recovery and bounded cleanup
- `legacy_product_branch_authority_is_absent`: facade, dependency, constructor,
  source, and consumer inventories find exact-zero Relational-only product
  identity, derived branch identity, ambient Signal selection, raw pairing,
  compatibility authority, and test-only bypass
- `product_branch_cost_is_population_independent`: unrelated product/component
  branches, history, consumers, subscriptions, and diagnostics do not change
  fixed basis-carriage and publication counters
- `composite_runtime_recovers_before_readiness`: destroy every process-local
  package, basis, receipt, and handle; a fresh host recovers package, Relational,
  Signal, Bridge product heads, Query carriage, and pending dispatch in owner
  order before readiness
- `existing_outbox_requires_performed_composite_publication`: kill after the
  Relational outbox commit while Signal/Bridge publication fails; the durable
  outbox remains exact owner state but no claimant can dispatch it
- `dispatch_crash_matrix_preserves_one_identity`: kill before send, after send,
  and before acknowledgement; fresh fenced claimants retry only with the same
  Query idempotency identity, and stale claimants cannot send or acknowledge
- `workflow_aftermath_reenters_composite_history`: operational delivery outcomes
  move no product head; any completed/unresolved workflow fact appears only
  through a subsequent performed Query/Bridge composite publication
- `external_effect_declares_relational_outbox_component`: an otherwise Signal-
  only change with external dispatch lowers as combined component work; deleting
  the Relational outbox footprint fails before preparation and cannot create a
  post-publication sideband write

The suite must include executable public examples and consolidated compiler
evidence proving intended branch journeys compile while raw minting, phase
skipping, cross-basis pairing, stale proof reuse, owner-candidate publication,
internal lower-runtime entry, and non-terminal consequence construction do not.
The compiler evidence protects the named public progression only; it must not
grow into a general compiler, macro expander, or complete Rust name-resolution
engine.

The final canonical certification bundle must bind:

- the frozen 9.17.1 owner-component evidence;
- the frozen 9.17.2 Bridge composition evidence;
- every 9.17.3 scenario and mutation-sensitive control;
- real-PostgreSQL component, composite, host-readiness, and dispatch crash
  evidence;
- exact Foundational canonical case/report identity;
- exact Proof phase/basis/performed progression evidence under owner wrappers;
- default/parallel, diagnostics-tier, lifecycle, performance, facade,
  dependency, docs, and residue results; and
- the exact source revision and support posture certified.

Milestone 9.17 closes only when all three named suites are green and the final
end-to-end suite proves the lower guarantees remain true through Query. A green
component test, Bridge publication test, facade compile test, or self-reported
manifest alone is not umbrella closure.

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

Together, these tests prove that `worth-query` is:

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
