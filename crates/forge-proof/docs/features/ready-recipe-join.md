# Ready Recipe Join

## What This Feature Is

Ready-recipe join combines two execution-ready recipes into one execution-ready joined recipe while preserving both payload positions and both basis positions explicitly.

## Why You Use It

- you already have two ready recipes
- you need one fixed-arity joined ready form
- you want non-success join composition to short-circuit honestly

## Stable Entry Points

- `join_ready_recipe_pair(...)`
- `compose_join_ready_recipe_pair(...)`
- `JoinInputs2<L, R>`

## Core Mental Model

This feature is narrower than generic artifact join.

Its laws are:

- both inputs are already execution-ready
- output payload remains a `JoinInputs2<L, R>`
- output basis remains a `JoinInputs2<LA, RA>`
- checked composition short-circuits non-success lanes before evaluating later inputs

It deliberately avoids collapsing two basis lanes into one fake monolithic basis.

## How It Executes

Plain ready join:

1. take `JoinInputs2<ExecutionReadyRecipe<L, LA>, ExecutionReadyRecipe<R, RA>>`
2. extract both payload and basis positions
3. return `ExecutionReadyRecipe<JoinInputs2<L, R>, JoinInputs2<LA, RA>>`

Composed ready join:

1. take a left `TransitionOutcome<ExecutionReadyRecipe<...>>`
2. lazily supply the right lane with a closure
3. join only if the left lane succeeded
4. preserve non-success categories unchanged otherwise

## Small Example

```rust
use forge_proof::{ExecutionReadyRecipe, JoinInputs2};

type Joined = ExecutionReadyRecipe<JoinInputs2<u8, u16>, JoinInputs2<u32, u64>>;
let _ = std::any::type_name::<Joined>();
```

This is the smallest honest example because public callers normally receive ready recipes from progression rather than minting them directly.

## Real Example

```rust
use forge_proof::{
    compose_join_ready_recipe_pair, ExecutionReadyRecipe, JoinInputs2, SuccessfulTransitionOutcome,
    TransitionOutcome,
};

fn join<LA, LB, A, B>(
    left: ExecutionReadyRecipe<LA, A>,
    right: ExecutionReadyRecipe<LB, B>,
) {
    let joined: TransitionOutcome<
        ExecutionReadyRecipe<JoinInputs2<LA, LB>, JoinInputs2<A, B>>,
    > = compose_join_ready_recipe_pair(
        SuccessfulTransitionOutcome::new(left).into(),
        || SuccessfulTransitionOutcome::new(right).into(),
    );

    let _ = joined;
}
```

What this shows:

- the joined ready surface preserves both positions explicitly
- the join happens at the ready layer, not at raw lowered stage
- success composition stays lazy and outcome-aware

## How It Relates To Other Features

- Pair this with [Execution-Ready And Executed](./execution-ready-and-executed.md) because the joined surface is still an execution-ready recipe.
- Pair this with [Transition Outcomes](./transition-outcomes.md) because composed ready join preserves non-success categories.
- Pair this with [Fork And Join](./fork-and-join.md) when the same domain also uses more general fixed-arity artifact composition.

## Inspection And Debugging

- inspect joined payload with `.payload().left()` and `.payload().right()`
- inspect joined basis with `.basis().left()` and `.basis().right()`
- if the right lane seems to execute unexpectedly, inspect the lazy closure path in composed join usage

## Anti-Patterns

- Do not flatten two basis positions into one ad hoc basis value.
- Do not join lowered recipes through this surface; it is for ready recipes only.
- Do not eagerly compute the right lane when the join should short-circuit on earlier non-success.

## Current Limits

- only pairwise ready join exists here
- the surface is intentionally explicit rather than fluent
- readiness is assumed, not established, by this feature

## Related Docs

- [Execution-Ready And Executed](./execution-ready-and-executed.md)
- [Transition Outcomes](./transition-outcomes.md)
- [Fork And Join](./fork-and-join.md)
