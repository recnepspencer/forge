# Feature Signal Contract

This document records the kernel-side contract for embedding `forge-signal`.

## Ownership Boundary

- `forge-signal` owns dependency scheduling, invalidation semantics, version-gated skip logic, transaction rollback, tier metadata, and condition/comparator policy storage.
- `forge-kernel` owns feature logic, `SolidEnvelope`/`SpecEnvelope`, semantic aspect version derivation, feature dependency declarations, and host-side cached outputs.

## Runtime Boundary

- Feature graph evaluation must flow through `SignalRuntime` transactions.
- Raw `SignalGraph` access is allowed only for structural graph edits and serialization snapshots.
- Kernel code must not call raw `forge-signal` `evaluate(...)` or `mark_dirty(...)` helpers in production paths.

## Aspect Boundary

- Current kernel feature graph aspects are `Topology` and `Geometry`.
- Dependencies must be declared with `FeatureDependency` bindings, never flattened into unconditional “read everything” wiring.
- `Topology` means structural/topological truth.
- `Geometry` means geometric embedding attached to that truth.

## Version Boundary

- Feature nodes must return semantic, monotonic per-aspect versions.
- Topology version changes only when topology meaningfully changes.
- Geometry version changes only when geometry meaningfully changes.
- Placeholder or constant aspect versions are forbidden.

## Policy Boundary

Supported today in `FeatureTree`:

- `EvaluationCondition::Always`
- `EvaluationCondition::AspectFilter(_)`
- comparator `Exact`
- comparator `Tolerance`
- static dependencies
- `FeatureSignalTier::Core`

Not supported yet in `FeatureTree`:

- `OnDemand`
- `Debounce`
- `DeltaThreshold`
- custom conditions
- custom comparators

Those features remain part of `forge-signal`, but kernel adoption must be explicit and test-backed before they are enabled for production feature nodes.

## Serialization Boundary

- Persist committed `SignalGraph` state plus kernel-owned feature/envelope caches.
- Rebuild a fresh `SignalRuntime` around the deserialized graph.
- Runtime shell concerns such as event bus/checkpoint runtime are reconstructed, not serialized as durable truth.

## Payload Boundary

- `forge-signal` is the computation backbone, not the owner of domain payloads.
- Host caches remain canonical.
- Feature input materialization should pass only the aspects actually required by a feature where practical.

## Enforcement

This contract is defended by:

- `engine/tests.rs` integration coverage
- `architecture_guard_tests.rs` guard checks
- explicit `FeatureSignalPolicy` validation during feature registration
