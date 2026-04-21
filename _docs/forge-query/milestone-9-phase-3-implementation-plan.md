# Milestone 9 Phase 3 Implementation Plan: Policy-Aware Execution Seam Lowering

> **Parent spec:** [milestone-9.md](./milestone-9.md)
>
> **Phase:** Phase 3 only
>
> **Purpose:** consume the Phase 2 `NarrowedPolicyQueryArtifact` and produce
> the policy-aware plan envelopes that every current read, branch read,
> historical read, historical diff, live subscription, optimizer, and delivery
> seam must consume before truth can be touched.

## Governing Context Summaries

- `MENTALITY.md`: the hard problem is the seam, not the happy path. Phase 3
  must make the illegal route unrepresentable so policy cannot be bolted onto
  ordinary execution after the fact.
- `arch_laws.md`: the proof chain must widen one phase at a time. Lowered
  policy-aware plan envelopes must be the only execution inputs, and executors
  must not rediscover legality, projection, relationship proof, or tenant basis
  semantics.
- `perf_laws.md`: policy decisions, topology admission, locality, delivery
  width, live density, and allocation scope belong before execution. Every
  seam needs exact counters proving the authorized width, proof topology, and
  delivery width it admitted.
- `domain_laws.md`: current, branch, historical, diff, live, delivery, optimizer,
  counters, errors, support metadata, and certification are separate
  responsibilities. Do not create one broad `policy_execution.rs` bag.
- `forge_query_vision.md`: one declared query intent must work across reads,
  branches, time travel, diffs, live promotion, and delivery without separate
  policy paths.
- `forge_query_roadmap.md`: Milestone 9 must prove policy masking, tenant
  schema variation, and relationship-proof denial across admitted execution
  modes while keeping store-backed parity and durable continuation explicit
  later work.
- `test-requirements.md`: the Milestone 9 certification suite must emit
  machine-checkable `query_digest`, `policy_digest`, `result_digest`,
  `failure_digest`, and `counter_snapshot` evidence for one-shot, live, and
  historical modes where admitted.
- `milestone-8.md` and `milestone-8-closeout.md`: scopes, templates, saved
  artifacts, view shapes, grouped truth, and identity-aware inspector semantics
  are already canonical. Phase 3 must lower those narrowed artifacts; it must
  not reinterpret composition or view meaning.
- `milestone-9.md`: one-shot reads, live subscriptions, branch reads,
  historical reads, historical diffs, and delivery are one policy surface. A
  mode that cannot consume the same authorized projection, relationship-proof
  admission, and tenant truth/schema basis must deny.
- `milestone-9-phase-1-implementation-plan.md`: policy and tenant admission
  already produce `AdmittedPolicyTenantContext`; Phase 3 must not accept raw
  policy, tenant, branch, or schema inputs.
- `milestone-9-phase-2-implementation-plan.md`: pre-execution narrowing already
  produces `NarrowedPolicyQueryArtifact` and `PolicyAwareOptimizerInput`;
  Phase 3 starts there and may not accept raw canonical query artifacts.

## Adversarial Constraint

Aspect-level masking and Zanzibar-style relationship-proof admission must apply
identically across current one-shot reads, branch reads, historical reads,
historical diffs, live subscriptions, optimizer input, and delivery lowering.

Phase 3 must survive this hostile condition:

> A developer attempts to execute the same canonical query through every
> supported mode, but one mode still starts from an ordinary pre-policy plan,
> another computes a raw diff and scrubs it later, another derives live
> relevance from masked truth, and delivery metadata is rebuilt from a wider
> internal shape. The implementation must make those routes fail compile-time
> or typed admission before truth is touched.

If any supported seam can observe raw canonical projections, raw predicates,
raw ordering/grouping fields, raw relationship-proof callbacks, broader
pre-mask execution plans, unadmitted tenant branch/schema basis, or unmasked
delivery shape, Phase 3 has failed.

## Phase 3 Goal

