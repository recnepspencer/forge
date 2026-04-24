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
- store-backed durability, pushdown, and artifact portability
- blob-backed delivery and upload-associated query semantics

Those are adversarial surfaces. They need certification tests, not just feature
checks.

## Global Adversarial Constraint

The query test suite must prove the following:

> Under alternate builder paths, schema variation, branch divergence,
> historical replay, live-update churn, policy masking, tenant-scoped schema
> drift, lineage ambiguity, store/runtime path variation, and restart/resume
> pressure, the same canonical query intent must produce the same query
> meaning, the same typed result/delivery contract, and the same machine-
> checkable explanation of why results changed, unless the scenario is
> intentionally semantically different or intentionally rejected.

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

## Milestone 1 Named Certification Suites

### 1. Canonical Query Normalization Parity Test

Purpose

Prove that equivalent query intent expressed through different admitted
construction paths produces the same canonical query artifact.

Scenario

- build equivalent detail and collection queries through at least:
  - direct construction
  - builder/combinator composition
  - scope/template expansion where admitted at this milestone
- vary helper ordering and host binding descriptors without changing query
  meaning

Must verify

- equivalent query construction yields identical canonical query digests
- result-shape meaning is preserved across equivalent construction paths
- host-local helper layering does not create alternate canonical meaning

Required verification output

- `query_digest`
- `result_shape_digest`
- `canonicalization_report`
- `counter_snapshot`

Pass condition

Equivalent construction paths normalize to identical canonical query meaning.

## Milestone 2 Named Certification Suites

### 2. Schema-Aware Rejection And Projection Legality Test

Purpose

Prove that invalid predicates, projection requests, traversal clauses, and
structured-content queries fail before execution.

Scenario

- attempt legal and illegal queries involving:
  - unknown aspects
  - incompatible field predicates
  - illegal traversal edges
  - invalid result-shape bindings
  - structured-content projections/predicates outside schema allowance
  - workflow-aware predicates with invalid context/shape

Must verify

- illegal queries fail during validation rather than planning/execution
- legal queries lower deterministically after validation
- no silent whole-entity widening occurs on invalid or unsupported requests

Required verification output

- `query_digest`
- `failure_digest`
- `validation_rejection_matrix`
- `counter_snapshot`

Pass condition

Schema-invalid and structurally illegal queries fail early, typed, and without
semantic widening.

## Milestone 3 Named Certification Suites

### 3. Planner / Executor / Binding Parity Test

Purpose

Prove that canonical query meaning survives planning, execution, and admitted
type-bound binding paths.

Scenario

- execute the same canonical queries through:
  - direct runtime-backed execution
  - independently re-planned runtime-backed execution
  - admitted type-bound binding descriptors
  - store-backed execution where the store path is already admitted

Must verify

- equivalent runs produce identical plan and result semantics
- executor does not rediscover planner-owned legality or scope decisions
- type-bound descriptors round-trip to the same canonical plan
- intentionally different admitted runtime route shapes produce distinct
  canonical plan/result evidence
- admitted runtime/store path pairs compare equal

Required verification output

- `query_digest`
- `plan_digest`
- `result_digest`
- `basis_digest`
- `counter_snapshot`

Pass condition

Planning and execution paths are parity-safe for the same canonical query and
basis.

## Milestone 4 Named Certification Suites

### 4. Collection, Cursor, Rollup, And CDC Shape Parity Test

Purpose

Prove that large-surface query behavior remains query-shaped, bounded, and
basis-honest.

Scenario

- run collection queries with:
  - ordering
  - cursor advancement
  - bounded traversal/materialization
  - aggregation/rollups
  - query-time derived fields
  - CDC-shaped output rendering

Must verify

- cursor advancement is stable for one basis
- traversal stays within declared scope
- rollups and derived fields remain basis-honest
- CDC-shaped output matches ordinary query meaning for the same query

Required verification output

- `query_digest`
- `result_digest`
- `delivery_digest`
- `cursor_progress_report`
- `counter_snapshot`

Pass condition

Collection semantics, derived result semantics, and CDC-shaped output remain
canonical and bounded.

## Milestone 5 Named Certification Suites

### 5. Live Promotion Convergence And Suppression Test

Purpose

Prove that live-maintained query results converge to the same truth as fresh
query re-execution for the same basis.

Scenario

- promote admitted detail, collection, and bounded-materialization queries to
  live mode
- inject truth changes that are:
  - relevant
  - irrelevant
  - suppressible by declared live suppression policy
- compare live-maintained results to repeated fresh execution

Must verify

- live and fresh execution converge to the same result meaning
- irrelevant updates are suppressed
- query-shaped patches preserve ordering, membership, and projection semantics
- no raw CDC is exposed as the primary consumer contract

Required verification output

- `query_digest`
- `result_digest`
- `delivery_digest`
- `replay_digest`
- `counter_snapshot`

Pass condition

Live promotion preserves canonical query meaning and converges under churn.

## Milestone 5.1 Named Certification Suites

### 5.1. Region-Scoped Live Narrowing And Stream Contract Test

Purpose

Prove that region- or partition-scoped live invalidation and stream-backed
delivery contracts remain query-shaped, narrower than broad aspect invalidation
where admitted, and parity-safe with the same canonical live query meaning.

Scenario

- promote admitted live queries with locality-sensitive scope
- inject changes that:
  - hit a relevant region
  - miss the query's declared region
  - require stream-contract admission or typed denial
- compare region-narrowed live maintenance to fresh re-execution and to the
  broader aspect-level control surface

Must verify

- region-scoped invalidation narrows below broad aspect invalidation where the
  lower runtimes admit that narrowing
