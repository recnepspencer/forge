# Route Capabilities

## What This Feature Is

Route capabilities are the route-owned controller, graph, resource, breadcrumb,
and forms-authority surfaces exposed on projected and admitted routes.

## Why You Use It

- inspect what a route owns without reading private declaration internals
- pair route structure with route-local resources or forms authority
- build adjacent features from route truth instead of parallel registries

## Stable Entry Points

- `candidate.route()`
- `outcome.route()`
- `routeCapability.controllerNames()`
- `routeCapability.graphNames()`
- `routeCapability.resourceNames()`
- `routeCapability.breadcrumb()`
- `routeCapability.breadcrumbTrail()`
- `routeCapability.formsAuthority()`

## Core Mental Model

The route capability is the public route-owned surface. It is where projection
and admission expose what the route makes available to later runtime features.

## How It Executes

1. route declarations capture route-local composition
2. projection builds a projected route capability
3. admission upgrades that into an admitted route capability

## Small Example

```ts
const capability = routes.project("/projects/p7")?.route();

console.log(capability?.resourceNames());
```

## Real Example

```ts
const outcome = await routes.admit("/projects/p7");

if (outcome.kind === "admitted") {
  const route = outcome.route();
  console.log(route.controllerNames());
  console.log(route.graphNames());
  console.log(route.breadcrumbTrail());
}
```

## How It Relates To Other Features

- use [Admit](../admission/admit.md) when you need the admitted form
- use [History](../history/README.md) when these capabilities need retained
  provenance later

## Inspection And Debugging

- `controllerNames()`
- `graphNames()`
- `resourceNames()`
- `descriptor()`
- `verification()`

## Anti-Patterns

- reaching into declaration internals instead of using the capability surface
- assuming projected and admitted capabilities have identical authority

## Current Limits

- some capability families have richer docs later in the router docs set
- the route capability exposes route-owned public truth, not every lower runtime
  detail

## Related Docs

- [Projected Candidates](./projected_candidates.md)
- [Admit](../admission/admit.md)
