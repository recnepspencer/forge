# Read Projected And Admitted Resource Capabilities

## What This Feature Is

This page explains the route capability surfaces that expose route-bound
resource lines before and after admission.

## Why You Use It

Use it when you need to:

- inspect a projected route's declared resource capabilities
- read the admitted route's real resource line
- compare projected prefetch posture to admitted continuity

## Stable Entry Points

- `projected.route().resourceNames()`
- `projected.route().resource(name)`
- `admitted.route().resource(name)`

## Core Mental Model

Projected route resources tell you what the route can bind.
Admitted route resources tell you what line the route now owns.

Both stay on the same native resource substrate.

## How It Executes

Projected capabilities expose:

- `prefetchPosture()`
- `prefetch(...)`
- `warmup(...)`
- declaration verification

Admitted capabilities expose:

- `line()`
- `current()`
- declaration verification

## Small Example

```ts
const projected = routes.project("/users/user-1");

console.log(projected.route().resourceNames());
console.log(projected.route().resource("detail").prefetchPosture());
```

## Real Example

```ts
const projected = routes.project("/users/user-1");
const prefetch = projected.route().resource("detail").prefetch();
const admitted = await routes.admit("/users/user-1");

console.log(prefetch.line().descriptor().runtimeLineId);
console.log(admitted.route().resource("detail").line().descriptor().runtimeLineId);
console.log(admitted.route().resource("detail").current().diagnosticsSummary.current.status.kind);
```

## How It Relates To Other Features

- Use [Prefetch And Warmup Route Resources](./prefetch-and-warmup-route-resources.md)
  for the transition artifact story.
- Use [Inspecting And Debugging Resources](../debugging/README.md) once you need
  deeper line lifecycle, history, or verification.

## Inspection And Debugging

Start with `resourceNames()` and `prefetchPosture()`, then drop to `current()`
or `line()` when you need the native line reads.

## Anti-Patterns

- Do not assume projected capabilities already own an admitted line.
- Do not compare route resources by route id alone when the line descriptor is
  the real identity proof.

## Current Limits

Projected capability is declaration truth. Admitted capability is runtime line
truth.

## Related Docs

- [Projected Resource Capabilities](../../router/resources/projected_resource_capabilities.md)
- [Admitted Resource Capabilities](../../router/resources/admitted_resource_capabilities.md)
