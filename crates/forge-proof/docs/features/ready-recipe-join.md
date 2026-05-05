# Ready Recipe Join

## What This Feature Is

Ready-recipe join combines two execution-ready recipes into one execution-ready joined recipe while preserving both payload positions and both basis positions explicitly.

## Why You Use It

- you already have two ready recipes
- you need one fixed-arity joined ready form
- you want non-success join composition to short-circuit honestly

## Stable Entry Points

- pleasant lane:
  - `use forge_proof::prelude::*;`
  - `join_ready(left, right)`
  - `compose_ready(left_outcome, || right_outcome)`
- raw lane:
  - `use forge_proof::raw::*;`
  - `join_ready_recipe_pair(...)`
  - `compose_join_ready_recipe_pair(...)`
  - `JoinInputs2<L, R>`

## Core Mental Model

Its laws are:

- both inputs are already execution-ready
- output payload remains a `JoinInputs2<L, R>`
- output basis remains a `JoinInputs2<LA, RA>`
- checked composition short-circuits non-success lanes before evaluating later inputs

## Pleasant Lane First

```rust
use forge_proof::prelude::*;

fn join<LA, LB, A, B>(
    left: forge_proof::ExecutionReadyRecipe<LA, A>,
    right: forge_proof::ExecutionReadyRecipe<LB, B>,
) {
    let joined = join_ready(left, right);
    let _ = joined.payload().left();
}
```

## Equivalent Raw Surface

```rust
use forge_proof::raw::*;

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

## Related Docs

- [Execution-Ready And Executed](./execution-ready-and-executed.md)
- [Transition Outcomes](./transition-outcomes.md)
- [Fork And Join](./fork-and-join.md)
