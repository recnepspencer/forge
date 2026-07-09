# Common Performance Claims And Layout Intent

## What This Feature Is

This is the descriptive entry lane for Milestone 8 performance vocabulary. You
use it to state what boundary a claim describes, how strong the claim is right
now, what work is included and excluded, what freshness and fallback posture
apply, and what layout family the path was designed around. It does not lower
into policy results, canonical bundles, executed receipts, reports, or
proof-bearing certification.

## Why You Use It

- to declare honest performance meaning before any execution or report work
- to keep layout intent separate from claim boundary and evidence strength
- to make hot-path, support, replay, and policy-admission claims mechanically
  non-substitutable
- to disclose included and excluded work at the point the claim is authored

## Stable Entry Points

- `worth_foundational::performance_api::common_path`
- `worth_foundational::performance_api::common_path::performance()`
- `FoundationalPerformanceClaimAuthoringFrontDoor`
- `FoundationalLayoutIntentClaim`

Good to know:

- this lane is stable and shipped
- it is descriptive only; it does not claim executed counter rows or stronger
  certification
- if you need runtime admission, move to
  [Policy Admission Receipts](./policy-admission-receipts.md)

## Core Mental Model

Truth in this lane lives in the claim shape itself.

- a `PerformanceClaim` says what boundary is being described
- evidence strength says how much proof exists now, not how good the path is
- included and excluded work are part of the meaning, not optional garnish
- `LayoutIntentClaim` explains representation family, access posture, and
  allocation posture without claiming cost equivalence

The handle you hold here is still descriptive. The runtime has not yet admitted
budget, attached exact counter rows, widened into support-bearing reports, or
minted stronger proof-bearing bundles.

## How It Executes

The common lane is an authoring and legality lane, not an execution lane.

1. Choose the claim family:
   authoritative execution, support-derived, replay/materialization, or
   policy-admission.
2. Fill in boundary, evidence strength, breadth/locality, access posture,
   execution temperature, freshness/retention, and fallback/debt posture.
3. Declare included and excluded work explicitly.
4. Finish the claim and let the legality floor reject contradictory shapes.
5. Optionally define a `LayoutIntentClaim` and attach it to the claim.

No runtime policy, no canonical comparison, and no report widening happen here.

## Small Example

```rust
use worth_foundational::performance_api::common_path as performance;

let claim = performance::performance()
    .claim()
    .authoritative_execution()
    .boundary_authoritative_execution()
    .counter_backed_execution()
    .hot_path()
    .point_local()
    .point_lookup()
    .exact_basis_current()
    .verified()
    .include_authoritative_mutation_work()
    .exclude_support_report_assembly_work()
    .exclude_replay_reconstruction_work()
    .exclude_forensic_parity_work()
    .finish()?;
```

This is the smallest honest hot-path example because it names boundary,
strength, temperature, freshness, fallback, and work disclosure explicitly.

## Real Example

```rust
use worth_foundational::performance_api::common_path as performance;
use worth_foundational::FoundationalPerformanceAllocationPosture;

let claim = performance::performance()
    .claim()
    .policy_admission()
    .boundary_authoritative_execution()
    .runtime_policy_admission()
    .delta_bound()
    .point_lookup()
    .warm_path()
    .exact_basis_current()
    .deferred()
    .include_validation_planning_work()
    .exclude_support_report_assembly_work()
    .finish()?;

let layout = performance::performance().define_layout_intent(
    worth_foundational::FoundationalPerformanceLayoutIntent::AoS,
    worth_foundational::FoundationalPerformanceAccessPatternPosture::PointLookup,
    FoundationalPerformanceAllocationPosture::ActionLocal,
);

let annotated = performance::performance().attach_layout_intent(claim, layout)?;
```

What is authoritative:

- the claim meaning itself

What is derived:

- the attached layout explanation

What gets retained:

- only descriptive meaning

What gets inspected:

- boundary, evidence strength, postures, and work disclosure

## How It Relates To Other Features

- Pair this with
  [Policy Admission Receipts](./policy-admission-receipts.md) when a runtime
  policy decision has been made but execution has not yet happened.
- Pair it with
  [Canonical Bundles And Comparison](./canonical-bundles-and-comparison.md)
  when independent producers need shared lowering and comparison semantics.
- Pair it with
  [Performance Report Planning And Materialization](./performance-report-planning-and-materialization.md)
  only after you need broader support-bearing visibility.

## Inspection And Debugging

Inspect these surfaces directly:

- `claim.boundary()`
- `claim.evidence_strength()`
- `claim.included_work()`
- `claim.excluded_work()`
- `claim.execution_temperature()`
- `claim.freshness_retention()`

If claim construction fails, the denial is part of the feature:

- missing disclosure
- incompatible evidence strength for the claim family
- contradictory hot/support or freshness combinations
- overlapping included and excluded work

## Anti-Patterns

- using layout intent as if it proves executed cost
- using a support-derived or replay-derived claim where current-basis
  authoritative execution is required
- omitting included or excluded work from a verified hot-path claim
- treating this lane as a policy or receipt API

## Current Limits

- this lane is descriptive only; it does not admit budgets or attach rows
- layout intent does not imply cost equivalence across representation families
- stronger proof-bearing certification belongs to
  [Certified And Readmitted Performance Bundles](./certified-and-readmitted-performance-bundles.md)

## Related Docs

- [Policy Admission Receipts](./policy-admission-receipts.md)
- [Canonical Bundles And Comparison](./canonical-bundles-and-comparison.md)
- [Grouped Public Lanes And Stronger Readiness](./grouped-public-lanes-and-stronger-readiness.md)
