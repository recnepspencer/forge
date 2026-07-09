# Outlet Contracts

## What This Feature Is

Outlet contracts are the explicit occupant contracts produced when a route is
projected or admitted through one or more layouts.

## Why You Use It

- inspect which route occupies which outlet
- keep layout composition explicit across projection, admission, history, and
  restore

## Stable Entry Points

- `candidate.outlet()`
- `candidate.outlets()`
- `outcome.outlet()`
- `outcome.outlets()`

## Core Mental Model

An outlet contract is the router-owned record of a placement slot and its
current occupant route. It is not inferred from rendering code.

## How It Executes

1. projection finalizes outlet contracts from matched layouts and leaf route
2. admission carries those contracts forward into admitted truth
3. history can preserve outlet composition later

## Small Example

```ts
const outlet = routes.project("/app/dashboard")?.outlet();

console.log(outlet?.outletId);
console.log(outlet?.occupantRouteId);
```

## Real Example

```ts
const outcome = await routes.admit("/app/projects/p7");

if (outcome.kind === "admitted") {
  console.log(outcome.outlets().map((outlet) => ({
    outletId: outlet.outletId,
    occupantRouteId: outlet.occupantRouteId,
  })));
}
```

## How It Relates To Other Features

- use [Layout Placement](./layout_placement.md) to understand the layout side
- use [History Inspection](../history/history_inspection.md) when outlet
  composition needs to be inspected after navigation

## Inspection And Debugging

- `outletId`
- `occupantRouteId`
- outlet verification and summary surfaces exposed through history composition

## Anti-Patterns

- deriving outlet occupancy from component tree folklore
- assuming a route has one implicit outlet shape in every context

## Current Limits

- outlet contracts describe route composition truth, not arbitrary view-tree
  state outside the router

## Related Docs

- [Layout Placement](./layout_placement.md)
- [History Inspection](../history/history_inspection.md)