- irrelevant off-region changes suppress before visible delivery
- query-shaped live delivery can lower into formal stream contracts without
  semantic drift
- unsupported region/stream combinations fail typed and early

Required verification output

- `query_digest`
- `delivery_digest`
- `replay_digest`
- `counter_snapshot`

Pass condition

Region-scoped live narrowing and stream-backed delivery remain canonical,
explicit, and non-leaking.

## Milestone 5.2 Named Certification Suites

### 5.2. Preview Session Basis And Promotion Parity Test

Purpose

Prove that preview-session-bound query contexts preserve explicit basis and
lifecycle identity, and that preview-versus-promoted comparisons remain
query-native rather than ambient host orchestration.

Scenario

- execute the same canonical query against:
  - ordinary branch basis
  - admitted preview session basis
  - promoted-result comparison where the workflow admits it
- vary preview session lifecycle state without changing the declared canonical
  query shape

Must verify

- preview-session identity is explicit in the bundle
- preview-bound results preserve the same query meaning apart from the
  declared preview basis
- preview-versus-promoted comparison remains typed and explicit
- unsupported preview-session combinations fail typed and early

Required verification output

- `query_digest`
- `basis_digest`
- `result_digest`
- `replay_digest`
- `counter_snapshot`

Pass condition

Preview-session query contexts remain basis-explicit, lifecycle-explicit, and
parity-safe.

## Milestone 5.3 Named Certification Suites

### 5.3. Frontier Planning And Parallel Admission Parity Test

Purpose

Prove that frontier-aware planning and deterministic parallel admission alter
cost posture, not canonical query meaning.

Scenario

- plan and execute admitted bulk/live query families through:
  - frontier-aware serial route
  - frontier-aware parallel-admitted route
  - typed serial fallback where parallel admission is denied
- compare predicted breadth to realized breadth

Must verify

- serial and parallel admitted routes produce identical canonical query/result
  meaning
- planning emits explicit frontier and parallel-admission posture
- serial fallback remains explicit rather than hidden executor behavior
- breadth posture stays mechanically visible in counters

Required verification output

- `query_digest`
- `plan_digest`
- `result_digest`
- `counter_snapshot`

Pass condition

Frontier-aware planning and deterministic parallel admission remain
meaning-preserving and mechanically visible.

## Milestone 5.4 Named Certification Suites

### 5.4. Structural Correspondence And Historical Materialization Path Test

Purpose

Prove that structural correspondence and historical materialization-path
artifacts remain explicit about ambiguity, advisability, and how historical
truth was actually materialized.

Scenario

- run correspondence-aware queries over:
  - lineage-backed cases
  - structural-fingerprint-backed cases
  - ambiguous disagreement cases
- run admitted historical queries over:
  - retained snapshot path
  - delta replay path
  - full reconstruction path where admitted
- compare admitted lanes where:
  - structural candidate discovery stays within one bounded planner-owned
    discovery class
  - historical path execution stays within one admitted planner-owned cost
    posture
  - predicted breadth or span differs from realized work but remains
    explicitly reported as drift rather than silent executor mutation

Must verify

- structural correspondence never silently upgrades into authoritative
  continuity
- ambiguous correspondence remains explicit and typed
- historical result bundles expose materialization-path identity
- correspondence and historical result bundles expose planner-owned cost
  posture identity where admitted
- prediction drift between planned and realized correspondence/history work is
  explicit and typed
- execution never broadens structural candidate discovery into one successful
  broad-scan lane when the plan denied that posture
- execution never chooses replay versus reconstruction on its own after
  planning
- unsupported correspondence or historical-path cases fail typed and early

Required verification output

- `query_digest`
- `lineage_digest`
- `basis_digest`
- `result_digest`
- `failure_digest`
- `counter_snapshot`

Pass condition

Correspondence and historical materialization-path semantics remain explicit,
typed, and ambiguity-honest.

## Milestone 5.5 Named Certification Suites

### 5.5. Query Workflow Lowering And Writeback Boundary Test

Purpose

Prove that query-authored mutation, merge, branch-workflow, and writeback
declarations lower into lower-crate authorities without `forge-query`
becoming a second mutation engine.

Scenario

- declare admitted query-authored workflows for:
  - mutation intent lowering
  - preview / compare / merge intent
  - conflict inspection
  - post-merge inspection
  - query-triggered writeback declaration
- compare lowered artifacts and outcomes against authoritative lower-crate
  control lanes

Must verify

- query-authored mutation intents lower into relational commit/merge surfaces
  without semantic drift
- query-triggered writeback declarations lower into bridge-owned writeback
  surfaces without hiding causality or idempotence semantics
- workflow bundles preserve explicit authority boundaries
- unsupported workflow families fail typed and early

Required verification output

- `query_digest`
- `plan_digest`
- `result_digest`
- `delivery_digest`
- `failure_digest`
- `counter_snapshot`

Pass condition

Workflow lowering remains authority-preserving, typed, and non-duplicative.

## Milestone 5.6 Named Certification Suites

### 5.6. Unified Facade And Configuration Boundary Test

Purpose

Prove that the unified application facade and unified runtime configuration
make `forge-query` a real daily-driver surface without erasing subsystem
ownership or collapsing configuration into a bag.

Scenario

- exercise admitted application-facing surfaces through the unified facade
- resolve unified configuration for admitted runtime-backed capability mixes
- compare support metadata/capability advertisement to actual admission
  behavior

Must verify

- the unified facade preserves lower-crate authority boundaries explicitly
- unified configuration remains sectioned by subsystem ownership
- unsupported composed capabilities fail typed and early
- support metadata and executable admission behavior stay in sync

