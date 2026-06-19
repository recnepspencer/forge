<!-- worth-doc
crate: worth-kernel
kind: feature
id: construction-simulation
query_integration_required: true
query_proof_required: false
touches_query: false
-->

# Construction Simulation

## What This Feature Is

Construction simulation is the kernel-owned way to inspect or pressure a
construction workflow without collapsing the runtime, replay, and artifact
boundaries into ad hoc caller experimentation.

## Why You Use It

Use this when you need to compare or inspect a construction path before
committing to its final interpretation, especially when parity or branch-local
proof matters.

## Stable Entry Points

- `worth_kernel::workload_composition`
- Query preview / branch posture through the runtime path described in
  [Worth To Query](../boundaries/worth-to-query.md)

## Common Path

Simulation is still runtime-backed work. It is not local caller folklore and it
is not a hidden second planner.

The kernel uses the same admitted runtime story, but the execution context is
preview, branch, or replay-sensitive rather than ordinary current-head flow.

## Small Example

Use this when the question is "what happens on the admitted runtime path if I
run this workflow in a non-final context?"

## Advanced Path

Simulation is the shared substrate for parity and hostile proof. That means the
same artifact family must stay inspectable across direct, replayed, and
branch-local runs.

## Query Integration

Simulation still runs through Query-owned runtime posture. The distinction is
workflow intent and artifact policy, not a separate runtime.

## How It Relates To Other Features

- [Construction Replay](./construction-replay.md)
- [Worth To Query](../boundaries/worth-to-query.md)

## Inspection And Debugging

Check basis, replay, and branch-local posture before assuming the semantic
problem belongs to topology or spatial truth.

## Anti-Patterns

- local fake simulation helpers that bypass Query basis semantics
- reinterpreting simulation as a different authority lane

## Current Limits

This doc covers the shipped Milestone 4 simulation posture only.

## Related Docs

- [Construction Replay](./construction-replay.md)
