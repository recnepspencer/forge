<!-- worth-doc
crate: worth-kernel
kind: feature
id: construction-replay
query_integration_required: true
query_proof_required: false
touches_query: false
-->

# Construction Replay

## What This Feature Is

Construction replay is the kernel feature that proves an admitted construction
workflow can be rerun through the shipped artifact and runtime path with the
same outcome class and proof-bearing result family.

## Why You Use It

Use replay when determinism, parity, or closeout evidence matters more than a
single successful run.

## Stable Entry Points

- kernel workload and artifact surfaces
- Query basis / branch / preview semantics described in
  [Worth To Query](../boundaries/worth-to-query.md)

## Common Path

Replay is not log archaeology. It is a named proof surface over admitted
construction workflows.

Replay reuses the same workflow path and checks that the resulting artifact and
outcome class stay aligned with the original admitted semantics.

## Small Example

Use replay when a workflow "worked once" is not enough and you need to know
whether the same admitted inputs still produce the same authoritative meaning.

## Advanced Path

Replay parity is one of the main ways Milestone 4 proves that the kernel did
not hide branch-local, preview, or synthetic runtime folklore under its common
path.

## Query Integration

Replay depends on Query for retained runtime posture, basis identity, and
ordinary execution. Kernel replay docs should therefore point to Query as the
runtime authority rather than implying a local replay engine.

## How It Relates To Other Features

- [Construction Simulation](./construction-simulation.md)
- [Construction Results And Diagnostics](./construction-results-and-diagnostics.md)

## Inspection And Debugging

Inspect replay parity artifacts, basis posture, and rejection-locality outputs
before debugging lower-layer geometry behavior.

## Anti-Patterns

- treating replay as a best-effort convenience
- comparing replay results by presentation strings instead of shipped artifacts

## Current Limits

Replay covers the admitted Milestone 4 workflow classes and should fail closed
outside them.

## Related Docs

- [Construction Simulation](./construction-simulation.md)
