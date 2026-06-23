<!-- worth-doc
crate: worth-kernel
kind: feature
id: primitive-construction
query_integration_required: true
query_proof_required: false
touches_query: false
-->

# Primitive Construction

## What This Feature Is

Primitive construction is the kernel-owned workflow that turns an admitted
primitive request into one canonical artifact family spanning Query runtime
posture, spatial birth truth, and topology certification.

## Why You Use It

Use this when you want the shipped primitive construction path rather than
assembling topology, spatial, and Query pieces by hand.

## Stable Entry Points

- `worth_kernel::workload_composition`
- `worth_kernel::query_adoption`

## Common Path

The kernel owns orchestration. Query owns runtime lifecycle. `worth-spatial`
owns birth semantics. `worth-topo` owns topology authority.

Primitive construction is the feature where those boundaries become one usable
workflow.

1. Kernel prepares the primitive workflow.
2. Query admits and executes the runtime-backed path.
3. Spatial birth truth attaches.
4. Topology authority and certification attach.
5. The caller inspects one artifact family.

## Small Example

Use this path when the question is "can I produce an admitted primitive result
and inspect why it succeeded or failed?" not "which lower crate should I wire
manually?"

## Advanced Path

A serious primitive run may need to preserve:

- runtime authoring posture
- spatial birth truth
- topology certification
- replay parity
- rejection locality

This doc is the common path for that combined workflow.

## Query Integration

Query owns runtime admission, workspace execution, and the proof posture that
keeps Worth from inventing a second local runtime lane. Kernel docs should
always name that Query boundary explicitly for this workflow.

## How It Relates To Other Features

- [Construction Results And Diagnostics](./construction-results-and-diagnostics.md)
- [Construction Replay](./construction-replay.md)
- [Primitive Realization Strategies](../../../worth-geom/docs/features/primitive-realization-strategies.md)
- [Topology Workloads And Seeds](../../../worth-topo/docs/features/topology-workloads-and-seeds.md)
- [Kernel To Spatial](../boundaries/kernel-to-spatial.md)
- [Worth To Query](../boundaries/worth-to-query.md)

## Inspection And Debugging

Inspect:

- the returned artifact family
- the Query adoption reports when runtime posture looks suspicious
- the spatial boundary when birth truth or impossibility is the issue

## Anti-Patterns

- treating primitive construction as a topology-only feature
- teaching a local runtime path under the kernel
- rebuilding proof from payload bags or local digests

## Current Limits

This doc covers the admitted Milestone 4 primitive workflow class. Unsupported
neighbors should fail closed and stay explicit.

## Related Docs

- [Construction-Time Birth Bindings](../../../worth-spatial/docs/features/construction-time-birth-bindings.md)
- [Construction Replay](./construction-replay.md)
- [Construction Results And Diagnostics](./construction-results-and-diagnostics.md)
- [Primitive Realization Strategies](../../../worth-geom/docs/features/primitive-realization-strategies.md)
- [Topology Workloads And Seeds](../../../worth-topo/docs/features/topology-workloads-and-seeds.md)
