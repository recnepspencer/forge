## Fintech Test Domain

This directory holds crude but legitimate fintech-shaped graph setup for hostile
`worth-signal` testing.

It is intentionally not a real pricing engine. The purpose is to model:

- multi-aspect nodes
- tolerance-driven recomputation
- FX and cross-rate structure
- bucketed curve and vol-surface sources
- scenario fanout
- bucketed aggregation
- book and desk hierarchy
- branch/snapshot workflows
- partition-locality workflows
- keyed audit caches
- tier-policy-driven audit behavior
- retained vs reconstructed artifact access

Default world setup is opinionated:

- `setup_world()` builds a smoke-scale world and seeds a deterministic calm market by default
- the world exposes primary audit, rollback, and checkpoint verbs directly
- hostile workflows should use world methods rather than reaching into raw node IDs

The module split is moving toward narrower responsibilities:

- `aspects.rs`: fintech aspect catalog
- `audit_surface.rs`: typed desk/scenario audit truth
- `branch_checkpoint.rs`: branch-local checkpoint records for restore/audit workflows
- `scales.rs`: named graph sizes
- `regimes.rs`: deterministic market regimes
- `market_seed.rs`: reusable market seeding presets
- `node_families.rs`: reusable node-family builders
- `hierarchy.rs`: book/desk grouping helpers
- `market_state.rs`: market-regime seeding for runtime sources
- `branch_history.rs`: branch, snapshot, replay, and lineage helpers
- `world_setup.rs`: default world-assembly smoke coverage
- `branch_isolation.rs`: hostile branch/replay/isolation workflows
- `snapshot_recovery.rs`: snapshot restore workflows after partial refresh pressure
- `threshold_flapping.rs`: condition and rollback storm workflows
- `executor_overlap.rs`: serial-vs-parallel overlap certification workflows
- `fanout_tolerance.rs`: high-fanout tolerance pressure workflows
- `partition_locality.rs`: partition/detail invalidation isolation workflows
- `keyed_cache.rs`: keyed and memoized audit-cache workflows
- `tier_policy.rs`: mixed live/audit tier policy workflows
- `artifact_materialization.rs`: retained-vs-reconstructed artifact workflows
- `fixture.rs`: assembled `FintechWorld` and world-facing ergonomics
- `world_handles.rs`: stable primary/aggregate handles owned by the world
- `world_assembly.rs`: typed world setup profiles behind `setup_world()`
- `scenarios.rs`: compatibility entrypoints for `setup_world()`
- `world_shape.rs`: structural assertions for the assembled world
