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
for Milestone 9.16.2. It starts at production package and
`worth-query-host` audience facades, crosses Query execution's production
persistent-opening surface, supplies the production `worth-runtime-postgres`
implementations of owner-defined ports, and uses a real supported PostgreSQL
server across fresh application and database process boundaries. An in-memory
adapter, SQL fixture mutation, or test-only recovery branch cannot close a
production claim.

This suite is the aggregate court, not a substitute for phase closure. Every
proof family records:

- the production claim and the plausible defective implementation it convicts;
- the causal world, semantic handles, and authority provenance;
- the real public entry surface and physical composition root;
- the exact fault, crash point, concurrency schedule, version boundary, or
  resource limit;
- the required typed result and states/effects that must and must not survive;
- an independent observation that does not merely read the producer's output;
- exact work, scan, retry, pool, replay-tail, and amplification counters where
  cost is part of the claim;
- at least one sabotage mutation whose disputed bypass, inversion, deletion,
  stale reuse, or scope widening must turn the proof red; and
- its ordinary, PostgreSQL, process-crash, database-crash, or disaster-recovery
  cost lane.

Fault control uses production boundary telemetry, a process supervisor, network
fault proxy, database lifecycle control, and process termination. A substitute
owner port may isolate a local state-machine property, but the same guarantee
must close against the real PostgreSQL adapter whenever physical atomicity,
durability, migration, isolation, or recovery is claimed.

The cert-only `worth-runtime-persistence-certification` kit owns reusable
conformance cases for Relational durability, immutable package archives,
runtime-stream lifecycle, dispatch coordination, and the required capability
profile. PostgreSQL certification instantiates all of them. The kit cannot be a
production dependency, cannot provide a fake passing implementation, and must
be reusable unchanged by a future physical adapter except for adapter-supplied
fixtures and independently required crash/operations evidence.

### Required world and independent oracle

The suite extends the Bank world through
`BankPersistenceWorldDefinition -> CompiledBankPersistenceRelease ->
ProductionSeededPersistentBankWorld -> CertifiedPersistentBankBaseline`.
Scenarios receive semantic handles for actors, accounts, operations, effects,
pending notices, releases, namespaces, and runtime streams; raw ids, digests,
ordinals, and database keys do not appear in scenario setup.

Required valid baselines are empty persistent installation, ordinary balanced
bank, pending payment notice, pending notice after checkpoint and compaction,
coexisting release/version boundary, and separately restored bank. Corrupt,
cross-spliced, partially restored, or stale-fence worlds are explicitly invalid
fixtures. Each scenario applies a small causal delta to one audited baseline.

The independent oracle checks a separately decoded Relational owner artifact or
owner-supported audit export plus the external payment rail. Query reads,
pending-work projections, adapter status rows, and production result assembly
cannot be the only oracle for the claim they produced. The final NCR consumer
court independently proves Workflow Editor adoption and does not replace the
Bank world's ledger/outbox adversity.

### Required phase proof families

1. `stable_identity_and_declared_provenance` proves module moves preserve
   identity, semantic changes alter it, collisions/blank ids fail, and only the
   declaration can mint operation/effect membership. It includes compiler
   denial and a mutation that restores package-canonical `type_name` or accepts
   a same-spelled forged reference.
2. `complete_typed_package_export` inventories every package-relevant family,
   proves deterministic canonical ordering and declared work bounds, and fails
   for omitted, duplicated, reordered, cross-package, callback, provider,
   secret, or authority-bearing material. Removing one record-family export
   must fail closure evidence.
3. `bounded_reconstruction_and_fresh_validation` starts only from untrusted
   records, enforces byte/count/nesting/work limits, reconstructs a candidate,
   and obtains fresh Query validation against an independently supplied expected
   identity. Trusting the claimed digest, accepting trailing required meaning,
   or skipping fresh validation must fail.
4. `neutral_archive_and_trust_compatibility` proves golden canonical bytes,
   exact envelope/manifest/record versions, signature coverage, downgrade and
   unknown-field posture, tamper refusal, store-neutral round trip, and
   coexisting same-named releases. Selecting latest-by-name or treating a valid
   signature as Query authority must fail.
