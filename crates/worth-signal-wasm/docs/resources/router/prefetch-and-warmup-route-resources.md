# Prefetch And Warmup Route Resources

## What This Feature Is

This page covers the route-triggered resource artifacts that materialize native
resource lines before admission.

## Why You Use It

Use it when you want to:

- prefetch one declared route resource
- warm matching route resources for `hover`, `focus`, `viewport`, or `intent`
- reuse the same line when the route later admits

## Stable Entry Points

- `projected.route().resource(name).prefetch(trigger?)`
- `projected.warmup(trigger)`
- `routes.warmup(href, trigger)`

## Core Mental Model

Prefetch and warmup are not "loading hints." They are real artifacts owning real
resource lines until you free them.

## How It Executes

Prefetch:

1. resolves route params into family params
2. materializes one native line
3. returns a `RouteResourcePrefetchArtifact`

Warmup:

1. projects the route
2. compares the trigger to declared resource postures
3. materializes only matching resources
4. records skipped resources explicitly

## Small Example

```ts
const projected = routes.project("/users/user-1");
const prefetch = projected.route().resource("detail").prefetch();

console.log(prefetch.current().descriptor.canonicalParams.canonicalKey);
```

## Real Example

```ts
const projected = routes.project("/warm/user-4");
const hoverWarmup = projected.warmup("hover");

console.log(hoverWarmup.declaredResourceNames());
console.log(hoverWarmup.resourceNames());
console.log(hoverWarmup.skippedResourceNames());

const admitted = await routes.admit("/warm/user-4");
console.log(admitted.route().resource("hoverCard").line().descriptor().runtimeLineId);

hoverWarmup.free();
```

## How It Relates To Other Features

- Use [Declare Route Resources](./declare-route-resources.md) before this if the
  route binding does not exist yet.
- Use [Read Projected And Admitted Resource Capabilities](./read-projected-and-admitted-resource-capabilities.md)
  when you want the capability reads rather than the transition artifact story.

## Inspection And Debugging

Inspect:

- `prefetch.current()`
- `prefetch.line()`
- `warmup.resourceNames()`
- `warmup.skippedResourceNames()`

## Anti-Patterns

- Do not treat prefetch as fire-and-forget.
- Do not warm every declared resource for every trigger.
- Do not assume warmup creates a route-owned cache separate from the resource
  family line.

## Current Limits

Unmatched warmup triggers fail closed, and prefetched artifacts still own
explicit lifecycle via `free()` or `Symbol.dispose`.

## Related Docs

- [Resource Prefetch](../../router/resources/resource_prefetch.md)
- [Resource Warmup](../../router/resources/resource_warmup.md)
