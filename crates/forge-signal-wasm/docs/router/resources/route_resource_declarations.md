# Route Resource Declarations

## What This Feature Is

Route resource declarations attach native resource families to routes with
`signals.router.resourceLine(...)`.

## Why You Use It

- declare route-local data needs beside the route
- prefetch or warm those resources before admission
- keep router resource behavior on the same native resource line substrate

## Stable Entry Points

- `signals.router.resourceLine(...)`
- `route(..., { resources: { ... } })`

## Core Mental Model

The router does not invent a second route loader cache. A route resource
declaration just teaches the router how to map route truth into a real resource
family line.

## How It Executes

1. choose a resource family
2. declare how route params/search/hash become family params
3. declare a prefetch posture
4. let projected or admitted routes materialize the line

## Small Example

```ts
const detail = signals.router.resourceLine(userFamily, {
  params: ({ params }) => ({ userId: params.userId }),
  prefetch: "hover",
});
```

## Real Example

```ts
const routes = signals.router.define({
  userDetail: signals.router.route("/users/:userId", {
    resources: {
      detail: signals.router.resourceLine(userFamily, {
        params: ({ params }) => ({ userId: params.userId }),
        prefetch: "hover",
      }),
    },
  }),
});
```

## How It Relates To Other Features

- projected behavior is covered in [Projected Resource Capabilities](./projected_resource_capabilities.md)
- admitted behavior is covered in [Admitted Resource Capabilities](./admitted_resource_capabilities.md)

## Inspection And Debugging

- `route.resourceNames()`
- `route.resource(name).prefetchPosture()`
- `resource.verification()`

## Anti-Patterns

- treating route resources as a separate fake cache
- declaring plain objects instead of `signals.router.resourceLine(...)`
- hiding trigger posture in app-local conventions

## Current Limits

- declarations must lower to supported route resource families
- invalid declarations fail closed at route authoring time

## Related Docs

- [Projected Resource Capabilities](./projected_resource_capabilities.md)
- [Admitted Resource Capabilities](./admitted_resource_capabilities.md)
