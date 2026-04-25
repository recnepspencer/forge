# Milestone C Engineering Spec: Async Resource Policy Families

## Goal

Milestone C completes the async/resource policy layer that Milestone B freezes
as an extensibility boundary.

Milestone B makes async/resource lifecycle truth first-class and introduces
deterministic policy descriptor registries. Milestone C fills those registries
with production-grade policy families, certification rows, compatibility
rules, and performance contracts so higher-level resource products do not have
to invent policy behavior above the runtime.

## Why This Milestone Exists

Async is a bottleneck for Forge product surfaces: route resources, query-backed
views, form actions, background refresh, long-lived subscriptions, and
operator-facing tools all need richer policy behavior than a fixed retry delay
and a fixed timeout.

The runtime cannot hard-code one product interpretation of retry, cancellation,
visibility, retention, or revalidation. It also cannot outsource those choices
to arbitrary callbacks without destroying replay, branch restore,
diagnostics, and certification.

This milestone exists to make policy variation first-class, deterministic,
digestible, and testable.

## Adversarial Constraint

The same async/resource workload, subjected to different declared policy
families across retry, timeout, cancellation, supersession, revalidation,
observation, output continuity, and retention, must remain deterministic,
bounded, replay-compatible where declared compatible, and explicitly denied
where policy identity or policy history is incompatible.

If a policy choice:

- changes lifecycle truth without a policy digest change
- triggers host callback behavior inside runtime legality
- hides retry storms, timer churn, or retention work behind a cheap facade
- makes replay depend on process-local policy code not captured in descriptors
- lets observation or output visibility mutate committed lifecycle truth
- cannot explain why a retry, timeout, cancellation, or revalidation decision
  happened

then the policy family is not fit for the generic runtime.

## Phase 1: Policy Registry Completion

Deliver:

- frozen registries for retry, timeout, cancellation, supersession,
  revalidation, observation, output continuity, and retention policies
- policy ids, names, versions, digests, compatibility posture, and selection
  basis for every registered policy
- duplicate-name, duplicate-id, unknown-policy, and incompatible-version denial
  classifications
- descriptor lowering that records policy identity for replay, diagnostics, and
  certification

Must prove:

- all policy declarations lower before execution work is constructed
- built-in and caller-registered policies use the same registry path
- policy descriptor digests are stable, deterministic, and parameter-sensitive
- unknown or incompatible policy references deny at declaration or restore time

## Phase 2: Retry And Backoff Policy Families

Deliver:

- disabled retry
- fixed delay retry
- exponential backoff
- capped exponential backoff
- jitter families with deterministic seeded jitter basis
- max attempts
- max elapsed retry window
- retry by failure class
- retry by timeout, host failure, semantic rejection, or explicit manual intent
- shared retry budgets by resource node, resource family, runtime, and
  caller-declared scope
- duplicate pending retry coalescing and retry-storm denial

Must prove:

- every admitted retry preserves generation and attempt lineage
- retry eligibility is denied before temporal wake allocation when policy
  already proves it cannot admit
- jitter is deterministic under replay and branch restore
- budget exhaustion is typed and observable
- retry cost is reported in decision width, wake footprint, and budget scope
  touches, not elapsed time

## Phase 3: Timeout And Deadline Policy Families

Deliver:

- disabled timeout
- fixed timeout
- deadline inherited from transaction/runtime context
- per-attempt timeout
- total request-lifetime timeout
- progress-heartbeat extension
- timeout-as-terminal and timeout-as-revalidation-eligible classifications
- timeout-triggered retry eligibility

Must prove:

- timeout admission always consumes Milestone A temporal wake truth
- timeout policy does not invent a second clock model
- changing timeout scope changes descriptor digest
- timeout cost is reported in temporal frontier width and affected request
  count

## Phase 4: Cancellation And Supersession Policy Families

Deliver:

- runtime-hard cancellation
- best-effort host cancellation signalling
- cancellation grace periods
- cancellation after supersession
- cancellation propagation to dependent or child resources where declared
- newest-generation-wins supersession
- overlapping-generation policy
- intent-equivalence coalescing
- old-host-work-left-running with hard completion denial
- old-host-work-cancelled on supersession

