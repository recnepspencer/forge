# Fixed-Arity Join

## What This Feature Is

This workflow shows how to split one artifact into two explicit lanes and then join two explicit artifact lanes back into one artifact or one ready-recipe join result.

## Why You Use It

- you have fixed-arity static composition
- proof and basis routing must stay explicit
- you want static composition without a dynamic graph engine

## Stable Entry Points

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

## How It Executes

1. start with one artifact or two ready recipes
2. fork or join through the fixed-arity carriers
3. preserve explicit lane position throughout
4. produce one joined artifact or one joined ready recipe

## Small Example

```rust
use forge_proof::{ForkOutputs2, JoinInputs2};

let outputs = ForkOutputs2::new("left", "right");
let inputs = JoinInputs2::new("left", "right");

assert_eq!(outputs.left(), &"left");
assert_eq!(inputs.right(), &"right");
```

This is the smallest honest example because the lane structure itself is the stable backbone of the workflow.

## Real Example

```rust
use forge_proof::{
    fork_artifact_pair, join_artifact_pair, Artifact, ForkOutputs2, JoinInputs2, NoAssumptionBasis,
    NoProofs, PhaseMarker,
};

struct RawPhase;
impl PhaseMarker for RawPhase {}

fn split_and_join() {
    let source = Artifact::<RawPhase, _>::new((3_u8, 5_u8));

    let forked = fork_artifact_pair(source, |payload, proofs, basis| {
        let _ = proofs;
        let _ = basis;
        ForkOutputs2::new(
            (payload.0, NoProofs, NoAssumptionBasis),
            (payload.1, NoProofs, NoAssumptionBasis),
        )
    });

    let (left, right) = forked.into_parts();
    let joined = join_artifact_pair(JoinInputs2::new(left, right), |left, right| {
        (left.0 + right.0, NoProofs, NoAssumptionBasis)
    });

    assert_eq!(joined.payload(), &8_u8);
}
```

What this shows:

- forking is explicit redistribution, not magic duplication
- joining is explicit recomposition, not bag merging
- the lane carriers keep the topology visible throughout

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

## Current Limits

- the stable surface is fixed-arity 2
- actual payload/proof/basis routing remains domain-owned
- this workflow is static and explicit rather than builder-driven

## Related Docs

- [Fork And Join](../features/fork-and-join.md)
- [Ready Recipe Join](../features/ready-recipe-join.md)
- [Composition-Family Lowering](./composition-family-lowering.md)
