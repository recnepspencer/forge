# Declare Route Resources

## What This Feature Is

This is the route-facing way to bind a declared resource family to a route.

## Why You Use It

Use it when a route should own:

- a route-local detail line
- a route-local collection or paged line
- a declared prefetch posture
- route-driven param resolution into a resource family

## Stable Entry Points

- `signals.router.resourceLine(...)`
- `route(..., { resources: { ... } })`

## Core Mental Model

A route-resource declaration does not declare a new resource model. It points to
an existing family and teaches the router how to resolve route truth into that
family's line params.

## How It Executes

Each route resource declaration carries:

- `family`
- `resolveParams(route)` after `signals.router.resourceLine(...)` lowers the
  route-facing `params(route)` option
- `prefetch`

At runtime the router resolves route params, search, and hash into resource
line params, then materializes the same native line that ordinary app code would
have created directly.

## Small Example

```ts
const userDetail = signals.api({ baseUrl: "/api" })
  .url("/users/:userId")
  .detail({
    load: ({ userId }) => ({ id: userId, title: `User ${userId}` }),
  });

const routes = signals.router({
  users: signals.router.route("/users/:userId", {
    resources: {
      detail: signals.router.resourceLine(userDetail, {
        params: ({ params }) => ({ userId: params.userId }),
        prefetch: "hover",
      }),
    },
  }),
});
```

## Real Example

```ts
const taskSearch = signals.api({ baseUrl: "/api" })
  .url("/workspaces/:workspaceId/tasks")
  .items((item: { id: string; title: string }) => item.id)
  .paged({
    accumulatePage: (existing, next) => [...existing, ...next],
    load: ({ workspaceId }) => [{ id: `${workspaceId}:1`, title: "First" }],
  });

const routes = signals.router({
  tasks: signals.router.route("/workspaces/:workspaceId/tasks", {
    resources: {
      search: signals.router.resourceLine(taskSearch, {
        params: ({ params }) => ({ workspaceId: params.workspaceId }),
        prefetch: "intent",
      }),
    },
  }),
});
```

## How It Relates To Other Features

- Use [Prefetch And Warmup Route Resources](./prefetch-and-warmup-route-resources.md)
  when the main question is early materialization.
- Use [Read Projected And Admitted Resource Capabilities](./read-projected-and-admitted-resource-capabilities.md)
  when you already have a projected or admitted route.

## Inspection And Debugging

Inspect:

- the resolved route params
- the family line descriptor after prefetch or admission
- the route-resource verification digest

## Anti-Patterns

- Do not pass raw objects as route resources. They must be declared with
  `signals.router.resourceLine(...)`.
- Do not treat a route resource as a second cache beside the resource family.

## Current Limits

The route owns binding and trigger posture. The resource family still owns
request, lifecycle, diagnostics, and history truth.

## Related Docs

- [Router Route Resources](../../router/resources/route_resource_declarations.md)
- [Raw Resource Lines](../advanced/raw-resource-lines.md)
