# Rollback Strategy Contract (Phase 2.5 / Item 10.15)

This document is normative for rollback semantics in `forge-topo`.

## Contract Version

- `RollbackContractVersion::V1`

## Chosen Strategy

- `RollbackStrategy::SnapshotRestore`

Rollback restores an earlier immutable checkpoint state, then appends lineage
rollback events describing that restoration.

## Explicitly Rejected Strategy (for V1)

- `RollbackStrategy::InverseReplay`

Rationale:

1. `forge-topo` already models state as immutable snapshots (`TopologyState`),
   so checkpoint restore is structurally aligned with the current transaction
   architecture.
2. Inverse replay requires each operator to maintain and validate a robust
   inverse payload, which is not yet guaranteed across the full operator set.
3. Persistent naming needs deterministic, unambiguous rollback traces; snapshot
   restore with explicit `Reverted` events is simpler to audit.

## Lineage Event-Sourcing Boundary

For V1:

1. Lineage history is append-only.
2. Rollback does not rewrite historical lineage events.
3. Rollback emits explicit `LineageEvent::EntityReverted` entries.

This keeps causality auditable and deterministic across replay.

## Replay Contract

Replay consumers must treat rollback as:

1. restore checkpoint state
2. apply appended `EntityReverted` lineage events

Replay determinism compares both topology/result hashes and event ordering.

## Persistent Naming Contract

Name resolution must only consume:

1. canonical topology state
2. canonical lineage event ordering (including `EntityReverted`)

Resolvers must never infer identity from incidental dirty-state/cache order.

## Future Evolution

If strategy changes in a future contract version:

1. bump `RollbackContractVersion`
2. add explicit migration/replay compatibility rules
3. do not silently reinterpret older lineage histories