Required verification output

- `query_digest`
- `plan_digest`
- `support_matrix_digest`
- `capability_registry_digest`
- `counter_snapshot`

Pass condition

The unified facade/configuration surface is coherent for developers while
remaining structurally honest about ownership and support.

## Milestone 6 Named Certification Suites

### 6. Historical / Diff / Basis Parity Test

Purpose

Prove that current, branch-scoped, historical, and diff query contexts preserve
the same canonical query meaning apart from the explicitly declared basis.

Scenario

- run the same canonical query against:
  - current branch head
  - alternate branch head
  - historical commit/snapshot basis where admitted
  - diff/comparison between two declared bases
- compare runtime-backed and store-backed historical execution where admitted

Must verify

- basis identity is explicit in every lane
- historical results preserve the same result-shape meaning as current reads
- diff outputs remain query-shaped rather than raw storage deltas
- admitted runtime/store historical paths compare equal

Required verification output

- `query_digest`
- `basis_digest`
- `result_digest`
- `replay_digest`
- `counter_snapshot`

Pass condition

Historical and diff query execution remains basis-explicit and parity-safe.

## Milestone 7 Named Certification Suites

### 7. Lineage And Correspondence Query Parity Test

Purpose

Prove that lineage traversal and correspondence-aware queries remain explicit
about continuity, ambiguity, and rejection.

Scenario

- run lineage-aware queries over:
  - replacement
  - split
  - branch-local divergence
  - ambiguous correspondence candidates
  - explicitly rejected correspondence

Must verify

- authoritative lineage remains distinct from advisory correspondence
- ambiguous correspondence never silently becomes continuity
- branch-local identity evolution stays local unless the truth basis says
  otherwise
- replay preserves lineage/correspondence meaning

Required verification output

- `query_digest`
- `lineage_digest`
- `result_digest`
- `failure_digest`
- `replay_digest`

Pass condition

Identity-evolution query meaning remains typed, replay-safe, and ambiguity-
honest.

## Milestone 8 Named Certification Suites

### 8. Scope / Template / View-Shape Semantic Parity Test

Purpose

Prove that reusable query composition and admitted view shapes preserve the
same canonical meaning as direct construction while adding real planning and
live-maintenance semantics.

Scenario

- compare direct query construction to:
  - scope-composed queries
  - template-instantiated queries
- run admitted view shapes including:
  - table/detail
  - one grouped or temporal view
  - inspector-style detail if shipped

Must verify

- scopes/templates normalize to the same canonical query meaning as direct
  construction
- view shapes affect planning, invalidation, delivery, and patch semantics
- shipped view shapes do not exist only as cosmetic typing

Required verification output

- `query_digest`
- `plan_digest`
- `result_shape_digest`
- `delivery_digest`
- `counter_snapshot`

Pass condition

Composition and view-shape surfaces are semantic query artifacts, not sugar.

## Milestone 9 Named Certification Suites

### 9. Policy, Tenant Schema, And Relationship-Proof Boundary Test

Purpose

Prove that policy masking, tenant basis/schema variation, and graph-native
relationship proofs fail closed and preserve parity across execution modes.

Scenario

- run equivalent queries under:
  - masked and unmasked policy contexts
  - two tenant schema variants
  - valid and broken relationship-proof chains
  - one-shot, live, and historical execution modes where admitted

Must verify

- masked aspects never enter the execution plan
- tenant-specific schema variation changes validation/projection explicitly
- relationship-proof denials fail before unauthorized truth is exposed
- one-shot/live/historical policy behavior remains parity-safe for the same
  declared basis

Required verification output

- `query_digest`
- `policy_digest`
- `result_digest`
- `failure_digest`
- `counter_snapshot`

Pass condition

Policy and tenant boundaries are structural, typed, and non-leaking.

## Milestone 9.1 Named Certification Suites

### 9.1. Query Subscription Declaration And Lowering Parity Test

Purpose

Prove that query-owned subscription declaration families, basis binding, bridge
lowering, and admission preserve canonical live query meaning across equivalent
construction paths, while policy, tenant, basis, relationship-proof, and
view-shape variations that change live meaning change subscription meaning
explicitly or fail before activation.

Scenario

- use a concrete `EmployeeRecord` fixture with:
  - visible fields `identity.employee_id`, `profile.display_name`,
    `profile.department`, and `management.manager_id`
  - masked field `compensation.salary_band`
  - tenant variants where compensation schema differs or is masked
  - relationship-proof path `employee -> department -> manager`
- lower equivalent live query inputs into subscription declarations through:
  - direct canonical live query construction
  - scope-composed construction
  - template-instantiated construction
  - saved-query exact reuse
  - admitted runtime facade helper construction
- exercise admitted subscription family lowerings for:
  - detail exact
  - inspector detail exact
  - ordered collection membership
  - grouped collection membership
  - bounded materialization where required bridge slice kinds are admitted
- vary:
  - masked and unmasked policy basis
  - tenant truth/schema basis
  - valid and broken relationship-proof admission
  - current, branch-head, snapshot, and unsupported basis requests
  - table/detail/grouped/inspector view-shape posture
  - bridge-supported and bridge-unsupported slice families

Must verify

- equivalent live query inputs lower to the same query subscription family,
  declaration digest, equivalence digest, bridge declaration digest, and bridge
  basis request where semantics are equal
- each phase carries a declared work budget, slice budget, bridge-lowering
  budget, or admission budget before it performs work