Phase 3 implements the transformation:

```rust
NarrowedPolicyQueryArtifact
    -> PolicyAwareCurrentPlan
    -> PolicyAwareBranchPlan
    -> PolicyAwareHistoricalPlan
    -> PolicyAwareDiffPlan
    -> PolicyAwareLivePlan
    -> PolicyAwareDeliveryShape
```

It must produce:

- mode-specific policy-aware plan envelopes
- a shared `PolicyAwareExecutionSeam` identity and counter model
- typed denials for raw-plan execution, raw-diff scrub, unmasked live relevance,
  delivery overexposure, unsupported store-backed parity, and execution-mode
  mismatch
- optimizer/lowering entrypoints that consume only
  `NarrowedPolicyQueryArtifact` or `PolicyAwareOptimizerInput`
- certification rows proving seam parity and denial coverage

It must not produce:

- durable saved-query reload
- durable cursor resume
- store-backed restore or snapshot-plus-tail replay
- actual graph relationship-proof truth evaluation
- new policy rule evaluation semantics
- network transport payloads
- post-read redaction

## Hard Boundary

The Phase 3 artifact is a policy-aware plan envelope, not a full runtime engine.

Allowed:

- derive synthetic or existing runtime-backed plan envelopes from
  `NarrowedPolicyQueryArtifact`
- bind existing current/branch/historical/live/diff/delivery metadata to the
  narrowed artifact
- deny unsupported mode combinations before execution
- expose counters proving no wider truth path was admitted

Forbidden:

- fetching entity payloads to decide policy
- evaluating relationship-proof graph truth
- computing raw historical diffs and scrubbing afterward
- deriving live relevance from masked fields
- emitting transport payloads from unmasked internal shape
- claiming store-backed parity while `forge-store` is unfinished

## Proposed Module Topology

Add Phase 3 modules beside the Phase 2 narrowing modules:

```text
crates/forge-query/src/policy_execution_seam/
  mod.rs
  seam.rs
  modes.rs
  counters.rs
  errors.rs
  support.rs
  tests.rs

crates/forge-query/src/policy_plan/
  mod.rs
  current.rs
  branch.rs
  historical.rs
  diff.rs
  optimizer.rs
  artifacts.rs
  tests.rs

crates/forge-query/src/policy_live/
  mod.rs
  admission.rs
  drift.rs
  relevance.rs
  tests.rs

crates/forge-query/src/policy_delivery/
  mod.rs
  shape.rs
  width.rs
  tests.rs

crates/forge-query/src/harness/milestone_nine_certification/
  phase_three.rs
```

Responsibilities:

- `policy_execution_seam` owns shared seam identity, mode vocabulary, failure
  classes, counters, and support profile aggregation.
- `policy_plan` owns current, branch, historical, and diff plan envelopes plus
  optimizer admission.
- `policy_live` owns live admission, authorized live relevance, and policy
  drift disposition.
- `policy_delivery` owns delivery shape and delivery width derived after
  masking.
- the Milestone 9 harness owns only fixtures and certification rows.

Do not mutate the existing raw `planning`, `execution`, `query_context`,
`live`, `historical`, or `view_shape_live` modules into policy-aware surfaces
piecemeal. Phase 3 should wrap or bridge them through explicit policy-aware
plan envelopes first, then later batches can decide which old facades should
be deprecated or made crate-private.

## Batch 1: Execution Seam Inventory And Gate Types

Create shared seam vocabulary before mode-specific lowering.

Required types:

- `PolicyAwareExecutionSeam`
- `PolicyAwareExecutionMode`
- `PolicyAwareExecutionSeamIdentity`
- `PolicyAwareSeamCounters`
- `PolicyAwareExecutionSeamError`
- `PolicyAwareExecutionSeamFailureClass`

Required modes:

- `CurrentRead`
- `BranchRead`
- `HistoricalRead`
- `HistoricalDiff`
- `LiveSubscription`
- `DeliveryShape`
- `OptimizerInput`

