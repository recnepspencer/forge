# Aftermath Review, Support, Eligibility, And Materialization

## What This Feature Is

The aftermath category exposes one step-at-a-time path from ordinary domain
drafts to review, support report, eligibility, admitted consumption, and final
projection contract materialization.

## Why You Use It

- you need to stop before final contract materialization sometimes
- you want warning-bearing support or eligibility truth for projection aftermath
- you need an inspectable lane for certification, tooling, or geometry workflow
  debugging

## Stable Entry Points

From an admitted-plan aftermath contribution:

- `.review()`
- `.materialize_support_report()`
- `.materialize_eligibility()`
- `.materialize_admitted()`
- `.materialize()`

Checked variants:

- `.try_review()`
- `.try_materialize_support_report()`
- `.try_materialize_eligibility()`
- `.try_materialize_admitted()`
- `.try_materialize()`

## Core Mental Model

This is one progression, not five unrelated helper methods.

You are holding one domain aftermath contribution. Query can then stop at the
inspection depth you actually need.

## How It Executes

1. author the aftermath contribution
2. choose the lane depth you need
3. let Query run the same core progression through the selected stop point

## Small Example

```rust
let review = forge_query_domain("worth.spatial")
    .for_admitted_intent_plan(&plan)
    .establishes_projection_contract("projection.review", request)
    .because("the projection contract should stay inspectable before commitment")
    .review()?;
```

## Real Example

```rust
let support = forge_query_domain("worth.spatial")
    .for_admitted_intent_plan(&plan)
    .establishes_projection_contract("projection.warning", request)
    .because("query-context projection aftermath should preserve warning truth")
    .materialize_support_report()?;

let eligibility = forge_query_domain("worth.spatial")
    .for_admitted_intent_plan(&plan)
    .establishes_projection_contract("projection.warning", request)
    .because("query-context projection aftermath should preserve warning truth")
    .materialize_eligibility()?;
```

Use that shape when you need to inspect warning-bearing support without forcing
final contract materialization.

## How It Relates To Other Features

- [Projection Contract Consumption](./projection-contract-consumption.md) is the
  feature-level contract story
- [Projection Consumption](../../capabilities/projection-consumption.md) is the
  broader capability family that consumes typed facts from source artifacts

## Inspection And Debugging

- use support reports when you want support posture row detail
- use eligibility when you need admitted, warning, deferred, or mismatch truth
- use review when you want the Query-owned inspected lane without final contract
  materialization

## Anti-Patterns

- forcing final contract materialization when review or eligibility is enough
- duplicating the same aftermath progression outside Query
- assuming support reports and eligibility are redundant

## Current Limits

- the ordinary lane is admitted-plan-bound
- aftermath requests still expose real projection vocabulary because projection
  authority is the honest substrate boundary

## Related Docs

- [Projection Contract Consumption](./projection-contract-consumption.md)
- [Projection Consumption](../../capabilities/projection-consumption.md)
