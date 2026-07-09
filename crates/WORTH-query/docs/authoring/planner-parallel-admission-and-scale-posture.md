# Planner Parallel Admission and Scale Posture

## What This Feature Is

Planner parallel admission **changes how preflight bundles dispatch** across parallel admission routes and serial fallbacks—it does **not** change query meaning. Entry points: `lower_preflight_to_parallel_admission_route` and `execute_parallel_admission_route` (facade-exported). This is **not** the read-composition **anchored frontier operators** (`anchored_frontier_*`); those are query operators, not planner dispatch.

Frontier planning artifacts are largely **not facade-exported** (`tests/ui/facade_does_not_export_frontier_artifacts.rs`).

## Why You Use It

- scale admission work when preflight evidence supports parallel routes
- fall back to serial routes when parallel lowering denies
- avoid confusing frontier query operators with frontier planning harness types
- read harness certification without exposing unstable planner internals to apps

## Core Mental Model

```text
Preflight bundle (admitted + evidence)
  → lower_preflight_to_parallel_admission_route
  → execute_parallel_admission_route   (parallel dispatch)
  OR lower_preflight_bundle_to_serial_fallback_routes
  → execute_serial_fallback_route      (serial dispatch)
```

**Parallel admission** = execution dispatch topology. **Frontier operators in read composition** = semantic query windows over retained graphs.

## Main Entry Points

- `planning/mod.rs` — `lower_preflight_to_parallel_admission_route` (delegates to `frontier_planning`)
- `execution/preflight.rs` — `execute_parallel_admission_route`, `execute_serial_fallback_route`
- Facade: `exports_foundation.rs` re-exports execute routes
- Harness: `harness/frontier_planning.rs` — evidence fixtures, denial cases
- **Not this feature:** `read_composition` `anchored_frontier_*` operators

## Typical Flow

1. Build and admit preflight bundle with evidence snapshot.
2. Attempt `lower_preflight_to_parallel_admission_route`—handle `Err` with serial fallback lowering.
3. `execute_parallel_admission_route` on success path.
4. Compare timing/scale in harness only; apps should not depend on frontier artifact types.

## How It Relates

- [Read composition](read-composition.md) — semantic execution; frontier **operators** live there
- [Collections, cursors, ordering](collections-cursors-ordering-and-aggregations.md) — planner `plan_validated_bundle` neighbors
- [Query operating modes](../foundations/query-operating-modes.md) — runtime-backed dispatch default
- [Support matrix](../foundations/support-matrix-and-admission.md) — admission evidence requirements

## Not This Feature

| Name collision | Belongs to |
|----------------|------------|
| `anchored_frontier_*` read-composition operators | [Read composition](read-composition.md) |
| `frontier_planning` certification artifacts | Harness / CI, not stable facade |
| Region-scoped live locality | [Region-scoped live](../runtime-surfaces/region-scoped-live-invalidation-and-stream-contracts.md) |

## Good to Know

- Duplicate evidence snapshots can change parallel route digests—harness tests lock this behavior.
- Parallel lowering can deny while serial fallback still admits—always have fallback path in orchestration.
- UI test `facade_does_not_export_frontier_artifacts` is intentional public-surface discipline.

## Anti-Patterns

- Exporting frontier planning manifest types from domain crates.
- Claiming parallel admission changes filter semantics or aggregate meaning.
- Using frontier operator results as proof parallel admission route was taken.

## Current Limits

| Concern | Status |
|---------|--------|
| `execute_parallel_admission_route` / serial fallback (facade) | **Exported** execute entrypoints |
| Frontier planning artifact types on facade | **Not exported** |
| Store-backed parallel admission durability | **Deferred** per runtime/matrix neighbors |
| Parallel route without admitted preflight evidence | **Denied** (harness cases) |

## Related Docs

- [Read composition](read-composition.md)
- [Collections, cursors, ordering](collections-cursors-ordering-and-aggregations.md)
- [Support matrix and admission](../foundations/support-matrix-and-admission.md)