Required denial classes:

- `RawCanonicalQueryBypass`
- `RawExecutionPlanBypass`
- `RawDiffScrubForbidden`
- `RawLiveRelevanceForbidden`
- `DeliveryShapeOverexposure`
- `UnsupportedPolicyExecutionMode`
- `StoreBackedPolicyExecutionDeferred`

Rules:

- every seam identity must bind the `NarrowedPolicyQueryArtifact` digest,
  authorized projection digest, relationship-proof digest, tenant truth/schema
  basis digests, narrowed result-shape digest, and execution mode
- every seam counter snapshot must expose authorized projection width,
  relationship-proof topology width, delivery width where relevant, and
  executor semantic rediscovery count
- executor semantic rediscovery must be exactly zero for admitted plan envelopes

## Batch 2: Shared Policy-Aware Plan Artifact

Create the internal parent plan artifact that mode-specific plans embed.

Required types:

- `PolicyAwarePlanCore`
- `PolicyAwarePlanDigest`
- `PolicyAwarePlanCostPosture`
- `PolicyAwarePlanWorkBudget`
- `PolicyAwarePlanLoweringReport`

Required posture variants:

- `RuntimeCurrentBounded`
- `RuntimeBranchBounded`
- `RuntimeHistoricalBounded`
- `RuntimeDiffBounded`
- `LiveSparseAuthorized`
- `DeliveryWidthBounded`
- `DeferredStoreBacked`
- `DeniedWouldWiden`

Required budget dimensions:

- authorized field width
- proof descriptor count
- proof topology width
- tenant/schema basis count
- delivery field width
- live relevance field width
- expected allocation scope
- digest part count

Rules:

- `PolicyAwarePlanCore` is crate-constructed only from
  `NarrowedPolicyQueryArtifact`
- no mode-specific plan may store raw pre-mask projection data
- no mode-specific plan may expose mutable access to authorized projection,
  proof admission, tenant basis, or result-shape digest
- `PolicyAwarePlanLoweringReport` is a proof artifact, not a log

## Batch 3: Current And Branch Read Lowering

Implement runtime-backed current and branch plan envelopes first.

Required types:

- `PolicyAwareCurrentPlan`
- `PolicyAwareBranchPlan`
- `PolicyAwareReadBasis`
- `PolicyAwareReadPlanReport`

Required behavior:

- lower from `NarrowedPolicyQueryArtifact` only
- bind existing runtime/current and branch basis metadata without accepting raw
  tenant or branch strings
- preserve identical authorized projection and relationship-proof admission
  across current and branch modes when only basis changes
- deny branch execution if the narrowed artifact's branch access digest does
  not match the requested branch basis
- produce read-plan counters without reading truth

Tests:

- current plan consumes narrowed artifact and emits seam identity
- branch plan changes only basis identity when policy/proof/projection are the
  same
- mismatched branch basis denies before execution
- raw `ExecutionPlanBundle` cannot be passed as a policy-aware current plan

## Batch 4: Historical Read And Historical Diff Lowering

Implement runtime-backed historical plan shells and diff plan shells. This is
not store-backed restore.

Required types:

- `PolicyAwareHistoricalPlan`
- `PolicyAwareHistoricalBasis`
- `PolicyAwareDiffPlan`
- `PolicyAwareDiffBasisPair`
- `PolicyAwareDiffScrubDisposition`

Required disposition variants:

- `AuthorizedDeltaOnly`
- `DeniedRawDeltaWouldLeak`
- `DeferredStoreBackedHistoricalParity`

Rules:

- historical read plans consume the same narrowed result-shape and authorized
  projection identity as current reads
- diff plans derive delta shape from authorized projection before any delta
  payload exists
- raw diff computation followed by policy scrubbing is denied
- historical store-backed materialization stays `DeferredStoreBacked`
- unsupported historical or diff basis classes deny rather than silently
  falling back to current read

