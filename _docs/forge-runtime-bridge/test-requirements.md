# Forge Runtime Bridge Test Requirements

## Scope

This document defines the certification-grade bridge test requirements for:

- Milestone 6
- Milestone 7
- Milestone 8
- Milestone 9
- Milestone 10
- Milestone 11
- Milestone 12
- Milestone 12b
- Milestone 13

Milestones 1 through 5 already have their own acceptance and closeout proof
surfaces. This document starts at Milestone 6 because the bridge is now moving
from geometry-kernel-critical routing foundations into broader protocol,
policy, merge, preview, and writeback surfaces that need a much stronger
crate-level certification bar.

## Purpose

The bridge cannot be treated as shipped merely because individual milestone
features appear to work in direct tests.

From Milestone 6 onward, the bridge is making claims about:

- protocol correctness under restart, replay, and multi-consumer pressure
- host-agnostic source contracts
- structural remapping without identity fusion
- merge-bearing history interpretation
- speculative branch coordination
- cross-runtime policy propagation
- bridge-mediated writeback
- end-to-end causality and bridge-native certification

Those are all adversarial surfaces. They need certification tests, not just
behavior checks.

## Global Adversarial Constraint

The bridge test suite from Milestone 6 onward must prove the following:

> Under restart, replay, host-adapter variation, branch divergence, merge-like
> history pressure, speculative execution, policy changes, hostile diagnostics
> tiers, and writeback failure injection, the bridge must preserve canonical
> truth interpretation, canonical routing meaning, replay-safe artifacts, typed
> failures, explicit authority boundaries, and machine-checkable diagnostics
> without allowing host-local glue, scheduler timing, or convenience policy to
> redefine canonical semantics.

If a bridge surface works only under one adapter, one scheduling mode, one
consumer shape, one diagnostics tier, or one happy-path history shape, it is
not certified.

## Meta-Rules

These tests are all certification tests. They must:

- emit canonical machine-checkable artifacts, not "logs looked good"
- compare canonical digests across equivalent runs
- prove typed failure localization for rejection paths
- prove diagnostics richness changes retained detail, not semantic truth
- prove replay from canonical bridge artifacts rather than ambient host state
- verify counter contracts where the milestone makes boundedness or scale claims
- prove that any lowered bridge taxonomy remains losslessly traceable back to
  the authoritative truth/runtime taxonomy it consumes
- prove replay parity over full canonical result bundles when a milestone claims
  replay-safe explanations, routing, continuity, remap, or denial
- prove discovery work boundedness when a milestone claims bounded lowering or
  bounded execution cost, not just post-lowering work boundedness

These requirements are mandatory, not advisory. A test does not satisfy this
document merely because it exercises the right API shape or emits a non-empty
bundle. Certification requires mechanical comparison across independently
produced runs with declared semantic relationships.

### Global Certification Shape

Every named certification suite must define at least these lanes unless the
suite explicitly states a narrower reason:

- `control_lane` — the canonical no-failure, no-hostility baseline
- `hostile_lane` — the adversarial variation being certified
- `replay_lane` — replay, resume, restart, or retained-artifact reproduction of
  the same semantic workload

If a suite is about explicit rejection, the hostile lane may terminate in a
typed failure rather than a completed result, but the suite must still compare
that failure against a successful or otherwise equivalent control basis.

### Mandatory Assertion Classes

Every named certification suite must include all applicable assertion classes:

- equality assertions for semantically equivalent lanes
- inequality assertions for intentionally divergent semantic lanes
- typed-failure assertions for rejected lanes
- zero-or-absence assertions for forbidden residue, forbidden fallback, or
  forbidden diagnostics influence

A test that only checks one lane in isolation, only checks that a bundle is
present, or only checks that a digest is non-empty does not satisfy this
document.

### Certification Bundle Rules

Certification bundles must use fixed, milestone-appropriate fields rather than
free-form "debug info". A suite may emit additional detail, but it must emit a
stable canonical bundle shape for its scope.

At minimum, every certification bundle must make it possible to answer all of
these questions mechanically:

- did two semantically equivalent lanes produce the same canonical truth?
- did an intentionally different semantic lane produce different canonical
  truth?