5. `relational_durability_port_conformance` runs the local filesystem and
   PostgreSQL implementations against the same owner contract for authorized
   append, durable-before-publication, checkpoint, ordered recovery, corruption,
   duplicates, forks, and bounded work. Allowing a successful backend write to
   mint publication or accepting an unissued append request must fail.
6. `postgres_lifecycle_capability_and_migration` proves bounded connection and
   shutdown behavior, namespace qualification, supported migrations,
   incompatible/partial migration refusal, and admitted durability settings
   including `fsync`, `full_page_writes`, and `synchronous_commit` or the
   admitted equivalent. Readiness under an incapable or downgraded profile and
   partial canonical mutation on migration failure must fail.
7. `release_coexistence_stream_binding_and_activation` stores exact immutable
   archives, rebuilds only derived projections, races generation-qualified host
   activation, and binds each runtime stream to exact package, owner-artifact,
   schema, and provider requirements. Latest-name selection, cross-package
   stream reinterpretation, caller-forged/generic activation authority,
   missing/wrong provider binding, and pointer-flip migration must fail with
   typed compatibility posture.
8. `commit_acknowledgement_checkpoint_tail_crash_matrix` kills before database
   commit, after durable commit before response, during checkpoint, and during
   tail recovery. Fresh recovery observes complete mutation plus canonical
   outbox or neither and returns performed, already-performed, rejected,
   conflict, or indeterminate-with-locator honestly. Treating an ambiguous
   response as safe new work, acknowledging before append, or tearing state and
   outbox must fail.
9. `owner_first_opening_recovery_and_readiness` destroys all live ids, receipts,
   proofs, and handles; then proves exact release trust, fresh validation,
   Relational recovery, runtime generation, provider rebinding, projection
   repair, and dispatch reconciliation all close before readiness. Adapter rows,
   a physical snapshot, old authority, or readiness-before-rebinding must open
   no door. The host crate remains a reexport-only leaf.
10. `performed_publication_outbox_admission` rediscovers the exact canonical
    Query outbox occurrence and admits it only through the current performed
    product publication, runtime, package/effect contract, correlation,
    provider binding, and source commit. A copied/shadow payload, database
    status alone, stale runtime, wrong release, or caller-forged publication
    carrier must fail; the 9.17 Bridge source substitution must not move the
    stable Query meaning or facade.
11. `claim_lease_and_fence_coordination` races claimants, expires and renews
    leases, supersedes fences, isolates namespaces, and proves bounded indexed
    polling through Query execution's provider-neutral coordination port. At
    most one fence is current. Starting a send or accepting an outcome under a
    stale fence, holding a database transaction across network I/O, or scanning
    global history must fail.
12. `dispatch_ambiguity_retry_and_reconciliation` crashes before send, after
    send, after external acceptance before local outcome, during backoff, and
    after fence replacement. Every retry uses the one canonical payload and
    stable idempotency identity; terminal, unresolved, poison, and cancelled
    postures remain typed. A second outbox payload, new retry identity, stale
    acknowledgement, or direct sideband workflow mutation must fail.
13. `compaction_projection_rebuild_backup_and_restore` checkpoints and compacts
    with unresolved work, deletes all and only derived projections, rebuilds,
    backs up, restores into a separate database, and opens from restored owner
    artifacts. The exact pending occurrence, source publication, payload, and
    idempotency identity survive. Pruning their last canonical source, using
    diagnostics as truth, partial restore activation, or unsupported migration
    mutation must fail.
14. `isolation_saturation_scale_and_warm_path` proves hostile cross-namespace
    denial, bounded pool/blocking/queue/retry posture, exact lookup among at
    least 4,096 unrelated packages, bounded pending polling over long history,
    checkpoint-tail bounds, and zero archive/reconstruction/recovery/global-work
    scans during warm execution. Removing an index, qualification predicate,
    admission bound, or counter assertion must fail rather than pass slowly.
15. `persistent_bank_and_ncr_product_courts` runs the production Bank transfer,
    payment-notice, checkpoint/compaction, crash, competing recovery, fenced
    dispatch, coexistence, and separate-database restore journey, then runs the
    signed NCR state/notification restart and Workflow Editor cutover through
    the same public surfaces. Both use real PostgreSQL and fresh processes;
    replacing either with an in-memory reenactment or a direct private facade
    call must fail certification.

