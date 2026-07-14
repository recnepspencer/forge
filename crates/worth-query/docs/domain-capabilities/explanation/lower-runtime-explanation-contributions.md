# Lower-Runtime Explanation Contributions

## What This Feature Is

Lower-runtime explanation contributions let a domain ask Query to materialize
reviews or explanation artifacts about lower-runtime boundaries without reaching
through bridge or replay internals directly.

## Why You Use It

- you need causal explanation for a lower-runtime boundary
- you want the public Query lane instead of rebuilding cross-runtime
  explanation requests locally
- you need a review lane and a final artifact lane

## Stable Entry Points

- `worth_query_domain(...).for_lower_runtime_boundary_envelope(...).requires_cross_runtime_context(...).because(...).review()`
- `worth_query_domain(...).for_lower_runtime_boundary_envelope(...).explains_cross_runtime_fallback(...).because(...).materialize_artifact()`
- `worth_query_domain(...).for_lower_runtime_boundary_envelope(...).explains_store_backed_replay_gap(...).because(...).materialize_artifact()`

Supporting request type:

- `WorthQueryLowerRuntimeExplanationRequest`

## Core Mental Model

The ordinary lane is Query-owned, but the request still carries real causal
inspection vocabulary because explanation is only honest if it preserves the
lower-runtime evidence boundary.

That means Query is not hiding bridge or replay truth. It is giving you one
stable way to author it.

## How It Executes

1. obtain a lower-runtime boundary envelope
2. construct a `WorthQueryLowerRuntimeExplanationRequest`
3. choose context, fallback, or replay-gap posture
4. review or materialize the resulting explanation artifact

## Small Example

```rust
let artifact = worth_query_domain("worth.spatial")
    .for_lower_runtime_boundary_envelope(&envelope)
    .explains_store_backed_replay_gap("explanation.store_backed_replay", request)
    .because("replay lacks the retained lower-runtime evidence needed for exact edge explanation")
    .materialize_artifact()?;
```

## Real Example

```rust
let review = worth_query_domain("worth.spatial")
    .for_lower_runtime_boundary_envelope(&envelope)
    .requires_cross_runtime_context("explanation.cross_runtime_context", request)
    .because("the face split spans bridge and signal evidence that must be reviewed together")
    .review()?;
```

For geometry work, the replay-gap path is especially important when store-backed
restore can show that something changed but cannot reconstruct the full
authoritative explanation detail.

## How It Relates To Other Features

- [Cross-Runtime Fallback Vs Store-Backed Replay Gap](./cross-runtime-fallback-vs-store-backed-replay-gap.md)
  explains how to choose between the explanation postures
- [Lower-Runtime Support And Boundary Traceability](../support/lower-runtime-support-and-boundary-traceability.md)
  is the lighter-weight support-only neighbor

## Inspection And Debugging

- use `.review()` when you want inspected explanation planning
- use `.materialize_artifact()` when you want the final Query causal inspection
  artifact
- the checked lane preserves denial metadata when the request is malformed or
  unsupported

## Anti-Patterns

- flattening cross-runtime explanation into plain text strings
- using lower-runtime explanation when a declaration-scoped support artifact is
  enough
- teaching these wrappers as if they erased lower-runtime authority boundaries

## Current Limits

- lower-runtime explanation requests still expose substrate-shaped causal nouns
  because those boundaries are real today
- the common lane wraps them, but does not pretend Query invented a new causal
  ontology above them

## Related Docs

- [Cross-Runtime Fallback Vs Store-Backed Replay Gap](./cross-runtime-fallback-vs-store-backed-replay-gap.md)
- [Lower-Runtime Support And Boundary Traceability](../support/lower-runtime-support-and-boundary-traceability.md)
- [Inspection](../../capabilities/inspection.md)
