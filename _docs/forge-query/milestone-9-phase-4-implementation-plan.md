# Milestone 9 Phase 4 Implementation Plan: Certification Closure And Runtime Parity Hardening

> **Parent spec:** [milestone-9.md](./milestone-9.md)
>
> **Phase:** Phase 4 only
>
> **Purpose:** close the runtime-backed Milestone 9 certification gap by making
> the concrete policy/tenant fixture, hidden-influence denials, live
> drift/density behavior, delivery-width honesty, scale-slope counters, and
> composed/saved/view-shaped parity mechanically visible.

## Governing Context Summaries

- `MENTALITY.md`: protect the system from plausible happy-path policy support;
  the plan must state the adversarial condition first and enforce it with
  types, counters, and certification instead of convention.
- `arch_laws.md`: protect proof progression. Phase 4 must consume the
  Phase 1-3 artifacts and widen proof only through typed diagnostics and
  certification bundles; it must not re-open raw query, raw plan, or raw view
  payload access.
- `perf_laws.md`: protect boundedness and cost honesty. Policy closure needs
  structural counters and scale-sensitive slopes, not elapsed-time guesses or
  one tiny fixture row.
- `domain_laws.md`: protect responsibility boundaries. Fixture modeling,
  influence classification, live drift, delivery width, support truth, and
  certification rows must remain separate subdomains.
- `forge_query_vision.md`: protect the product thesis that policy masking,
  tenant scoping, live promotion, history, saved queries, scopes, and view
  shapes are one typed query model rather than separate APIs.
- `forge_query_roadmap.md`: protect sequencing. Milestone 9 may close the
  runtime-backed semantic surface now, while store-backed execution remains
  Milestone 10 and durable saved/cursor/reload remains Milestone 11.
- `test-requirements.md`: protect certification-grade evidence. The Milestone 9
  suite must emit machine-checkable query, policy, result, failure, and counter
  artifacts across masked/unmasked policy, tenant schema variants, relationship
  proofs, and admitted one-shot/live/historical modes.
- `milestone-8-closeout.md`: protect the already-closed composition,
  ephemeral saved-query, grouped view, and identity-aware inspector semantics.
  Phase 4 must govern those surfaces through policy artifacts rather than
  rebuilding or bypassing them.

## Adversarial Constraint

Phase 4 must survive this hostile condition:

> The current policy-aware implementation passes direct current-read tests, but
> leaks or drifts when the same query is expressed as an EmployeeRecord fixture
> with salary masking, grouped/view-shaped membership, cursor/aggregation
> influence, live policy drift, saved-query reuse, and scope/template parity.
> One path derives delivery width from a wider shape, another keeps masked
> placeholders, another allows live relevance to watch masked salary changes,
> and another claims bounded policy work without scale-slope evidence.

If any admitted runtime-backed path lacks machine-checkable evidence that it
uses the same narrowed artifact, hidden-influence rules, tenant basis,
relationship-proof admission, delivery shape, and performance counters as the
direct one-shot lane, Phase 4 has failed.

## Current Implementation Baseline

Already implemented:

- Phase 1 policy/tenant admission artifacts and support profile.
- Phase 2 authorized projection, mask snapshot, relationship-proof admission,
  policy-aware validation report, narrowed artifact, and saved narrowing reuse
  classification.
- Phase 3 policy-aware current/branch/runtime-historical/runtime-diff plans,
  live admission, delivery shape, optimizer input, seam support profile, and
  store/durable handoff denials.
- Milestone 9 certification matrix rows through Phase 3, including typed
  rejection rows for raw plan bypass, raw diff scrub, masked live relevance,
  delivery overexposure, store-backed deferral, and durable overclaim.

Still not closed enough for Milestone 9:

- the concrete EmployeeRecord fixture is not first-class
- hidden influence is not exhaustive for aggregation, cursor, and view
  membership