### Cost topology and closure

Immutable release compilation and a supported PostgreSQL image may be
suite-scoped. Ordinary cases receive per-test isolated databases or schemas and
streams. Application-process crash cases may share a database server; database
process, storage, migration, and backup/restore cases use a dedicated cold lane.
Authentik, HTTP nodes, and the external rail appear only where the product claim
requires them. Test topology uses one intentional integration target with
responsibility-named modules per certification crate, plus at most one
separately justified cold process/Docker target for each genuinely distinct
process-lifecycle court; scenario files do not become dozens of integration
crates.

Milestone 9.16.2 may report closed only when all fifteen proof families and
their sabotage cases agree with the truth-class ledger, public dependency
direction, executable documentation, destination topology, and residue search.
An aggregate green result cannot close an earlier authority, crash, retention,
compatibility, or work guarantee whose phase-local evidence is absent.

## Milestone 9.17.1 Required Suite

`Owner Component Basis And Relational Branch-Local MVCC Certification` is
required for Milestone 9.17.1. Its governing architecture, profiles, deltas,
authority boundaries, counters, and closure ledger are defined in
[milestone-9.17.1.md](./milestone-9.17.1.md). The suite uses the real Relational
schema, transaction, branch, history, inspection, and retention facades and the
real Signal branch-basis facade. Direct head, root, version, snapshot,
generation, id, index, or retention-table mutation cannot certify a claim. Its
durability lane uses the production `worth-runtime-postgres` owner adapters
against a real PostgreSQL server and a fresh process.

### Required world and oracle

The suite first ships the canonical **Supply Chain** world. Its
immutable definition compiles through production facades into a fresh runtime;
semantic names bind only to owner-issued handles. Court, Standard, and Scale
profiles share the same schema, semantic-key, scenario-delta, observation, and
oracle vocabulary. Phase 3 certifies the empty-installation and operating
baselines through public schema, transaction, snapshot, and read facades. The
contested-planning, retention-pressure, and schema-version-boundary baselines
remain semantic declarations until the later branch-reference, MVCC, and
schema-version phases add the owner-issued basis and transaction capabilities
they require. A Phase-3 baseline's descriptive branch envelope is not an
admitted operational basis.

The production compiler and independent oracle are distinct causal paths. The
oracle interprets semantic deltas over ordered semantic maps and ancestry; it
does not use Relational MVCC, roots, indexes, queries, history classifiers,
encoders, digests, or visibility logic. A separate adapter projects public
runtime results to semantic observations and a separate comparator evaluates
them. Fixture construction, owner execution, observation, oracle, and
comparison failures remain distinguishable.

The world is a prerequisite deliverable, not incidental test setup. These
world-certification cases are required:

- `supply_chain_world_compiles_causally_through_public_facades`
- `supply_chain_named_handles_are_owner_issued_and_complete`
- `supply_chain_baseline_matches_independent_oracle`
- `supply_chain_oracle_rejects_missing_write_and_sibling_leak`
- `supply_chain_oracle_rejects_floating_head_and_wrong_ancestry`
- `supply_chain_profiles_preserve_meaning_while_scaling_density`
- `supply_chain_failure_trace_replays_from_profile_seed_and_delta_log`

The existing Fintech and generic worlds remain green preservation suites. They
do not replace Supply Chain closure evidence.

Phase 3 must also contain causal negative twins for each production boundary.
The failure family is part of the certification contract, not optional test
decoration: invalid declarations fail before runtime admission; schema kind
collisions fail at initial installation; a bad endpoint and an over-tight
publication budget fail at transaction/publication admission; missing,
incomplete, duplicate, wrong-kind, and wrong-endpoint semantic correspondences
fail before a baseline is certified; foreign-runtime and unknown snapshot
handles are denied; unbound entity/relation identities are rejected instead of
ignored; and oracle, observation, and comparison failures remain distinct.
The empty lane must execute a required public no-op commit and public read-view
observation, with zero patch records, rather than constructing an in-memory
empty observation or admitting an optional/current snapshot. Standard's 8,211 patch records must be below the explicit
16,384 public budget, and a deliberately tiny budget must produce a typed
publication denial.