- did a rejected lane fail at the correct protocol or planning boundary?
- did diagnostics richness change retained detail only?
- did counters remain within the contract claimed by the milestone?

If a bundle cannot answer those questions without reading host logs or runtime
internals, the bundle is insufficient.

### Mutation-Sensitivity Rule

Every named certification suite must include at least one perturbation from each
applicable class:

- a perturbation that changes pacing, scheduling, retention timing, or
  diagnostics richness without changing canonical meaning
- a perturbation that changes canonical meaning and must therefore change at
  least one declared digest or typed report
- a perturbation that must fail explicitly before semantic drift occurs

This rule exists to prevent dead bundles and ornamental assertions. If a suite
never proves what must stay the same, what must change, and what must fail, it
has not certified its surface.

### Counter Assertion Rule

Whenever a milestone claims boundedness, scale-path correctness, replay safety,
resource discipline, or pacing discipline, the suite must assert exact counter
values for the representative scenario, including counters that must remain
zero.

Presence-only counter checks do not satisfy this document. Range checks are
acceptable only when the suite explicitly proves why the counter is
intentionally variable and why that variability is itself part of the contract.

### Offline Sufficiency Rule

Certification should be possible from the canonical bundle alone. Suites may use
live execution to produce the bundle, but pass/fail analysis must not depend on
ambient host state, ad hoc log inspection, or debugger-only knowledge.

Milestone 13 makes this requirement explicit for the full matrix, but every
Milestone 6+ suite should be written so that an auditor can evaluate the
certification result from the emitted bundle and declared comparison rules
alone.

At minimum, certification bundles should emit digests or structured reports for:

- `stream_digest`
- `window_digest`
- `checkpoint_digest`
- `routing_digest`
- `truth_view_digest`
- `consumer_contract_digest`
- `policy_digest`
- `replay_digest`
- `diagnostics_digest`
- `failure_digest`
- `counter_snapshot`

Not every test uses every artifact, but every test should emit the canonical
bundle that matches its scope.

### Anti-Fake-Test Rule

The following do not count as certification:

- asserting that a run "completed successfully"
- asserting that a digest or report field is merely present or non-empty
- comparing a value only to itself from the same run
- treating log text as the primary proof artifact
- validating only happy-path success without an adversarial lane
- validating only a failure lane without a control or replay comparison basis

A certification test must compare independently produced artifacts under a
declared semantic relationship. If the relationship is not explicit, the test is
not certification-grade.

## Milestone 6 Named Certification Suites

### 1. Change Stream Checkpoint Fracture Equivalence Test

Purpose

Prove that checkpoint, resume, and replay remain semantically exact under
partial progress, restart, and stream truncation pressure.

Scenario

- run one canonical change stream with bursty commit windows
- consume it through the bridge with checkpoint publication at several
  boundaries
- inject restart after partial batch delivery
- resume from canonical checkpoint tokens
- replay from canonical bridge records
- compare against a no-failure control run

Must verify

- resumed consumption preserves routing semantics exactly
- checkpoint meaning is tied to canonical acknowledged members, not latest seen
- truncation and incompatibility fail explicitly and typed
- replay from checkpoint records matches original admitted delivery windows
- equivalent resumed and control lanes compare equal on canonical stream and
  routing digests
- stale, truncated, or incompatible checkpoint lanes compare unequal and fail at
  the declared protocol boundary

Required verification output

- `stream_digest`
- `checkpoint_digest`
- `resume_matrix`
- `replay_digest`
- `protocol_failure_digest`

Pass condition

Equivalent runs converge to identical canonical stream and routing truth; bad
checkpoint resumes fail explicitly.

### 2. Multi-Consumer Coalescing Parity Test

Purpose

Prove that different admitted consumer shapes can consume the same canonical
stream with different pacing and coalescing policies without changing stream
meaning.

Scenario

- feed one canonical stream into at least:
  - a routing consumer
  - a replay/audit consumer
- run one lane with narrow windows and no coalescing
- run one lane with wider legal coalescing windows
- vary consumer pacing and restart points

Must verify