Must prove:

- host cancellation failure cannot allow a late completion to commit
- superseded completion denial remains stable across replay
- overlapping-generation policy cannot erase request identity
- supersession and cancellation cost report affected request footprint and
  host-signal advisory width separately

## Phase 5: Revalidation And Freshness Policy Families

Deliver:

- explicit revalidation only
- stale-after revalidation
- dependency-change revalidation
- observer-demand revalidation
- terminal-state revalidation
- fulfilled-only revalidation
- forced revalidation with active-handle proof
- revalidation dedupe and coalescing

Must prove:

- revalidation remains distinct from retry
- active request overwrite requires explicit expected-active proof or a policy
  that produces an equivalent proof-bearing force token
- stale-after revalidation consumes runtime temporal truth
- coalescing does not suppress a semantically required refresh

## Phase 6: Observation And Output Continuity Policy Families

Deliver:

- lifecycle-only observation
- output-continuity observation
- denied-completion observation
- retry-schedule observation
- coalesced per-transaction observation
- previous-output-preserved while pending
- previous-output-hidden while pending
- rejected/timeout/cancelled output preservation policies
- supersession visibility policies

Must prove:

- observation policy cannot mutate lifecycle truth
- output visibility digest stays separate from lifecycle digest
- observer packets remain commit-bounded and rollback-safe
- visibility choices are replayable and diagnostics-visible

## Phase 7: Retention, Diagnostics, And Replay Compatibility Policies

Deliver:

- retain all lifecycle transitions
- retain terminal summaries only
- retain denied completions by budget
- retain retry lineage by budget
- compact superseded/cancelled/timed-out records
- retained-history unavailable classifications
- diagnostics expansion budgets
- replay compatibility rules for policy version drift

Must prove:

- retained summary reads perform zero cold reconstruction
- diagnostics expansion is explicitly budgeted cold work
- policy history loss is classified, not silently ignored
- replay either proves descriptor compatibility or denies with a typed
  incompatibility artifact

## Phase 8: Policy Certification Surface

Deliver:

- `async_resource_policy_family_certification`
- `async_retry_budget_and_backoff_certification`
- `async_timeout_deadline_certification`
- `async_cancellation_supersession_policy_certification`
- `async_revalidation_freshness_certification`
- `async_observation_output_continuity_certification`
- `async_retention_replay_policy_certification`
- compile-fail fixtures for private policy descriptor constructors and
  unforgeable force/eligibility tokens

Must prove:

- every policy family has at least one hostile scenario row
- every policy decision is traceable to a descriptor id/name/version/digest
- policy variation does not alter hard lifecycle laws
- performance envelopes report policy-specific cost surfaces

## Must Preserve

- `forge-signal` remains derived-computation authority only
- async/resource lifecycle law remains hard-coded and proof-bearing
- policy variation must lower before execution
- policy descriptors, not callbacks, are replay authority
- completion identity, generation, attempt, and branch epoch proofs remain
  unchanged by policy families
- observation and diagnostics may vary by policy, but committed lifecycle truth
  may not

## Acceptance Evidence

Milestone C is complete only when certification emits canonical artifacts for:

- policy registries and descriptor digests
- policy selection bases
- retry lineage and budget decisions
- timeout and deadline decisions
- cancellation and supersession decisions
- revalidation and freshness decisions
- observation and output-continuity decisions
- retention and diagnostics-budget decisions
- replay compatibility or incompatibility artifacts
- boundary performance envelopes for every policy family

## Sequencing Notes

Milestone C belongs after Milestone B because policy families need the
resource lifecycle substrate, request identity proofs, temporal wake truth,
transactional completion apply, branch/replay artifacts, and public boundary
envelopes that Milestone B establishes.

It belongs before wasm, route-resource, form, or query product layers claim
resource behavior, because those layers must consume runtime policy truth
rather than define parallel state machines.