The owner-correspondence proof must cover all three relation-creation paths:
normal relation creation, relation-aspect creation, and bulk relation creation.
Each path must resolve the exact semantic client key and endpoint references
through the sealed commit result; changed-record order, endpoint matching, or
allocator arithmetic is not evidence.

Phase 1's Foundational/owner-reference contract tests are deliberately
structural: they use deterministic owner-shaped descriptors to pin lowering,
owner affinity, exact bytes, malformed transport, and concrete witness doors.
They are not production-world or currentness evidence. The causal production
compiler gate is `supply_chain_world_compiles_causally_through_public_facades`
in Phase 3; Signal's production cutover and live-basis proof are a Phase 11
gate. A green phase-1 adapter test must never be cited as proof that a copied
descriptor is current, admitted, or able to mutate an owner.

### Phase 4 required subset: immutable commits and fork-only references

Phase 4 is the first causal branch-reference gate, but it is deliberately
narrower than the later MVCC courtroom. The existing Supply Chain compiler
must issue a real owner branch observation for the installed baseline. The
fork API consumes a concrete Proof-backed, fork-only source basis; tests must
not construct a target, branch head, root, retention lease, or expected head
from raw ids, strings, `CommitReference`, snapshots, or oracle state. A Phase
3 descriptive baseline envelope is not sufficient by itself.

The required Phase 4 cases are:

- `immutable_commit_is_not_mutable_branch_reference`: the canonical artifact
  and ordered parentage are immutable while each branch owns a separate
  reference cell, generation line, and local truth version;
- `fork_targets_one_canonical_source_artifact`: Court and Standard forks
  target the exact owner-issued source artifact, allocate no second commit
  envelope, and start local truth version zero;
- `fork_provenance_is_not_target_authority`: source observation and authoring
  provenance are recorded separately; neither a source branch name nor
  provenance payload can operate a target reference;
- `reference_version_and_generation_laws_are_checked`: truth movements advance
  local version plus generation, metadata movements advance generation only,
  and generation/version overflow denies before registry or catalog effects;
- `fork_denials_are_typed_and_residue_free`: stale source generation,
  foreign-runtime/equal-ordinal twin, duplicate target, empty-source policy,
  malformed target, and missing owner basis fail distinctly with no catalog or
  reference-registry mutation;
- `runtime_clone_rebinds_reference_affinity`: a cloned runtime cannot operate
  source-runtime observations even when commit ids and local versions match;
- `canonical_catalog_has_one_artifact`: history readers resolve immutable
  commit identity/parentage from the append-only catalog, while diagnostic
  catalog-latest output cannot select a branch or read root; and
- `retired_branch_authority_is_absent`: compile-pass/fail and source residue
  checks find no public combined `CommitReference`, `BranchHead` authority,
  `ExpectedBranchHead`, optional/ambient branch routing, latest-publication
  currentness fallback, broad `HistoryAuthority::publish_commit`/
  `publish_metadata_only_commit`/`append_index_generations`, generic authority
  bound, or public raw target constructor.

The Phase 4 UI suite must also prove that `AdmittedRelationalForkSourceBasis`
cannot be passed to snapshot/read, transaction, publication, general-retention,
or Phase-6 readmission operations, and that callers cannot construct either
fork-only or general admitted bases from `FoundationalBranchReferenceObservation`
values. The later general admitted-basis UI cases remain phase-gated and are
not counted as Phase 4 evidence.

The Phase 4 mutation controls are equally narrow: duplicate the source
artifact per fork, drop generation from comparison, accept a foreign-runtime
twin, derive a target from authoring provenance, re-open a public raw target
constructor, or route currentness through publication-latest. Each mutation
must make its associated case fail. Phase 4 does **not** claim immutable root
visibility, semantic sibling reads, copy-on-write bytes, repeatable reads,
descriptor readmission, external retention, atomic publication, cancellation,
or reclamation; those cases remain assigned to Phases 5, 6, 9, and 10/12.

