# Prefetch Admission

## What This Feature Is

Prefetch admission is the transition lane where a prefetched or warmed route
artifact becomes the target route for transition.

## Why You Use It

- reuse existing route-resource work during navigation
- explain that the target route came from prefetched route truth
- keep prefetch and transition on the same route-native artifact family

## Stable Entry Points

- `candidate.prefetch(...)`
- `candidate.warmup(...)`
- `routes.transition(currentOutcome, prefetchedArtifact)`

## Core Mental Model

Prefetch admission is not a second transition API. It is one of the allowed
transition targets, and the transition artifact records that it came from
prefetched route work.

## How It Executes

1. prefetch or warm a projected route
2. optionally settle the prefetched resource work
3. pass that artifact into `routes.transition(...)`
4. inspect `prefetchAdmission` as the source

## Small Example

```ts
const prefetched = routes.project("/users/user-1")?.prefetch("hover");
const transition = await routes.transition(home, prefetched);
```

## Real Example

```ts
const warmup = routes.warmup("/users/user-1", "hover");

if (warmup) {
  const transition = await routes.transition(home, warmup);
  console.log(transition.diagnostics().requestedSource);
  console.log(transition.diagnostics().visibleChangeSource);
}
```

## How It Relates To Other Features

- route-resource work starts in [Route Resources](../resources/README.md)
- overall transition explanation is in [Transition Artifacts](./transition_artifacts.md)

## Inspection And Debugging

- `transition.diagnostics().requestedSource`
- `transition.diagnostics().visibleChangeSource`
- `prefetch.resourceNames()`

## Anti-Patterns

- rebuilding prefetch transitions from raw hrefs when you already have the
  prefetched artifact
- assuming prefetch admission always means immediate visible switch

## Current Limits

- the target must be a valid prefetched route artifact
- trigger posture rules still come from the route-resource declaration

## Related Docs

- [Transition Artifacts](./transition_artifacts.md)
- [Route Resources](../resources/README.md)