- placeholder masking is mostly implied by delivery shape behavior instead of
  a named denial lane
- live drift and live density posture are too shallowly certified
- delivery-width classes are not certified across scalar, collection, grouped,
  and diff delivery
- policy scale-slope evidence does not yet exist
- direct/scope/template/saved/view-shaped parity is not certified through the
  policy-aware execution seam as one closeout matrix

## Phase 4 Goal

Phase 4 implements the transformation:

```text
Phase 1-3 policy artifacts
    -> concrete EmployeeRecord certification fixture
    -> exhaustive hidden influence and placeholder denial evidence
    -> live drift/density and delivery width evidence
    -> policy scale-slope certification evidence
    -> direct/scope/template/saved/view-shaped policy parity rows
    -> closeout-ready runtime-backed Milestone 9 certification
```

It must produce executable evidence, not prose, for the remaining runtime-backed
closeout claims.

It must not produce:

- store-backed policy-aware execution
- durable saved-query reload
- durable cursor resume
- restart-stable subscription metadata
- new policy rule evaluation semantics
- actual relationship graph traversal execution

## Proposed Module Topology

Prefer adding focused modules under existing Phase 1-3 subdomains:

```text
crates/forge-query/src/policy_certification/
  mod.rs
  employee_record.rs
  scale.rs
  parity.rs
  tests.rs

crates/forge-query/src/authorized_projection/
  influence.rs       # extend existing purpose vocabulary
  tests.rs

crates/forge-query/src/policy_live/
  drift.rs           # extend existing drift/density evidence
  tests.rs

crates/forge-query/src/policy_delivery/
  width.rs           # extend existing width posture evidence
  tests.rs

crates/forge-query/src/harness/milestone_nine_certification/
  mod.rs             # add Phase 4 rows using policy_certification evidence
```

`policy_certification` owns fixture and certification-only evidence. It must
not become a new policy engine, query planner, live engine, or delivery engine.

## Batch 1: Concrete EmployeeRecord Fixture

Create a named certification fixture that replaces abstract policy rows with
concrete leakage paths.

Required types:

- `EmployeeRecordPolicyFixture`
- `EmployeeRecordTenantVariant`
- `EmployeeRecordQueryFamily`
- `EmployeeRecordPolicyScenario`
- `EmployeeRecordCertificationBundle`

Required fixture shape:

- public fields:
  - `employee_id`
  - `display_name`
  - `department`
  - `manager_id`
- masked field:
  - `compensation.salary_band`
- tenant variants:
  - `TenantAlpha` with salary present but masked for ordinary users
  - `TenantBeta` with distinct tenant schema basis and an incompatible or
    differently shaped compensation field
- query families:
  - direct detail
  - collection ordered by `display_name`
  - hostile filter by `salary_band`
  - hostile order by `salary_band`
  - hostile group by `salary_band`
  - hostile aggregation over `salary_band`
  - hostile cursor placement by `salary_band`
  - grouped view membership over `salary_band`
  - live relevance over `salary_band`
  - saved-query reuse under changed policy or tenant schema
  - runtime-backed historical read over the same admitted masked basis

Rules:

- fixture construction must not read truth
- fixture outputs are certification artifacts, not production payloads
- every fixture lane must emit stable query, policy, tenant, schema,
  projection, proof, result-shape, delivery, failure, and counter evidence
  where applicable

Tests:

- EmployeeRecord masked salary projection is removed, not placeholdered
- TenantAlpha and TenantBeta produce distinct tenant/schema basis digests
- the fixture bundle is deterministic across repeated construction

## Batch 2: Exhaust Hidden Influence Coverage

Extend hidden influence from predicate/order/grouping/template/derived coverage
to every influence purpose named by the Milestone 9 spec.

Required additions:

- `PolicyInfluencePurpose::Aggregation`
- `PolicyInfluencePurpose::Cursor`
- `PolicyInfluencePurpose::ViewMembership`
- purpose-specific counters:
  - `masked_aggregation_use_denial_count`
  - `masked_cursor_use_denial_count`
  - `masked_view_membership_use_denial_count`

Rules:

- masked aggregation, cursor, and view-membership influence deny by default
- an admitted non-disclosing witness for one purpose may not satisfy another
  purpose
- influence checks happen before narrowing succeeds and before any execution
  plan exists

Tests:

- masked aggregation influence denies exactly once
- masked cursor influence denies exactly once
- masked view-membership influence denies exactly once
- non-disclosing predicate permission does not admit aggregation, cursor, or
  view-membership influence

## Batch 3: Placeholder Masking Denial

Make placeholder redaction a named denial path rather than an implication of
ordinary delivery narrowing.

Required types:

- `PolicyPlaceholderMaskingRequest`
- `PolicyPlaceholderMaskingDenial`
- `PolicyPlaceholderMaskingFailureClass`

Rules:

- a caller-visible field that exists only to hold `None`, `REDACTED`, empty
  payloads, or any other policy placeholder must deny
- denial must occur before delivery-shape construction succeeds
- denial counters must distinguish placeholder redaction from ordinary delivery
  overexposure

Tests:

- placeholder result shape over `compensation.salary_band` denies
- placeholder denial has a distinct failure digest from delivery width
  inflation
- delivery over authorized visible fields remains admitted

## Batch 4: Live Drift And Density Evidence

Strengthen live policy semantics beyond the current authorized-field-only
admission check.

Required types:

- `PolicyLiveEpochBinding`
- `PolicyLiveDriftEvidence`
- `PolicyLiveDensityEvidence`
- `PolicyLiveReadmissionDecision`

Required counters:

- `policy_epoch_drift_readmission_count`
- `tenant_basis_drift_readmission_count`
- `policy_sparse_to_burst_readmission_count`
- `policy_dense_restart_debt_count`

Rules:

- `NoChange` admits only when policy and tenant epoch digests match
- `FreshAdmissionFromCheckpoint` must bind a fresh narrowed artifact digest
  and may not reinterpret cached wider truth
- `FullRestartDebt` remains explicit debt, not sparse execution
- `BurstReadmission` is counter-visible and distinct from `SparseDelta`
- `DenseRestartDebt` denies or debts before live admission, never silently
  follows sparse maintenance

Tests:

- policy epoch drift produces readmission evidence with a new narrowed digest
- tenant basis drift produces readmission evidence with a new tenant basis
  digest
- sparse-to-burst transition increments exactly one burst counter
- dense unsupported lane increments dense restart debt and does not produce a
  sparse live plan

## Batch 5: Delivery Width Class Honesty

Certify delivery width across the width classes named in Phase 3 instead of
only a scalar happy path and denial path.

Required evidence:

- one admitted `ScalarDetail` delivery
- one admitted `NarrowCollection` delivery
- one admitted `GroupedDelta` delivery
- one admitted `DiffDelta` delivery
- one denied width inflation lane

Rules:

- delivery width must be counted before payload emission
- grouped delivery width is derived from grouped/view metadata only through the
  narrowed artifact surface
- diff delivery width is derived from authorized delta shape, not raw diff
  payload
- delivery digest must bind the narrowed result-shape digest

Tests:

- scalar, collection, grouped, and diff delivery classes produce distinct
  width-class digests where semantics differ
- width overflow denies with `delivery_width_inflation_denial_count == 1`
- grouped delivery does not expose masked group key structure

## Batch 6: Policy Scale-Slope Evidence

Add structural scale evidence for small, medium, and larger EmployeeRecord
fixture sizes.

Required types:

- `PolicyScaleFixtureSize`
- `PolicyScaleCounterSnapshot`
- `PolicyScaleSlopeDigest`
- `PolicyScaleSlopeReport`

Required counters:

- authorized projection width
- relationship proof descriptor count
- relationship proof topology width
- delivery width
- live relevance width
- allocation scope count
- digest part count
- executor semantic rediscovery count

Rules:

- evidence uses exact structural counters, not elapsed time
- admitted slopes must remain constant or linear in the declared fixture
  dimension
- any executor semantic rediscovery count above zero is a certification
  failure
- per-row allocation in an admitted hot path is denied or marked explicit debt

Tests:

- small/medium/large fixtures produce stable semantic digests for identical
  policy meaning
- scale counter slopes match declared contracts
- slope drift changes `policy_scale_counter_slope_digest`
- executor rediscovery remains zero across all fixture sizes

## Batch 7: Direct, Scope, Template, Saved, And View-Shaped Parity

Prove that Milestone 8 construction and view surfaces obey the same
policy-aware narrowing and execution-seam rules as direct construction.

Required evidence:

- direct query narrowed artifact digest
- scope-composed narrowed artifact digest
- template-instantiated narrowed artifact digest
- saved exact-reuse narrowed artifact digest
- saved drift/fresh-freeze-required classification
- table/detail delivery shape digest
- grouped view delivery shape digest
- identity-aware inspector delivery shape digest

Rules:

- equivalent direct/scope/template/saved-exact lanes lower to the same
  policy-aware plan core where their narrowed artifact is equal
- saved policy or tenant drift requires fresh narrowing before plan lowering
- grouped view delivery width is counted after masking
- identity-aware inspector delivery preserves identity classification without
  exposing masked shape
- unsupported policy/workflow/stream composition denies before execution

Tests:

- direct/scope/template/saved exact reuse parity row compares equal on policy
  artifacts
- saved policy drift produces fresh-freeze-required or illegal drift evidence
  before plan lowering
- grouped view delivery hides masked grouping influence unless admitted by a
  purpose-specific witness
- identity-aware inspector keeps Milestone 7 identity classification typed
  under policy delivery

## Batch 8: Certification Matrix Rows

Extend `harness/milestone_nine_certification` with Phase 4 rows while keeping
Phase 1-3 rows intact.

Required canonical rows:

- `employee-record-fixture-policy-basis`
- `tenant-alpha-versus-tenant-beta-schema`
- `masked-versus-unmasked-policy-parity`
- `live-policy-epoch-drift-readmission`
- `live-policy-density-posture-honesty`
- `delivery-width-class-honesty`
- `policy-scale-slope-honesty`
- `policy-direct-scope-template-saved-parity`
- `policy-view-shape-delivery-parity`
- `policy-identity-aware-inspector-parity`

Required rejection rows:

- `masked-placeholder-shape-forbidden`
- `masked-aggregation-without-witness-forbidden`
- `masked-cursor-without-witness-forbidden`
- `masked-view-membership-without-witness-forbidden`
- `policy-per-row-allocation-forbidden`
- `policy-cross-tenant-fanout-forbidden`
- `saved-query-policy-bypass-forbidden`
- `unsupported-policy-workflow-composition-forbidden`

Required bundle fields:

- `employee_fixture_digest`
- `policy_scale_counter_slope_digest`
- `live_drift_evidence_digest`
- `delivery_width_class_digest`
- `composition_policy_parity_digest`
- `view_shape_policy_parity_digest`
- `placeholder_denial_digest`

## Batch 9: Compile-Fail Boundaries

Add compile-fail tests for the new closure surfaces.

Targets:

- `policy_employee_fixture_cannot_fabricate_bundle.rs`
- `policy_aggregation_influence_requires_policy_influence_set.rs`
- `policy_cursor_influence_requires_policy_influence_set.rs`
- `policy_view_membership_influence_requires_policy_influence_set.rs`
- `policy_placeholder_masking_cannot_construct_delivery.rs`
- `policy_live_drift_evidence_constructor_private.rs`
- `policy_scale_slope_report_constructor_private.rs`
- `policy_saved_exact_reuse_cannot_skip_narrowing.rs`

