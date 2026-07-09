# Projection Contract Consumption

## What This Feature Is

Projection contract consumption lets a domain declare which projection facts it
consumes after an admitted plan and materialize a stable Query projection
contract.

## Why You Use It

- you want typed projection aftermath instead of re-reading source authority
- you need a stable contract for geometry facts such as visible fields, target
  identity, or residue
- you want projection use to participate in the same contribution and
  certification story as other domain capabilities

## Stable Entry Points

- `worth_query_domain(...).for_admitted_intent_plan(...).consumes_projection_contract(...).because(...).materialize()`
- `worth_query_domain(...).for_admitted_intent_plan(...).establishes_projection_contract(...).because(...).materialize()`
- `worth_query_domain(...).for_admitted_intent_plan(...).declares_projection_residue(...).because(...).materialize()`

Supporting request type:

- `WorthQueryProjectionContractRequest`

## Core Mental Model

This feature is the aftermath side of projection consumption, not raw read
materialization.

The domain declares:

- which projection source is being consumed
- which binding context applies
- which materialized facts are required

Query then owns the review, eligibility, admitted, and final contract artifacts.

## How It Executes

1. start with an admitted intent plan
2. build a `WorthQueryProjectionContractRequest`
3. choose establish, consume, or residue posture
4. materialize the contract or a lower lane such as review or eligibility

## Small Example

```rust
let contract = worth_query_domain("worth.spatial")
    .for_admitted_intent_plan(&plan)
    .consumes_projection_contract(
        "projection.edge_display",
        request,
    )
    .because("the edge display projection is consumed by the topology tool")
    .materialize()?;
```

## Real Example

```rust
let contract = worth_query_domain("worth.spatial")
    .for_admitted_intent_plan(&plan)
    .declares_projection_residue(
        "projection.face_residue",
        request,
    )
    .because("face decomposition leaves residue facts that must stay inspectable")
    .materialize()?;

let digest = contract.contract_digest();
let query_digest = contract.query_digest();
```

## How It Relates To Other Features

- [Aftermath Review, Support, Eligibility, And Materialization](./aftermath-review-support-eligibility-and-materialization.md)
  explains the full lane progression around the same request
- [Projection Consumption](../../capabilities/projection-consumption.md)
  explains the broader Query feature that consumes projection facts from source
  artifacts

## Inspection And Debugging

- the contract digest is the stable identity to compare across lanes
- if you need pre-materialization introspection, stay on the review or
  eligibility lanes instead of forcing final materialization

## Anti-Patterns

- reopening source authority instead of consuming typed projection aftermath
- teaching establish, consume, and residue posture as interchangeable
- treating the request wrapper as if it were the canonical contract

## Current Limits

- the ordinary aftermath lane is admitted-plan-bound
- projection contract requests still carry real projection-source vocabulary,
  because that is the honest authority boundary today

## Related Docs

- [Aftermath Review, Support, Eligibility, And Materialization](./aftermath-review-support-eligibility-and-materialization.md)
- [Projection Consumption](../../capabilities/projection-consumption.md)
