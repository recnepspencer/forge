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

### 9.5. Async Resource Query Family And Completion Causality Test

Purpose

Prove that async/resource-backed query families preserve typed result-state
meaning and reject stale, superseded, cancelled, policy-invalid, or
tenant-invalid completions before they can mutate query results.

Scenario

- declare async/resource-backed query families for admitted detail,
  collection, grouped, and bounded-materialization query shapes
- exercise resource result states:
  - pending
  - fulfilled
  - failed
  - stale
  - cancelled
  - retried
  - revalidating
  - superseded
- vary:
  - resource source family
  - async generation
  - retry generation
  - branch and preview basis
  - policy and tenant basis
  - completion order
  - cancellation timing
  - bridge-supported and bridge-unsupported resource lifecycle classes

Required concrete lanes

- fulfilled-current lane where completion generation, query basis, policy
  basis, tenant basis, and result shape all match and delivery is admitted
- stale-completion denial lane where an older async completion arrives after a
  newer truth or query basis has already superseded it
- policy-remask lane where a resource completion becomes invalid because the
  policy context changed before materialization
- retry-revalidation lane where retry preserves query identity but changes
  resource generation and emits explicit retry evidence
- cancellation race lane where cancellation wins before completion delivery
  and no result mutation occurs
- unsupported-resource-family lane where Query denies the resource family
  before bridge lifecycle activation

Must verify

- async result states are typed query result states, not host-local strings or
  optional UI metadata
- completion causality binds query digest, result shape digest, truth basis,
  policy digest, tenant digest, resource source identity, and async generation
- stale, cancelled, denied, and superseded completions cannot emit fulfilled
  query results
- retry and revalidation preserve canonical query identity unless the declared
  query basis intentionally changes
- policy and tenant masking apply before async result materialization
- failure taxonomy distinguishes source failure, cancellation, supersession,
  retry exhaustion, policy denial, tenant drift, bridge denial, and unsupported
  family denial
- diagnostics localize whether failure occurred during query declaration,
  source admission, bridge resource lifecycle, signal async generation,
  completion causality, materialization, or support certification
- compile-fail boundaries prove external callers cannot forge async resource
  state, completion-causality artifacts, supersession witnesses, or fulfilled
  delivery from raw completion payloads
- small/medium/larger fixture runs prove async completion checks are bounded by
  declared inflight generation width, completion batch width, retry width, and
  affected result width rather than unrelated resource or row count

Required verification output

- `query_digest`
- `result_shape_digest`
- `basis_digest`
- `policy_digest`
- `tenant_basis_digest`
- `async_resource_digest`
- `async_generation_digest`
- `completion_causality_digest`
- `supersession_digest`
- `retry_digest`
- `cancellation_digest`
- `result_digest`
- `delivery_digest`
- `failure_digest`
- `diagnostic_trace_digest`
- `counter_snapshot`
- `async_resource_scale_slope_digest`
- `compile_fail_boundary_digest`

Pass condition

Async/resource query families remain basis-bound, causally ordered,
policy-safe, and fail-closed before stale or unsupported completions can affect
query-shaped results.

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

