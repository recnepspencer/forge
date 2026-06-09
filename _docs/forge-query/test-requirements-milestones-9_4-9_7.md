## Milestone 9.4 Named Certification Suites

### 9.4. Temporal Query Basis And Time-Aware Subscription Parity Test

Purpose

Prove that time-aware query contexts and temporal subscription lowerings
preserve canonical query meaning while keeping historical truth basis distinct
from signal/runtime temporal execution basis.

Scenario

- use the concrete `EmployeeRecord` fixture from Milestones 9.1 through 9.3
- construct equivalent time-aware live queries through:
  - direct canonical construction
  - scope-composed construction
  - template-instantiated construction
  - saved-query exact reuse where admitted
  - unified facade helper construction
- exercise admitted temporal query families for:
  - stale-after detail and inspector queries
  - refresh-interval collection queries
  - rolling-window timeline queries
  - tolerance-aware chart queries with time-only reevaluation
- vary:
  - current, branch-head, snapshot, historical, and preview-scoped truth bases
  - monotonic clock, manual test clock, deadline, and interval wake classes
  - time-only wakes, truth-only patches, and truth-plus-time interleavings
  - masked and unmasked policy contexts
  - tenant schema variants
  - supported and unsupported temporal bridge basis requests

Required concrete lanes

- time-only stale-after lane where no truth patch occurs but delivery changes
  result freshness state through a query-shaped temporal cause
- historical-truth-versus-clock lane where the same query runs against a
  historical snapshot while temporal execution advances independently
- rolling-window lane where an entity enters or leaves a window because clock
  time advances, not because truth changed
- ambient-clock denial lane where a temporal query attempts to use an unbound
  host clock or implicit timer
- bridge-temporal-basis mismatch lane where query temporal basis and bridge
  temporal basis digests diverge before delivery

Must verify

- truth basis and temporal execution basis produce separate digests and cannot
  be substituted for each other
- equivalent time-aware query declarations lower to the same temporal
  subscription declaration and bridge temporal basis request
- time-only deliveries are query-shaped and never expose raw signal wake events
- historical truth reads do not advance merely because temporal execution time
  advances
- temporal wakes do not widen projection, bypass policy masks, or rescan
  unrelated truth outside declared query scope
- previous-value comparison basis is explicit where temporal delivery depends
  on prior result state
- unsupported clock classes, unbound timers, ambient host timers, and
  unsupported temporal bridge basis requests fail typed and early
- diagnostics localize temporal denial to query basis binding, bridge temporal
  lowering, signal temporal strategy, previous-value basis, or support
  metadata
- small/medium/larger fixture runs prove temporal delivery cost slopes are
  bounded by declared temporal wake width, projection width, affected result
  width, and previous-value comparison width rather than unrelated row count

Required verification output

- `query_digest`
- `subscription_declaration_digest`
- `subscription_equivalence_digest`
- `basis_digest`
- `temporal_basis_digest`
- `bridge_temporal_basis_digest`
- `signal_strategy_digest`
- `previous_value_basis_digest`
- `result_digest`
- `delivery_digest`
- `temporal_delivery_digest`
- `diagnostic_trace_digest`
- `failure_digest`
- `counter_snapshot`
- `temporal_query_scale_slope_digest`
- `compile_fail_boundary_digest`

Pass condition

Temporal query basis binding and time-aware subscription lowering remain
query-owned, bridge-honest, time-travel-honest, and fail-closed before ambient
clock or raw signal timing can leak into query results.

## Milestone 9.5 Named Certification Suites

### 9.5. Phase 1 Named Scope Expansion Productization Closure Test

Purpose

Prove that runtime-backed named scope expansion is a verified composition lane:
equivalent direct construction and admitted scope-composed construction must
produce the same canonical query meaning while preserving explicit scope
lineage, typed basis evidence, and honest support-profile posture.

Scenario