- canonical member interpretation stays identical across consumers
- coalesced windows remain reconstructable into canonical member truth
- duplicate observation handling stays contract-correct
- diagnostics richness does not create a third stream meaning
- pacing-only and coalescing-only perturbations preserve canonical member,
  routing, and replay truth while allowing window-shape differences
- illegal coalescing boundaries fail explicitly rather than widening silently

Required verification output

- `consumer_contract_digest`
- `window_digest`
- `coalescing_report`
- `routing_digest`
- `diagnostics_digest`

Pass condition

Consumers may differ in window shape and retained detail, but not in canonical
stream meaning.

### 3. Backpressure And Retention Anchor Hostility Test

Purpose

Prove that backpressure changes pacing only and that retention-anchor loss or
lag is localized as typed protocol failure rather than semantic drift.

Scenario

- run slow and saturated consumers against a bursty stream
- vary backpressure classes during delivery
- drop retention for older stream material in hostile timing windows
- attempt resume with stale and valid checkpoint anchors

Must verify

- backpressure never reorders or semantically merges stream members
- pressure only affects admitted pacing and optional richness
- lost retention anchors fail explicitly
- pressure and truncation diagnostics identify the exact failed boundary
- pressure-only perturbations leave canonical stream and routing digests equal to
  the control lane
- retention-anchor loss produces typed failure with zero false-success residue

Required verification output

- `pressure_report`
- `checkpoint_digest`
- `retention_anchor_matrix`
- `failure_digest`
- `counter_snapshot`

Pass condition

Pressure changes cost and pacing only; retention loss is typed and explicit.

## Milestone 7 Named Certification Suites

### 4. Multi-Host Source Parity Test

Purpose

Prove that multiple host-shaped source implementations satisfy one canonical
bridge source contract without changing read semantics.

Scenario

- implement at least two source adapters with different host shapes
- run snapshot, historical, branch, and admitted field/facet reads through both
- execute identical bridge evaluation requests against each adapter

Must verify

- identical source contracts yield identical read and routing truth
- host adapter shape does not leak into public bridge semantics
- parity holds across current, historical, and branch-local reads

Required verification output

- `truth_view_digest`
- `source_contract_digest`
- `adapter_parity_matrix`
- `routing_digest`

Pass condition

Different host adapters produce identical canonical bridge-visible results.

### 5. Source Capability Rejection Boundary Test

Purpose

Prove that unsupported or mismatched source capabilities fail during admission,
not late during evaluation.

Scenario

- request unsupported historical, branch, or facet modes
- vary source capability declarations across adapters
- attempt evaluation under incompatible source contracts

Must verify

- unsupported source modes fail before materialized read execution
- failure classes distinguish capability mismatch from host transport failure
- no adapter-specific fallback silently widens the request

Required verification output

- `source_contract_digest`
- `capability_matrix`
- `failure_digest`
- `diagnostics_digest`

Pass condition

Capability mismatches are typed, early, and non-ambiguous.

### 6. Builder Surface Swap Parity Test

Purpose

Prove that bridge setup remains explicit and comprehensible while swapping
source adapters, diagnostics policy, and registration order.

Scenario

- construct equivalent bridge setups through different explicit builder orders
- swap source adapters while keeping the same admitted source contract
- vary diagnostics policy and registration ordering

Must verify

- setup order does not change canonical source contract truth
- builder/config surfaces remain explicit rather than ambient
- lifecycle propagation is complete when adapters are swapped

Required verification output

- `source_contract_digest`
- `builder_configuration_digest`
- `setup_parity_matrix`
- `counter_snapshot`

Pass condition

Construction order and adapter swap do not alter canonical bridge meaning.

## Milestone 8 Named Certification Suites

### 7. Structural Match Ambiguity Torture Test

Purpose

Prove that structural identity assists remapping only when it is actually
honest, and that ambiguous matches fail explicitly.

Scenario

- create histories with several structurally similar but semantically distinct
  candidates
- run remapping under branch divergence and historical comparison
- inject cases with one exact structural match, many ambiguous matches, and no
  safe match

Must verify

- structural ambiguity never silently picks a winner
- exact matches remain subordinate to authoritative identity
- mismatch and ambiguity diagnostics remain replay-safe

Required verification output

