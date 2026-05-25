# Projected Resource Capabilities

## What This Feature Is

Projected resource capabilities are the route-resource handles available on a
projected route before route admission.

## Why You Use It

- prefetch or warm resources before making a route visible
- inspect which resources a projected route declares
- coordinate route transition work without admitting yet

## Stable Entry Points

- `candidate.route().resource(name)`
- `candidate.prefetch(...)`
- `candidate.warmup(...)`

## Core Mental Model

Projected resource capabilities are for candidate-route work. They are route
aware, but they do not mean the route has been admitted yet.

## How It Executes

1. project a route candidate
2. read its route resource names
3. prefetch or warm specific route resources
4. later admit or transition using that work

## Small Example

```ts
const candidate = routes.project("/users/user-1");
const detail = candidate?.route().resource("detail");

detail?.prefetch();
```

## Real Example

```ts
const candidate = routes.project("/warm/user-4");
const warmup = candidate?.warmup("hover");

console.log(warmup?.resourceNames());
console.log(warmup?.skippedResourceNames());
```

## How It Relates To Other Features

- admission converts projected route resources into
  [Admitted Resource Capabilities](./admitted_resource_capabilities.md)
- transition behavior stays adjacent to route resource warmup and prefetch, and
  will get its own dedicated transition docs later

## Inspection And Debugging

- `candidate.route().resourceNames()`
- `resource.prefetchPosture()`
- `prefetch.resourceNames()`
- `prefetch.skippedResourceNames()`

## Anti-Patterns

- assuming projected resource access means route admission already happened
- warming every declared resource when trigger posture only matches one

## Current Limits

- trigger posture mismatches fail closed
- projected capabilities are intentionally route-candidate scoped

## Related Docs

- [Route Resource Declarations](./route_resource_declarations.md)
- [Resource Prefetch](./resource_prefetch.md)
- [Resource Warmup](./resource_warmup.md)
