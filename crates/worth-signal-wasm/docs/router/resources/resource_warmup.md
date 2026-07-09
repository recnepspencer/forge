# Resource Warmup

## What This Feature Is

Resource warmup is the trigger-aware route resource lane that warms the subset
of route resources matching `hover`, `focus`, `viewport`, or `intent`.

## Why You Use It

- warm only the route resources relevant to a user signal
- keep route warmup on the same native resource substrate as prefetch
- admit or transition from a warmup artifact later

## Stable Entry Points

- `candidate.warmup(trigger)`
- `routes.warmup(href, trigger)`
- `ProjectedRoutePrefetchArtifact`

## Core Mental Model

Warmup is prefetch with route-trigger semantics. It can warm all resources on
`intent`, or only a subset on narrower triggers.

## How It Executes

1. project the route
2. compare the trigger to each declared resource posture
3. warm matching resources
4. record skipped resources explicitly
5. optionally admit or transition from the warmup artifact

## Small Example

```ts
const warmup = routes.warmup("/warm/user-4", "focus");
```

## Real Example

```ts
const warmup = routes.project("/warm/user-4")?.warmup("hover");

console.log(warmup?.declaredResourceNames());
console.log(warmup?.resourceNames());
console.log(warmup?.skippedResourceNames());
```

## How It Relates To Other Features

- low-level resource artifacts are documented in [Resource Prefetch](./resource_prefetch.md)
- host-side trigger envelopes are covered in [Warmup Ingress](./warmup_ingress.md)

## Inspection And Debugging

- `warmup.trigger`
- `warmup.declaredResourceNames()`
- `warmup.resourceNames()`
- `warmup.skippedResourceNames()`
- `warmup.admit(...)`

## Anti-Patterns

- warming every route resource for every trigger
- using warmup to hide a second router-owned cache
- materializing skipped resources just to compute explanations

## Current Limits

- unmatched trigger posture fails closed
- warmup artifacts still own explicit resource lifecycle and should be freed

## Related Docs

- [Resource Prefetch](./resource_prefetch.md)
- [Warmup Ingress](./warmup_ingress.md)