- `structural_match_digest`
- `ambiguity_report`
- `remap_artifact_digest`
- `failure_digest`

Pass condition

Structural signals help only when unambiguous and never fuse identity.

### 8. Structural Reuse Without Identity Fusion Test

Purpose

Prove that structural reuse can improve remapping and comparison without
collapsing truth identity or signal identity into one fused space.

Scenario

- run replacement and restore sequences with structurally equivalent shapes
- compare remapping behavior across branches and after replay
- include same-shape/different-authority cases

Must verify

- reuse surfaces remain advisory to authoritative identity
- replay reproduces the same reuse decisions
- restore and branch-local history do not fabricate identity continuity

Required verification output

- `structural_reuse_digest`
- `identity_separation_report`
- `replay_digest`
- `diagnostics_digest`

Pass condition

Reuse is explicit and helpful, but never becomes accidental identity authority.

### 9. Branch Comparison Drift Test

Purpose

Prove that structural branch comparison stays deterministic under drift,
oscillation, and near-match histories.

Scenario

- compare branches with repeated small structural edits
- oscillate between near-identical states
- replay comparisons after restart and after additional unrelated publication

Must verify

- comparison outcomes remain deterministic
- branch-local drift does not contaminate other branches
- explanation surfaces can localize what changed structurally

Required verification output

- `branch_compare_digest`
- `structural_diff_report`
- `replay_digest`
- `counter_snapshot`

Pass condition

Branch comparison remains deterministic, local, and explainable.

## Milestone 9 Named Certification Suites

### 10. Merge Parent Order Determinism Test

Purpose

Prove that ordered multi-parent truth history is consumed deterministically and
that parent order is never treated as incidental.

Scenario

- construct merge-bearing histories with ordered parent lists
- vary host adapter ordering and replay paths
- vary canonical relational merge-class labels that lower into the same
  bridge-consumption class
- run invalidation, continuity, and remapping over those histories

Must verify

- parent ordering survives bridge ingestion and replay
- merge-influenced routing changes are deterministic
- no single-parent fallback assumptions leak into bridge logic
- bridge class lowering remains losslessly traceable back to canonical
  relational merge ontology
- canonical result bundles stay identical across adapter-order variation, not
  only routing digests

Required verification output

- `merge_history_digest`
- `merge_ontology_mapping_report`
- `parent_order_report`
- `routing_digest`
- `result_bundle_digest`
- `replay_digest`

Pass condition

Ordered multi-parent histories produce identical canonical result bundles across
runs, and every lowered bridge merge class remains losslessly attributable to
canonical relational merge ontology.

### 11. Unsupported Merge Class Denial Test

Purpose

Prove that unsupported merge classes fail explicitly and typed instead of
degrading into heuristic branch reconciliation.

Scenario

- feed supported and unsupported merge classes through the same bridge surface
- vary causal frontier metadata and merge-policy outcomes
- include cases denied at different precedence stages:
  lineage, deletion/topology gates, causal admissibility, and policy outcome
- attempt replay and diagnostics queries over rejected cases

Must verify

- unsupported classes are rejected before semantic drift occurs
- diagnostics distinguish unsupported merge class from malformed history
- rejected merges leave no misleading derived artifacts
- denial artifacts identify the exact precedence stage that blocked
  continuation or publication
- structural similarity never reopens continuity or remap after an
  authoritative merge denial
- rejected cases preserve canonical ontology provenance even when no
  bridge-level publication artifact is emitted

Required verification output

- `merge_support_matrix`
- `merge_denial_stage_report`
- `failure_digest`
- `diagnostics_digest`
- `counter_snapshot`

Pass condition

Unsupported or denied merge cases fail closed, remain mechanically attributable,
and cannot be reinterpreted into continuity or reconciliation through
structural convenience.

### 12. Merge Replay And Explanation Parity Test

Purpose

Prove that merge-bearing histories replay identically and that diagnostics can
explain merge-influenced invalidation and continuity behavior.

Scenario

- run merge-bearing histories through original execution
- replay from canonical bridge artifacts
- reconstruct explanations and continuity records for selected subscriptions
- vary diagnostics richness and adapter implementation shape across equivalent
  replay lanes