- budget denials are typed distinctly from semantic unsupported-family denials
- policy, tenant, relationship-proof, basis, and view-shape differences that
  change live meaning also change subscription declaration meaning explicitly
- admitted query subscription declarations lower to explicit bridge
  declaration families and bridge basis requests
- grouped and inspector query-side subscription meanings remain distinct in
  query declaration digests even when they lower onto bridge collection or
  detail families
- masked field influence in subscription slice intent, ordering, grouping, live
  relevance, or delivery intent denies before bridge lowering
- unsupported bridge families, unsupported slice kinds, unsupported bases,
  raw CDC fallback, host observer inference, generic subscription kinds, and
  durable overclaims fail typed and early
- no activation input can be produced without query-owned declaration,
  bridge-lowering, basis-binding, and admission evidence
- declaration code does not allocate active lifecycle state, fanout state,
  delivery windows, acknowledgement frontiers, checkpoints, or continuation
  indexes
- small/medium/larger fixture runs prove declaration and bridge-lowering cost
  slopes are bounded by declared slice, projection, grouping, proof, and
  registry widths rather than unrelated fixture row count
- bridge lowering consumes deduplicated slice proof from query declaration and
  does not rediscover projection, policy masks, saved-query registries, or
  view-shape registries

Required verification output

- `query_digest`
- `live_family_digest`
- `subscription_family_digest`
- `subscription_declaration_digest`
- `subscription_equivalence_digest`
- `policy_digest`
- `tenant_basis_digest`
- `relationship_proof_digest`
- `view_shape_digest`
- `basis_digest`
- `bridge_declaration_digest`
- `bridge_basis_digest`
- `signal_strategy_digest`
- `admission_digest`
- `failure_digest`
- `fixture_digest`
- `subscription_work_budget_digest`
- `subscription_scale_slope_digest`
- `compile_fail_boundary_digest`
- `counter_snapshot`
- `support_matrix_digest`

Pass condition

Subscription declaration and lowering remain query-owned, bridge-honest,
basis-explicit, parity-safe for equivalent live meaning, and fail-closed before
activation for unsupported or ambiguous subscription semantics.

## Milestone 9.2 Named Certification Suites

### 9.2. Subscription Lifecycle Sharing And Preview Parity Test

Purpose

Prove that admitted query subscription activation input becomes active
runtime-backed query subscription lifecycle without changing canonical query or
subscription meaning, and that active delivery, sharing, continuation, and
preview isolation remain query-shaped, parity-safe, and residue-free.

Scenario

- use the concrete `EmployeeRecord` fixture from Milestone 9.1
- activate admitted subscription declarations for:
  - detail exact
  - inspector detail exact
  - ordered collection membership
  - grouped collection membership
  - bounded materialization where admitted
- exercise:
  - single-consumer active lifecycle
  - equivalent-subscription sharing
  - multi-consumer fanout with different delivery pacing
  - query-shaped delivery windows and patch batches
  - bridge/signal update lowering into family-typed query maintenance deltas
  - receipt-based acknowledgement and per-consumer backpressure
  - continuation/remap after admitted identity evolution or correspondence
  - preview-scoped active subscriptions
  - preview discard and preview promotion boundaries
- vary:
  - equal and unequal subscription equivalence digests
  - masked and unmasked policy digests
  - tenant truth/schema basis digests
  - relationship-proof digests
  - view-shape digests
  - current, branch-head, runtime snapshot, and preview-scoped bases
  - sparse and bursty active delivery workloads

Required concrete lanes

- detail exact lane where `management.manager_id` changes and delivery emits a
  detail field patch, receipt, and acknowledgement frontier advancement
- grouped collection lane where an employee moves from `engineering` to
  `design`, producing grouped membership movement rather than host-side
  regrouping
- masked sharing denial lane where masked and unmasked salary-band policies
  cannot share one active lane
- identity continuation lane where advisory successors or explicit identity
  break remain continuation events rather than ordinary field patches
- preview discard lane where an emitted preview delivery is followed by a
  `PreviewDiscarded` closeout with zero authoritative residue by residue class

Must verify

- active lifecycle can be admitted only from `SubscriptionActivationInput`
- active lanes are registry-owned resources and public handles cannot mutate
  lane meaning, acknowledgement state, delivery windows, or continuation
  indexes
- active lane identity preserves query, subscription-family, declaration,
  equivalence, policy, tenant, relationship-proof, view-shape, basis, bridge,
  and signal-strategy meaning
- equivalent subscriptions can share one active maintenance lane while
  retaining independent consumer attachments, acknowledgement frontiers,
  pacing, diagnostics richness, and delivery windows
- meaning-changing policy, tenant, basis, proof, view-shape, delivery, bridge,
  or signal-strategy differences deny sharing before an active lane is joined
- active delivery emits query-shaped patch batches rather than raw CDC
- raw bridge invalidation or raw CDC must lower into a family-typed
  `QuerySubscriptionMaintenanceDelta` before any delivery window consumes it
- maintenance deltas cover distinct detail field, inspector focus, collection
  membership, collection order, grouped membership, bounded-materialization
  scope, continuation, and gap-notice variants
- patch groups cover distinct detail field, focused inspector, collection
  membership/order, grouped membership, bounded-materialization scope,
  continuation, and delivery-gap variants
- detail/inspector and collection/grouped delivery semantics remain distinct
  in active delivery digests even when bridge families are shared beneath them
- slow-consumer or backpressure behavior does not change another consumer's
  query meaning or delivery digest
- acknowledgement frontiers advance only when presented with an emitted
  `QueryDeliveryBatchReceipt` for the same attachment and sequence
