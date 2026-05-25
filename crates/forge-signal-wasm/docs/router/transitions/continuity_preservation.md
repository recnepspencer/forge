# Continuity Preservation

## What This Feature Is

Continuity preservation is the transition lane where the router keeps visible
target truth alive through native route-resource continuity.

## Why You Use It

- let users see the target route while its resources refresh
- preserve visible target value without hiding pending state
- keep continuity on the resource substrate instead of in ad hoc UI memory

## Stable Entry Points

- `transition.diagnostics().visibleChangeSource`
- `transition.diagnostics().pendingResourceNames`
- `continuity: "preserve-visible-while-pending"`

## Core Mental Model

This lane is not "pretend nothing is loading." It is "target resource already
has visible truth, so the router can keep that visible while the line refreshes
in pending state."

## How It Executes

1. warm or prefetch target route resources
2. let the target resource line enter `preservedVisibleValue`
3. transition with continuity preservation enabled
4. surface `resourceContinuityPreservation` as the visible change source

## Small Example

```ts
const transition = await routes.transition(home, prefetched, {
  continuity: "preserve-visible-while-pending",
});
```

## Real Example

```ts
const prefetched = routes.project("/users/user-2")?.prefetch("hover");
const prefetchedLine = prefetched?.resource("detail").line();
prefetchedLine?.refresh();

const transition = await routes.transition(home, prefetched, {
  continuity: "preserve-visible-while-pending",
});

console.log(transition.diagnostics().visibleChangeSource);
console.log(transition.diagnostics().pendingResourceNames);
```

## How It Relates To Other Features

- this depends on route resources from [Route Resources](../resources/README.md)
- target transition posture still surfaces through
  [Transition Artifacts](./transition_artifacts.md)

## Inspection And Debugging

- `transition.diagnostics().visibleChangeSource`
- `transition.diagnostics().pendingResourceNames`
- `resource.current().diagnosticsSummary.activity.continuity`

## Anti-Patterns

- materializing skipped warmup resources just to explain visibility
- using continuity preservation as a generic cache story

## Current Limits

- continuity preservation depends on real visible target resource value already
  being present

## Related Docs

- [Pending Visibility](./pending_visibility.md)
- [Route Resources](../resources/README.md)
