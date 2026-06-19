<!-- worth-doc
crate: worth-topo
kind: feature
id: topology-workloads-and-seeds
query_integration_required: false
query_proof_required: false
touches_query: false
-->

# Topology Workloads And Seeds

## What This Feature Is

This feature owns the topology-authored workload, seed, and scope surfaces
used to build and pressure topology truth generically rather than one mutation
at a time.

## Why You Use It

Use this when you need reusable topology workload authoring, seeded topology
construction, or scope-class receipts instead of ad hoc fixture setup.

## Stable Entry Points

- `worth_topo::workload_platform`
- `worth_topo::facade::{TopologyWorkload, TopologyWorkloadDeclaration, TopologyWorkloadEnvelope}`
- `worth_topo::facade::{TopologySeed, TopologySeedRecipe, TopologySeedReceipt}`

## Common Path

1. Describe the workload or seed recipe.
2. Produce the topology workload / seed receipts.
3. Use those receipts to drive topology proof or workload pressure surfaces.

## Advanced Path

Use the advanced path when you need to inspect:

- scope counters and scope denials
- seed neighborhood and query receipts
- posture receipts for open topology patterns
- workload declaration identity and support posture

## Inspection And Debugging

Reach for this surface when the question is about how topology scenarios are
authored and replayed, not just how one graph object is interpreted.

## Anti-Patterns

- replacing workload receipts with one-off test worlds
- teaching seeds as disposable fixture glue
- hiding topology scope meaning inside generic certification harnesses

## Current Limits

This doc backfills the enduring topology workload substrate that the newer
construction and parity slices inherit.

## Related Docs

- [Topology Certification And Parity](./topology-certification-and-parity.md)
- [Primitive Construction](../../../worth-kernel/docs/features/primitive-construction.md)
- [Domain Reads](./domain-reads.md)
