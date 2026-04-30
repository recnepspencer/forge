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


## Section Index

- [Milestones 1-4](./test-requirements-milestones-1-4.md)
- [Milestones 5-5.6](./test-requirements-milestones-5-5_6.md)
- [Milestones 6-8](./test-requirements-milestones-6-8.md)
- [Milestones 9-9.2](./test-requirements-milestones-9-9_2.md)
- [Milestone 9.3 And Runtime Gates](./test-requirements-milestone-9_3-and-runtime-gates.md)
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
