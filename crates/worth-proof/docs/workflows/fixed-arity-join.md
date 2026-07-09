# Fixed-Arity Join

## What This Feature Is

This workflow shows how to split one artifact into two explicit lanes and then join two explicit artifact lanes back into one artifact or one ready-recipe join result.

## Why You Use It

- you have fixed-arity static composition
- proof and basis routing must stay explicit
- you want static composition without a dynamic graph engine

## Stable Entry Points

- pleasant lane:
  - `use worth_proof::prelude::*;`
  - `join_ready(left, right)`
  - `compose_ready(left_outcome, || right_outcome)`
- raw lane:
  - `use worth_proof::raw::*;`
  - `fork_artifact_pair(...)`
  - `ForkOutputs2`
  - `join_artifact_pair(...)`
  - `JoinInputs2`
  - `join_ready_recipe_pair(...)`
  - `compose_join_ready_recipe_pair(...)`

## Core Mental Model

This workflow is about explicit redistribution and recomposition.

The important law is:

- payload, proof, and basis routing are caller-owned decisions
- the fixed-arity carriers make those lane boundaries visible

This is true both for generic artifacts and for ready-recipe joins.

## Pleasant Lane First

```rust
use worth_proof::prelude::*;

fn join<LA, LB, A, B>(
    left: worth_proof::ExecutionReadyRecipe<LA, A>,
    right: worth_proof::ExecutionReadyRecipe<LB, B>,
) {
    let joined = join_ready(left, right);
    let _ = joined.payload().left();
}
```

For checked composition:

```rust
use worth_proof::prelude::*;

fn checked_join<LA, LB, A, B>(
    left: worth_proof::TransitionOutcome<worth_proof::ExecutionReadyRecipe<LA, A>, &'static str>,
    right: worth_proof::ExecutionReadyRecipe<LB, B>,
) {
    let joined = compose_ready(left, || worth_proof::TransitionOutcome::success(right));
    let _ = joined;
}
```

What this keeps visible:

- ready-only join remains a distinct API from generic artifact join
- non-success short-circuiting is still explicit in the checked helper
- the pleasant lane does not hide fixed-arity position

## Equivalent Raw Surface

```rust
use worth_proof::raw::*;

fn join<LA, LB, A, B>(
    left: ExecutionReadyRecipe<LA, A>,
    right: ExecutionReadyRecipe<LB, B>,
) {
    let joined = compose_join_ready_recipe_pair(
        SuccessfulTransitionOutcome::new(left).into(),
        || SuccessfulTransitionOutcome::new(right).into(),
    );

    let _ = joined;
}
```

Use the raw lane when:

- you are routing payload, proof, and basis explicitly through `fork_artifact_pair(...)`
- you need direct access to `ForkOutputs2` or `JoinInputs2`
- you are building a domain-specific fixed-arity composition helper

## How It Relates To Other Features

- Use [Ready Recipe Join](../features/ready-recipe-join.md) when both inputs are already execution-ready recipes.
- Use [Happy-Path Recipe Progression](./happy-path-recipe-progression.md) before this when the inputs still need to become stronger first.
- Use [Composition-Family Lowering](./composition-family-lowering.md) when the fixed-arity structure feeds same-family lifecycle lowering.

## Inspection And Debugging

- inspect fork and join closures first; that is where routing law actually lives
- inspect the lane carriers rather than raw tuples when position mistakes are suspected
- if proof or basis shape looks wrong afterward, the routing closure is the first suspect

## Anti-Patterns

- Do not duplicate proof or basis lanes implicitly during fork.
- Do not treat join as a generic merge bag when positional meaning matters.
- Do not reach for a dynamic composition abstraction when the shape is fixed and explicit.

## Related Docs

- [Fork And Join](../features/fork-and-join.md)
- [Ready Recipe Join](../features/ready-recipe-join.md)
- [Composition-Family Lowering](./composition-family-lowering.md)
