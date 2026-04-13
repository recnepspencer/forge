# Host Adapters

This guide covers the boundary between the bridge and host-specific integration
code.

The bridge is meant to be a protocol boundary, not a bag of host folklore.

That means host adapters have an important but limited role:

- translate host capabilities into bridge contracts
- supply truth access and sink execution through declared interfaces
- preserve bridge-owned semantics rather than replacing them

## Adapter Responsibilities

Adapters are the right place for:

- opening host truth snapshots
- materializing read packets
- exposing source capability information
- delivering invalidation to compute hosts
- executing admitted writeback authority where applicable

## Adapter Non-Responsibilities

Adapters should not silently redefine:

- routing semantics
- truth-view semantics
- speculative isolation semantics
- failure taxonomy
- replay semantics

Those belong to the bridge.

## Relevant Public Traits

The primary adapter-facing facade surface includes traits like:

- `RelationalBridgeSource`
- `TruthBranchHeadSource`
- `SignalBridgeSink`
- `TruthWritebackAuthority`
- `ContinuityLineageSource`
- `TruthSnapshotReader`

These traits exist so host integration can remain explicit and typed while the
bridge still owns the cross-runtime meaning.

## Product Rule

If understanding a bridge result requires private knowledge of one adapter's
special behavior, the adapter boundary is too powerful and the bridge boundary
is too weak.