- include histories whose lowering requires non-trivial causal-frontier,
  lineage, and structural consultation work

Must verify

- replay matches original merge-aware routing
- explanation artifacts identify merge inputs and merge outcomes exactly
- continuity behavior remains branch- and merge-aware after restart
- replay matches the original full canonical result bundle:
  routing, continuity, remap, denial/failure, and explanation
- diagnostics richness changes retained detail only, not causal meaning
- discovery-work counters remain bounded and parity-safe across equivalent runs

Required verification output

- `merge_history_digest`
- `result_bundle_digest`
- `continuity_digest`
- `explanation_digest`
- `replay_digest`
- `counter_snapshot`

Pass condition

Merge-aware replay is bundle-equivalent and causally exact: routing,
continuity, remap, denial, explanation, and bounded discovery work all remain
parity-safe across replay.

## Milestone 10 Named Certification Suites

### 13. Speculative Discard Zero-Residue Test

Purpose

Prove that discarded preview and speculative flows leave no authoritative bridge
residue.

Scenario

- open speculative truth and compute branches
- run preview evaluation and branch-local derived work
- discard selected speculative flows after heavy routing and diagnostics use

Must verify

- discard leaves zero authoritative stream, checkpoint, routing, or writeback residue
- temporary bridge resources are fully reclaimed
- later authoritative runs do not see speculative leftovers

Required verification output

- `speculative_resource_digest`
- `discard_residue_report`
- `routing_digest`
- `counter_snapshot`

Pass condition

Discarded preview work is fully non-authoritative and leaves no residue.

### 14. Speculative Commit Boundary Clarity Test

Purpose

Prove that speculative outcomes become authoritative only through explicit
commit semantics and remain distinguishable from previews.

Scenario

- run parallel preview and authoritative flows over similar branch states
- commit some speculative results and discard others
- replay both histories after restart

Must verify

- committed speculative flows become canonical and explainable
- preview and authoritative outcomes cannot be confused in artifacts
- branch identity remains explicit across commit and discard transitions

Required verification output

- `speculative_commit_digest`
- `preview_vs_authoritative_matrix`
- `replay_digest`
- `diagnostics_digest`

Pass condition

Commit and discard boundaries stay explicit, typed, and replay-safe.

### 15. Preview Lifecycle Leak Resistance Test

Purpose

Prove that repeated preview churn, branch churn, and diagnostics churn do not
leak temporary resources or blur branch-local semantics.

Scenario

- create and discard many preview sessions
- vary branch-local truth views and diagnostics richness
- interleave authoritative work with preview churn

Must verify

- temporary resource lifecycles stay bounded
- branch-local preview state never leaks into authoritative runs
- diagnostics retention policy does not silently retain authoritative-looking preview state

Required verification output

- `preview_lifecycle_digest`
- `resource_bound_report`
- `branch_isolation_matrix`
- `counter_snapshot`

Pass condition

Preview churn remains bounded and isolated.

## Milestone 11 Named Certification Suites

### 16. Policy Provenance Equivalence Test

Purpose

Prove that deterministic and optimized policy modes can change execution policy
without changing the canonical explanation of what policy changed.

Scenario

- run identical bridge flows under deterministic and optimized modes
- vary artifact policy and diagnostics policy explicitly
- compare policy provenance after replay

Must verify

- policy provenance artifacts explain exactly which policy surfaces changed behavior
- identical policy inputs produce identical policy digests
- replay preserves policy-source attribution

Required verification output

- `policy_digest`
- `policy_provenance_report`
- `routing_digest`
- `replay_digest`

Pass condition

Policy changes are explicit, replay-safe, and non-ambient.

### 17. Illegal Policy Combination Rejection Test

Purpose

Prove that illegal or unsupported cross-runtime policy combinations fail
explicitly rather than silently degrading into defaults.

Scenario

- request incompatible combinations of deterministic mode, optimization mode,
  diagnostics richness, and artifact retention
- vary host and truth-side policy inputs

Must verify

- invalid combinations are rejected before execution
- failure classes localize policy-source ambiguity versus policy illegality
- no fallback default hides a bad policy request

Required verification output

- `policy_matrix`
- `failure_digest`
- `diagnostics_digest`

