# Lower-Runtime Support And Boundary Traceability

## What This Feature Is

Lower-runtime support and boundary traceability let a domain attach support
meaning to a lower-runtime boundary envelope and materialize a typed
lower-runtime support artifact through Query.

## Why You Use It

- you need support posture for bridge, signal, or routing boundaries
- you want the ordinary lane to stay Query-shaped while preserving lower-runtime
  authority boundaries
- you need lower-runtime support artifacts for certification and incident
  debugging

## Stable Entry Points

- `worth_query_domain(...).for_lower_runtime_boundary_envelope(...).supports_boundary_traceability(...).because(...).materialize()`
- `worth_query_domain(...).for_lower_runtime_boundary_source(...).supports_boundary_traceability(...).because(...).materialize()`

Checked lane:

- `.try_materialize()`

Proof lane:

- `WorthQuerySupportContributionAuthoring::narrowed_support(...).for_lower_runtime_boundary_envelope(...)`
- `WorthQuerySupportContributionAuthoring::narrowed_support(...).for_lower_runtime_boundary_source(...)`

## Core Mental Model

This feature does not move lower-runtime authority into Query.

Instead, Query owns the contribution lifecycle and artifact family, while the
lower-runtime envelope remains the authoritative binding surface. The domain is
contributing support posture about that boundary.

## How It Executes

1. obtain a `WorthQueryLowerRuntimeBoundaryEnvelope` from a real Query boundary
   receipt, or keep the receipt and use
   `for_lower_runtime_boundary_source(...)`
2. enter the lower-runtime domain capability surface with
   `for_lower_runtime_boundary_envelope(...)` or
   `for_lower_runtime_boundary_source(...)`
3. author support meaning with `supports_boundary_traceability(...)`
4. materialize a lower-runtime support artifact

## Small Example

```rust
let artifact = worth_query_domain("worth.spatial")
    .for_lower_runtime_boundary_source(&write_authority_receipt)
    .supports_boundary_traceability("routing.signal_invalidation")
    .because("signal invalidation is the active authority seam for this shape")
    .materialize()?;
```

## Real Example

```rust
let artifact = worth_query_domain("worth.spatial")
    .for_lower_runtime_boundary_envelope(&envelope)
    .supports_boundary_traceability("routing.face_split_authority")
    .because("face-split writeback is routed through one retained lower-runtime envelope")
    .materialize()?;

let digest = artifact.materialization_digest();
let target = artifact.target_digest();
```

## How It Relates To Other Features

- pair this with [Lower-Runtime Explanation Contributions](../explanation/lower-runtime-explanation-contributions.md)
  when support is not enough and you also need causal explanation
- pair it with [Declaration-Scoped Support And Traceability](./declaration-scoped-support-and-traceability.md)
  when some support belongs to the declaration and some belongs to the routed
  runtime boundary

## Inspection And Debugging

- the lower-runtime artifact keeps target semantics explicit
- use the checked lane when you need denial details instead of a raised error

## Anti-Patterns

- treating boundary support as if it were declaration support
- bypassing the lower-runtime envelope and rebuilding boundary identity locally
- implying Query owns the lower-runtime route itself
- constructing or synthesizing boundary envelopes in downstream code instead of
  using real Query boundary receipts

## Current Limits

- this feature is boundary-envelope-bound
- it is for support posture, not full causal explanation
- if you need replay-gap or fallback explanation, use the explanation category

## Related Docs

- [Lower-Runtime Explanation Contributions](../explanation/lower-runtime-explanation-contributions.md)
- [Cross-Runtime Fallback Vs Store-Backed Replay Gap](../explanation/cross-runtime-fallback-vs-store-backed-replay-gap.md)
- [Downstream Runtime Integration](../../foundations/downstream-runtime-integration.md)
