# Fork And Join

## What This Feature Is

Fork-and-join surfaces provide the fixed-arity static composition primitives for splitting one artifact into two explicit outputs and joining two explicit artifact inputs into one output artifact.

## Why You Use It

- one artifact must split into two typed outputs
- two artifacts must join into one typed output
- you want proof, basis, and payload redistribution to stay explicit
- you need fixed-arity composition without a dynamic graph runtime

## Stable Entry Points

- `ForkOutputs2<L, R>`
- `ForkOutputs2::new(left, right)`
- `ForkOutputs2::left()`
- `ForkOutputs2::right()`
- `ForkOutputs2::into_parts()`
- `JoinInputs2<L, R>`
- `JoinInputs2::new(left, right)`
- `JoinInputs2::left()`
- `JoinInputs2::right()`
- `JoinInputs2::into_parts()`
- `fork_artifact_pair(...)`
- `join_artifact_pair(...)`

## DX Posture

This feature is mostly substrate/reference material.

- the pleasant lane only covers ready-recipe composition through `join_ready(...)` and `compose_ready(...)`
- generic artifact fork/join remains raw-substrate-first and should be taught with `use forge_proof::raw::*;`
- if your composition target is ready recipes rather than generic artifacts, prefer [Ready Recipe Join](./ready-recipe-join.md)

## Core Mental Model

Fork and join here are static composition helpers, not a workflow engine.

The key laws are:

- the shape is fixed and explicit
- artifact state redistribution is explicit
- proof duplication is never implicit
- basis redistribution is never implicit

If a fork or join changes payload, proof, or basis shape, the closure must say exactly how.

## How It Executes

Fork:

1. consume one artifact
2. destructure payload, proofs, and basis
3. return `ForkOutputs2<(payload, proofs, basis), (payload, proofs, basis)>`
4. receive two explicit artifact outputs

Join:

1. consume `JoinInputs2<Artifact<...>, Artifact<...>>`
2. destructure both inputs
3. return one joined payload/proof/basis tuple
4. receive one explicit artifact output

## Small Example

```rust
use forge_proof::{ForkOutputs2, JoinInputs2};

let outputs = ForkOutputs2::new("left", "right");
let inputs = JoinInputs2::new("left", "right");

assert_eq!(outputs.left(), &"left");
assert_eq!(inputs.right(), &"right");
```

This is the smallest honest example because the fixed-arity carriers themselves are part of the stable surface, even before artifact redistribution is involved.

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

- the fork closure is responsible for explicit redistribution
- the join closure is responsible for explicit recomposition
- no proof or basis duplication happens silently

## How It Relates To Other Features

- Pair this with [Artifact](./artifact.md) because fork/join operates directly on artifacts.
- Pair this with [Ready Recipe Join](./ready-recipe-join.md) when the composition target is execution-ready recipes rather than generic artifacts.
- Pair this with [Deterministic Family Lowering](./deterministic-family-lowering.md) when fixed-arity composition must feed same-family lifecycle lowering.

## Inspection And Debugging

- inspect the fork closure first when something appears to "duplicate" state
- inspect the join closure when the resulting proof or basis shape looks wrong
- `ForkOutputs2` and `JoinInputs2` make the positional lanes visible in both code and debug output

## Anti-Patterns

- Do not fork by cloning payload, proof, or basis implicitly unless that is the explicit domain law.
- Do not use raw tuples and raw closures when the stable boundary is really a fixed-arity fork/join surface.
- Do not treat these helpers as a substitute for dynamic graph orchestration.

## Current Limits

- only fixed arity 2 is modeled here
- the feature is explicit and static, not builder-driven
- proof and basis routing stays caller-defined for the actual domain law

## Related Docs

- [Artifact](./artifact.md)
- [Ready Recipe Join](./ready-recipe-join.md)
- [Deterministic Family Lowering](./deterministic-family-lowering.md)