- delivery-window overflow emits explicit `RetainWithinWindow`,
  `DropWithGapNotice`, `TerminateConsumer`, or `DebtExplicit` posture rather
  than hidden replay/drop behavior
- hot-path budgets are typed dimension bundles, not raw numeric knobs, and
  include fanout width, delivery-window width, maintenance-delta width,
  patch-group width, continuation-remap width, preview-residue width,
  allocation-scope width, and registry-lookup width
- active lane admission records `ActiveLaneLookupClass`; direct/generation or
  equivalence-index lookup is admitted, while linear scan is either explicit
  debt or typed denial
- delivery emission consumes an `ActiveDeliveryWorkPacket` built from a lowered
  maintenance delta, affected lane/attachment ids, density posture, patch
  width, continuation width, preview residue width, allocation scope, and a
  consumed budget receipt
- sparse delivery logic runs only under `SparseDelta` or `BurstCoalesced`;
  dense refresh must become a delivery-gap patch group, explicit debt, or typed
  denial rather than hidden full-result rebuild
- shared maintenance cost and per-consumer fanout/delivery/acknowledgement cost
  are asserted separately
- allocation posture is phase-local and bounded; unbounded heap allocation is
  explicit debt or denial
- performance receipt digests change when lookup class, density posture,
  allocation posture, or budget consumption changes even if functional query
  results are identical
- continuation/remap consumes typed identity-evolution or correspondence
  evidence before active delivery state changes
- advisory correspondence, explicit identity breaks, collection membership
  remaps, and grouped membership remaps remain distinct continuation classes
- preview-scoped subscriptions cannot share with authoritative subscriptions
  before promotion
- preview discard emits zero authoritative residue evidence for active lanes,
  delivery windows, acknowledgement frontiers, and continuation indexes
- preview residue evidence distinguishes authoritative routing, checkpoint,
  replay, diagnostics, and writeback residue rather than one generic residue
  flag
- preview promotion crosses an explicit authority boundary and mints new
  authoritative active evidence rather than mutating preview state in place
- durable checkpoint, durable restart, and store-backed replay claims fail
  typed and early
- small/medium/larger fixture runs prove active lifecycle cost slopes are
  bounded by declared fanout width, delivery width, patch width, continuation
  remap width, preview residue width, and allocation scope rather than
  unrelated fixture row count
- scale fixtures vary one dimension at a time: unrelated row count, active
  lane count, consumers per lane, patch width, group count, delivery-window
  width, continuation-remap width, preview-residue width, and allocation scope

Required verification output

- `query_digest`
- `subscription_family_digest`
- `subscription_declaration_digest`
- `subscription_equivalence_digest`
- `active_lane_digest`
- `active_lane_handle_digest`
- `active_lane_lookup_class_digest`
- `subscription_budget_digest`
- `subscription_performance_receipt_digest`
- `consumer_attachment_digest`
- `acknowledgement_frontier_digest`
- `delivery_window_digest`
- `maintenance_delta_digest`
- `active_delivery_work_packet_digest`
- `active_delivery_density_posture_digest`
- `allocation_posture_digest`
- `delivery_batch_digest`
- `patch_group_digest`
- `delivery_receipt_digest`
- `continuation_digest`
- `preview_isolation_digest`
- `preview_residue_digest`
- `policy_digest`
- `tenant_basis_digest`
- `relationship_proof_digest`
- `view_shape_digest`
- `basis_digest`
- `bridge_declaration_digest`
- `signal_strategy_digest`
- `failure_digest`
- `lifecycle_denial_digest`
- `counter_snapshot`
- `subscription_lifecycle_scale_slope_digest`
- `compile_fail_boundary_digest`
- `support_matrix_digest`

Pass condition

Active subscription lifecycle, sharing, query-shaped delivery, continuation,
and preview isolation remain family-aware, parity-safe with one-shot meaning,
and fail-closed before lifecycle, delivery, or preview residue can drift.

## Milestone 9.3 Named Certification Suites

### 9.3. Query Subscription Bridge Parity And Diagnostic Sufficiency Test

Purpose

Prove that every admitted automatic query subscription family is
bridge-explainable, support-reportable, offline-diagnosable, and
runtime-certified without inventing hidden semantics beyond canonical query,
bridge, signal, and runtime lifecycle artifacts.

Scenario

- use the concrete `EmployeeRecord` fixture from Milestones 9.1 and 9.2
- derive admitted automatic subscription families for:
  - detail exact
  - inspector detail exact
  - ordered collection membership
  - grouped collection membership
  - bounded materialization where admitted
- assemble for each admitted family:
  - support report for explicit declaration/lifecycle/preview support subjects
  - support lookup receipt
  - manual bridge witness
  - bridge parity explanation
  - bridge parity receipt
  - lifecycle certification bundle
  - offline admitted diagnostic bundle
  - diagnostic assembly receipt
  - runtime family certification bundle
  - certification coverage receipt
- exercise:
  - declaration-family-preserving lifecycle closure
  - grouped and inspector families that share lower bridge classes while
    remaining query-side distinct
  - continuation and preview evidence carried into diagnostic bundles
  - hostile support, bridge-parity, and certification-coverage denials
- vary:
  - masked and unmasked policy digests
  - tenant truth/schema basis digests
  - relationship-proof digests
  - view-shape digests
  - current, branch-head, runtime snapshot, and preview-scoped bases
  - admitted and denied family coverage sets

Required concrete lanes

- detail support lane where admitted declaration, bridge lowering, lifecycle
  certification, and diagnostic bundle all bind the same declaration and bridge
  digests
- grouped family lane where grouped query-side meaning remains distinct from an
  ordered collection family even if the bridge family is shared underneath