Tests:

- historical plan binds the same policy/proof/projection digests as current
  plan
- diff plan over unauthorized aspect change emits no raw unauthorized delta
  dependency
- raw diff scrub attempt is typed denial
- store-backed historical policy plan is explicit deferred debt

## Batch 5: Live Subscription Admission And Drift

Implement policy-aware live plan admission without maintaining actual live
state beyond existing runtime-backed shells.

Required types:

- `PolicyAwareLivePlan`
- `PolicyAwareLiveRelevanceContract`
- `PolicyDriftDisposition`
- `PolicyLiveDensityPosture`
- `PolicyLiveAdmissionReport`

Required drift variants:

- `NoChange`
- `FreshAdmissionFromCheckpoint`
- `FullRestartDebt`

Required density variants:

- `SparseDelta`
- `BurstReadmission`
- `DenseRestartDebt`

Rules:

- live relevance is derived from `AuthorizedProjectionArtifact` and
  `PolicyInfluenceSet`, never from raw canonical projection fields
- masked live relevance without a sealed non-disclosing witness denies
- policy or tenant epoch drift terminates or re-admits; it may not reinterpret
  cached wider truth
- dense live churn must be explicit debt or denial, not sparse-path execution

Tests:

- live plan relevance only includes authorized fields
- masked live relevance denies before admission
- policy epoch drift produces `FreshAdmissionFromCheckpoint` or
  `FullRestartDebt`
- sparse-to-burst posture changes are counter-visible

## Batch 6: Delivery Shape And Width Lowering

Implement delivery metadata derived after masking.

Required types:

- `PolicyAwareDeliveryShape`
- `PolicyAwareDeliveryDigest`
- `DeliveryWidthClass`
- `PolicyAwareDeliveryReport`

Required width classes:

- `ScalarDetail`
- `NarrowCollection`
- `GroupedDelta`
- `DiffDelta`
- `DeniedWidthInflation`

Rules:

- delivery shape uses `narrowed_result_shape_digest`, not canonical pre-mask
  result shape
- delivery metadata may not expose masked field names, placeholder fields,
  denied branch/tenant structure, or raw proof topology internals
- delivery width is counted before any payload emission
- a width overflow denies before delivery shape construction succeeds

Tests:

- delivery digest matches caller-visible narrowed result shape
- placeholder masking remains impossible through delivery shape
- over-wide delivery class denies with exact counters
- grouped/view-shaped delivery consumes existing Milestone 8 view metadata
  only through the narrowed artifact surface

## Batch 7: Optimizer Input And Raw Planner Denial

Harden optimizer and planner boundaries so no policy-aware lane can start from
an ordinary pre-policy plan.

Required behavior:

- policy-aware optimizer entrypoints consume `PolicyAwareOptimizerInput` or
  `NarrowedPolicyQueryArtifact`
- policy-aware lowering never accepts `CanonicalQueryArtifact`,
  `ValidatedQueryBundle`, `ExecutionPlanBundle`, or raw `LiveQueryPlan`
  directly
- existing ordinary planner surfaces may continue to exist for non-policy
  lanes, but they must not claim Milestone 9 policy-aware support
- attempts to optimize then apply policy as redaction, delivery filtering,
  diff scrubbing, or live suppression deny

Compile-fail targets:

- `policy_current_plan_requires_narrowed_artifact.rs`
- `policy_branch_plan_requires_narrowed_artifact.rs`
- `policy_historical_plan_requires_narrowed_artifact.rs`
- `policy_diff_plan_requires_narrowed_artifact.rs`
- `policy_live_plan_requires_narrowed_artifact.rs`
- `policy_delivery_requires_narrowed_artifact.rs`
- `raw_execution_plan_cannot_be_policy_current_plan.rs`
- `validated_bundle_cannot_be_policy_optimizer_input.rs`

## Batch 8: Performance Encoding

Phase 3 performance must be encoded as structural counters and plan budgets.