The legacy transaction engine may keep its broad runtime borrow while this
subset is implemented, but public transaction construction must already carry
one required `RelationalLegacyBranchBinding`. That binding is privately minted,
runtime-affine, non-serializable, non-forgeable, and accepted only by the
private transition adapter; it is not a renamed `BranchId`, fork basis, or
general admitted read basis, and has no `Default`/`None` path. Optional
`target_branch`, ambient main defaults, and commit-id-only `ExpectedBranchHead`
inputs are forbidden. Only the private adapter may bridge the unchanged legacy
executor. `merge_parent_branches` and all other branch-bearing transaction
inputs are private owner-resolved bindings or non-operational provenance; raw
public `BranchId` selectors cannot resolve a current head. The public
`TransactionOptions` construction path has no `Default`, `Serialize`, or
`Deserialize` fallback.

`runtime_fork_rebinds_branch_cells` is mandatory: `RelationalRuntime::fork`
creates an operational clone with a fresh runtime identity and freshly
rebound branch cells. A source-runtime observation must deny as foreign in the
clone even when commit ordinals and local versions are equal.

`phase4_reference_cost_probe` is mandatory at fan-outs 1, 64, and 512. It
reports fixture setup separately and asserts constant per-fork catalog lookup,
artifact-clone, and branch-cell-contact counts with no branch-population scan.
This is metadata-only scaling evidence; physical bytes, copy-on-write, root
visibility, and full cost slopes remain later phases. Residue checks inspect
exported types and operational call paths rather than banning diagnostic names
such as `protect_branch_heads` by substring alone.

The repository's pre-existing public historical-read, Bridge, application-
commit, and replay adapters are a bounded compatibility inventory for later
consumer cutovers. Phase-4 certification must not call
`admit_application_commit`, Bridge lease admission, `project_version`,
`replay_commit`/`replay_range`, or replay-retention methods. A dedicated residue
check rejects those names under `tests/relational_certification`; the exact
application-commit compatibility proof also snapshots branch cells before and
after lease admission and requires byte-for-byte equality. Compatibility APIs
must validate owner/runtime affinity, must not move a branch cell, and must not
reach fork, transaction, or publication admission. Their eventual exact-basis
replacement is a later-phase requirement, not a new Phase-4 authority lane.

The recovery and currentness court additionally requires exact checkpoint
rebinding to the recovering runtime, fail-closed mismatch handling, metadata
generation idempotence, no empty-parent fallback, exact ordered merge parents,
foreign-runtime identity denial, and a local-empty truth version of zero. The
focused evidence targets are `merge_replay_continuity`, the foreign-runtime
owner-admission case, the local-empty-basis case, the branch-reference
contract/UI suites, and the full `relational_certification` target (currently
91 cases).

### Phase 5 ordinary and scheduled certification lanes

The ordinary certification lane must keep fast mutation-sensitive evidence in
the common path. The real Scale admission court is mandatory evidence but is
scheduled because its honest production installation constructs more than
100,000 live records. The ordinary lane therefore runs:

```text
cargo test -p worth-relational --test relational_certification --no-fail-fast
```

It must report **130 passed, 0 failed, 1 ignored** for the current Phase 5
packet. The single ignored case is not weakened or removed; it is the exact
Scale court below. The scheduled/manual lane runs it on the same source
fingerprint:

```text
cargo test -p worth-relational --test relational_certification \
  scale_invariant_admission::large_runtime_keeps_global_enforcement_and_filters_graph_planning \
  -- --ignored --exact --nocapture --test-threads=1
```

The scheduled court must continue to prove the Scale definition is above the
Large threshold, the installed live snapshot matches the causal definition,
commit and baseline publication retain `Global` enforcement, direct
GraphComposition returns a `Touched` ceiling and invokes its probe exactly
once, ordinary follow-up publication lowers to `Partition`, ordinary commits
do not execute graph probes, and rejected duplicate uniqueness leaves no value,
branch-reference, catalog, or snapshot residue. The scheduled lane is required
before Phase 5 closure and after changes to Scale construction, invariant scale
classification, admission policy, GraphComposition routing, selected-state
uniqueness, or baseline publication. CI owns this lane in the
`worth-relational-scale-certification` scheduled/manual job; CI scheduling
mints no runtime or branch authority.