- preview certification lane where preview discard or promotion evidence is
  surfaced in the diagnostic bundle and family certification scope
- support-overclaim denial lane where runtime-backed support is requested for a
  family lacking required hostile coverage
- denied diagnostic bundle lane where declaration or bridge denial is still
  emitted as one offline-readable denied bundle without fake lifecycle slots
- bridge-parity mismatch lane where a foreign declaration or signal strategy
  source is supplied and parity fails before a certification bundle exists

Must verify

- support reporting is query-family-aware and does not collapse distinct query
  subscription families into shared bridge-family claims
- runtime-backed support cannot be claimed for uncertified families
- support reporting is phase-typed and distinguishes declaration support,
  active lifecycle support, continuation support, preview-closeout support, and
  deferred durable/store-backed support
- support reporting exposes exact lookup posture and lookup width through a
  receipt; indexed lookup is admitted, while broader scans are explicit debt or
  denial
- store-backed restart, durable replay, and persisted continuation claims
  remain explicit deferred or denied support surfaces
- every admitted automatic family has one bridge-facing explanation binding
  query family, declaration, bridge family, bridge slices, basis posture, and
  signal strategy
- every bridge-facing explanation is bound to one tangible manual bridge
  witness describing the host-equivalent bridge request that Query claims to
  automate
- bridge parity is comparison over pre-lowered or canonically composed witness
  artifacts; witness rebuild or semantic rediscovery after witness construction
  is explicit debt or denial
- grouped and inspector families remain mechanically distinct in support,
  diagnostics, and certification artifacts even when their bridge lowerings
  share infrastructure below Query
- diagnostic bundles are sufficient for offline explanation of admitted and
  denied paths without re-running Query or consulting hidden host state
- admitted and denied diagnostic bundles are different proof types rather than
  one optional-hole envelope
- admitted and denied bundle assembly emit receipts proving stage evidence and
  semantic labels were composed rather than re-derived
- diagnostic traces localize whether failure occurred during family selection,
  declaration, bridge lowering, support reporting, lifecycle certification,
  bridge parity, or coverage closure
- declaration-family drift remains distinct from lifecycle-instance churn such
  as lane handle, attachment, delivery sequence, continuation epoch, or preview
  closeout changes
- runtime family certification requires at least one admitted row and one
  hostile row for each supported family
- runtime family certification also requires representative basis, policy,
  tenant, relationship-proof, view-shape, and lifecycle-class variation rows
  for each supported family
- runtime family certification consumes family-scoped indexed coverage handles
  or explicit matrix-scan debt/denial posture; raw row iteration is not an
  invisible implementation choice
- support, bridge parity, lifecycle certification, and runtime family
  certification bind the same canonical query/declaration/bridge digests
- compile-fail boundaries prove external callers cannot mint support reports,
  bridge parity explanations, diagnostic bundles, or runtime family
  certification bundles directly
- small/medium/larger fixture runs prove support and bundle assembly cost
  slopes are bounded by family coverage width, diagnostic bundle width, and
  indexed coverage width rather than unrelated row count

Required verification output

- `query_digest`
- `subscription_family_digest`
- `subscription_declaration_digest`
- `subscription_equivalence_digest`
- `bridge_declaration_digest`
- `bridge_basis_digest`
- `signal_strategy_digest`
- `support_report_digest`
- `support_matrix_digest`
- `support_lookup_receipt_digest`
- `manual_bridge_witness_digest`
- `bridge_parity_digest`
- `bridge_parity_receipt_digest`
- `diagnostic_trace_digest`
- `admitted_diagnostic_bundle_digest`
- `denied_diagnostic_bundle_digest`
- `diagnostic_assembly_receipt_digest`
- `lifecycle_certification_digest`
- `runtime_certification_bundle_digest`
- `certification_coverage_receipt_digest`
- `continuation_digest`
- `preview_isolation_digest`
- `failure_digest`
- `counter_snapshot`
- `subscription_support_scale_slope_digest`
- `compile_fail_boundary_digest`

Pass condition

Automatic subscription family selection remains bridge-honest, support claims
remain synchronized with certified runtime behavior, diagnostic bundles remain
offline-sufficient, and unsupported or uncertified family claims fail-closed
before store-backed or durable milestones build on top of them.

## Milestone 10 Named Certification Suites

### 10. Store-Backed Execution And Historical Parity Test

Purpose

Prove that store-backed execution and historical restore preserve canonical
query meaning for admitted shared capability families.

Scenario

- execute admitted query families through runtime-backed and store-backed lanes
- restore admitted historical bases through persisted store state
- compare store-backed diff execution to runtime-backed diff execution where
  both paths are admitted

Must verify

- store-backed and runtime-backed results compare equal for admitted paths
- restored historical bases preserve explicit basis identity
- store-backed diff outputs remain query-shaped rather than raw storage deltas

Required verification output

- `query_digest`
- `plan_digest`
- `result_digest`
- `basis_digest`
- `replay_digest`
- `counter_snapshot`

Pass condition

Store-backed execution and admitted historical restore remain parity-safe with
runtime-backed execution for the same canonical query and basis.

## Milestone 11 Named Certification Suites

### 11. Durable Query Artifact And Continuation Parity Test

Purpose

Prove that saved queries, durable cursors, and restart-stable query artifacts
preserve canonical query meaning across reload and continuation.

Scenario

- persist and reload saved queries
- resume durable query-shaped cursors/checkpoints where admitted
- export/import portable query artifacts and re-run them
- compare pre-restart and post-restart continuation semantics

Must verify