- construct equivalent detail and collection queries through:
  - direct canonical authoring
  - named predicate scope expansion
  - named ordering scope expansion
  - named projection scope expansion
  - named traversal-bound scope expansion
  - basis-aware scope expansion where admitted
  - admitted multi-scope composition combining projection, ordering, and
    traversal-bound scopes in one path
- vary:
  - detail and collection query families
  - single-scope and multi-scope expansion
  - current-basis evidence on basis-aware scope paths
  - illegal traversal widening
  - unsupported scope-family admission

Required concrete lanes

- predicate-scope parity lane where direct and scope-composed detail queries
  converge on the same canonical query digest
- multi-scope collection parity lane where projection, ordering, and
  traversal-bound scopes converge on the same canonical query and result-shape
  digests as equivalent direct construction
- basis-aware parity lane where admitted basis evidence preserves canonical
  query meaning while emitting explicit basis metadata
- basis-aware mismatch denial lane where evidence bound to a different canonical
  query fails typed and early
- traversal-widening denial lane where a traversal-bound scope attempts to
  exceed its declared depth bound
- unsupported-scope denial lane where non-admitted scope families fail before
  authored-request lowering

Must verify

- named scope expansion is canonical declaration composition rather than string
  substitution or caller-owned rewrite folklore
- equivalent direct and scope-composed declarations produce the same canonical
  `query_digest`
- result-shape parity holds for admitted scope families on the covered paths
- `scope_lineage_digest` is explicit and non-empty on admitted named-scope
  expansion artifacts
- basis-aware scope evidence stays typed, query-bound, and fail-closed
- admitted named-scope paths keep `scope_rediscovery_count == 0`
- support/profile publication reports `named_scope_expansion:verified`
- diagnostics localize denial to unsupported scope family, illegal widening, or
  basis-evidence query mismatch rather than broad composition failure
- small/medium/larger fixture runs prove scope expansion cost stays bounded by
  declared scope count and scope width rather than unrelated row or schema
  breadth

Required verification output

- `query_digest`
- `result_shape_digest`
- `composition_digest`
- `scope_lineage_digest`
- `basis_digest`
- `support_profile_digest`
- `failure_digest`
- `counter_snapshot`
- `scope_expansion_scale_slope_digest`
- `compile_fail_boundary_digest`

Pass condition

Named scope expansion is certified only when admitted scope families preserve
canonical query meaning, typed basis evidence, explicit lineage, zero
rediscovery on the ordinary path, and a public `verified` support posture while
unsupported or widening neighbors fail typed and early.

### 9.5. Phase 2 Template Instantiation Productization Closure Test

Purpose

Prove that runtime-backed template instantiation is a verified composition
lane: equivalent direct construction and admitted template-instantiated
construction must produce the same canonical query meaning while preserving
explicit typed binding artifacts, exact binding counters, and honest
support-profile posture.

Scenario

- construct equivalent detail and collection queries through:
  - direct canonical authoring
  - detail template instantiation with typed predicate binding
  - collection template instantiation with typed traversal binding
  - basis-aware template instantiation where admitted
- vary:
  - detail and collection query families
  - binding kinds across predicate, ordering, projection, and traversal slots
  - missing, duplicate, undeclared, and kind-mismatched bindings
  - duplicate slot declaration
  - deferred template-family admission for observed inspector, focused
    inspector, and grouped collection template families

Required concrete lanes

- detail-template parity lane where direct and template-instantiated detail
  queries converge on the same canonical query and result-shape digests
- collection-template parity lane where direct and template-instantiated
  collection queries converge on the same canonical query and result-shape
  digests
- basis-aware template parity lane where admitted basis evidence remains bound
  to the fully instantiated canonical query and emits explicit basis metadata
- missing-binding denial lane where a declared slot lacks a bound value
- duplicate-binding denial lane where one slot receives multiple bindings
- undeclared-slot denial lane where a binding targets a slot the template did
  not declare