Pass condition

Unsupported policy combinations fail early, typed, and non-ambiguously.

### 18. Ambient Policy Leak Resistance Test

Purpose

Prove that policy does not leak across branches, requests, or consumer contexts
through hidden ambient state.

Scenario

- alternate bridge flows with different policy bundles
- interleave branch-local and historical requests
- replay with reordered host call sequences

Must verify

- each request consumes only its declared policy bundle
- branch-local flows do not inherit stale policy from prior requests
- reordered host execution does not change policy attribution

Required verification output

- `policy_digest`
- `request_policy_matrix`
- `replay_digest`
- `counter_snapshot`

Pass condition

Policy propagation remains explicit and request-scoped.

## Milestone 12 Named Certification Suites

### 19. Bridge Writeback Idempotence And Diff Truth Test

Purpose

Prove that bridge-mediated writeback is idempotent where declared and that the
bridge reflects only explicit output diffs into truth mutation plans.

Scenario

- run signal-driven writeback over repeated equivalent inputs
- compare no-op, changed-output, and repeated-output cases
- replay writeback histories after restart

Must verify

- idempotent writeback emits no extra authoritative mutations
- output diffs are explicit and replay-safe
- read-only and writeback flows remain distinguishable

Required verification output

- `writeback_digest`
- `mutation_plan_digest`
- `idempotence_report`
- `replay_digest`

Pass condition

Equivalent writeback inputs do not corrupt truth with duplicate authority effects.

### 20. Strategy Failure Containment Test

Purpose

Prove that failing, panicking, or divergent bridge-mediated strategies do not
corrupt authoritative truth.

Scenario

- inject failures during mutation-plan production, validation, and commit handoff
- run successful and failing strategies over equivalent inputs
- compare authoritative truth before and after failure

Must verify

- failed strategies publish no authoritative truth
- failure diagnostics distinguish strategy failure from truth-runtime rejection
- later successful retries match the no-failure control run

Required verification output

- `writeback_digest`
- `failure_digest`
- `truth_integrity_report`
- `retry_parity_digest`

Pass condition

Strategy failure is contained, typed, and non-corrupting.

### 21. Authority Bypass Rejection Test

Purpose

Prove that bridge-mediated writeback cannot bypass invariant, merge, or commit
authority.

Scenario

- attempt bridge flows that try to skip invariant validation
- attempt merge-bearing writeback without admitted merge authority
- attempt direct publication-shaped shortcuts

Must verify

- all bypass attempts fail explicitly and typed
- the bridge remains an effect producer, not a second commit authority
- rejection artifacts localize the blocked authority boundary

Required verification output

- `authority_boundary_matrix`
- `failure_digest`
- `diagnostics_digest`

Pass condition

The bridge cannot create a parallel authority path into truth mutation.

## Milestone 12b Named Certification Suites

### 22. Multi-Family Writeback Admission Boundary Test

Purpose

Prove that multiple writeback families can be admitted through one bridge-owned
protocol boundary without turning new families into bridge-core special cases
or opaque host payloads.

Scenario

- admit at least two materially different writeback families
- run equivalent and non-equivalent requests through each family
- attempt undeclared family, wrong-family, and opaque-family admission
- attempt compile-time construction of unwired or skipped-phase family values
- replay admitted and rejected family-bearing paths

Must verify

- family identity remains explicit from admission through replay
- different families do not alias into one bridge writeback meaning
- undeclared or opaque family paths fail before authority execution
- family-aware diagnostics remain bridge-native rather than host-defined
- undeclared families and skipped proof phases are uncompilable wherever the
  type system can enforce them

Required verification output

- `writeback_family_digest`
- `family_contract_digest`
- `bridge_effect_digest`
- `failure_digest`
- `replay_digest`
- `family_admission_record_digest`
- `decision_trace_digest`
- `counter_snapshot`

Pass condition

Multiple families can enter the bridge honestly, and family identity remains a
first-class bridge artifact rather than host-local folklore.

### 23. Cross-Family Replay And Loop Isolation Test

Purpose

Prove that replay, idempotence, and loop-prevention semantics remain
family-correct and do not collapse across writeback families that may appear
structurally similar.