- durable saved-query reload preserves canonical identity
- durable cursor continuation resumes the same query-shaped progression
- imported/exported artifacts preserve basis and query meaning
- restart and replay do not alter parameter binding or continuation semantics

Required verification output

- `query_digest`
- `replay_digest`
- `artifact_freeze_digest`
- `artifact_binding_matrix`
- `counter_snapshot`

Pass condition

Durable query artifacts and continuations remain parity-safe across restart,
reload, and portability boundaries.

## Milestone 12 Named Certification Suites

### 12. Blob-Backed Query Delivery And Upload Parity Test

Purpose

Prove that blob/media-backed query results and upload-associated query
semantics remain canonical, policy-safe, and basis-honest.

Scenario

- query blob/media-backed result shapes where the schema admits them
- compare scalar-only and blob-bearing variants of the same canonical query
- exercise upload-associated query results where the platform admits them
- replay or reload durable blob handles where admitted

Must verify

- blob/media-backed query results preserve canonical query identity
- policy masking and basis identity apply equally to blob-backed aspects
- upload-associated query results remain replay-safe and basis-honest
- durable blob handles preserve the semantics they claim where the platform
  admits restart/export survival

Required verification output

- `query_digest`
- `result_digest`
- `delivery_digest`
- `replay_digest`
- `counter_snapshot`

Pass condition

Blob-backed delivery and upload-associated query semantics remain parity-safe,
query-shaped, and non-leaking.

## Milestone 13 Named Certification Suites

### 13. Query Certification Matrix Sufficiency Test

Purpose

Prove that the query certification bundle itself is sufficient to certify every
roadmap capability row in the query vision coverage appendix.

Scenario

- run a mixed milestone 1-12 certification matrix plus any decimal insertion
  milestones claimed as shipped
- emit canonical certification bundles only
- compare coverage against the roadmap's Vision Coverage Appendix

Must verify

- every shipped capability row has at least one hostile certification path
- canonical bundles are sufficient for offline pass/fail analysis
- runtime-backed/store-backed distinctions remain explicit where relevant
- no shipped capability survives only on milestone-local prose

Required verification output

- `certification_bundle_digest`
- `coverage_matrix_digest`
- `bundle_completeness_report`
- `counter_snapshot`

Pass condition

The query subsystem can be certified from canonical artifacts alone, with full
coverage traceable to the roadmap appendix.

## Cross-Milestone Query Support And Honesty Suites

These suites cut across milestone boundaries. They exist to prove that the
query subsystem's admitted support surface, fallback behavior, semantic
reference truth, artifact lifecycle, schema evolution behavior, diagnostics
sufficiency, and beta support claims remain honest as the feature surface
widens.

### 14. Admitted Query Family Boundary Test

Purpose

Prove that admitted query-family combinations execute canonically, while
non-admitted combinations fail explicitly before semantic drift, fallback
degradation, or silent widening occurs.

Scenario

- exercise a curated admitted/non-admitted matrix including cases like:
  - supported detail + live + policy mask + historical basis
  - supported preview-session basis + diff inspection + admitted merge workflow
  - unsupported grouped-view + lineage + CDC-shaped output + saved-query reload
  - supported rollup + tenant schema variant
  - unsupported writeback declaration + masked aspect trigger + denied tenant
    context
  - unsupported structured-content predicate inside an unshipped view-shape
    family
  - supported subscription declaration + policy mask + grouped view
  - unsupported subscription declaration + raw CDC fallback + durable restart
    request
- compare runtime capability advertisement against actual admission behavior

Must verify

- admitted combinations execute and preserve canonical meaning
- non-admitted combinations fail typed and early
- no unsupported combination sneaks through via fallback, widening, or partial
  degradation

Required verification output

- `query_digest`
- `failure_digest`
- `support_matrix_digest`
- `counter_snapshot`

Pass condition

Admitted combinations pass canonically, and non-admitted combinations fail
explicitly before semantic drift.

### 15. Fallback Non-Leakage / No Silent Widening Test

Purpose

Prove that unsupported or non-admitted query requests never widen, degrade, or
fall back silently into a semantically different execution path.

Scenario

- request unsupported projection shapes
- request unsupported view-shape/live combinations
- request unsupported policy/tenant/history combinations
- request unsupported subscription declaration, bridge lowering, basis binding,
  or activation combinations
- request unsupported store-backed capabilities where runtime-backed execution
  would be semantically different

Must verify

- unsupported projection shapes do not widen to whole-entity reads
- unsupported view/live combinations do not degrade into misleading best-effort
  behavior
- unsupported policy/tenant/history combinations do not partially execute and
  redact later
- unsupported subscription declaration combinations do not degrade into raw
  CDC, host observer inference, generic subscription kinds, or direct
  activation
- unsupported store-backed capabilities do not silently fall to a semantically
  different path without explicit diagnostics

Required verification output

- `failure_digest`
- `counter_snapshot`
- `fallback_report`
- `forbidden_widening_zero_report`
- `forbidden_delivery_residue_zero_report`

Pass condition

Fallback is explicit, typed, diagnosable, and non-leaking.

### 16. Cross-Feature Composition Matrix Test

Purpose

Prove that the nastiest admitted cross-feature compositions remain canonical,
and that unsupported compositions fail explicitly instead of drifting.

Scenario