- binding-kind mismatch lane where slot kind and binding kind diverge
- duplicate-slot declaration lane where the same slot is declared twice on one
  template
- deferred-template-family lane where observed inspector, focused inspector,
  and grouped collection template families remain typed-and-early deferred

Must verify

- template instantiation is canonical declaration composition rather than
  caller-owned rewrite folklore
- equivalent direct and template-instantiated declarations produce the same
  canonical `query_digest`
- result-shape parity holds for admitted template families on the covered
  paths
- `template_binding_digest` is explicit and non-empty on admitted
  template-instantiation artifacts
- composition reports preserve the same binding digest as the emitted
  instantiation artifact
- admitted template-instantiated paths keep `template_rediscovery_count == 0`
- basis-aware template evidence stays typed, canonical-query-bound, and
  fail-closed
- diagnostics localize denial to missing binding, duplicate binding, undeclared
  slot, binding-kind mismatch, duplicate slot declaration, or deferred
  template-family posture rather than broad composition failure
- support/profile publication reports `template_instantiation:verified`
- small/medium/larger fixture runs prove template instantiation cost stays
  bounded by declared slot count and binding width rather than unrelated row or
  schema breadth

Required verification output

- `query_digest`
- `result_shape_digest`
- `composition_digest`
- `template_binding_digest`
- `basis_digest`
- `support_profile_digest`
- `failure_digest`
- `counter_snapshot`
- `template_instantiation_scale_slope_digest`
- `compile_fail_boundary_digest`

Pass condition

Template instantiation is certified only when admitted template families
preserve canonical query meaning, typed binding identity, exact slot and
binding counters, zero rediscovery on the ordinary path, and a public
`verified` support posture while malformed or deferred neighbors fail typed and
early.

## Milestone 9.6 Named Certification Suites

### 9.6. Mixed Truth Time Async Query Delivery Ordering Test

Purpose

Prove that mixed truth, temporal, async, retry, cancellation, policy, tenant,
preview, promotion, and discard causes produce one deterministic query-shaped
delivery stream whose meaning is independent of host event arrival order.

Scenario

- activate admitted temporal and async query subscriptions from Milestones 9.4
  and 9.5
- deliver equivalent cause sets through multiple arrival orders:
  - truth patch before temporal wake
  - temporal wake before truth patch
  - async completion racing with truth patch
  - cancellation racing with completion
  - retry racing with policy or tenant remask
  - preview discard racing with temporal/async residue
  - preview promotion followed by authoritative rebinding
- compare live delivery to replay of the canonical cause sequence
- compare final live result to fresh query execution for the same basis

Required concrete lanes

- truth-plus-time lane where a field patch and a stale-after wake coalesce into
  one deterministic query-shaped delivery
- async-plus-truth lane where stale completion is rejected because the truth
  patch changes the query basis first
- cancellation-plus-completion lane where two host arrival orders produce the
  same canonical cancellation result
- policy-remask-plus-wake lane where a temporal wake cannot reveal a newly
  masked aspect
- preview-discard residue lane where all temporal and async residue attached
  to the preview basis is closed out with zero authoritative residue
- unsupported-composition lane where a mixed-cause combination with no admitted
  ordering rule fails before delivery

Must verify

- host event arrival order variation does not change canonical delivery digest
  for admitted mixed-cause families
- canonical cause ordering is explicit and replayable
- coalescing and suppression decisions carry receipts and cannot erase
  semantic differences
- mixed-cause deliveries remain query-shaped and never expose raw CDC, raw
  signal wake events, raw resource completions, or transport-local events as
  the consumer contract
- policy and tenant masking hold under every admitted cause ordering
- preview promotion mints new authoritative temporal/async basis evidence
  rather than mutating preview residue in place
- preview discard proves zero authoritative residue for temporal wakes, async
  generations, retry state, cancellation state, diagnostic state, and delivery
  windows
