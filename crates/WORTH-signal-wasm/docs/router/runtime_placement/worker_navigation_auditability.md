# Worker Navigation Auditability

## What This Feature Is

Worker navigation auditability is the worker-side summary of current visible
route truth, boundary source, coherence, and replay or restore availability.

## Why You Use It

- explain visible route truth from the worker side
- compare worker-side boundary stories to the main runtime
- debug coherence drift and not-admitted boundary events

## Stable Entry Points

- `story.auditability()`
- `story.inspection()`

## Core Mental Model

Worker auditability is the same kind of explanation surface as main-runtime
auditability, but it must stay honest about the thinner data the worker bridge
may actually have.

## How It Executes

1. record worker ingress and writeback boundary reports
2. retain route-history entries where possible
3. summarize current route truth, latest boundary truth, and coherence counts

## Small Example

```ts
const auditability = story.auditability();
console.log(auditability.summary());
```

## Real Example

```ts
const summary = story.auditability().summary();

console.log(summary.currentBoundaryArtifact);
console.log(summary.currentCoherenceKind);
console.log(summary.driftedBoundaryEventCount);
```

## How It Relates To Other Features

- full main-runtime explanation is documented in
  [Navigation Auditability](../history/navigation_auditability.md)
- worker fallback history is in [Worker History Fallback](./worker_history_fallback.md)

## Inspection And Debugging

- `story.auditability().summary()`
- `story.inspection().summary()`
- `story.currentRouteTruthEvent()`

## Anti-Patterns

- assuming worker auditability implies full route-layout authority
- hiding worker fallback categories behind generic “synced” labels

## Current Limits

- worker auditability can fail closed to weaker availability than the main
  runtime

## Related Docs

- [Host And Worker Boundary](./host_worker_boundary.md)
- [Navigation Auditability](../history/navigation_auditability.md)