- run a curated adversarial composition matrix including rows like:
  - scope + template + saved-query reload
  - scope + policy mask + historical basis
  - lineage + correspondence + diff
  - preview-session basis + conflict inspection + merge intent lowering
  - inspector view + live promotion + aspect-focused projection
  - rollup + tenant schema variation
  - structured content + policy mask + live maintenance
  - CDC-shaped output + diff + branch basis
  - query-triggered writeback + policy mask + branch workflow basis
  - relationship-proof denial + saved-query artifact reload
  - subscription declaration + policy mask + grouped view
  - subscription declaration + saved-query exact reuse + tenant schema drift
  - subscription declaration + inspector view + relationship-proof denial
  - unsupported subscription declaration + durable restart request

Must verify

- semantically equivalent rows produce the same canonical meaning
- intentionally different rows produce distinct digests
- out-of-support compositions fail typed and early

Required verification output

- `query_digest`
- `result_digest`
- `delivery_digest`
- `failure_digest`
- `composition_matrix_digest`

Pass condition

Cross-feature composition remains canonical where admitted and fail-closed
where not admitted.

### 17. Reference Semantics Test

Purpose

Prove that a bounded set of load-bearing admitted query families agrees with an
independent, deliberately slow, obviously correct semantic oracle rather than
only agreeing with the main planner/executor pipeline.

Scenario

- build a reference executor for a bounded admitted subset covering at least:
  - detail queries
  - collection queries with ordering and pagination
  - bounded traversal/materialization
  - policy-masked results
  - diff output for admitted shapes
  - live end-state convergence for admitted live families
- compare canonical system results against the reference executor

Must verify

- planner and executor results agree with the independent semantic oracle
- result shapes match the oracle's declared semantics
- live end-state converges to the same truth as the oracle's re-executed end
  state
- diff and policy-masked results remain oracle-equivalent for the admitted set

Required verification output

- `query_digest`
- `result_digest`
- `reference_result_digest`
- `oracle_parity_report`
- `counter_snapshot`

Pass condition

The main query system agrees with the independent semantic oracle for the
bounded admitted subset.

### 18. Saved Artifact Semantic Freeze Test

Purpose

Prove that saved queries and related query artifacts retain canonical semantic
identity across reload, export/import, and admitted parameter rebinding.

Scenario

- create a saved query from direct construction
- reload it
- export/import it
- re-bind admitted parameters
- execute it across admissible bases and policy contexts

Must verify

- artifact reload preserves canonical query identity
- export/import preserves semantic meaning rather than "close enough" behavior
- admitted parameter rebinding changes only the semantics the bound context is
  supposed to change
- artifact identity changes when semantic meaning actually changes

Required verification output

- `query_digest`
- `replay_digest`
- `artifact_freeze_digest`
- `artifact_binding_matrix`
- `counter_snapshot`

Pass condition

Saved query artifacts remain semantically frozen unless an intentionally
meaning-changing rebinding occurs.

### 19. Schema Evolution Compatibility Test

Purpose

Prove that query artifacts remain legal and semantically stable under
compatible schema evolution, and fail typed and early under incompatible schema
evolution.

Scenario

- evolve schemas through compatible and incompatible changes
- execute ordinary queries, saved queries, templates, and scopes across those
  schema boundaries
- compare result-shape identity and artifact identity before and after
  evolution

Must verify

- compatible schema evolution preserves legal query meaning where it should
- incompatible schema evolution fails early and typed
- saved query/template/scope artifacts do not silently remap to new meaning
- result-shape evolution changes artifact identity when semantic meaning
  changed

Required verification output

- `query_digest`
- `failure_digest`
- `schema_compatibility_digest`
- `artifact_identity_drift_report`
- `counter_snapshot`

Pass condition

Schema evolution is compatibility-classified, semantically honest, and fail-
closed when meaning changes incompatibly.

### 20. Diagnostic Sufficiency Test

Purpose

Prove that canonical failure and drift bundles are not merely correct, but
sufficient to localize what failed and why without ambient debugging context.

Scenario

- run rejected or drifting cases covering:
  - legality failure
  - unsupported combination
  - policy denial
  - basis mismatch
  - artifact portability failure
  - explicit fallback denial
- inspect only the emitted canonical bundles

Must verify

- bundles identify which clause failed
- bundles identify whether the failure class was legality, unsupported
  combination, policy denial, basis mismatch, or artifact portability
- bundles identify whether fallback was considered and denied
- bundles identify which digest changed and why for drift cases

Required verification output

- `failure_digest`
- `diagnostics_sufficiency_report`
- `bundle_completeness_report`
- `counter_snapshot`

Pass condition

Rejected and drifting cases are mechanically localizable from canonical bundles
alone.

### 21. Beta Support Matrix Enforcement Test

Purpose

Prove that shipped beta surfaces, executable capability advertisement, support
metadata, and certification coverage stay in sync.

Scenario

- compare:
  - shipped beta support metadata
  - executable capability registry / admitted family registry
  - roadmap vision coverage appendix
  - named certification suite coverage
- include certified and non-certified query surfaces

Must verify

- every shipped beta surface maps to at least one certification row
- every non-certified surface is excluded from beta support metadata
- runtime capability advertisement matches actual admitted query families
- documentation/support metadata and executable capability registry remain in
  sync

Required verification output

- `support_matrix_digest`
- `capability_registry_digest`
- `coverage_matrix_digest`
- `support_enforcement_report`

Pass condition

Beta support claims do not outrun certification or admitted runtime behavior.

## What These Tests Collectively Prove

Together, these tests prove that `forge-query` is:

- canonical about query meaning rather than builder-path dependent
- schema-aware before execution rather than repaired by runtime fallback
- snapshot- and basis-honest across runtime-backed and store-backed paths
- query-shaped across collection, live, diff, and delivery surfaces
- bridge-honest across query-owned subscription declaration and admission
  surfaces
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
