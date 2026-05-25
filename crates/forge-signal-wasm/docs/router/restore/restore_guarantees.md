# Restore Guarantees

## What This Feature Is

Restore guarantees describe exactly what a route restore boundary promises to
recover.

## Why You Use It

- know what route state is safe to rely on after restore
- avoid overclaiming what route restore means
- explain restore semantics to application code and support tooling

## Stable Entry Points

- `boundary.guarantees()`
- `RouteRestoreBoundaryGuarantees`

## Core Mental Model

Restore guarantees are intentionally narrow and explicit:

- exact route truth
- admitted outlet composition
- graph-owned state within the snapshot boundary

They are not a claim about arbitrary host state or app-local effects outside
that boundary.

## How It Executes

1. a restore boundary captures a snapshot
2. its guarantees stay attached to the boundary
3. restore consumers can inspect those guarantees before restore runs

## Small Example

```ts
const guarantees = boundary.guarantees();

console.log(guarantees.routeTruth);
console.log(guarantees.outletComposition);
```

## Real Example

```ts
const restoreResult = await story.backProvenance().restore(signals.history());

console.log(restoreResult.restoreBoundary.guarantees());
```

## How It Relates To Other Features

- outlet-specific restore is covered in
  [Outlet Composition Restore](./outlet_composition_restore.md)

## Inspection And Debugging

- `boundary.guarantees()`
- `restoreResult.restoreBoundary`

## Anti-Patterns

- assuming restore also recreates arbitrary host capability state
- explaining restore in vague "go back to before" language when precise
  guarantees already exist

## Current Limits

- guarantees are intentionally route- and snapshot-scoped

## Related Docs

- [Restore Boundaries](./restore_boundaries.md)
- [Outlet Composition Restore](./outlet_composition_restore.md)
