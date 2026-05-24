# Resource Prefetch

## What This Feature Is

Resource prefetch is the route-resource artifact that materializes native
resource lines before route admission.

## Why You Use It

- load route-local data ahead of time
- reuse the same runtime line when the route later admits
- free prefetched lines explicitly when you no longer need them

## Stable Entry Points

- `resource.prefetch(trigger?)`
- `candidate.prefetch(trigger?)`
- `RouteResourcePrefetchArtifact`

## Core Mental Model

Prefetch is a real resource artifact with lifecycle, not a promise of future
convenience. It owns a native resource line until you free it.

## How It Executes

1. resolve route params into resource family params
2. materialize the native line
3. expose descriptor, status, freshness, and diagnostics
4. optionally reuse that line when the route admits

## Small Example

```ts
const prefetch = routes.project("/users/user-1")?.route().resource("detail").prefetch();
```

## Real Example

```ts
const prefetch = routes.project("/users/user-1")?.route().resource("detail").prefetch();
const outcome = await routes.admit("/users/user-1");

if (outcome.kind === "admitted" && prefetch) {
  console.log(prefetch.line().descriptor().runtimeLineId);
  console.log(outcome.route().resource("detail").line().descriptor().runtimeLineId);
  prefetch.free();
}
```

## How It Relates To Other Features

- broad trigger-aware lane is [Resource Warmup](./resource_warmup.md)
- host event envelopes are [Warmup Ingress](./warmup_ingress.md)

## Inspection And Debugging

- `prefetch.line()`
- `prefetch.current()`
- `prefetch.prefetchPosture`
- `prefetch.trigger`
- `prefetch.verification()`

## Anti-Patterns

- treating prefetch as a fire-and-forget string helper
- forgetting to free prefetched lines when they should no longer stay resident
- assuming prefetch may change trigger posture without matching the declaration

## Current Limits

- prefetch trigger must match the declared resource posture
- prefetched artifacts own explicit lifecycle via `free()` or `Symbol.dispose`

## Related Docs

- [Projected Resource Capabilities](./projected_resource_capabilities.md)
- [Resource Warmup](./resource_warmup.md)