Rules:

- external crates cannot construct certification proof bundles directly
- external crates cannot fabricate live drift, scale slope, or placeholder
  denial evidence
- saved exact reuse cannot masquerade as a policy-aware plan input without a
  narrowed artifact

## Batch 10: Support And Diagnostics Honesty

Update support metadata and diagnostics so Phase 4 certification claims cannot
outrun executable evidence.

Required support statuses:

- `EmployeeRecordFixtureVerified`
- `HiddenInfluenceExhaustivenessVerified`
- `PlaceholderMaskingDenialVerified`
- `LiveDriftReadmissionVerified`
- `DeliveryWidthClassVerified`
- `PolicyScaleSlopeVerified`
- `PolicyCompositionParityVerified`
- `StoreBackedPolicyExecutionDeferred`
- `DurablePolicyArtifactsDeferred`

Rules:

- support profile truth must derive from executable certification rows or an
  explicit registry
- unsupported or unimplemented policy/view/workflow combinations must advertise
  denial/debt, not partial support
- diagnostics must identify which phase failed: authority admission,
  narrowing, execution seam, live drift, delivery width, or certification
  slope

Tests:

- support profile digest changes when any Phase 4 required surface is removed
- diagnostics localize placeholder, hidden influence, live drift, delivery
  width, and scale-slope failures without reading logs
- store/durable support remains deferred after Phase 4

## Verification Commands

Run incrementally:

```powershell
cargo fmt -p forge-query
cargo test -p forge-query authorized_projection --lib
cargo test -p forge-query policy_live --lib
cargo test -p forge-query policy_delivery --lib
cargo test -p forge-query policy_certification --lib
cargo test -p forge-query milestone_nine_certification --lib
cargo test -p forge-query --test phase_boundaries_compile_fail -- --test-threads=1
cargo test -p forge-query
```

If module names change during implementation, update this plan in the same
batch so the verification recipe remains executable.

## Non-Goals

- no store-backed execution parity
- no store-backed historical restore
- no durable saved-query reload
- no durable cursor resume
- no restart-stable subscription metadata
- no new policy rule engine
- no open-ended relationship proof graph traversal
- no network transport or delivery serialization
- no post-read redaction

## Done Criteria

Phase 4 is complete when:

- the EmployeeRecord fixture exists and is deterministic
- hidden influence denial covers predicate, ordering, grouping, aggregation,
  cursor, view membership, derived result, template predicate, and live
  relevance
- placeholder masking has a distinct typed denial path
- live drift and density posture are certified with exact counters
- delivery-width classes are certified across scalar, collection, grouped, and
  diff delivery
- policy scale-slope evidence exists for small/medium/large fixtures
- direct, scope, template, saved exact reuse, grouped view, and
  identity-aware inspector policy parity rows exist
- compile-fail guards prevent fabrication or bypass of new proof artifacts
- support metadata reports Phase 4 verified surfaces and keeps store/durable
  debt explicit
- `cargo test -p forge-query` passes

## Self-Check

- This plan solves a real structural problem: Milestone 9 currently has the
  policy-aware seam, but it still needs concrete closeout evidence for the
  surfaces most likely to leak under composition, live maintenance, delivery,
  and scale.
- The adversarial constraint is load-bearing because it targets the exact
  failure modes that happy-path Phase 1-3 tests could miss.
- Authority boundaries are preserved: `forge-query` owns certification,
  narrowing, delivery, and support truth; lower runtimes still own truth,
  history, live execution, and persistence.
- The plan defines proof obligations, not only tasks: every batch names
  required evidence, counters, rows, and compile-fail boundaries.
- A competent engineer can map the plan into honest modules and tests without
  inventing a new architecture.
- The plan belongs before Milestone 10 because store-backed parity should
  extend a fully certified runtime-backed policy surface.

