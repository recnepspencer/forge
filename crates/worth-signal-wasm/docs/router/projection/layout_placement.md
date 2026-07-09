# Layout Placement

## What This Feature Is

Layout placement is the matched stack of layouts around a projected or admitted
route.

## Why You Use It

- understand nested route structure
- inspect which layout chain surrounds a leaf route
- preserve outlet composition in history and restore work

## Stable Entry Points

- `candidate.layouts()`
- `outcome.layouts()`

## Core Mental Model

Layouts are part of route truth, not external UI decoration. The router keeps
them as explicit placements with route ids, hrefs, and outlet ownership.

## How It Executes

1. match layouts by route-path prefix
2. accumulate placements in declaration order
3. finalize outlet contracts against the leaf route

## Small Example

```ts
const placements = routes.project("/app/projects/p7")?.layouts() ?? [];

console.log(placements.map((layout) => layout.routeId));
```

## Real Example

```ts
const outcome = await routes.admit("/app/projects/p7");

if (outcome.kind === "admitted") {
  console.log(outcome.layouts().map((layout) => ({
    routeId: layout.routeId,
    outletId: layout.outletId,
  })));
}
```

## How It Relates To Other Features

- use [Outlet Contracts](./outlet_contracts.md) for the occupant side
- use [Route History Entries](../history/route_history_entries.md) when layout
  truth must survive replay or restore

## Inspection And Debugging

- `layout.routeId`
- `layout.outletId`
- `layout.href`

## Anti-Patterns

- treating layout placement as implicit framework nesting
- recreating layout stacks from UI tree structure instead of route truth

## Current Limits

- layout placement describes route composition, not arbitrary local UI chrome

## Related Docs

- [Outlet Contracts](./outlet_contracts.md)
- [Route History Entries](../history/route_history_entries.md)