Scenario

- run same-causality and changed-causality flows through at least two admitted
  families
- include same-output-lookalike and different-output family pairs
- replay original and restart-shaped family-bearing histories
- inject bridge-origin feedback pressure into multiple families
- assert exact family lookup, family dispatch, and decision-log retention
  counters on the hot path

Must verify

- cross-family lookalike outputs do not alias replay identity when family
  semantics differ
- loop-prevention classification remains family-visible and replay-stable
- idempotence suppression remains scoped to the correct family contract
- same-family equivalent runs remain equal while cross-family divergent runs
  remain mechanically distinct
- family tracing remains reconstructable from retained native execution and
  replay records alone

Required verification output

- `writeback_family_digest`
- `causality_digest`
- `idempotence_report`
- `loop_prevention_report`
- `replay_digest`
- `counter_snapshot`
- `family_execution_record_digest`
- `family_replay_record_digest`
- `decision_trace_digest`

Pass condition

Replay and feedback pressure preserve family-specific meaning instead of
flattening all families into one writeback lane.

### 24. Host Mapper Parity And Shadow-Protocol Rejection Test

Purpose

Prove that host mappers remain translation layers into admitted family
contracts rather than becoming shadow writeback protocols that redefine bridge
semantics outside canonical artifacts.

Scenario

- run equivalent family-bearing workloads through at least two host mappers
- vary mapper implementation shape while preserving admitted family semantics
- attempt mapper-side redefinition of no-op, retry, failure, or family identity
- attempt family execution with missing bridge-visible mapper evidence
- attempt mapper outputs that try to author replay identity, idempotence
  identity, loop disposition, failure class, or authority classification

Must verify

- mapper-only implementation differences do not change canonical bridge meaning
- host attempts to redefine bridge protocol semantics fail explicitly
- mapper paths cannot hide family-specific authority or loop semantics inside
  host-local strings or opaque payloads
- parity holds only when the mapper is translation, not shadow protocol
- illegal mapper outputs are compile-time rejected where the API can enforce it
- mapper parity and rejection are explainable from retained mapper and
  execution records without reopening host code

Required verification output

- `writeback_family_digest`
- `mapper_parity_matrix`
- `authority_boundary_matrix`
- `failure_digest`
- `diagnostics_digest`
- `family_mapper_record_digest`
- `family_execution_record_digest`
- `decision_trace_digest`
- `counter_snapshot`

Pass condition

Host mappers can translate into bridge writeback families, but they cannot
become the real writeback protocol.

## Milestone 13 Named Certification Suites

### Milestone 13 Reference Workload Requirement

Milestone 13 must include one concrete Rust-only reference workload in addition
to the abstract suite definitions below.

The required reference shape is:

- authoritative products and component costs in `forge-relational`
- derived tariff, tax, margin, and final-price nodes in `forge-signal`
- bridge-coordinated live updates, speculative branch-local shocks, discard,
  commit promotion, replay, and diagnostics over the boundary

Minimum required reference scenarios:

- a high-fanout main-branch component-cost wave over at least 100 products
- a speculative branch-local `rubber +300%` style shock that remains isolated
  from the main branch
- a main-versus-speculative branch comparison bundle over the same fork basis
- a speculative discard lane proving zero authoritative and bridge residue
- a speculative commit-promotion lane proving clear authority-boundary
  promotion through the Milestone 12 writeback contract
- a restart-safe replay lane over the same ordinary pricing route
- a restart-shaped replay-drift lane proving typed mismatch localization after
  truth-shape change

The reference workload should converge on one top-level workload certification
bundle rather than remaining a loose set of unrelated end-to-end assertions.
At minimum that workload bundle should contain nested artifacts for:

- ordinary path reference and replay
- aspect-aware routing and comparison
- hostile missing-snapshot or missing-basis failure
- discard lifecycle under interleaved main churn
- promotion lifecycle under interleaved main churn
- 100-product high-fanout live churn
- restart-safe replay
- restart-shaped replay drift rejection
- writeback authority outcomes covering commit, noop, and typed rejection
- merge-bearing pricing history with revisitable pre-merge, speculative, and
  merged truth states
