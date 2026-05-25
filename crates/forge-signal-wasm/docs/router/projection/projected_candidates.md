# Projected Candidates

## What This Feature Is

A projected candidate is the matched route/layout/outlet structure for a piece
of route authority before admission runs.

## Why You Use It

- preview which route matches an href
- inspect matched layout and outlet structure
- warm or speculate before full admission

## Stable Entry Points

- `routes.project(...)`
- `candidate.route()`
- `candidate.layouts()`
- `candidate.outlet()`
- `candidate.outlets()`
- `candidate.prefetch(...)`
- `candidate.warmup(...)`
- `candidate.speculate(...)`
- `candidate.admission(...)`

## Core Mental Model

Projection answers "what matches?" not "is it allowed?" It gives you a matched
candidate with structural facts, then admission can turn that candidate into a
route outcome later.

## How It Executes

1. normalize route authority
2. match the best route and layout chain
3. build projected route and outlet contracts
4. expose preview, warmup, speculation, and admission lanes

## Small Example

```ts
const projected = routes.project("/items/i1");

console.log(projected?.route().routeId);
console.log(projected?.outlet().occupantRouteId);
```

## Real Example

```ts
const projected = routes.project("/app/projects/p7?tab=files");

if (projected) {
  console.log(projected.route().resourceNames());
  console.log(projected.layouts().map((layout) => layout.routeId));
  console.log(projected.outlets().map((outlet) => outlet.outletId));
}
```

Authoritative truth is still the route authority you projected. The candidate is
derived structure.

## How It Relates To Other Features

- use [Admit](../admission/admit.md) when you need terminal outcomes
- use [Layout Placement](./layout_placement.md) and
  [Outlet Contracts](./outlet_contracts.md) for the composition pieces

## Inspection And Debugging

- `candidate.href`
- `candidate.route().descriptor()`
- `candidate.layouts()`
- `candidate.outlet()`
- `candidate.verification()`

## Anti-Patterns

- treating a projected candidate as admitted truth
- re-matching route structure outside `routes.project(...)`

## Current Limits

- projected candidates are preview objects, not retained history entries
- policy decisions still belong to admission

## Related Docs

- [Route Schema Authoring](./route_schema_authoring.md)
- [Admit](../admission/admit.md)
- [Projection Verification](./projection_verification.md)
