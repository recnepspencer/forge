## Fintech Test Domain

This directory holds crude but legitimate fintech-shaped graph setup for hostile
`forge-signal` testing.

It is intentionally not a real pricing engine. The purpose is to model:

- multi-aspect nodes
- tolerance-driven recomputation
- scenario fanout
- bucketed aggregation
- branch/snapshot workflows

The module split is deliberate:

- `aspects.rs`: fintech aspect catalog
- `scales.rs`: named graph sizes
- `node_families.rs`: reusable node-family builders
- `fixture.rs`: assembled domain fixture
- `scenarios.rs`: named domain scenarios
- `invariants.rs`: fixture-shape assertions
- `workflows.rs`: test entrypoints