- unsupported mixed-cause compositions fail typed and early rather than
  choosing host event order as an implicit policy
- replay of the same canonical cause sequence produces identical query,
  result, delivery, and diagnostic digests
- small/medium/larger fixture runs prove delivery ordering cost is bounded by
  declared cause width, coalescing width, suppressed group width, preview
  residue width, and affected result width rather than unrelated row count

Required verification output

- `query_digest`
- `basis_digest`
- `temporal_basis_digest`
- `async_resource_digest`
- `cause_ordering_digest`
- `coalescing_receipt_digest`
- `suppression_receipt_digest`
- `preview_residue_digest`
- `policy_digest`
- `tenant_basis_digest`
- `result_digest`
- `delivery_digest`
- `replay_digest`
- `failure_digest`
- `diagnostic_trace_digest`
- `counter_snapshot`
- `mixed_cause_scale_slope_digest`
- `compile_fail_boundary_digest`

Pass condition

Mixed truth/time/async delivery remains deterministic, replayable,
query-shaped, policy-safe, and fail-closed before unsupported interleavings can
be interpreted by host arrival order.

## Milestone 9.7 Named Certification Suites

### 9.7. Temporal Async Query Certification Matrix Sufficiency Test

Purpose

Prove that the runtime-backed temporal/async query surface is certified by
hostile bundles, support metadata, diagnostics, exact counters, and a reference
workload before store-backed and durable milestones build on top of it.

Scenario

- run a temporal/async certification matrix covering Milestones 9.4, 9.5, and
  9.6
- compare:
  - shipped temporal/async support metadata
  - executable admission behavior
  - bridge/signal lowering digests
  - named certification suite coverage
  - roadmap Vision Coverage Appendix rows
- execute a reference workload containing:
  - branch-scoped live query
  - time-only wake
  - truth patch plus temporal wake
  - async success
  - async failure
  - cancellation
  - retry
  - stale completion
  - supersession
  - preview promotion
  - preview discard
  - policy remask
  - tenant remask
  - unsupported ambient clock
  - unsupported resource family
  - unsupported mixed-cause composition

Must verify

- every advertised runtime-backed temporal/async query capability has at least
  one admitted row, one hostile row, and one unsupported-neighbor row
- temporal basis, async resource basis, cause ordering, support metadata,
  diagnostics, and certification coverage bind the same canonical query and
  subscription declaration digests
- diagnostic bundles are sufficient for offline localization without re-running
  Query, Bridge, Signal, or a host runtime
- support metadata cannot advertise uncertified temporal, async, or
  mixed-cause families
- durable restore, persisted inflight resource continuation, store-backed
  temporal replay, and durable mixed-cause continuation remain deferred or
  denied until Milestones 10 and 11
- the reference workload emits machine-checkable bundles rather than relying on
  visual inspection, logs, or self-comparison within one run
- exact counters prove certification coverage and support lookup cost are
  bounded by declared family coverage width rather than unrelated fixture size
- compile-fail boundaries prove external callers cannot mint certification
  bundles, support rows, temporal basis proofs, async completion causality, or
  mixed-cause ordering receipts

Required verification output

- `certification_bundle_digest`
- `coverage_matrix_digest`
- `support_matrix_digest`
- `capability_registry_digest`
- `query_digest`
- `subscription_declaration_digest`
- `temporal_basis_digest`
- `async_resource_digest`
- `cause_ordering_digest`
- `diagnostic_trace_digest`
- `admitted_diagnostic_bundle_digest`
- `denied_diagnostic_bundle_digest`
- `reference_workload_digest`
- `failure_digest`
- `counter_snapshot`
- `temporal_async_support_enforcement_report`
- `compile_fail_boundary_digest`

Pass condition

The temporal/async query surface is certified only when support claims,
admission behavior, diagnostic bundles, bridge/signal lowering, hostile
coverage, and reference workloads agree, while durable and store-backed claims
remain explicit later-milestone debt.

