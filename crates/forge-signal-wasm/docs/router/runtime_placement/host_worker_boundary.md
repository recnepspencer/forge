# Host And Worker Boundary

## What This Feature Is

This is the router boundary where host-owned browser and capability events are
admitted into the worker runtime bridge.

## Why You Use It

- keep worker-side route truth synchronized with host-owned events
- understand what the worker bridge really guarantees
- avoid assuming the worker sees richer route structure than the bridge carries

## Stable Entry Points

- `bridge.admitBrowserHistoryIngress(...)`
- `bridge.applyBrowserHistoryWriteback(...)`
- `bridge.browserHistoryStory(...)`

## Core Mental Model

The worker bridge mirrors the public router contract. It should preserve the
same semantic categories where the public types already promise them and fail
closed where richer route truth is unavailable.

## How It Executes

1. the host creates browser ingress or writeback artifacts
2. the bridge normalizes them into worker-safe boundary reports
3. worker-side history story, inspection, and auditability consume those
   reports

## Small Example

```ts
const report = await bridge.admitBrowserHistoryIngress(
  {
    navigationKind: "load",
    rawLocation: "/",
    routeIdentity: "homeRoute",
  },
);
```

## Real Example

```ts
const story = bridge.browserHistoryStory(homeIngress);
story.record(localWriteback);
story.record(crossTabDrift);
story.record(externalMiss);

console.log(story.inspection().summary());
console.log(story.auditability().summary());
```

## How It Relates To Other Features

- browser authority categories are defined in
  [Browser Authority Coherence](../boundaries/browser_authority_coherence.md)
- fallback breadcrumb behavior is covered in [Worker History Fallback](./worker_history_fallback.md)

## Inspection And Debugging

- `report.diagnostics()`
- `story.events()`
- `story.inspection()`
- `story.auditability()`

## Anti-Patterns

- assuming the worker bridge always has full declared breadcrumb capability
- overclaiming coherence or restore authority that the bridge did not actually
  carry

## Current Limits

- some worker-side artifacts remain thinner than the main runtime and expose
  fallback truth instead of richer route structure

## Related Docs

- [Worker History Fallback](./worker_history_fallback.md)
- [Worker Navigation Auditability](./worker_navigation_auditability.md)