### Required branch-local MVCC scenarios

All scenarios start from a `CertifiedSupplyChainBaseline`, execute named deltas
through public owner facades, and compare every affected branch to the
independent oracle:

- `immutable_commit_is_not_mutable_branch_reference`: fork Storm and
  Maintenance from the exact contested baseline; each receives an independent
  reference/generation and local version zero while targeting one source
  commit, and moving either reference does not move the other
- `fork_reuses_baseline_without_copy`: Court, Standard, and Scale forks report
  zero materialized entities, relations, authoritative truth bytes, and commit
  envelopes; the ancestor commit/root is unique and exactly retained
- `branch_write_copies_only_touched_regions`: Storm Reroute and Atlas
  Maintenance materialize only their declared persistent regions/root paths;
  unchanged cargo, port, and infrastructure regions remain shared
- `branches_share_history_without_sharing_mutable_fate`: Storm, Maintenance,
  and Medical Hold observe one exact ancestor and independent overlay,
  generation, lifecycle, coordination, and publication state
- `branch_observations_have_no_sibling_crossover`: each branch matches parent
  plus its own accepted deltas; no latest-head lookup, sibling overlay, index,
  or cache can leak another branch's facts
- `blocked_branch_a_does_not_stop_branch_b`: Storm pauses immediately before
  its bounded publication critical section while Maintenance commits; branch-B
  unrelated-wait and branch-A-contact counters remain exactly zero
- `same_relational_reference_has_one_winner`: two Competing Aurora Arrival
  candidates share one expected reference; exactly one publishes and the loser
  reports exact expected/observed mismatch with no partial residue
- `branch_root_publication_is_atomic`: readers around publication observe the
  complete prior or next storage/schema/index/visibility root, never a mixture
- `equal_ordinals_do_not_substitute_authority`: equal local versions,
  generations, digests, or valid old commit ids from different branches or
  runtimes are swapped one axis at a time and deny before effects
- `metadata_movement_advances_reference_generation`: metadata-only movement
  retains declared truth version/root while advancing reference generation
- `boundary_crossing_requires_owner_readmission`: serialized, restored, and
  checkpoint-derived descriptors cannot operate without current owner
  validation
- `retention_follows_live_obligations`: branch, observation, transaction,
  candidate, and external pins retain exact ancestors independently
- `branch_deletion_reclaims_only_unique_unretained_regions`: deleting Storm
  cannot reclaim shared baseline or Maintenance-visible regions and eventually
  exposes only Storm-unique unretained bytes as reclaimable
- `cancellation_obeys_the_linearization_point`: every named pre-effect seam
  leaves no movement/residue, while cancellation after movement returns the
  performed commit
- `seeded_supply_chain_sequences_match_oracle_per_branch`: generated fork/delta/
  observe/retain/archive/delete traces compare truth and ancestry after every
  step and emit a shrinkable reproduction trace
- `branch_local_work_has_branch_local_slopes`: unrelated branches/history do
  not increase selected-branch work, and copy/materialization scales with the
  declared footprint rather than complete world size
- `signal_basis_reuse_is_exact_and_immutable`: consumers share one admitted
  basis with zero graph copy/evaluation/cache duplication; mutation requires an
  owner-issued transition
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

The closure matrix is mandatory; adding assertions without filling a row does
not add proof:

| Production claim | Plausible defect | Supply Chain world/delta | Independent observation | Required consequential evidence |
| --- | --- | --- | --- | --- |
| Baseline is production-reachable | compiler injects ids, roots, or indexes | Operating baseline | public baseline projection versus pure definition | construction report, handle audit, baseline mismatch class |
| Sibling truth is isolated | latest/global root or sibling overlay leaks | Storm + Maintenance + Medical Hold | oracle state per exact branch basis | per-branch truth/ancestry digest and zero crossover report |
| Immutable ancestry is physically shared | fork deep-clones truth/envelopes | 1/64/4,096 contested forks | stable sharing inspection plus commit lookup | zero fork materialization and one ancestor artifact |
| Copy-on-write is proportional | first write clones complete world | Storm, Maintenance, one-cargo hold | touched semantic handles versus stable regions | touched/reused regions and new authoritative bytes |
| Branch publication is independent | global borrow/lock/actor serializes work | paused Storm, active Maintenance | Maintenance completion plus scoped owner counters | zero unrelated wait/contact and exact M1 observation |
| Same-reference CAS is exact | partial/equal-ordinal comparison admits two | competing Aurora arrival | publication outcomes plus history/root projections | one performed commit, exact stale loser, zero residue |
| Visibility is atomic | root components move separately | Storm publication with concurrent readers | whole public observations before/after | no mixed storage/schema/index/visibility tuple |
| Retention preserves shared and unique truth | delete releases too early or never reclaims | retained M1 then release/delete | public retained read plus sharing/reclamation inspection | shared ancestor survives; only unique unretained bytes reclaim |
| Model sequences cover interactions | hand-picked examples miss order defect | seeded fork/delta/lifecycle traces | pure semantic state and ancestry after every step | replayable/shrinkable trace and first divergent observation |
| Signal basis reuse is zero-work | clone/use revalidates or evaluates | 1/64/1,024 holders | exact basis identity plus Signal owner counters | zero graph/evaluation/cache work after admission |
| Component recovery is exact | global stream or foreign artifact selects the wrong owner basis | acknowledged Relational and Signal publications followed by process loss | fresh-process owner observations after PostgreSQL reload/readmission | exact recovered bases and typed cross-branch/cross-generation denials |
| Stored artifacts mint no authority | adapter/SQL descriptor bypasses owner readmission | serialized checkpoint/tail and Signal artifact substitution | owner facade denial before effects | zero head movement and no operational authority from stored bytes |

### Required structural and cost observations

The public read-only inspection surface reports fork-materialized entity,
relation, authoritative-byte, and commit-envelope counts; shared-root
acquisitions; touched/reused regions and new authoritative bytes per
publication; unique canonical commit artifacts; logical branch bytes; unique
physical authoritative bytes; and reclaimable unique bytes. Safe stable region
locators are acceptable; pointer equality alone is not.

Required axes include Supply Chain Court/Standard/Scale profiles, 1/64/4,096
branches, 1/1,024/65,536 retained commits where practical, 1/64/4,096-record
footprints, and 1/64/1,024 immutable basis holders. Fixture compilation cost is
reported separately. At 4,096 unchanged forks, unique truth bytes remain flat
apart from reference/cell/retention metadata. A one-record or one-relation
write cannot materialize the whole world.

### Compiler, mutation, and residue evidence

Consolidated public compile-pass/compile-fail evidence covers raw basis
minting, cross-branch transaction/snapshot/candidate/lease pairing, phase
skipping, prepared-candidate publication, generic `Auth: AuthorityMarker`
governed facades, forged owner markers, cross-owner witness substitution,
restored-descriptor operation, and consumed-witness reuse. Compiler evidence
is limited to current public authority guarantees, not a general API census.

Required sabotage includes private fixture injection, production-derived
oracle output, eager fork clone, whole-world first-write clone, global latest-
root reads, sibling-overlay reuse, duplicated ancestor envelopes, partial root
publication, global publication mutex, omitted retention class, and trusted
restored descriptor. A claim is not closed if its test still passes after its
associated sabotage.

The phase-1 reference suite records these sabotage controls as deferred
release-court obligations: dropping runtime/graph affinity, erasing the Empty
versus Basis tag, omitting generation, reopening a generic authority or
readmission door, or replacing a concrete owner target with optional fields
must turn the corresponding test red. The mutation artifacts are collected
with the production Supply Chain and owner cutover suites rather than being
counted as phase-1 production evidence.

Milestone 9.17.1 may report closed only when the causal world and independent
oracle pass; semantic isolation and physical sharing are both proved; the
ordinary Relational path contains no combined commit/reference authority,
global commit coordinator, broad mutable transaction entry, split visible
root, ambient branch default, eager fork/world clone, or test-only authority
lane; every owner basis is private-minted and readmission-bound; Signal and
Relational use the same Foundational reference grammar; both owners recover
exactly from real PostgreSQL through owner-first readmission; and no component
artifact claims composite product authority.

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