Required counters:

- policy seam admission count
- policy seam denial count
- authorized projection width
- relationship-proof topology width
- tenant/schema basis count
- current/branch/historical/diff/live/delivery plan count
- live relevance field width
- delivery width
- plan digest part count
- executor semantic rediscovery count
- raw plan bypass denial count
- raw diff scrub denial count
- raw live relevance denial count
- delivery overexposure denial count
- store-backed deferred count

Required performance proof obligations:

- every admitted seam has `executor_semantic_rediscovery_count == 0`
- plan lowering walks mode descriptors once and records exact inspected counts
- digest part counts stay within declared budgets
- delivery width and live relevance width are bounded before payload or patch
  emission
- small/medium/large fixture rows prove counter slopes using exact structural
  counters, not elapsed time

## Batch 9: Saved Query, Scope, Template, And View-Shape Integration

Phase 3 must preserve the Phase 2 rule that composition has already lowered.

Required behavior:

- saved-query exact reuse may reuse a narrowed artifact only when Phase 2 reuse
  classified it as exact
- policy-aware plan envelopes bind the saved-query digest when present, but do
  not trust saved artifacts as execution authority
- scope/template/view-shape metadata may influence delivery/live mode only
  through the narrowed artifact and its validation report
- identity-aware inspector and grouped view-shape semantics consume
  `PolicyAwareDeliveryShape` and `PolicyAwareLivePlan`, not raw view payloads

Tests:

- direct, scope, template, and saved exact reuse lower to equal policy-aware
  current plan digests when their narrowed artifact is equal
- saved policy drift requires fresh narrowing before plan lowering
- grouped view delivery width is counted after policy masking
- identity-aware inspector delivery preserves identity classification without
  exposing masked shape

## Batch 10: Support Profile Honesty

Extend Milestone 9 support metadata with Phase 3 surfaces.

Required support statuses:

- `PolicyAwareCurrentPlanVerified`
- `PolicyAwareBranchPlanVerified`
- `PolicyAwareHistoricalPlanRuntimeBackedVerified`
- `PolicyAwareHistoricalDiffRuntimeBackedVerified`
- `PolicyAwareLiveAdmissionVerified`
- `PolicyAwareDeliveryShapeVerified`
- `PolicyAwareOptimizerInputVerified`
- `PolicyAwareStoreBackedExecutionDeferred`
- `DurablePolicyCursorDeferred`
- `DurablePolicyArtifactReloadDeferred`

Rules:

- runtime-backed Phase 3 support may be verified
- store-backed parity must remain deferred until Milestone 10
- durable saved-query/cursor/reload semantics must remain deferred until
  Milestone 11
- support metadata must be derived from executable admission behavior or a
  registry, not doc prose

## Batch 11: Unit Tests

Add focused unit tests close to the new subdomains.

`policy_execution_seam` tests:

- seam identity binds narrowed artifact, projection, proof, tenant/schema, and
  mode digests
- each mode emits exact seam counters
- raw seam bypass attempts return typed denial

`policy_plan` tests:

- current, branch, historical, and diff plans lower from narrowed artifact
- branch mismatch denies before execution
- historical store-backed request is deferred
- raw diff scrub is denied
- optimizer input cannot include masked fields

`policy_live` tests:

- live relevance is authorized-field-only
- masked relevance denies
- policy drift produces readmission or debt disposition
- dense unsupported lane is explicit debt

`policy_delivery` tests:

- delivery shape derives from narrowed result shape
- delivery width overflow denies
- delivery metadata does not expose masked placeholder structure
- grouped and inspector delivery retain Milestone 8 semantics through the
  policy-aware shape

## Batch 12: Certification Harness Rows

Extend `crates/forge-query/src/harness/milestone_nine_certification/` with
Phase 3 rows. Keep Phase 1 and Phase 2 rows intact.

Required canonical rows:

- `policy-aware-current-plan-lowering`
- `policy-aware-branch-plan-lowering`
- `policy-aware-historical-plan-runtime-backed-lowering`
- `policy-aware-diff-plan-runtime-backed-lowering`
- `policy-aware-live-admission`
- `policy-aware-delivery-shape-derived-after-mask`
- `policy-aware-optimizer-input-only`
- `policy-execution-seam-parity`

Required rejection/debt rows:

- `raw-current-plan-bypass-forbidden`
- `raw-branch-plan-bypass-forbidden`
- `raw-historical-plan-bypass-forbidden`
- `raw-diff-scrub-forbidden`
- `masked-live-relevance-forbidden`
- `delivery-shape-overexposure-forbidden`
- `store-backed-policy-execution-deferred`
- `durable-policy-cursor-deferred`
- `phase-three-no-truth-touch-before-plan-admission`

Required bundle fields:

- `query_digest`
- `policy_digest`
- `tenant_truth_basis_digest`
- `tenant_schema_basis_digest`
- `authorized_projection_digest`
- `relationship_proof_digest`
- `narrowed_artifact_digest`
- `policy_plan_digest`
- `policy_execution_seam_digest`
- `delivery_digest` where relevant
- `failure_digest`
- `counter_snapshot`

Certification must include hostile lanes and at least one parity row comparing
current, branch, historical, diff, live, and delivery seams for equal policy
inputs where their semantics are intentionally equal.

## Batch 13: Verification Commands

Run these incrementally while implementing:

```powershell
cargo test -p forge-query policy_execution_seam --lib
cargo test -p forge-query policy_plan --lib
cargo test -p forge-query policy_live --lib
cargo test -p forge-query policy_delivery --lib
cargo test -p forge-query milestone_nine_certification --lib
cargo test -p forge-query --test phase_boundaries_compile_fail
cargo test -p forge-query
```

If module names change during implementation, update this plan before closing
the batch so the verification recipe stays executable.

## Non-Goals For This Batch

- no durable saved-query reload
- no durable cursor resume
- no store-backed restore or execution parity
- no actual graph relationship-proof truth evaluation
- no new policy rule engine
- no network transport implementation
- no post-read redaction
- no store-backed historical reconstruction
- no durable subscription checkpoint reload
- no generic policy wrapper around existing raw execution paths

## Done Criteria

This batch is complete when:

- every Phase 3 policy-aware mode starts from `NarrowedPolicyQueryArtifact`
- current, branch, runtime-backed historical, runtime-backed diff, live, and
  delivery plan envelopes exist with shared seam identity
- raw canonical query, raw validated bundle, raw execution plan, raw live plan,
  and raw diff artifacts cannot masquerade as policy-aware inputs
- live relevance and delivery shape derive only from authorized projection and
  validated influence
- store-backed execution and durable continuation remain explicit deferred debt
- support metadata reports verified/deferred Phase 3 surfaces honestly
- compile-fail tests block bypass paths
- certification emits canonical bundles with stable digests and exact counters
- `cargo test -p forge-query` passes

## Self-Check

This plan solves a real structural problem because it targets the exact seam
where policy leaks happen: mode-specific lowering after pre-execution narrowing.

The adversarial constraint is load-bearing because it forbids the naive
"ordinary plan plus policy wrapper" implementation across current, branch,
historical, diff, live, and delivery paths.

The plan preserves authority boundaries because `forge-query` owns policy-aware
lowering artifacts while lower runtimes remain authoritative for truth,
history, live execution, and store persistence.

The plan defines proof obligations rather than implementation chores because
each batch names the types, denial lanes, compile-fail boundaries, counters,
and certification rows required to close the seam.

A competent engineer can map this plan into modules and tests without inventing
architecture mid-implementation.

This plan belongs now because Phase 1 admitted policy/tenant basis and Phase 2
created the narrowed artifact; the next honest step is making every execution
mode consume that artifact before Milestone 10 extends the same semantics to
store-backed parity.