- historical provenance sufficient to inspect retained shock criteria from
  bridge-visible truth at a historical commit

This workload is a certification fixture, not a bridge-owned finance product
surface. Its role is to prove that the bridge can carry a concrete dual-runtime
story end to end without relying on a UI or on host-local debug folklore.

### 25. End-To-End Causality Bundle Equivalence Test

Purpose

Prove that causality survives from truth commit through bridge routing into
derived explanation and replay.

Scenario

- run change, routing, historical, merge-bearing, and policy-bearing bridge flows
- capture original execution bundles
- replay from canonical bridge artifacts
- compare end-to-end causality records

Must verify

- the same causality tokens survive original execution and replay
- explanation surfaces remain aligned with routing and truth-view records
- diagnostics tiers change retained richness only
- the reference workload preserves the same main-branch and speculative-branch
  causality digests across original execution and replay
- retained historical pricing commits can expose their upstream shock criteria
  through bridge-visible truth and bundle artifacts rather than hidden scenario
  memory

Required verification output

- `causality_digest`
- `routing_digest`
- `explanation_digest`
- `replay_digest`
- `reference_workload_bundle_digest`

Pass condition

End-to-end bridge causality is canonical, replay-safe, and mechanically inspectable.

### 26. Failure Taxonomy Localization Test

Purpose

Prove that bridge-native failure classes are complete enough to localize real
adversarial failures without collapsing into host strings.

Scenario

- inject failures across stream, source, remap, merge, preview, policy, and
  writeback paths
- compare typed failure capture across original execution and replay

Must verify

- failures map into explicit bridge-native classes
- failure localization identifies the exact failed protocol or planning boundary
- replay preserves failure meaning
- reference-workload failures such as wrong-branch comparison, preview misuse,
  source mismatch, and writeback denial remain typed and replay-stable
- retained provenance and residue surfaces are sufficient to distinguish
  routing, branch-isolation, policy, source, preview, merge, writeback, and
  residue failures mechanically

Required verification output

- `failure_digest`
- `failure_localization_matrix`
- `replay_failure_digest`
- `diagnostics_digest`
- `reference_workload_failure_bundle_digest`

Pass condition

Bridge failures are typed, composable, and replay-stable.

### 27. Certification Matrix Sufficiency Test

Purpose

Prove that the bridge certification bundle itself is sufficient to diagnose
routing, merge-aware, policy-aware, historical, preview, and writeback failures
mechanically.

Scenario

- run a mixed milestone 6-13 workload matrix
- produce canonical certification bundles only
- attempt offline diagnosis from those bundles without live host/runtime access

Must verify

- certification artifacts are enough to distinguish the major failure families
- bundle completeness does not depend on ambient runtime state
- the bridge has one coherent public diagnostics entrypoint
- the reference workload can be diagnosed offline for main-branch live runs,
  speculative runs, discard residue checks, and commit-promotion outcomes
- the reference workload can diagnose retained historical shock lineage from
  canonical artifacts alone

Required verification output

- `certification_bundle_digest`
- `bundle_completeness_report`
- `diagnostics_entrypoint_matrix`
- `counter_snapshot`
- `reference_workload_bundle_digest`
- `reference_workload_bundle_comparison`

Pass condition

The bridge can be diagnosed and certified from its canonical artifacts alone.

## What These Tests Collectively Prove

Together, these tests prove that the bridge from Milestone 6 onward is:

- protocol-grade rather than adapter-grade
- replay-safe under restart, backpressure, and multi-consumer pressure
- host-agnostic at its public source and stream boundaries
- explicit about structural identity, merge semantics, speculative flows, and policy provenance
- lossless about authority provenance when lowering parent-runtime truth into
  bridge-consumption vocabularies
- unable to bypass truth authority during writeback
- certifiable through canonical artifacts rather than intuition

## Milestone 6+ Certification Rule

No Milestone 6+ bridge capability should be considered closed until its named
certification suites emit canonical machine-checkable outputs and pass across:

- original execution
- replay from canonical bridge artifacts
- hostile adapter or scheduling variation
- diagnostics-tier variation where admitted

Without that, the bridge may still be promising, but it is not yet trust-grade.
